# wacli

메시지 전송, 채팅 및 연락처 관리를 위한 WhatsApp 명령줄 클라이언트. `send`, `messages`, `contacts` 등 일부 하위 명령은 각각 별도의 사용 문서를 제공.

## Usage
- `wacli auth` : QR 코드를 사용하여 인증
- `wacli sync --once` : 메시지를 한 번 동기화한 후 종료
- `wacli send text --to 전화번호 --message "메시지"` : 전화번호로 텍스트 메시지 전송
- `wacli messages list --chat 전화번호@s.whatsapp.net --json` : 지정한 채팅의 메시지 목록을 JSON 형식으로 표시
- `wacli chats list --json` : 모든 채팅 목록을 JSON 형식으로 표시
- `wacli contacts search "이름" --json` : 이름으로 연락처 검색 후 JSON 형식으로 표시
- `wacli doctor` : 저장소 진단 실행
- `wacli version` : 버전 표시

## Links
- More information: https://wacli.sh/
