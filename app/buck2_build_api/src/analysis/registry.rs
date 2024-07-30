/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

use allocative::Allocative;
use buck2_artifact::actions::key::ActionKey;
use buck2_artifact::artifact::artifact_type::Artifact;
use buck2_artifact::artifact::artifact_type::DeclaredArtifact;
use buck2_artifact::artifact::artifact_type::OutputArtifact;
use buck2_artifact::artifact::build_artifact::BuildArtifact;
use buck2_artifact::deferred::id::DeferredId;
use buck2_artifact::deferred::key::DeferredHolderKey;
use buck2_core::base_deferred_key::BaseDeferredKey;
use buck2_core::execution_types::execution::ExecutionPlatformResolution;
use buck2_core::fs::paths::forward_rel_path::ForwardRelativePath;
use buck2_core::fs::paths::forward_rel_path::ForwardRelativePathBuf;
use buck2_error::internal_error;
use buck2_error::BuckErrorContext;
use buck2_execute::execute::request::OutputType;
use derivative::Derivative;
use dupe::Dupe;
use indexmap::IndexSet;
use starlark::any::ProvidesStaticType;
use starlark::codemap::FileSpan;
use starlark::environment::FrozenModule;
use starlark::environment::Module;
use starlark::eval::Evaluator;
use starlark::values::starlark_value;
use starlark::values::typing::FrozenStarlarkCallable;
use starlark::values::typing::StarlarkCallable;
use starlark::values::AllocValue;
use starlark::values::Freeze;
use starlark::values::Freezer;
use starlark::values::FrozenHeap;
use starlark::values::FrozenHeapRef;
use starlark::values::FrozenValue;
use starlark::values::FrozenValueTyped;
use starlark::values::Heap;
use starlark::values::NoSerialize;
use starlark::values::OwnedFrozenValue;
use starlark::values::OwnedFrozenValueTyped;
use starlark::values::StarlarkValue;
use starlark::values::Trace;
use starlark::values::Tracer;
use starlark::values::Value;
use starlark::values::ValueTyped;
use starlark_map::small_map::SmallMap;

use crate::actions::registry::ActionsRegistry;
use crate::actions::registry::RecordedActions;
use crate::actions::RegisteredAction;
use crate::actions::UnregisteredAction;
use crate::analysis::anon_promises_dyn::AnonPromisesDyn;
use crate::analysis::anon_targets_registry::AnonTargetsRegistryDyn;
use crate::analysis::anon_targets_registry::ANON_TARGET_REGISTRY_NEW;
use crate::analysis::extra_v::AnalysisExtraValue;
use crate::analysis::extra_v::FrozenAnalysisExtraValue;
use crate::artifact_groups::deferred::TransitiveSetIndex;
use crate::artifact_groups::deferred::TransitiveSetKey;
use crate::artifact_groups::promise::PromiseArtifact;
use crate::artifact_groups::promise::PromiseArtifactId;
use crate::artifact_groups::registry::ArtifactGroupRegistry;
use crate::artifact_groups::ArtifactGroup;
use crate::deferred::calculation::ActionLookup;
use crate::deferred::types::DeferredRegistry;
use crate::dynamic::registry::DynamicRegistryDyn;
use crate::dynamic::registry::DYNAMIC_REGISTRY_NEW;
use crate::interpreter::rule_defs::artifact::associated::AssociatedArtifacts;
use crate::interpreter::rule_defs::artifact::output_artifact_like::OutputArtifactArg;
use crate::interpreter::rule_defs::artifact::starlark_declared_artifact::StarlarkDeclaredArtifact;
use crate::interpreter::rule_defs::transitive_set::FrozenTransitiveSet;
use crate::interpreter::rule_defs::transitive_set::FrozenTransitiveSetDefinition;
use crate::interpreter::rule_defs::transitive_set::TransitiveSet;

#[derive(Derivative, Trace, Allocative)]
#[derivative(Debug)]
pub struct AnalysisRegistry<'v> {
    #[derivative(Debug = "ignore")]
    deferred: DeferredRegistry,
    #[derivative(Debug = "ignore")]
    actions: ActionsRegistry,
    #[derivative(Debug = "ignore")]
    artifact_groups: ArtifactGroupRegistry,
    #[derivative(Debug = "ignore")]
    dynamic: Box<dyn DynamicRegistryDyn>,
    pub anon_targets: Box<dyn AnonTargetsRegistryDyn<'v>>,
    analysis_value_storage: AnalysisValueStorage<'v>,
    pub short_path_assertions: HashMap<PromiseArtifactId, ForwardRelativePathBuf>,
}

#[derive(buck2_error::Error, Debug)]
enum DeclaredArtifactError {
    #[error("Can't declare an artifact with an empty filename component")]
    DeclaredEmptyFileName,
}

impl<'v> AnalysisRegistry<'v> {
    pub fn new_from_owner(
        owner: BaseDeferredKey,
        execution_platform: ExecutionPlatformResolution,
    ) -> anyhow::Result<AnalysisRegistry<'v>> {
        Self::new_from_owner_and_deferred(
            owner.dupe(),
            execution_platform,
            DeferredRegistry::new(DeferredHolderKey::Base(owner)),
        )
    }

    pub fn new_from_owner_and_deferred(
        owner: BaseDeferredKey,
        execution_platform: ExecutionPlatformResolution,
        deferred: DeferredRegistry,
    ) -> anyhow::Result<Self> {
        Ok(AnalysisRegistry {
            deferred,
            actions: ActionsRegistry::new(owner.dupe(), execution_platform.dupe()),
            artifact_groups: ArtifactGroupRegistry::new(),
            dynamic: (DYNAMIC_REGISTRY_NEW.get()?)(owner.dupe()),
            anon_targets: (ANON_TARGET_REGISTRY_NEW.get()?)(PhantomData, execution_platform),
            analysis_value_storage: AnalysisValueStorage::new(),
            short_path_assertions: HashMap::new(),
        })
    }

    pub fn set_action_key(&mut self, action_key: Arc<str>) {
        self.actions.set_action_key(action_key);
    }

    /// Reserves a path in an output directory. Doesn't declare artifact,
    /// but checks that there is no previously declared artifact with a path
    /// which is in conflict with claimed `path`.
    pub fn claim_output_path(
        &mut self,
        eval: &Evaluator<'_, '_, '_>,
        path: &ForwardRelativePath,
    ) -> anyhow::Result<()> {
        let declaration_location = eval.call_stack_top_location();
        self.actions.claim_output_path(path, declaration_location)
    }

    pub fn declare_dynamic_output(
        &mut self,
        artifact: &BuildArtifact,
    ) -> anyhow::Result<DeclaredArtifact> {
        self.actions.declare_dynamic_output(artifact)
    }

    pub fn declare_output(
        &mut self,
        prefix: Option<&str>,
        filename: &str,
        output_type: OutputType,
        declaration_location: Option<FileSpan>,
    ) -> anyhow::Result<DeclaredArtifact> {
        // We want this artifact to be a file/directory inside the current context, which means
        // things like `..` and the empty path `.` can be bad ideas. The `::new` method checks for those
        // things and fails if they are present.

        if filename == "." || filename.is_empty() {
            return Err(DeclaredArtifactError::DeclaredEmptyFileName.into());
        }

        let path = ForwardRelativePath::new(filename)?.to_owned();
        let prefix = match prefix {
            None => None,
            Some(x) => Some(ForwardRelativePath::new(x)?.to_owned()),
        };
        self.actions
            .declare_artifact(prefix, path, output_type, declaration_location)
    }

    /// Takes a string or artifact/output artifact and converts it into an output artifact
    ///
    /// This is handy for functions like `ctx.actions.write` where it's nice to just let
    /// the user give us a string if they want as the output name.
    ///
    /// This function can declare new artifacts depending on the input.
    /// If there is no error, it returns a wrapper around the artifact (ArtifactDeclaration) and the corresponding OutputArtifact
    ///
    /// The valid types for `value` and subsequent actions are as follows:
    ///  - `str`: A new file is declared with this name.
    ///  - `StarlarkOutputArtifact`: The original artifact is returned
    ///  - `StarlarkArtifact`/`StarlarkDeclaredArtifact`: If the artifact is already bound, an error is raised. Otherwise we proceed with the original artifact.
    pub fn get_or_declare_output<'v2>(
        &mut self,
        eval: &Evaluator<'v2, '_, '_>,
        value: OutputArtifactArg<'v2>,
        output_type: OutputType,
    ) -> anyhow::Result<(ArtifactDeclaration<'v2>, OutputArtifact)> {
        let declaration_location = eval.call_stack_top_location();
        let heap = eval.heap();
        let declared_artifact = match value {
            OutputArtifactArg::Str(path) => {
                let artifact =
                    self.declare_output(None, path, output_type, declaration_location.dupe())?;
                heap.alloc_typed(StarlarkDeclaredArtifact::new(
                    declaration_location,
                    artifact,
                    AssociatedArtifacts::new(),
                ))
            }
            OutputArtifactArg::OutputArtifact(output) => output.inner(),
            OutputArtifactArg::DeclaredArtifact(artifact) => artifact,
            OutputArtifactArg::WrongArtifact(artifact) => {
                return Err(artifact.0.as_output_error());
            }
        };

        let output = declared_artifact.output_artifact();
        output.ensure_output_type(output_type)?;
        Ok((
            ArtifactDeclaration {
                artifact: declared_artifact,
                heap,
            },
            output,
        ))
    }

    pub fn register_action<A: UnregisteredAction + 'static>(
        &mut self,
        inputs: IndexSet<ArtifactGroup>,
        outputs: IndexSet<OutputArtifact>,
        action: A,
        associated_value: Option<Value<'v>>,
        error_handler: Option<StarlarkCallable<'v>>,
    ) -> anyhow::Result<()> {
        let id = self
            .actions
            .register(&mut self.deferred, inputs, outputs, action)?;
        self.analysis_value_storage
            .set_action_data(id, (associated_value, error_handler));
        Ok(())
    }

    pub fn create_transitive_set(
        &mut self,
        definition: FrozenValueTyped<'v, FrozenTransitiveSetDefinition>,
        value: Option<Value<'v>>,
        children: Option<Value<'v>>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<ValueTyped<'v, TransitiveSet<'v>>> {
        self.artifact_groups.create_transitive_set(
            definition,
            value,
            children,
            &mut self.deferred,
            &mut self.analysis_value_storage,
            eval,
        )
    }

    pub fn register_dynamic_output(
        &mut self,
        dynamic: IndexSet<Artifact>,
        outputs: IndexSet<OutputArtifact>,
        attributes_plugins_lambda: Value<'v>,
    ) -> anyhow::Result<()> {
        let id = self
            .dynamic
            .register(dynamic, outputs, &mut self.deferred)?;
        self.analysis_value_storage
            .set_value(id, attributes_plugins_lambda);
        Ok(())
    }

    pub(crate) fn take_promises(&mut self) -> Option<Box<dyn AnonPromisesDyn<'v>>> {
        self.anon_targets.take_promises()
    }

    pub fn consumer_analysis_artifacts(&self) -> Vec<PromiseArtifact> {
        self.anon_targets.consumer_analysis_artifacts()
    }

    pub fn record_short_path_assertion(
        &mut self,
        short_path: ForwardRelativePathBuf,
        promise_artifact_id: PromiseArtifactId,
    ) {
        self.short_path_assertions
            .insert(promise_artifact_id, short_path);
    }

    pub fn assert_no_promises(&self) -> anyhow::Result<()> {
        self.anon_targets.assert_no_promises()
    }

    pub fn num_declared_actions(&self) -> u64 {
        self.actions.actions_len() as u64
    }

    pub fn num_declared_artifacts(&self) -> u64 {
        self.actions.artifacts_len() as u64
    }

    /// You MUST pass the same module to both the first function and the second one.
    /// It requires both to get the lifetimes to line up.
    pub fn finalize(
        self,
        env: &'v Module,
    ) -> anyhow::Result<
        impl FnOnce(Module) -> anyhow::Result<(FrozenModule, DeferredRegistry)> + 'static,
    > {
        let AnalysisRegistry {
            mut deferred,
            dynamic,
            actions,
            artifact_groups,
            anon_targets: _,
            analysis_value_storage,
            short_path_assertions: _,
        } = self;

        analysis_value_storage.write_to_module(env)?;
        Ok(move |env: Module| {
            let frozen_env = env.freeze()?;
            let analysis_value_fetcher = AnalysisValueFetcher {
                frozen_module: Some(frozen_env.dupe()),
            };
            let actions = actions.ensure_bound(&mut deferred, &analysis_value_fetcher)?;
            artifact_groups.ensure_bound(&mut deferred, &analysis_value_fetcher)?;
            dynamic.ensure_bound(&mut deferred, &analysis_value_fetcher)?;
            deferred.register_values(analysis_value_fetcher.get_recorded_values(actions)?)?;
            Ok((frozen_env, deferred))
        })
    }

    pub fn execution_platform(&self) -> &ExecutionPlatformResolution {
        self.actions.execution_platform()
    }
}

pub struct ArtifactDeclaration<'v> {
    artifact: ValueTyped<'v, StarlarkDeclaredArtifact>,
    heap: &'v Heap,
}

impl<'v> ArtifactDeclaration<'v> {
    pub fn into_declared_artifact(
        self,
        extra_associated_artifacts: AssociatedArtifacts,
    ) -> ValueTyped<'v, StarlarkDeclaredArtifact> {
        self.heap.alloc_typed(
            self.artifact
                .with_extended_associated_artifacts(extra_associated_artifacts),
        )
    }
}

/// Store `Value<'v>` values for actions registered in an implementation function
///
/// Threading lifetimes through the various action registries is kind of a pain. So instead,
/// store the starlark values in this struct, using the `DeferredId` as the key.
///
/// These values eventually are written into the mutable `Module`, and a wrapper is
/// made available to get the `OwnedFrozenValue` back out after that `Module` is frozen.
///
/// Note that this object has internal mutation and is only expected to live for the duration
/// of impl function execution.
///
/// At the end of impl function execution, `write_to_module` should be called
/// to write this object to `Module` extra value to get the values frozen.
#[derive(
    Debug,
    Allocative,
    derive_more::Display,
    ProvidesStaticType,
    NoSerialize
)]
#[display(fmt = "{:?}", "self")]
pub(crate) struct AnalysisValueStorage<'v> {
    values: SmallMap<DeferredId, Value<'v>>,
    action_data: SmallMap<ActionKey, (Option<Value<'v>>, Option<StarlarkCallable<'v>>)>,
    transitive_sets: SmallMap<TransitiveSetKey, ValueTyped<'v, TransitiveSet<'v>>>,
}

#[derive(
    Debug,
    Allocative,
    derive_more::Display,
    ProvidesStaticType,
    NoSerialize
)]
#[display(fmt = "{:?}", "self")]
pub(crate) struct FrozenAnalysisValueStorage {
    values: SmallMap<DeferredId, FrozenValue>,
    action_data: SmallMap<ActionKey, (Option<FrozenValue>, Option<FrozenStarlarkCallable>)>,
    transitive_sets: SmallMap<TransitiveSetKey, FrozenValueTyped<'static, FrozenTransitiveSet>>,
}

impl<'v> AllocValue<'v> for AnalysisValueStorage<'v> {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_complex(self)
    }
}

unsafe impl<'v> Trace<'v> for AnalysisValueStorage<'v> {
    fn trace(&mut self, tracer: &Tracer<'v>) {
        let AnalysisValueStorage {
            values,
            action_data,
            transitive_sets,
        } = self;
        for (k, v) in values.iter_mut() {
            tracer.trace_static(k);
            v.trace(tracer);
        }
        for (k, v) in action_data.iter_mut() {
            tracer.trace_static(k);
            v.trace(tracer);
        }
        for (k, v) in transitive_sets.iter_mut() {
            tracer.trace_static(k);
            v.trace(tracer);
        }
    }
}

impl<'v> Freeze for AnalysisValueStorage<'v> {
    type Frozen = FrozenAnalysisValueStorage;

    fn freeze(self, freezer: &Freezer) -> anyhow::Result<Self::Frozen> {
        let AnalysisValueStorage {
            values,
            action_data,
            transitive_sets,
        } = self;

        Ok(FrozenAnalysisValueStorage {
            values: values
                .into_iter()
                .map(|(k, v)| Ok((k, v.freeze(freezer)?)))
                .collect::<anyhow::Result<_>>()?,
            action_data: action_data
                .into_iter()
                .map(|(k, v)| Ok((k, v.freeze(freezer)?)))
                .collect::<anyhow::Result<_>>()?,
            transitive_sets: transitive_sets
                .into_iter()
                .map(|(k, v)| Ok((k, FrozenValueTyped::new_err(v.to_value().freeze(freezer)?)?)))
                .collect::<anyhow::Result<_>>()?,
        })
    }
}

#[starlark_value(type = "AnalysisValueStorage")]
impl<'v> StarlarkValue<'v> for AnalysisValueStorage<'v> {}

#[starlark_value(type = "AnalysisValueStorage")]
impl<'v> StarlarkValue<'v> for FrozenAnalysisValueStorage {
    type Canonical = AnalysisValueStorage<'v>;
}

/// Simple fetcher that fetches the values written in `AnalysisValueStorage::write_to_module`
///
/// These values are pulled from the `FrozenModule` that results from `env.freeze()`.
/// This is used by the action registry to make an `OwnedFrozenValue` available to
/// Actions' register function.
#[derive(Default)]
pub struct AnalysisValueFetcher {
    frozen_module: Option<FrozenModule>,
}

impl<'v> AnalysisValueStorage<'v> {
    fn new() -> Self {
        Self {
            values: SmallMap::new(),
            action_data: SmallMap::new(),
            transitive_sets: SmallMap::new(),
        }
    }

    /// Write self to `module` extra value.
    fn write_to_module(self, module: &'v Module) -> anyhow::Result<()> {
        let extra_v = AnalysisExtraValue::get_or_init(module)?;
        let res = extra_v
            .analysis_value_storage
            .set(module.heap().alloc_typed(self));
        if res.is_err() {
            return Err(internal_error!("analysis_value_storage is already set"));
        }
        Ok(())
    }

    /// Add a value to the internal hash map that maps ids -> values
    pub fn set_value(&mut self, id: DeferredId, value: Value<'v>) {
        self.values.insert(id, value);
    }

    pub(crate) fn register_transitive_set<
        F: FnOnce(TransitiveSetKey) -> anyhow::Result<ValueTyped<'v, TransitiveSet<'v>>>,
    >(
        &mut self,
        self_key: DeferredHolderKey,
        func: F,
    ) -> anyhow::Result<ValueTyped<'v, TransitiveSet<'v>>> {
        let key = TransitiveSetKey::new(
            self_key,
            TransitiveSetIndex(self.transitive_sets.len().try_into()?),
        );
        let set = func(key.dupe())?;
        self.transitive_sets.insert(key, set.dupe());
        Ok(set)
    }

    fn set_action_data(
        &mut self,
        id: ActionKey,
        action_data: (Option<Value<'v>>, Option<StarlarkCallable<'v>>),
    ) {
        self.action_data.insert(id, action_data);
    }
}

impl AnalysisValueFetcher {
    fn extra_value(&self) -> anyhow::Result<Option<(&FrozenAnalysisValueStorage, &FrozenHeapRef)>> {
        match &self.frozen_module {
            None => Ok(None),
            Some(module) => {
                let analysis_extra_value = FrozenAnalysisExtraValue::get(module)?
                    .value
                    .analysis_value_storage
                    .internal_error("analysis_value_storage not set")?
                    .as_ref();
                Ok(Some((analysis_extra_value, module.frozen_heap())))
            }
        }
    }

    /// Get the `OwnedFrozenValue` that corresponds to a `DeferredId`, if present
    pub fn get(&self, id: DeferredId) -> anyhow::Result<Option<OwnedFrozenValue>> {
        let Some((storage, heap_ref)) = self.extra_value()? else {
            return Ok(None);
        };
        let Some(value) = storage.values.get(&id) else {
            return Ok(None);
        };
        unsafe { Ok(Some(OwnedFrozenValue::new(heap_ref.dupe(), *value))) }
    }

    /// Get the `OwnedFrozenValue` that corresponds to a `DeferredId`, if present
    pub fn get_action_data(
        &self,
        id: &ActionKey,
    ) -> anyhow::Result<(Option<OwnedFrozenValue>, Option<OwnedFrozenValue>)> {
        let Some((storage, heap_ref)) = self.extra_value()? else {
            return Ok((None, None));
        };
        let Some(value) = storage.action_data.get(id) else {
            return Ok((None, None));
        };

        unsafe {
            Ok((
                value.0.map(|v| OwnedFrozenValue::new(heap_ref.dupe(), v)),
                value.1.map(|v| OwnedFrozenValue::new(heap_ref.dupe(), v.0)),
            ))
        }
    }

    pub(crate) fn get_recorded_values(
        &self,
        actions: RecordedActions,
    ) -> anyhow::Result<RecordedAnalysisValues> {
        let analysis_storage = match &self.frozen_module {
            None => None,
            Some(module) => Some(FrozenAnalysisExtraValue::get(module)?.try_map(|v| {
                v.value
                    .analysis_value_storage
                    .internal_error("analysis_value_storage not set")
            })?),
        };

        Ok(RecordedAnalysisValues {
            analysis_storage,
            actions,
        })
    }
}

/// The analysis values stored in DeferredHolder.
#[derive(Debug, Allocative)]
pub struct RecordedAnalysisValues {
    analysis_storage: Option<OwnedFrozenValueTyped<FrozenAnalysisValueStorage>>,
    actions: RecordedActions,
}

impl RecordedAnalysisValues {
    pub fn new_empty() -> Self {
        Self {
            analysis_storage: None,
            actions: RecordedActions::new(),
        }
    }

    pub fn testing_new(
        transitive_sets: Vec<(TransitiveSetKey, OwnedFrozenValueTyped<FrozenTransitiveSet>)>,
        actions: RecordedActions,
    ) -> Self {
        let heap = FrozenHeap::new();
        let mut alloced_tsets = SmallMap::new();
        for (key, tset) in transitive_sets {
            heap.add_reference(tset.owner());
            let tset = tset.owned_frozen_value_typed(&heap);
            alloced_tsets.insert(key, tset);
        }

        let value = heap.alloc_simple(FrozenAnalysisValueStorage {
            values: SmallMap::new(),
            action_data: SmallMap::new(),
            transitive_sets: alloced_tsets,
        });
        Self {
            analysis_storage: Some(
                unsafe { OwnedFrozenValue::new(heap.into_ref(), value) }
                    .downcast()
                    .unwrap(),
            ),
            actions,
        }
    }

    pub(crate) fn lookup_transitive_set(
        &self,
        key: &TransitiveSetKey,
    ) -> Option<OwnedFrozenValueTyped<FrozenTransitiveSet>> {
        // TODO(cjhopman): verify that key matches this.
        match &self.analysis_storage {
            Some(values) => values.maybe_map(|v| v.transitive_sets.get(key).copied()),
            None => None,
        }
    }

    pub fn lookup_action(&self, key: &ActionKey) -> anyhow::Result<ActionLookup> {
        self.actions.lookup(key)
    }

    /// Iterates over the actions created in this analysis.
    pub fn iter_actions(&self) -> impl Iterator<Item = &Arc<RegisteredAction>> + '_ {
        self.actions.iter_actions()
    }
}
