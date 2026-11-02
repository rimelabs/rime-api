"""Call both packaged services over local gRPC with sync and asyncio channels."""

import argparse
import asyncio
from concurrent import futures
from importlib import metadata
from pathlib import Path
import sys
import threading
import unittest

import grpc


class LocalServer:
    def setUp(self):
        self.server = grpc.server(futures.ThreadPoolExecutor(max_workers=4))

        def transcribe(request, context):
            if (
                dict(context.invocation_metadata()).get("authorization")
                != "Bearer test"
            ):
                context.abort(grpc.StatusCode.UNAUTHENTICATED, "Missing authorization")
            if request.options.language == "reject":
                context.abort(grpc.StatusCode.INVALID_ARGUMENT, "Unsupported language")
            context.send_initial_metadata((("x-request-id", "test-request"),))
            return speech.TranscriptionResponse(text=request.audio.decode())

        def transcribe_streaming(requests, context):
            for request in requests:
                if request.HasField("config"):
                    yield speech.StreamingTranscriptionResponse(
                        accepted=speech.StreamingAccepted()
                    )
                else:
                    yield speech.StreamingTranscriptionResponse(
                        delta=speech.TranscriptionDelta(text=request.audio.decode())
                    )
            yield speech.StreamingTranscriptionResponse(
                done=speech.TranscriptionDone(text="done")
            )

        def synthesize(request, context):
            yield synthesis.SynthesisResponseStream(audio=request.text.encode())
            if request.text == "wait":
                cancelled = threading.Event()
                context.add_callback(cancelled.set)
                cancelled.wait(5)
            else:
                yield synthesis.SynthesisResponseStream(audio=b"second")

        def synthesize_streaming(requests, context):
            for request in requests:
                text = (
                    request.header.text
                    if request.HasField("header")
                    else request.text_chunk
                )
                yield synthesis.SynthesisResponseStream(audio=text.encode())

        def handler(factory, request_type, response_type, implementation):
            return factory(
                implementation,
                request_deserializer=request_type.FromString,
                response_serializer=response_type.SerializeToString,
            )

        self.server.add_generic_rpc_handlers(
            (
                grpc.method_handlers_generic_handler(
                    "rime.SpeechToText",
                    {
                        "Transcribe": handler(
                            grpc.unary_unary_rpc_method_handler,
                            speech.TranscriptionRequest,
                            speech.TranscriptionResponse,
                            transcribe,
                        ),
                        "TranscribeStreaming": handler(
                            grpc.stream_stream_rpc_method_handler,
                            speech.StreamingTranscriptionRequest,
                            speech.StreamingTranscriptionResponse,
                            transcribe_streaming,
                        ),
                    },
                ),
                grpc.method_handlers_generic_handler(
                    "rime.TextToSpeech",
                    {
                        "Synthesize": handler(
                            grpc.unary_stream_rpc_method_handler,
                            synthesis.SynthesisRequest,
                            synthesis.SynthesisResponseStream,
                            synthesize,
                        ),
                        "SynthesizeStreaming": handler(
                            grpc.stream_stream_rpc_method_handler,
                            synthesis.StreamingSynthesisRequest,
                            synthesis.SynthesisResponseStream,
                            synthesize_streaming,
                        ),
                        "NormalizeText": handler(
                            grpc.unary_unary_rpc_method_handler,
                            synthesis.NormalizeTextRequest,
                            synthesis.NormalizeTextResponse,
                            lambda request, context: synthesis.NormalizeTextResponse(
                                normalized_text=request.text.upper()
                            ),
                        ),
                        "GetSupportedLanguages": handler(
                            grpc.unary_unary_rpc_method_handler,
                            synthesis.GetSupportedLanguagesRequest,
                            synthesis.GetSupportedLanguagesResponse,
                            lambda request, context: (
                                synthesis.GetSupportedLanguagesResponse()
                            ),
                        ),
                        "GetSupportedSpeakers": handler(
                            grpc.unary_unary_rpc_method_handler,
                            synthesis.GetSupportedSpeakersRequest,
                            synthesis.GetSupportedSpeakersResponse,
                            lambda request, context: (
                                synthesis.GetSupportedSpeakersResponse()
                            ),
                        ),
                    },
                ),
            )
        )
        self.address = f"127.0.0.1:{self.server.add_insecure_port('127.0.0.1:0')}"
        self.server.start()
        self.addCleanup(lambda: self.server.stop(0).wait())


AUTHORIZATION = (("authorization", "Bearer test"),)


class SynchronousClientTest(LocalServer, unittest.TestCase):
    def test_all_methods(self):
        with grpc.insecure_channel(self.address) as channel:
            recognition = speech_grpc.SpeechToTextStub(channel)
            voice = synthesis_grpc.TextToSpeechStub(channel)
            response, call = recognition.Transcribe.with_call(
                speech.TranscriptionRequest(audio=b"hello"),
                metadata=AUTHORIZATION,
                timeout=5,
            )
            self.assertEqual(response.text, "hello")
            self.assertIn(("x-request-id", "test-request"), call.initial_metadata())
            responses = list(
                recognition.TranscribeStreaming(
                    iter(
                        [
                            speech.StreamingTranscriptionRequest(
                                config=speech.StreamingConfig(language="en")
                            ),
                            speech.StreamingTranscriptionRequest(audio=b"hello"),
                        ]
                    ),
                    timeout=5,
                )
            )
            self.assertEqual(
                [response.WhichOneof("payload") for response in responses],
                ["accepted", "delta", "done"],
            )
            self.assertEqual(responses[1].delta.text, "hello")
            self.assertEqual(
                [
                    response.audio
                    for response in voice.Synthesize(
                        synthesis.SynthesisRequest(text="hello"), timeout=5
                    )
                ],
                [b"hello", b"second"],
            )
            self.assertEqual(
                [
                    response.audio
                    for response in voice.SynthesizeStreaming(
                        iter(
                            [
                                synthesis.StreamingSynthesisRequest(
                                    header=synthesis.SynthesisRequest(text="first")
                                ),
                                synthesis.StreamingSynthesisRequest(
                                    text_chunk="second"
                                ),
                            ]
                        ),
                        timeout=5,
                    )
                ],
                [b"first", b"second"],
            )
            self.assertEqual(
                voice.NormalizeText(
                    synthesis.NormalizeTextRequest(text="hello"), timeout=5
                ).normalized_text,
                "HELLO",
            )
            self.assertIsInstance(
                voice.GetSupportedLanguages(
                    synthesis.GetSupportedLanguagesRequest(), timeout=5
                ),
                synthesis.GetSupportedLanguagesResponse,
            )
            self.assertIsInstance(
                voice.GetSupportedSpeakers(
                    synthesis.GetSupportedSpeakersRequest(), timeout=5
                ),
                synthesis.GetSupportedSpeakersResponse,
            )

    def test_error_and_cancellation(self):
        with grpc.insecure_channel(self.address) as channel:
            recognition = speech_grpc.SpeechToTextStub(channel)
            with self.assertRaises(grpc.RpcError) as failure:
                recognition.Transcribe(
                    speech.TranscriptionRequest(
                        options=speech.TranscriptionOptions(language="reject")
                    ),
                    metadata=AUTHORIZATION,
                    timeout=5,
                )
            self.assertEqual(failure.exception.code(), grpc.StatusCode.INVALID_ARGUMENT)
            with self.assertRaises(grpc.RpcError) as failure:
                recognition.Transcribe(speech.TranscriptionRequest(), timeout=5)
            self.assertEqual(failure.exception.code(), grpc.StatusCode.UNAUTHENTICATED)
            stream = synthesis_grpc.TextToSpeechStub(channel).Synthesize(
                synthesis.SynthesisRequest(text="wait"), timeout=5
            )
            self.assertEqual(next(stream).audio, b"wait")
            self.assertTrue(stream.cancel())
            self.assertEqual(stream.code(), grpc.StatusCode.CANCELLED)


class AsynchronousClientTest(LocalServer, unittest.IsolatedAsyncioTestCase):
    async def test_all_methods(self):
        async with grpc.aio.insecure_channel(self.address) as channel:
            recognition = speech_grpc.SpeechToTextStub(channel)
            voice = synthesis_grpc.TextToSpeechStub(channel)
            call = recognition.Transcribe(
                speech.TranscriptionRequest(audio=b"hello"),
                metadata=AUTHORIZATION,
                timeout=5,
            )
            self.assertEqual((await call).text, "hello")
            self.assertIn(
                ("x-request-id", "test-request"), tuple(await call.initial_metadata())
            )
            stream = recognition.TranscribeStreaming(timeout=5)
            await stream.write(
                speech.StreamingTranscriptionRequest(
                    config=speech.StreamingConfig(language="en")
                )
            )
            self.assertTrue((await stream.read()).HasField("accepted"))
            await stream.write(speech.StreamingTranscriptionRequest(audio=b"hello"))
            await stream.done_writing()
            self.assertEqual((await stream.read()).delta.text, "hello")
            self.assertEqual((await stream.read()).done.text, "done")
            self.assertIs(await stream.read(), grpc.aio.EOF)
            self.assertEqual(
                [
                    response.audio
                    async for response in voice.Synthesize(
                        synthesis.SynthesisRequest(text="hello"), timeout=5
                    )
                ],
                [b"hello", b"second"],
            )
            stream = voice.SynthesizeStreaming(timeout=5)
            await stream.write(
                synthesis.StreamingSynthesisRequest(
                    header=synthesis.SynthesisRequest(text="first")
                )
            )
            await stream.write(synthesis.StreamingSynthesisRequest(text_chunk="second"))
            await stream.done_writing()
            self.assertEqual(
                [response.audio async for response in stream], [b"first", b"second"]
            )
            self.assertEqual(
                (
                    await voice.NormalizeText(
                        synthesis.NormalizeTextRequest(text="hello"), timeout=5
                    )
                ).normalized_text,
                "HELLO",
            )
            self.assertIsInstance(
                await voice.GetSupportedLanguages(
                    synthesis.GetSupportedLanguagesRequest(), timeout=5
                ),
                synthesis.GetSupportedLanguagesResponse,
            )
            self.assertIsInstance(
                await voice.GetSupportedSpeakers(
                    synthesis.GetSupportedSpeakersRequest(), timeout=5
                ),
                synthesis.GetSupportedSpeakersResponse,
            )

    async def test_error_and_cancellation(self):
        async with grpc.aio.insecure_channel(self.address) as channel:
            recognition = speech_grpc.SpeechToTextStub(channel)
            with self.assertRaises(grpc.aio.AioRpcError) as failure:
                await recognition.Transcribe(speech.TranscriptionRequest(), timeout=5)
            self.assertEqual(failure.exception.code(), grpc.StatusCode.UNAUTHENTICATED)
            stream = recognition.TranscribeStreaming(timeout=5)
            await stream.write(
                speech.StreamingTranscriptionRequest(config=speech.StreamingConfig())
            )
            await asyncio.wait_for(stream.read(), timeout=5)
            self.assertTrue(stream.cancel())
            self.assertEqual(await stream.code(), grpc.StatusCode.CANCELLED)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--packages", type=Path)
    parser.add_argument("--allow-legacy", action="store_true")
    arguments = parser.parse_args()
    if arguments.packages:
        sys.path.insert(0, str(next((arguments.packages / "dist").glob("*.whl"))))
    distribution = metadata.metadata("rime-api")
    if arguments.allow_legacy and "grpc" not in distribution.get_all(
        "Provides-Extra", []
    ):
        if tuple(map(int, distribution["Version"].split("."))) <= (0, 4, 1):
            print("Skipping gRPC checks for a release before packaged clients.")
            sys.exit(0)
    from rime_api import speech_to_text_pb2 as speech
    from rime_api import speech_to_text_pb2_grpc as speech_grpc
    from rime_api import text_to_speech_pb2 as synthesis
    from rime_api import text_to_speech_pb2_grpc as synthesis_grpc

    unittest.main(argv=[sys.argv[0]])
