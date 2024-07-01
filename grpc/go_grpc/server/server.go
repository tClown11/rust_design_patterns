package server

import (
	"context"
	"log"
	"net"
	"os"
	"os/signal"
	"syscall"
	"time"

	"google.golang.org/grpc"

	pb "grpc-demo/proto"
)

type server struct {
	pb.UnimplementedGreeterServer
}

func (s *server) SayHello(ctx context.Context, in *pb.GreeterRequest) (*pb.GreeterResponse, error) {
	select {
	case <-ctx.Done():
		return nil, ctx.Err()
	default:
		log.Printf("Received: %v", in.GetName())
		return &pb.GreeterResponse{Message: "Hello " + in.GetName()}, nil
	}
}

func RunServer() {
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	s := grpc.NewServer()
	pb.RegisterGreeterServer(s, &server{})
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}

	// 创建一个通道来接收操作系统的信号
	stop := make(chan os.Signal, 1)
	signal.Notify(stop, os.Interrupt, syscall.SIGTERM)

	go func() {
		log.Println("Starting gRPC server...")
		if err := s.Serve(lis); err != nil {
			log.Fatalf("failed to serve: %v", err)
		}
	}()

	<-stop // 等待中断信号

	log.Println("Shutting down gRPC server...")

	// 创建一个带有超时的上下文
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	// 优雅地停止服务器
	stopped := make(chan struct{})
	go func() {
		s.GracefulStop()
		close(stopped)
	}()

	select {
	case <-ctx.Done():
		log.Println("Timeout: forcing shutdown")
		s.Stop()
	case <-stopped:
		log.Println("Server stopped gracefully")
	}

	log.Println("Server exiting")
}
