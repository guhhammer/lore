# stripe logs

Tail and filter live Stripe API request logs.

## Usage
- `stripe logs tail` : tail all API request logs in real time
- `stripe logs tail --filter-http-method GET` : filter logs by HTTP method
- `stripe logs tail --filter-request-path /v1/customers` : filter logs by API path
- `stripe logs tail --filter-http-status-code 404` : filter logs by response status code
- `stripe logs tail --format JSON` : show logs as JSON

## Categories
network

## Links
- More information: https://docs.stripe.com/cli/logs/tail
- tldr page: https://tldr.inbrowser.app/pages/common/stripe-logs
