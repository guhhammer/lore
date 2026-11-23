# icalbuddy

Utilidad de línea de comandos para imprimir eventos y tareas desde la base de datos del calendario de macOS.

## Usage
- `icalBuddy --includeOnlyEventsFromNowOn eventsToday` : muestra los eventos de hoy más tarde
- `icalBuddy uncompletedTasks` : muestra tareas no completadas
- `icalBuddy --formatOutput --separateByCalendar eventsToday` : muestra una lista formateada y discriminada de acuerdo al calendario de todos los eventos en el día de hoy
- `icalBuddy --includeOnlyEventsFromNowOn "tasksDueBefore:today+days"` : muestra las tareas para un número determinado de días
- `icalBuddy eventsFrom:start_date to:end_date` : muestra los eventos en un rango de tiempo

## Links
- More information: https://hasseg.org/icalBuddy/
