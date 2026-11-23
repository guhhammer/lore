# icalbuddy

Command-line utility for printing events and tasks from the macOS calendar database.

## Usage
- `icalBuddy --includeOnlyEventsFromNowOn eventsToday` : show events later today
- `icalBuddy uncompletedTasks` : show uncompleted tasks
- `icalBuddy --formatOutput --separateByCalendar eventsToday` : show a formatted list separated by calendar for all events today
- `icalBuddy --includeOnlyEventsFromNowOn "tasksDueBefore:today+number_of_days"` : show tasks for a specified number of days
- `icalBuddy eventsFrom:start_date to:end_date` : show events in a time range

## Categories
databases, text, macos

## Links
- More information: https://hasseg.org/icalBuddy/
- tldr page: https://tldr.inbrowser.app/pages/osx/icalbuddy
