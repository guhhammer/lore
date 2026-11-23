# stripe

Interact with Stripe payment processor. Some subcommands such as `docs`, `listen`, and `logs` have their own usage documentation.

## Usage
- `stripe docs` : browse Stripe documentation
- `stripe logs tail` : follow the logs of activity on the account
- `stripe listen --events="charge.succeeded" --forward-to="localhost:3000/events"` : listen for events, filtering on events with the name `charge.succeeded` and forwarding them to localhost:3000/events
- `stripe trigger charge.succeeded` : send a test webhook event
- `stripe customers create --email "test@example.com" --name "Jenny Rosen"` : create a customer
- `stripe listen --print-json` : print to JSON

## Categories
documents, monitoring

## Links
- More information: https://docs.stripe.com/cli
- tldr page: https://tldr.inbrowser.app/pages/common/stripe
