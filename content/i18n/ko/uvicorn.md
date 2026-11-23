# uvicorn

비동기 프로젝트를 위한 Python ASGI HTTP 서버.

## Usage
- `uvicorn import.path:app_object` : python 웹 앱 실행
- `uvicorn --host localhost --port 8080 임포트.경로:앱_객체` : localhost에서 포트 8080으로 수신 대기
- `uvicorn --reload 임포트.경로:앱_객체` : 라이브 리로드 활성화
- `uvicorn --workers 4 임포트.경로:앱_객체` : 요청을 처리하기 위해 4개의 워커 프로세스 사용
- `uvicorn --ssl-certfile cert.pem --ssl-keyfile key.pem 임포트.경로:앱_객체` : HTTPS를 통해 앱 실행

## Links
- More information: https://www.uvicorn.org/settings/
