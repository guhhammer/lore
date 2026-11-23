# ipptool

IPP 요청을 전송하고 프린터 또는 서버의 응답을 확인하는 도구.

## Usage
- `ipptool ipp://프린터_uri get-printer-attributes.test` : 프린터가 지원하는 모든 속성과 값 조회
- `ipptool ipp://프린터_uri get-completed-jobs.test` : 프린터의 완료된 작업 목록 조회
- `ipptool -d recipient=mailto:이메일 ipp://프린터_uri create-printer-subscription.test` : 프린터 상태 변경 시 이메일 알림 설정

## Links
- More information: https://openprinting.github.io/cups/doc/man-ipptool.html
