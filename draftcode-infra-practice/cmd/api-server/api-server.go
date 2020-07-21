// Copyright 2020 Google LLC
// Copyright 2020 Team Spacecat
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
