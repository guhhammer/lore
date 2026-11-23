# agetty

일반적으로 `init`에 의해 호출됩니다. 참고: 보드레이트는 터미널과 장치 간의 직렬 연결을 통한 데이터 전송 속도입니다.

## Usage
- `agetty tty 115200` : `stdin`을 포트(`/dev` 상대 경로)에 연결하고 선택적으로 보드레이트를 지정(기본값: 9600)
- `agetty -t 타임아웃_초 -` : `stdin`이 이미 `tty`에 연결되었다고 가정하고 로그인에 대한 타임아웃 설정
- `agetty -8 - term_var` : `tty`가 [8]비트라고 가정하고 `init`에 의해 설정된 `$TERM` 환경 변수를 재정의
- `agetty -n -l 로그인_프로그램 tty` : 로그인을 건너뛰고(로그인 없음) 루트 권한으로 다른 로그인 프로그램을 `/bin/login` 대신 호출
- `agetty -i -` : 로그인 프롬프트를 작성하기 전에 사전 로그인(이슈) 파일(`/etc/issue` 기본값)을 표시하지 않음
- `agetty -r /경로/대상/루트_디렉터리 -H 가짜_호스트 -` : 루트 디렉터리를 변경하고 `utmp` 파일에 특정 가짜 호스트 작성

## Links
- More information: https://manned.org/agetty
