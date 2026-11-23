# wacli messages search

전체 텍스트 검색을 사용해 WhatsApp 메시지를 검색.

## Usage
- `wacli messages search "검색어" --chat 전화번호@s.whatsapp.net --json` : 지정한 채팅에서 메시지 검색
- `wacli messages search "검색어" --from 전화번호@s.whatsapp.net --json` : 지정한 발신자가 보낸 메시지 검색
- `wacli messages search "검색어" --chat 전화번호@s.whatsapp.net --after 2026-01-28 --json` : 지정한 날짜 이후의 메시지 검색
- `wacli messages search "검색어" --type image|video|audio|document --json` : 지정한 유형의 메시지 검색
- `wacli messages search "검색어" --chat 전화번호@s.whatsapp.net --limit 10 --json` : 결과 개수를 제한하여 메시지 검색

## Links
- More information: https://wacli.sh/messages.html
