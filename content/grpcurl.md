# grpcurl

Interact with gRPC servers. Like `curl`, but for gRPC.

## Usage
- `grpcurl grpc.server.com:443 my.custom.server.Service/Method` : send an empty request
- `grpcurl -H "Authorization: Bearer $token" -d '{"foo": "bar"}' grpc.server.com:443 my.custom.server.Service/Method` : send a request with a header and a body
- `grpcurl grpc.server.com:443 list` : list all services exposed by a server
- `grpcurl grpc.server.com:443 list my.custom.server.Service` : list all methods in a particular service

## Categories
system

## Links
- More information: https://github.com/fullstorydev/grpcurl
- tldr page: https://tldr.inbrowser.app/pages/common/grpcurl
