# stripe listen

Forward Stripe webhook events to a local server for testing.

## Usage
- `stripe listen --forward-to "localhost:3000/webhook"` : forward all events to a local endpoint
- `stripe listen --events "payment_intent.succeeded,payment_intent.payment_failed" --forward-to="localhost:3000/webhook"` : forward only specific event types
- `stripe listen --print-json` : print events as JSON without forwarding
- `stripe listen --forward-to "localhost:3000/webhook" --forward-connect-to "localhost:3000/connect-webhook"` : forward connect events to a separate local endpoint
- `stripe listen --forward-to "localhost:3000/webhook" --skip-verify` : skip SSL certificate verification when forwarding

## Links
- More information: https://docs.stripe.com/cli/listen
- tldr page: https://tldr.inbrowser.app/pages/common/stripe-listen
