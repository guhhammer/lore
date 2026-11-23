# lt

Localtunnel은 로컬호스트를 외부에 노출시켜 손쉽게 테스트하고 공유할 수 있게 해줍니다.

## Usage
- `lt -p 8000` : 특정 포트에서 터널 시작
- `lt -p 8000 -h 호스트` : 포워딩을 수행하는 업스트림 서버 지정
- `lt -p 8000 -s 서브도메인` : 특정 서브도메인 요청
- `lt -p 8000 --print-requests` : 기본 요청 정보 출력
- `lt -p 8000 -o` : 기본 웹 브라우저에서 터널 URL 열기

## Links
- More information: https://github.com/localtunnel/localtunnel
