# xauth

X 서버에 연결할 때 사용되는 인증 정보를 편집하고 표시.

## Usage
- `xauth -f 경로/대상/파일` : 특정 인증 파일로 대화형 모드 시작 (`~/.Xauthority`가 기본값)
- `xauth info` : 인증 파일에 대한 정보 표시
- `xauth list` : 모든 디스플레이에 대한 인증 항목 표시
- `xauth add 디스플레이_이름 프로토콜_이름 키` : 특정 디스플레이에 대한 인증 추가
- `xauth remove 디스플레이_이름` : 특정 디스플레이에 대한 인증 제거
- `xauth extract - $DISPLAY` : 현재 디스플레이에 대한 인증 항목을 `stdout`에 출력
- `cat 경로/대상/파일 | xauth merge -` : 특정 파일에서 인증 항목을 인증 데이터베이스에 병합
- `xauth --help` : 도움말 표시

## Links
- More information: https://manned.org/xauth
