package web

import (
	"backend/core"
	"backend/proto"
	"context"
	"sync"
)

type Server struct {
	mutex  *sync.Mutex
	logger *core.Logger
	proto.UnimplementedTestServer
}

func NewServer(logger *core.Logger) *Server {
	server := &Server{}

	server.mutex = &sync.Mutex{}
	server.logger = logger

	return server
}

func (s *Server) TestConnection(_ context.Context, in *proto.DummyRequest) (*proto.DummyResponse, error) {
	s.logger.Info("received test connection request from client")

	if in.DummyData == "" {
		return &proto.DummyResponse{}, nil
	}

	return &proto.DummyResponse{DummyData: "gRPC server response: " + in.DummyData}, nil
}
