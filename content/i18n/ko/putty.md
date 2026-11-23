# putty

원격 서버에 연결하기 위한 SSH, Telnet 및 Rlogin 클라이언트. 참고: Linux에선 기본 `ssh` 클라이언트를 사용하는 것이 더 편리한 경우가 많음. PuTTY는 Windows에서 더 널리 사용됨.

## Usage
- `putty -ssh 사용자명@호스트명_또는_아이피` : SSH를 사용하여 원격 호스트에 연결
- `putty -ssh 사용자명@호스트명_또는_아이피 -P 포트` : 지정한 포트(Port)로 원격 host에 SSH 연결
- `putty -load 세션_이름` : 저장된 세션 불러오기
- `putty -ssh 사용자명@호스트명_또는_아이피 -i 경로/대상/개인_키.ppk` : 개인 키를 사용하여 SSH 인증 후 연결
- `putty -telnet 호스트명_또는_아이피` : telnet으로 원격 호스트에 연결
- `putty -ssh 사용자명@호스트명_또는_아이피 -X` : X11 포워딩 활성화
- `putty -ssh 사용자명@호스트명_또는_아이피 -L 로컬_포트:목적지_호스트:목적지_포트` : 로컬(Local) 포트 포워딩 설정
- `putty -help` : 도움말 표시
