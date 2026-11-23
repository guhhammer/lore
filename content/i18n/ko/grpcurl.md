# grpcurl

gRPC 서버와 상호 작용. `curl`과 비슷하지만, gRPC용임.

## Usage
- `grpcurl grpc.server.com:443 my.custom.server.Service/Method` : 빈 요청 보내기
- `grpcurl -H "Authorization: Bearer $token" -d '{"foo": "bar"}' grpc.server.com:443 my.custom.server.Service/Method` : 헤더와 본문이 포함된 요청을 보냄
- `grpcurl grpc.server.com:443 list` : 서버가 노출하는 모든 서비스를 나열
- `grpcurl grpc.server.com:443 list my.custom.server.Service` : 특정 서비스의 모든 메서드를 나열

## Links
- More information: https://github.com/fullstorydev/grpcurl
