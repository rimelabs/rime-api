import { copyFileSync, unlinkSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, resolve } from "node:path";
import typescript from "typescript";

const [directory, source] = process.argv.slice(2);
const require = createRequire(import.meta.url);
for (const [format, module] of [
  ["esm", typescript.ModuleKind.ES2022],
  ["commonjs", typescript.ModuleKind.CommonJS],
]) {
  const filename = resolve(directory, format, "grpc.ts");
  copyFileSync(source, filename);
  const options = {
    strict: true,
    declaration: true,
    noEmitOnError: true,
    target: typescript.ScriptTarget.ES2022,
    module,
    moduleResolution: typescript.ModuleResolutionKind.Node10,
    types: ["node"],
    typeRoots: [dirname(dirname(require.resolve("@types/node/package.json")))],
    paths: Object.fromEntries(["@bufbuild/protobuf", "@bufbuild/protobuf/codegenv2", "@bufbuild/protobuf/wkt", "@grpc/grpc-js"].map(name => [
      name, [require.resolve(name).replace(/\.js$/, ".d.ts")],
    ])),
  };
  const program = typescript.createProgram([filename], options);
  const diagnostics = typescript.getPreEmitDiagnostics(program);
  if (diagnostics.length) {
    throw new Error(typescript.formatDiagnosticsWithColorAndContext(diagnostics, {
      getCanonicalFileName: path => path,
      getCurrentDirectory: () => process.cwd(),
      getNewLine: () => "\n",
    }));
  }
  if (program.emit().emitSkipped) throw new Error("gRPC client compilation failed");
  unlinkSync(filename);
}
