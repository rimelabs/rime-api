"""Build public language packages from the canonical ProtoInfo sources."""

load("@protobuf//bazel/common:proto_info.bzl", "ProtoInfo")

def _schema_workspace_impl(ctx):
    proto = ctx.attr.proto[ProtoInfo]
    output = ctx.actions.declare_directory(ctx.label.name)
    arguments = ctx.actions.args()
    arguments.add("--output", output.path)
    arguments.add("--configuration", ctx.file.configuration.path)
    arguments.add_all(proto.transitive_proto_path, before_each = "--proto-path")
    arguments.add_all(proto.transitive_sources, before_each = "--source")
    ctx.actions.run(
        executable = ctx.executable._builder,
        arguments = [arguments],
        inputs = depset([ctx.file.configuration], transitive = [proto.transitive_sources]),
        outputs = [output],
        mnemonic = "PublicSchemaWorkspace",
    )
    return [DefaultInfo(files = depset([output]), runfiles = ctx.runfiles(files = [output]))]

schema_workspace = rule(
    implementation = _schema_workspace_impl,
    attrs = {
        "configuration": attr.label(allow_single_file = True, mandatory = True),
        "proto": attr.label(providers = [ProtoInfo], mandatory = True),
        "_builder": attr.label(default = "//tools:assemble_schema", executable = True, cfg = "exec"),
    },
)

def _api_packages_impl(ctx):
    proto = ctx.attr.proto[ProtoInfo]
    if len(proto.direct_sources) != 1:
        fail("The public TTS release must have exactly one entry-point schema")
    output = ctx.actions.declare_directory(ctx.label.name)
    arguments = ctx.actions.args()
    arguments.add("--source", proto.direct_sources[0].path)
    arguments.add("--proto-root", proto.proto_source_root)
    arguments.add_all(proto.transitive_proto_path, before_each = "--proto-path")
    arguments.add("--schema-workspace", ctx.file.schema_workspace.path)
    arguments.add("--plugin", ctx.executable.plugin.path)
    arguments.add("--templates", ctx.files.templates[0].dirname)
    arguments.add("--output", output.path)
    arguments.add("--version", ctx.file.version.path)
    arguments.add("--asyncapi", ctx.file.asyncapi.path)
    arguments.add_all(ctx.files.license_files, before_each = "--license-file")
    ctx.actions.run(
        executable = ctx.executable._builder,
        arguments = [arguments],
        inputs = depset(ctx.files.templates + ctx.files.license_files + [ctx.file.version, ctx.file.asyncapi, ctx.file.schema_workspace], transitive = [proto.transitive_sources]),
        tools = [ctx.attr.plugin[DefaultInfo].files_to_run],
        outputs = [output],
        env = {"SOURCE_DATE_EPOCH": "315532800", "BAZEL_BINDIR": ctx.bin_dir.path},
        mnemonic = "RimeApiPackages",
        progress_message = "Building Python and JavaScript protocol packages",
    )
    return [DefaultInfo(files = depset([output]), runfiles = ctx.runfiles(files = [output]))]

api_packages = rule(
    implementation = _api_packages_impl,
    attrs = {
        "asyncapi": attr.label(allow_single_file = True, mandatory = True),
        "license_files": attr.label_list(allow_files = True),
        "plugin": attr.label(executable = True, cfg = "exec", mandatory = True),
        "proto": attr.label(providers = [ProtoInfo], mandatory = True),
        "schema_workspace": attr.label(allow_single_file = True, mandatory = True),
        "templates": attr.label_list(allow_files = True),
        "version": attr.label(allow_single_file = True, mandatory = True),
        "_builder": attr.label(
            default = "//tools:build_packages",
            executable = True,
            cfg = "exec",
        ),
    },
)
