# synopkg

Synology DiskStation Manager의 패키지 관리 도구.

## Usage
- `synopkg list --name` : 설치된 패키지의 이름 나열
- `synopkg list --depend-on 패키지` : 특정 패키지에 의존하는 패키지 나열
- `sudo synopkg start|stop 패키지` : 패키지 시작/중지
- `synopkg status 패키지` : 패키지 상태 출력
- `sudo synopkg uninstall 패키지` : 패키지 제거
- `synopkg checkupdate 패키지` : 패키지 업데이트 가능 여부 확인
- `sudo synopkg upgradeall` : 모든 패키지를 최신 버전으로 업그레이드
- `sudo synopkg install 경로/대상/패키지.spk` : synopkg 파일에서 패키지 설치

## Links
- More information: https://www.synology.com/dsm
