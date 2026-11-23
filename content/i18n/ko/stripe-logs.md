# stripe logs

실시간 Stripe API 요청 로그 추적 및 필터링.

## Usage
- `stripe logs tail` : 모든 API 요청 로그를 실시간으로 추적
- `stripe logs tail --filter-http-method GET` : HTTP 메서드로 로그 필터링
- `stripe logs tail --filter-request-path /v1/customers` : API 경로로 로그 필터링
- `stripe logs tail --filter-http-status-code 404` : 응답 상태 코드로 로그 필터링
- `stripe logs tail --format JSON` : 로그를 JSON 형식으로 표시

## Links
- More information: https://docs.stripe.com/cli/logs/tail
