"""Generate Rust protocol sources with the pinned Bazel toolchain."""

def _rust_protocol_impl(ctx):
    output = ctx.actions.declare_directory(ctx.label.name)
    ctx.actions.run(
        executable = ctx.executable.generator,
        arguments = [ctx.executable.protoc.path, ctx.file.schema_workspace.path + "/schema", output.path],
        inputs = [ctx.file.schema_workspace],
        tools = [ctx.attr.generator[DefaultInfo].files_to_run, ctx.attr.protoc[DefaultInfo].files_to_run],
        outputs = [output],
        mnemonic = "RustProtocol",
    )
    return [DefaultInfo(files = depset([output]), runfiles = ctx.runfiles(files = [output]))]

rust_protocol = rule(
    implementation = _rust_protocol_impl,
    attrs = {
        "schema_workspace": attr.label(allow_single_file = True, mandatory = True),
        "generator": attr.label(default = "//tools/rust:generator", executable = True, cfg = "exec"),
        "protoc": attr.label(default = "@protobuf//:protoc", executable = True, cfg = "exec"),
    },
)

def _rust_cargo_impl(ctx):
    toolchain = ctx.toolchains["@rules_rust//rust:toolchain_type"]
    executable = ctx.actions.declare_file(ctx.label.name)
    ctx.actions.write(
        executable,
        """#!/usr/bin/env bash
set -euo pipefail
runfiles="${RUNFILES_DIR:-$0.runfiles}/_main"
export RUSTC="$runfiles/%s"
export RUSTDOC="$runfiles/%s"
cargo="$runfiles/%s"
export PATH="$(dirname "$cargo"):$PATH"
cd "$BUILD_WORKSPACE_DIRECTORY"
exec "$cargo" "$@"
""" % (toolchain.rustc.short_path, toolchain.rust_doc.short_path, toolchain.cargo.short_path),
        is_executable = True,
    )
    return [DefaultInfo(executable = executable, runfiles = ctx.runfiles(transitive_files = toolchain.all_files))]

rust_cargo = rule(
    implementation = _rust_cargo_impl,
    executable = True,
    toolchains = ["@rules_rust//rust:toolchain_type"],
)
