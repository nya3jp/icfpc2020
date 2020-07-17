// Binary api-server is a gRPC server for APIs.
package main

import (
	"context"
	"log"
	"net"
	"os"

	"github.com/golang/protobuf/ptypes/empty"
	pb "github.com/nya3jp/icfpc2020/draftcode/protos"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func main() {
	grpcServer := grpc.NewServer()
	// Cloud Run supports only unary RPCs. Because gRPC reflection uses bidi
	// streams, we cannot make it work.
	pb.RegisterControlServiceServer(grpcServer, &controlService{})

	port := os.Getenv("PORT")
	if port == "" {
		log.Fatalf("PORT not specified")
	}
	listener, err := net.Listen("tcp", ":"+port)
	if err != nil {
		log.Fatalf("net.Listen: %v", err)
	}
	log.Fatal(grpcServer.Serve(listener))
}

type controlService struct{}

func (*controlService) Ping(context.Context, *empty.Empty) (*empty.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Ping not implemented")
}
