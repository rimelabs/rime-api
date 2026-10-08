"""Build reproducible Go protocol sources with pinned compiler plugins."""

def _go_protocol_impl(ctx):
    output = ctx.actions.declare_directory(ctx.label.name)
    arguments = ctx.actions.args()
    arguments.add("--fixtures", ctx.file.fixtures.path)
    arguments.add("--workspace", ctx.file.schema_workspace.path)
    arguments.add("--output", output.path)
    arguments.add("--go-plugin", ctx.executable.go_plugin.path)
    arguments.add("--grpc-plugin", ctx.executable.grpc_plugin.path)
    arguments.add_all(ctx.files.licenses, before_each = "--license")
    arguments.add_all(ctx.files.asyncapis, before_each = "--asyncapi")
    ctx.actions.run(
        executable = ctx.executable._builder,
        arguments = [arguments],
        inputs = [ctx.file.schema_workspace, ctx.file.fixtures] + ctx.files.licenses + ctx.files.asyncapis,
        tools = [ctx.attr.go_plugin[DefaultInfo].files_to_run, ctx.attr.grpc_plugin[DefaultInfo].files_to_run],
        outputs = [output],
        mnemonic = "GoProtocol",
    )
    return [DefaultInfo(files = depset([output]), runfiles = ctx.runfiles(files = [output]))]

go_protocol = rule(
    implementation = _go_protocol_impl,
    attrs = {
        "fixtures": attr.label(allow_single_file = True, mandatory = True),
        "schema_workspace": attr.label(allow_single_file = True, mandatory = True),
        "go_plugin": attr.label(executable = True, cfg = "exec", mandatory = True),
        "grpc_plugin": attr.label(executable = True, cfg = "exec", mandatory = True),
        "licenses": attr.label_list(allow_files = True),
        "asyncapis": attr.label_list(allow_files = True),
        "_builder": attr.label(default = "//tools:build_go", executable = True, cfg = "exec"),
    },
)
