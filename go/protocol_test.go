package rimeapi_test

import (
	"bytes"
	"context"
	"encoding/hex"
	"encoding/json"
	"flag"
	"net"
	"os"
	"reflect"
	"testing"
	"time"

	api "github.com/rimelabs/rime-api/go"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/test/bufconn"
	"google.golang.org/protobuf/encoding/protojson"
	"google.golang.org/protobuf/proto"
	"google.golang.org/protobuf/reflect/protoreflect"
	"google.golang.org/protobuf/reflect/protoregistry"
)

var fixturesPath = flag.String("fixtures", "testdata/fixtures.json", "Shared wire and JSON fixtures")

func TestSharedFixtures(t *testing.T) {
	data, err := os.ReadFile(*fixturesPath)
	if err != nil {
		t.Fatal(err)
	}
	var fixtures []struct {
		Message string
		JSON    json.RawMessage
		Hex     string
	}
	if err := json.Unmarshal(data, &fixtures); err != nil {
		t.Fatal(err)
	}
	for _, fixture := range fixtures {
		t.Run(fixture.Message+"/"+fixture.Hex, func(t *testing.T) {
			kind, err := protoregistry.GlobalTypes.FindMessageByName(protoreflect.FullName("rime." + fixture.Message))
			if err != nil {
				t.Fatal(err)
			}
			message := kind.New().Interface()
			if err := protojson.Unmarshal(fixture.JSON, message); err != nil {
				t.Fatal(err)
			}
			wire, err := proto.MarshalOptions{Deterministic: true}.Marshal(message)
			if err != nil {
				t.Fatal(err)
			}
			if hex.EncodeToString(wire) != fixture.Hex {
				t.Fatalf("wire differs: %x", wire)
			}
			decoded := kind.New().Interface()
			if err := proto.Unmarshal(wire, decoded); err != nil {
				t.Fatal(err)
			}
			encodedJSON, err := protojson.Marshal(decoded)
			if err != nil {
				t.Fatal(err)
			}
			var actual, expected any
			if err := json.Unmarshal(encodedJSON, &actual); err != nil {
				t.Fatal(err)
			}
			if err := json.Unmarshal(fixture.JSON, &expected); err != nil {
				t.Fatal(err)
			}
			if !reflect.DeepEqual(actual, expected) {
				t.Fatalf("JSON differs: %s", encodedJSON)
			}
		})
	}
}

func TestUnknownFields(t *testing.T) {
	wire, _ := hex.DecodeString("220568656c6c6ff80701")
	message := new(api.WebSocketRequest)
	if err := proto.Unmarshal(wire, message); err != nil {
		t.Fatal(err)
	}
	result, err := proto.Marshal(message)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(wire, result) {
		t.Fatalf("unknown field lost: %x", result)
	}
}

type synthesisServer struct {
	api.UnimplementedTextToSpeechServer
}

func (synthesisServer) NormalizeText(_ context.Context, request *api.NormalizeTextRequest) (*api.NormalizeTextResponse, error) {
	return &api.NormalizeTextResponse{NormalizedText: request.Text}, nil
}

func TestGRPCClient(t *testing.T) {
	listener := bufconn.Listen(1024 * 1024)
	server := grpc.NewServer()
	api.RegisterTextToSpeechServer(server, synthesisServer{})
	t.Cleanup(server.Stop)
	go func() { _ = server.Serve(listener) }()
	connection, err := grpc.NewClient("passthrough:///test", grpc.WithTransportCredentials(insecure.NewCredentials()), grpc.WithContextDialer(func(ctx context.Context, _ string) (net.Conn, error) { return listener.DialContext(ctx) }))
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = connection.Close() })
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	response, err := api.NewTextToSpeechClient(connection).NormalizeText(ctx, &api.NormalizeTextRequest{Text: "hello"})
	if err != nil {
		t.Fatal(err)
	}
	if response.GetNormalizedText() != "hello" {
		t.Fatalf("unexpected response: %v", response)
	}
}
