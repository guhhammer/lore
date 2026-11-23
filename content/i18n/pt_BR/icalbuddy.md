# icalbuddy

Utilitário de linha de comando para exibir eventos e tarefas do banco de dados do calendário do macOS.

## Usage
- `icalBuddy --includeOnlyEventsFromNowOn eventsToday` : exibe eventos que acontecerão hoje
- `icalBuddy uncompletedTasks` : exibe tarefas incompletas
- `icalBuddy --formatOutput --separateByCalendar eventsToday` : exibe uma lista formatada separada por calendário para todos os eventos de hoje
- `icalBuddy --includeOnlyEventsFromNowOn "tasksDueBefore:today+dias"` : exibe tarefas para um determinado número de dias
- `icalBuddy eventsFrom:data_inicial to:data_final` : exibe eventos em um intervalo de tempo

## Links
- More information: https://hasseg.org/icalBuddy/
