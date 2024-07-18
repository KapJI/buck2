/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

#![feature(error_generic_member_access)]

mod lint;
mod typecheck;
mod util;

use async_trait::async_trait;
use buck2_cli_proto::ClientContext;
use buck2_events::dispatch::span_async;
use buck2_server_ctx::command_end::command_end;
use buck2_server_ctx::ctx::ServerCommandContextTrait;
use buck2_server_ctx::partial_result_dispatcher::PartialResultDispatcher;
use buck2_starlark::StarlarkOpaqueCommand;

#[async_trait]
pub(crate) trait StarlarkOpaqueSubcommand: Send + Sync + 'static {
    async fn server_execute(
        &self,
        server_ctx: &dyn ServerCommandContextTrait,
        stdout: PartialResultDispatcher<buck2_cli_proto::StdoutBytes>,
        client_server_ctx: ClientContext,
    ) -> anyhow::Result<()>;
}

pub async fn server_starlark_command(
    ctx: &dyn ServerCommandContextTrait,
    partial_result_dispatcher: PartialResultDispatcher<buck2_cli_proto::StdoutBytes>,
    req: buck2_cli_proto::GenericRequest,
) -> anyhow::Result<buck2_cli_proto::GenericResponse> {
    let start_event = buck2_data::CommandStart {
        metadata: ctx.request_metadata().await?,
        data: Some(buck2_data::StarlarkCommandStart {}.into()),
    };

    span_async(
        start_event,
        server_starlark_command_inner(ctx, partial_result_dispatcher, req),
    )
    .await
}

async fn server_starlark_command_inner(
    context: &dyn ServerCommandContextTrait,
    partial_result_dispatcher: PartialResultDispatcher<buck2_cli_proto::StdoutBytes>,
    req: buck2_cli_proto::GenericRequest,
) -> (
    anyhow::Result<buck2_cli_proto::GenericResponse>,
    buck2_data::CommandEnd,
) {
    let result = parse_command_and_execute(context, partial_result_dispatcher, req)
        .await
        .map_err(Into::into);
    let end_event = command_end(&result, buck2_data::StarlarkCommandEnd {});

    let result = result
        .map(|()| buck2_cli_proto::GenericResponse {})
        .map_err(Into::into);

    (result, end_event)
}

async fn parse_command_and_execute(
    context: &dyn ServerCommandContextTrait,
    partial_result_dispatcher: PartialResultDispatcher<buck2_cli_proto::StdoutBytes>,
    req: buck2_cli_proto::GenericRequest,
) -> anyhow::Result<()> {
    let command: StarlarkOpaqueCommand = serde_json::from_str(&req.serialized_opts)?;
    as_server_subcommand(&command)
        .server_execute(
            context,
            partial_result_dispatcher,
            req.context.expect("buck cli always sets a client context"),
        )
        .await
}

fn as_server_subcommand(cmd: &StarlarkOpaqueCommand) -> &dyn StarlarkOpaqueSubcommand {
    match cmd {
        StarlarkOpaqueCommand::Lint(cmd) => cmd,
        StarlarkOpaqueCommand::Typecheck(cmd) => cmd,
    }
}