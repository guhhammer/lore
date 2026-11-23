# tftp

Trivial File Transfer Protocol 클라이언트.

## Usage
- `tftp 서버_ip 포트` : TFTP 서버의 IP 주소와 포트를 지정하여 연결
- `tftp 서버_ip -c 명령` : TFTP 서버에 연결하고 TFTP c명령 실행
- `tftp 서버_ip -6 -R 포트:포트` : IPv6를 사용하여 TFTP 서버에 연결하고 시작 포트를 R범위 내로 강제 설정
- `mode binary|ascii` : [대화형] tftp 클라이언트를 통해 전송 모드를 바이너리 또는 ASCII로 설정
- `get 파일` : [대화형] tftp 클라이언트를 통해 서버로부터 파일 다운로드
- `put 파일` : [대화형] tftp 클라이언트를 통해 서버로 파일 업로드
- `quit` : [대화형] tftp 클라이언트 종료

## Links
- More information: https://manned.org/tftp
