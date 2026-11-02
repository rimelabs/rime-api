import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { create, fromBinary, toBinary } from "@bufbuild/protobuf";
import { credentials, Metadata, Server, ServerCredentials, status } from "@grpc/grpc-js";

const require = createRequire(import.meta.url);
const packages = process.argv[2];
const deadline = () => ({ deadline: Date.now() + 5000 });
const authorization = new Metadata();
authorization.set("authorization", "Bearer test");

for (const format of ["esm", "commonjs"]) {
  const load = async (entry) => {
    if (!packages) return format === "esm" ? import(entry) : require(entry);
    const filename = resolve(packages, "javascript", format, entry.endsWith("/grpc") ? "grpc.js" : "index.js");
    return format === "esm" ? import(pathToFileURL(filename)) : require(filename);
  };
  const definitions = await load("@rimelabs/api");
  const { SpeechToTextClient, TextToSpeechClient } = await load("@rimelabs/api/grpc");
  const server = new Server();
  const service = (name, methods) => Object.fromEntries(methods.map(([method, input, output, requestStream, responseStream]) => [method[0].toLowerCase() + method.slice(1), {
    path: `/rime.${name}/${method}`,
    requestStream,
    responseStream,
    requestDeserialize: bytes => fromBinary(definitions[`${input}Schema`], bytes),
    responseSerialize: message => Buffer.from(toBinary(definitions[`${output}Schema`], create(definitions[`${output}Schema`], message))),
  }]));
  server.addService(service("SpeechToText", [
    ["Transcribe", "TranscriptionRequest", "TranscriptionResponse", false, false],
    ["TranscribeStreaming", "StreamingTranscriptionRequest", "StreamingTranscriptionResponse", true, true],
  ]), {
    transcribe(call, callback) {
      if (call.metadata.get("authorization")[0] !== "Bearer test") {
        callback({ code: status.UNAUTHENTICATED, details: "Missing authorization" });
      } else if (call.request.options?.language === "reject") {
        callback({ code: status.INVALID_ARGUMENT, details: "Unsupported language" });
      } else {
        const headers = new Metadata();
        headers.set("x-request-id", "test-request");
        call.sendMetadata(headers);
        callback(null, { text: Buffer.from(call.request.audio).toString() });
      }
    },
    transcribeStreaming(call) {
      call.on("data", request => call.write({ payload: request.payload.case === "config"
        ? { case: "accepted", value: {} }
        : { case: "delta", value: { text: Buffer.from(request.payload.value).toString() } },
      }));
      call.on("end", () => {
        call.write({ payload: { case: "done", value: { text: "done" } } });
        call.end();
      });
    },
  });
  server.addService(service("TextToSpeech", [
    ["Synthesize", "SynthesisRequest", "SynthesisResponseStream", false, true],
    ["SynthesizeStreaming", "StreamingSynthesisRequest", "SynthesisResponseStream", true, true],
    ["NormalizeText", "NormalizeTextRequest", "NormalizeTextResponse", false, false],
    ["GetSupportedLanguages", "GetSupportedLanguagesRequest", "GetSupportedLanguagesResponse", false, false],
    ["GetSupportedSpeakers", "GetSupportedSpeakersRequest", "GetSupportedSpeakersResponse", false, false],
  ]), {
    synthesize(call) {
      call.write({ payload: { case: "audio", value: Buffer.from(call.request.text) } });
      if (call.request.text !== "wait") {
        call.write({ payload: { case: "audio", value: Buffer.from("second") } });
        call.end();
      }
    },
    synthesizeStreaming(call) {
      call.on("data", request => call.write({ payload: { case: "audio", value: Buffer.from(
        request.payload.case === "header" ? request.payload.value.text : request.payload.value,
      ) } }));
      call.on("end", () => call.end());
    },
    normalizeText: (call, callback) => callback(null, { normalizedSentences: [call.request.text.toUpperCase()] }),
    getSupportedLanguages: (call, callback) => callback(null, { languages: ["en"] }),
    getSupportedSpeakers: (call, callback) => callback(null, { speakers: ["test"] }),
  });
  const port = await new Promise((resolve, reject) => server.bindAsync("127.0.0.1:0", ServerCredentials.createInsecure(), (error, port) => error ? reject(error) : resolve(port)));
  const recognition = new SpeechToTextClient(`127.0.0.1:${port}`, credentials.createInsecure());
  const synthesis = new TextToSpeechClient(`127.0.0.1:${port}`, credentials.createInsecure());
  const message = (name, values = {}) => create(definitions[`${name}Schema`], values);
  const unary = (client, name, request, metadata = authorization) => new Promise((resolve, reject) => {
    client[name](request, metadata, deadline(), (error, response) => error ? reject(error) : resolve(response));
  });
  const collect = async stream => {
    const messages = [];
    for await (const response of stream) messages.push(response);
    return messages;
  };
  try {
    assert.equal(SpeechToTextClient.serviceName, "rime.SpeechToText");
    assert.equal(TextToSpeechClient.service.synthesize.path, "/rime.TextToSpeech/Synthesize");
    const response = await new Promise((resolve, reject) => {
      let requestId;
      recognition.transcribe(message("TranscriptionRequest", { audio: Buffer.from("hello") }), authorization, deadline(), (error, response) => {
        if (error) return reject(error);
        try { assert.equal(requestId, "test-request"); resolve(response); } catch (error) { reject(error); }
      }).on("metadata", metadata => { requestId = metadata.get("x-request-id")[0]; });
    });
    assert.equal(response.text, "hello");
    assert.equal(response.$typeName, "rime.TranscriptionResponse");
    const stream = recognition.transcribeStreaming(authorization, deadline());
    const responses = collect(stream);
    stream.write(message("StreamingTranscriptionRequest", { payload: { case: "config", value: { language: "en" } } }));
    stream.end(message("StreamingTranscriptionRequest", { payload: { case: "audio", value: Buffer.from("hello") } }));
    const updates = await responses;
    assert.deepEqual(updates.map(response => response.payload.case), ["accepted", "delta", "done"]);
    assert.equal(updates[1].payload.value.text, "hello");
    const audio = await collect(synthesis.synthesize(message("SynthesisRequest", { text: "hello" }), deadline()));
    assert.deepEqual(audio.map(response => Buffer.from(response.payload.value).toString()), ["hello", "second"]);
    const voiceStream = synthesis.synthesizeStreaming(deadline());
    const voiceResponses = collect(voiceStream);
    voiceStream.write(message("StreamingSynthesisRequest", { payload: { case: "header", value: { text: "first" } } }));
    voiceStream.end(message("StreamingSynthesisRequest", { payload: { case: "textChunk", value: "second" } }));
    assert.deepEqual((await voiceResponses).map(response => Buffer.from(response.payload.value).toString()), ["first", "second"]);
    assert.deepEqual((await unary(synthesis, "normalizeText", message("NormalizeTextRequest", { text: "hello" }))).normalizedSentences, ["HELLO"]);
    assert.deepEqual((await unary(synthesis, "getSupportedLanguages", message("GetSupportedLanguagesRequest"))).languages, ["en"]);
    assert.deepEqual((await unary(synthesis, "getSupportedSpeakers", message("GetSupportedSpeakersRequest"))).speakers, ["test"]);
    await assert.rejects(unary(recognition, "transcribe", message("TranscriptionRequest"), new Metadata()), error => error.code === status.UNAUTHENTICATED);
    await assert.rejects(unary(recognition, "transcribe", message("TranscriptionRequest", { options: { language: "reject" } })), error => error.code === status.INVALID_ARGUMENT);
    await new Promise((resolve, reject) => {
      const waiting = synthesis.synthesize(message("SynthesisRequest", { text: "wait" }), deadline());
      waiting.once("data", () => waiting.cancel());
      waiting.once("error", error => {
        try { assert.equal(error.code, status.CANCELLED); resolve(); } catch (error) { reject(error); }
      });
    });
  } finally {
    recognition.close();
    synthesis.close();
    server.forceShutdown();
  }
}
console.log("Packaged ESM and CommonJS clients pass all RPCs, metadata, errors, and cancellation.");
