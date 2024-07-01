package client

import (
	"context"
	pb "grpc-demo/proto"
	"grpc-demo/server"
	"testing"
	"time"

	"google.golang.org/grpc"
)

func TestSayHello(t *testing.T) {
	go server.RunServer()
	conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
	if err != nil {
		t.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	c := pb.NewGreeterClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	r, err := c.SayHello(ctx, &pb.GreeterRequest{Name: "Test"})
	if err != nil {
		t.Fatalf("could not greet: %v", err)
	}
	if r.GetMessage() != "Hello Test" {
		t.Errorf("unexpected response: got %v, want %v", r.GetMessage(), "Hello Test")
	}
}
