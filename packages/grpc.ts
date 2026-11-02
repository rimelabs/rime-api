import { fromBinary, toBinary } from "@bufbuild/protobuf";
import type { DescMethod, DescService, MessageShape } from "@bufbuild/protobuf";
import { makeGenericClientConstructor } from "@grpc/grpc-js";
import type {
  CallOptions,
  ChannelCredentials,
  Client,
  ClientDuplexStream,
  ClientOptions,
  ClientReadableStream,
  ClientUnaryCall,
  ClientWritableStream,
  Metadata,
  MethodDefinition,
  requestCallback as UnaryCallback,
} from "@grpc/grpc-js";
import { SpeechToText } from "./rime/speech_to_text_pb.js";
import { TextToSpeech } from "./rime/text_to_speech_pb.js";

type CallbackArguments<Response> =
  | [callback: UnaryCallback<Response>]
  | [metadata: Metadata, callback: UnaryCallback<Response>]
  | [options: CallOptions, callback: UnaryCallback<Response>]
  | [metadata: Metadata, options: CallOptions, callback: UnaryCallback<Response>];

type StreamArguments =
  | [options?: CallOptions]
  | [metadata: Metadata, options?: CallOptions];

type ClientMethod<Method extends DescMethod> =
  Method["methodKind"] extends "unary"
    ? (request: MessageShape<Method["input"]>, ...arguments_: CallbackArguments<MessageShape<Method["output"]>>) => ClientUnaryCall
    : Method["methodKind"] extends "server_streaming"
      ? (request: MessageShape<Method["input"]>, ...arguments_: StreamArguments) => ClientReadableStream<MessageShape<Method["output"]>>
      : Method["methodKind"] extends "client_streaming"
        ? (...arguments_: CallbackArguments<MessageShape<Method["output"]>>) => ClientWritableStream<MessageShape<Method["input"]>>
        : (...arguments_: StreamArguments) => ClientDuplexStream<MessageShape<Method["input"]>, MessageShape<Method["output"]>>;

type ServiceClient<Service extends DescService> = Client & {
  [Name in keyof Service["method"]]: ClientMethod<Service["method"][Name]>;
};

type GrpcServiceDefinition<Service extends DescService> = {
  [Name in keyof Service["method"]]: MethodDefinition<
    MessageShape<Service["method"][Name]["input"]>,
    MessageShape<Service["method"][Name]["output"]>
  >;
};

type ClientConstructor<Service extends DescService> = {
  new(address: string, credentials: ChannelCredentials, options?: ClientOptions): ServiceClient<Service>;
  service: GrpcServiceDefinition<Service>;
  serviceName: string;
};

function createClientConstructor<Service extends DescService>(service: Service): ClientConstructor<Service> {
  const descriptors: Service["method"] = service.method;
  const methods = {} as GrpcServiceDefinition<Service>;
  for (const name in descriptors) {
    type Method = Service["method"][typeof name];
    const method = descriptors[name];
    methods[name] = {
      path: `/${service.typeName}/${method.name}`,
      requestStream: method.methodKind === "client_streaming" || method.methodKind === "bidi_streaming",
      responseStream: method.methodKind === "server_streaming" || method.methodKind === "bidi_streaming",
      requestSerialize: message => Buffer.from(toBinary(method.input, message)),
      requestDeserialize: bytes => fromBinary<Method["input"]>(method.input, bytes),
      responseSerialize: message => Buffer.from(toBinary(method.output, message)),
      responseDeserialize: bytes => fromBinary<Method["output"]>(method.output, bytes),
    };
  }
  return makeGenericClientConstructor(methods, service.typeName) as ClientConstructor<Service>;
}

/** A native grpc-js client for whole-recording and streaming transcription. */
export const SpeechToTextClient: ClientConstructor<typeof SpeechToText> = createClientConstructor(SpeechToText);
export type SpeechToTextClient = ServiceClient<typeof SpeechToText>;

/** A native grpc-js client for synthesis, normalization, and catalog queries. */
export const TextToSpeechClient: ClientConstructor<typeof TextToSpeech> = createClientConstructor(TextToSpeech);
export type TextToSpeechClient = ServiceClient<typeof TextToSpeech>;
