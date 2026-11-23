# netsh

Windows 네트워크 설정을 관리. `wlan`과 같은 일부 하위 명령은 각각 별도의 사용 문서를 제공.

## Usage
- `netsh add helper 경로\대상\파일.dll` : helper Dynamic Link Library (DLL) 추가
- `netsh show helper` : 로드된 모든 helper DLL 목록 표시
- `netsh delete helper 경로\대상\파일.dll` : helper DLL 삭제
- `netsh dump >경로\대상\출력_파일.txt` : 네트워크 구성 설정을 파일로 내보내기
- `netsh trace show interfaces` : 추적에 사용할 수 있는 네트워크 인터페이스 목록 표시
- `exit` : 쉘 종료
- `netsh help` : 도움말 표시
