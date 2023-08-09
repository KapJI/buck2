/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::cell::RefCell;

use allocative::Allocative;
use buck2_core::cells::cell_path::CellPath;
use buck2_core::plugins::PluginKind;
use derive_more::Display;
use dupe::Dupe;
use starlark::environment::GlobalsBuilder;
use starlark::environment::Methods;
use starlark::environment::MethodsBuilder;
use starlark::environment::MethodsStatic;
use starlark::eval::Evaluator;
use starlark::starlark_module;
use starlark::starlark_simple_value;
use starlark::values::starlark_value;
use starlark::values::AllocValue;
use starlark::values::Freeze;
use starlark::values::Freezer;
use starlark::values::Heap;
use starlark::values::NoSerialize;
use starlark::values::ProvidesStaticType;
use starlark::values::StarlarkValue;
use starlark::values::Trace;
use starlark::values::Value;
use starlark::StarlarkDocs;

use crate::interpreter::build_context::BuildContext;

#[derive(Debug, derive_more::Display, Allocative)]
enum InnerStarlarkPluginKind {
    #[display(fmt = "<plugin_kind <unbound>>")]
    Unbound(CellPath),
    #[display(fmt = "<plugin_kind _0>")]
    Bound(PluginKind),
}

/// A kind of plugin, created via `plugins.kind()`.
///
/// These are used when declaring rules or attributes that make use of plugin deps.
#[derive(
    derive_more::Display,
    Debug,
    ProvidesStaticType,
    NoSerialize,
    Trace,
    Allocative
)]
#[display(fmt = "{}", "RefCell::borrow(_0)")]
pub struct StarlarkPluginKind(RefCell<InnerStarlarkPluginKind>);

#[starlark_value(type = "PluginKind")]
impl<'v> StarlarkValue<'v> for StarlarkPluginKind {
    fn export_as(&self, variable_name: &str, _eval: &mut Evaluator<'v, '_>) {
        let mut inner = self.0.borrow_mut();
        let InnerStarlarkPluginKind::Unbound(cell_path) = &*inner else {
            // Was already exported
            return;
        };
        let cell_path = cell_path.clone();
        let kind = PluginKind::new(variable_name.to_owned(), cell_path);
        *inner = InnerStarlarkPluginKind::Bound(kind);
    }
}

#[derive(Debug, thiserror::Error)]
enum PluginKindError {
    #[error("Plugin kind has not yet been assigned to a global")]
    NotBound,
}

impl StarlarkPluginKind {
    pub fn expect_bound(&self) -> anyhow::Result<PluginKind> {
        match &*self.0.borrow() {
            InnerStarlarkPluginKind::Unbound(_) => Err(PluginKindError::NotBound.into()),
            InnerStarlarkPluginKind::Bound(kind) => Ok(kind.dupe()),
        }
    }
}

impl<'v> AllocValue<'v> for StarlarkPluginKind {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_complex(self)
    }
}

#[derive(
    derive_more::Display,
    Debug,
    ProvidesStaticType,
    NoSerialize,
    Allocative
)]
#[display(fmt = "{_0}")]
pub struct FrozenStarlarkPluginKind(PluginKind);
starlark_simple_value!(FrozenStarlarkPluginKind);

#[starlark_value(type = "PluginKind")]
impl<'v> StarlarkValue<'v> for FrozenStarlarkPluginKind {}

impl Freeze for StarlarkPluginKind {
    type Frozen = FrozenStarlarkPluginKind;
    fn freeze(self, _: &Freezer) -> anyhow::Result<Self::Frozen> {
        self.expect_bound().map(FrozenStarlarkPluginKind)
    }
}

#[starlark_module]
fn plugins_module(registry: &mut MethodsBuilder) {
    /// Create a new plugin kind.
    ///
    /// The value returned should always be immediately bound to a global, like `MyPluginKind =
    /// plugins.kind()`
    fn kind<'v>(
        #[starlark(this)] _this: Value<'v>,
        eval: &mut Evaluator<'v, '_>,
    ) -> anyhow::Result<StarlarkPluginKind> {
        let cell_path = BuildContext::from_context(eval)?
            .starlark_path()
            .path()
            .into_owned();
        Ok(StarlarkPluginKind(RefCell::new(
            InnerStarlarkPluginKind::Unbound(cell_path),
        )))
    }
}

#[derive(
    Display,
    Debug,
    StarlarkDocs,
    Allocative,
    ProvidesStaticType,
    NoSerialize
)]
#[display(fmt = "<plugins>")]
struct Plugins;

#[starlark_value(type = "plugins")]
impl<'v> StarlarkValue<'v> for Plugins {
    fn get_methods() -> Option<&'static Methods> {
        static RES: MethodsStatic = MethodsStatic::new();
        RES.methods(plugins_module)
    }
}

pub fn register_plugins(globals: &mut GlobalsBuilder) {
    globals.set("plugins", globals.frozen_heap().alloc_simple(Plugins));
}
