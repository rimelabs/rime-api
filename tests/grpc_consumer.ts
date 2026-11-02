import { create as createMessage } from "@bufbuild/protobuf";
import { credentials, Metadata } from "@grpc/grpc-js";
import { SpeechToTextClient, TextToSpeechClient } from "@rimelabs/api/grpc";
import {
  NormalizeTextRequestSchema, StreamingTranscriptionRequestSchema,
  StreamingSynthesisRequestSchema, SynthesisRequestSchema,
  TranscriptionRequestSchema, GetSupportedLanguagesRequestSchema,
  GetSupportedSpeakersRequestSchema,
} from "@rimelabs/api";
import type { TranscriptionResponse } from "@rimelabs/api";

const recognition = new SpeechToTextClient("localhost:50051", credentials.createInsecure());
const synthesis = new TextToSpeechClient("localhost:50051", credentials.createInsecure());
const transcription = createMessage(TranscriptionRequestSchema, { audio: new Uint8Array([1, 2]) });
const metadata = new Metadata();
const options = { deadline: Date.now() + 1000 };
SpeechToTextClient.service.transcribe.requestSerialize(transcription);
// @ts-expect-error Service serializers require the matching request type.
SpeechToTextClient.service.transcribe.requestSerialize(createMessage(SynthesisRequestSchema));
// @ts-expect-error Unknown service methods are not defined.
SpeechToTextClient.service.missingMethod;
const callback = (error: Error | null, response?: TranscriptionResponse) => {
  if (response) {
    const text: string = response.text;
    // @ts-expect-error Transcript text is a string.
    const invalid: number = response.text;
  }
};
recognition.transcribe(transcription, callback);
recognition.transcribe(transcription, metadata, callback);
recognition.transcribe(transcription, options, callback);
recognition.transcribe(transcription, metadata, options, callback);
// @ts-expect-error A unary call requires a callback.
recognition.transcribe(transcription);
// @ts-expect-error Requests must use the matching message type.
recognition.transcribe(createMessage(SynthesisRequestSchema), callback);
// @ts-expect-error Audio must be bytes.
recognition.transcribe({ ...transcription, audio: "audio" }, callback);
// @ts-expect-error Unknown methods are not callable.
recognition.missingMethod();
const recognitionStream = recognition.transcribeStreaming(metadata, options);
recognition.transcribeStreaming(options);
recognition.transcribeStreaming();
recognitionStream.write(createMessage(StreamingTranscriptionRequestSchema, { payload: { case: "audio", value: new Uint8Array([1, 2]) } }));
// @ts-expect-error A transcription stream rejects synthesis messages.
recognitionStream.write(createMessage(StreamingSynthesisRequestSchema));
const streamedResponse = recognitionStream.read();
if (streamedResponse.payload.case === "delta") {
  const text: string = streamedResponse.payload.value.text;
  // @ts-expect-error Delta text is a string.
  const invalid: number = streamedResponse.payload.value.text;
}
const synthesisRequest = createMessage(SynthesisRequestSchema, { text: "Hello." });
synthesis.synthesize(synthesisRequest);
synthesis.synthesize(synthesisRequest, options);
synthesis.synthesize(synthesisRequest, metadata, options);
// @ts-expect-error Server streaming requires a synthesis request.
synthesis.synthesize(transcription);
synthesis.synthesizeStreaming().write(createMessage(StreamingSynthesisRequestSchema));
synthesis.normalizeText(createMessage(NormalizeTextRequestSchema), (error, response) => {
  if (response) { const sentences: string[] = response.normalizedSentences; }
});
synthesis.getSupportedLanguages(createMessage(GetSupportedLanguagesRequestSchema), (error, response) => {
  if (response) { const languages: string[] = response.languages; }
});
synthesis.getSupportedSpeakers(createMessage(GetSupportedSpeakersRequestSchema), (error, response) => {
  if (response) { const speakers: string[] = response.speakers; }
});
recognitionStream.cancel();
recognition.close();
synthesis.close();
