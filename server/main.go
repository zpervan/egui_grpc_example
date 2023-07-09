package main

import (
	"backend/core"
	"backend/proto"
	"backend/web"
	"google.golang.org/grpc"
	"net"
)

func main() {
	logger := core.NewLogger()

	listener, err := net.Listen("tcp", ":3500")
	if err != nil {
		logger.Fatal("")
	}

	serverOptions := []grpc.ServerOption{}

	server := grpc.NewServer(serverOptions...)
	proto.RegisterTestServer(server, web.NewServer(logger))

	logger.Info("starting server")

	if err := server.Serve(listener); err != nil {
		logger.Fatal("failed to serve: " + err.Error())
	}
}
