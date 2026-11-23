# ropper

바이너리 파일에서 ROP 가젯을 찾습니다.

## Usage
- `ropper --file 경로/대상/바이너리` : 바이너리 파일의 가젯 나열
- `ropper --file 경로/대상/바이너리 --search 정규_표현식` : 정규 표현식을 사용하여 바이너리 파일의 가젯 필터링
- `ropper --file 경로/대상/바이너리 --type rop|job|sys|all` : 지정된 유형의 가젯을 바이너리 파일에서 나열
- `ropper --file 경로/대상/바이너리 --badbytes 바이트_문자열` : 바이너리 파일에서 불량 바이트 가젯 제외
- `ropper --file 경로/대상/바이너리 --inst-count 수량` : 지정된 명령어 수까지의 가젯을 바이너리 파일에서 나열

## Links
- More information: https://scoding.de/ropper/
