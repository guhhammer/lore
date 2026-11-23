# eventcreate

Create custom entries in the event log. Event IDs can be any number between 1 and 1000.

## Usage
- `eventcreate /t success|error|warning|information /id 1..1000 /d "message"` : create a new event with a given ID (1-1000) in the log
- `eventcreate /l log_name /t type /id event_id /d "message"` : create an event in a specific event log
- `eventcreate /so source_name /t type /id event_id /d "message"` : create an event with a specific source
- `eventcreate /s hostname /u username /p password /t type /id event_id /d "message"` : create an event in a remote machine's event log

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
monitoring, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/eventcreate
