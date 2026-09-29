"""Build public language packages from the canonical ProtoInfo sources."""

load("@protobuf//bazel/common:proto_info.bzl", "ProtoInfo")

def _schema_workspace_impl(ctx):
    protos = [target[ProtoInfo] for target in ctx.attr.protos]
    sources = depset(transitive = [proto.transitive_sources for proto in protos])
    paths = depset(transitive = [proto.transitive_proto_path for proto in protos])
    output = ctx.actions.declare_directory(ctx.label.name)
    arguments = ctx.actions.args()
    arguments.add("--output", output.path)
    arguments.add("--configuration", ctx.file.configuration.path)
    arguments.add_all(paths, before_each = "--proto-path")
    arguments.add_all(sources, before_each = "--source")
    ctx.actions.run(
        executable = ctx.executable._builder,
        arguments = [arguments],
        inputs = depset([ctx.file.configuration], transitive = [sources]),
        outputs = [output],
        mnemonic = "PublicSchemaWorkspace",
    )
    return [DefaultInfo(files = depset([output]), runfiles = ctx.runfiles(files = [output]))]

schema_workspace = rule(
    implementation = _schema_workspace_impl,
    attrs = {
        "configuration": attr.label(allow_single_file = True, mandatory = True),
        "protos": attr.label_list(providers = [ProtoInfo], mandatory = True),
        "_builder": attr.label(default = "//tools:assemble_schema", executable = True, cfg = "exec"),
    },
)

def _api_packages_impl(ctx):
    protos = [target[ProtoInfo] for target in ctx.attr.protos]
    sources = depset(transitive = [proto.transitive_sources for proto in protos])
    output = ctx.actions.declare_directory(ctx.label.name)
    arguments = ctx.actions.args()
    for proto in protos:
        for source in proto.direct_sources:
            arguments.add("--schema", source.path.removeprefix(proto.proto_source_root + "/"))
    arguments.add("--schema-workspace", ctx.file.schema_workspace.path)
    arguments.add("--plugin", ctx.executable.plugin.path)
    arguments.add("--templates", ctx.files.templates[0].dirname)
    arguments.add("--output", output.path)
    arguments.add("--version", ctx.file.version.path)
    arguments.add_all(ctx.files.asyncapis, before_each = "--asyncapi")
    arguments.add_all(ctx.files.license_files, before_each = "--license-file")
    ctx.actions.run(
        executable = ctx.executable._builder,
        arguments = [arguments],
        inputs = depset(ctx.files.templates + ctx.files.license_files + ctx.files.asyncapis + [ctx.file.version, ctx.file.schema_workspace], transitive = [sources]),
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
        "asyncapis": attr.label_list(allow_files = True, mandatory = True),
        "license_files": attr.label_list(allow_files = True),
        "plugin": attr.label(executable = True, cfg = "exec", mandatory = True),
        "protos": attr.label_list(providers = [ProtoInfo], mandatory = True),
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
