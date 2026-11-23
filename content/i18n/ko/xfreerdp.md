# xfreerdp

Free Remote Desktop Protocol 구현체.

## Usage
- `xfreerdp /u:사용자명 /p:비밀번호 /v:ip_주소` : freeRDP 서버에 연결
- `xfreerdp /u:사용자명 /p:비밀번호 /v:ip_주소 /sound:sys:alsa` : freeRDP 서버에 연결하고 `sys:alsa` 장치를 사용하여 오디오 출력 리디렉션 활성화
- `xfreerdp /v:ip_주소 /u:사용자명 /p:비밀번호 /dynamic-resolution` : 동적 해상도로 FreeRDP 서버에 연결
- `xfreerdp /v:ip_주소 /u:사용자명 /p:비밀번호 +clipboard` : 클립보드 리디렉션과 함께 FreeRDP 서버에 연결
- `xfreerdp /v:ip_주소 /u:사용자명 /p:비밀번호 /cert:ignore` : 인증서를 무시하고 FreeRDP 서버에 연결
- `xfreerdp /v:ip_주소 /u:사용자명 /p:비밀번호 /drive:경로/대상/폴더,공유_이름` : 공유 디렉토리와 함께 FreeRDP 서버에 연결
