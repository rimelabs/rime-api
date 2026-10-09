# Repository instructions

- Use Bazel for builds and tests. Run commands from this repository.
- Use uv for Python dependencies. Update `uv.lock` and `requirements.txt` together.
- Use the pnpm version in `package.json` for Node.js dependencies.
- Run `bazel run //:format` after changes and `bazel test //tests/...` for package or sync changes.
- The files listed in `copy.bara.sky` come from `rimelabs/rime`. Do not edit or format them here. Change the source and sync it.
- Keep `schema/BUILD.bazel` and `schema/rime/BUILD.bazel` here. Copybara does not own them.
- Keep release versions in `VERSION`. Python/TypeScript generated code and archives stay in Bazel outputs. Go and Rust protocol files are copied from Bazel outputs with `bazel run //:update_go` and `bazel run //:update_rust` and committed for publication; never edit them by hand. Use `bazel run //:cargo -- ...` for Cargo checks and packaging with the pinned toolchain.
- Use full descriptive names. Standard terms such as API, URL, and JSON are acceptable.
- Use ASD-STE100 Simplified Technical English in user communication.
