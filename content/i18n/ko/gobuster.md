# gobuster

웹 서버 등의 숨겨진 경로를 무차별 공격.

## Usage
- `gobuster dir -u https://example.com/ -w 경로/대상/파일` : 단어 목록에서 일치하는 디렉터리와 파일을 검색
- `gobuster dns -do example.com -w 경로/대상/파일` : 하위 도메인 검색
- `gobuster s3 -w 경로/대상/파일` : amazon S3 버킷 검색
- `gobuster vhost -u https://example.com/ -w 경로/대상/파일` : 서버에서 다른 가상 호스트를 검색
- `gobuster fuzz -u https://example.com/?parameter=FUZZ -w 경로/대상/파일` : 매개변수 값을 퍼징
- `gobuster fuzz -u https://example.com/?FUZZ=value -w 경로/대상/파일` : 매개변수 이름을 퍼징

## Links
- More information: https://github.com/OJ/gobuster#modes
