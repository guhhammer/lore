# kubie

`kubectl` 컨텍스트와 네임스페이스를 전환하는 유틸리티.

## Usage
- `kubie ctx` : 선택 가능한 컨텍스트 메뉴 표시
- `kubie ctx 컨텍스트` : 현재 쉘을 지정한 컨텍스트로 전환
- `kubie ns 네임스페이스` : 현재 쉘을 지정한 네임스페이스로 전환
- `kubie ctx 컨텍스트 -n 네임스페이스` : 현재 쉘을 지정한 컨텍스트와 네임스페이스로 전환
- `kubie exec 컨텍스트 네임스페이스 명령어` : 쉘을 생성하지 않고, 지정한 컨텍스트와 네임스페이스에서 명령 실행
- `kubie lint` : kubernetes 설정 파일 문제점 검사

## Links
- More information: https://github.com/sbstp/kubie#usage
