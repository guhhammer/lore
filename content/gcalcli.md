# gcalcli

Interact with Google Calendar. Requests Google API authorization upon first launch.

## Usage
- `gcalcli agenda` : list your events for all your calendars over the next 7 days
- `gcalcli agenda mm/dd` : show events starting from or between specific dates (also takes relative dates e.g. "tomorrow")
- `gcalcli agenda mm/dd mm/dd` : show events between specific dates (also takes relative dates e.g. "tomorrow")
- `gcalcli --calendar calendar_name agenda` : list events from a specific calendar
- `gcalcli calw` : display an ASCII calendar of events by week
- `gcalcli calm` : display an ASCII calendar of events for a month
- `gcalcli --calendar calendar_name quick "mm/dd HH:MM event_name"` : quick-add an event to your calendar
- `gcalcli --calendar "calendar_name" add` : add an event to calendar. Triggers interactive prompt

## Links
- More information: https://github.com/insanum/gcalcli
- tldr page: https://tldr.inbrowser.app/pages/common/gcalcli
