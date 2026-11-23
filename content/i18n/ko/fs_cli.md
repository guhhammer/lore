# fs_cli

FreeSWITCH 명령줄 인터페이스 (ESL 클라이언트)를 사용하여 실행중인 FreeSWITCH 서버에 연결하고 제어.

## Usage
- `fs_cli` : 로컬 FreeSWITCH 인스턴스에 연결하여 대화형 세션 시작
- `fs_cli -H 호스트 -P port -p 패스워드` : 원격 FreeSWITCH 서버에 연결
- `fs_cli -x "명령어"` : 단일 FreeSWITCH 명령을 실행한 후 종료
- `fs_cli -x "status"` : freeSWITCH 시스템 상태 표시
- `fs_cli -x "reloadxml"` : freeSWITCH XML 설정 다시 불러오기
- `fs_cli -x "module_exists 모듈_이름"` : 모듈이 로드되어 있는지 확인
- `fs_cli -x "show calls"` : 현재 활성 통화 목록 표시
- `fs_cli -r` : 연결 실패 시 재시도
