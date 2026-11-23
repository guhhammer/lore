# openclaw

A personal AI assistant that you run on your own devices. Some subcommands such as `onboard`, `agent`, `doctor`, etc. have their own usage documentation.

## Usage
- `openclaw onboard --install-daemon` : run the onboarding wizard to set up the gateway and channels
- `openclaw gateway --allow-unconfigured --port 18789 --verbose` : start the gateway server
- `openclaw message send -t +1234567890 -m "Hello"` : send a message to a contact
- `openclaw agent -m "Ship checklist"` : talk to the assistant with a single message
- `openclaw agent` : start interactive chat mode
- `openclaw doctor` : run diagnostics and check system health
- `openclaw update` : update OpenClaw to the latest version
- `openclaw channels list` : list configured channels

## See also
- zeroclaw

## Categories
documents, hardware, monitoring

## Links
- More information: https://docs.openclaw.ai/cli
- tldr page: https://tldr.inbrowser.app/pages/common/openclaw
