# stripe

Stripe 계정과 상호 작용.

## Usage
- `stripe logs tail` : 계정 활동 로그 팔로우
- `stripe listen --events="charge.succeeded" --forward-to="localhost:3000/events"` : `charge.succeeded` 이름의 이벤트를 필터링하여 이벤트 수신 대기하고 이를 localhost:3000/events로 전달
- `stripe trigger charge.succeeded` : 테스트 웹훅 이벤트 전송
- `stripe customers create --email="테스트@example.com" --name="Jenny Rosen"` : 고객 생성
- `stripe listen --print-json` : JSON으로 출력

## Links
- More information: https://docs.stripe.com/cli
