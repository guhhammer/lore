# clear-history

Удалять записи из истории команд текущей сессии PowerShell. Примечание: в качестве псевдонима для `Clear-History` можно использовать `clhy`.

## Usage
- `Clear-History` : очистить всю историю команд текущей сессии
- `Clear-History -CommandLine "команда"` : удалить команды с указанным именем
- `Clear-History -CommandLine "команда1", "команда2", ...` : удалить несколько команд по имени
- `Clear-History -Id номер_id` : удалить конкретную запись истории по ID
- `Clear-History -Id id1, id2, ...` : удалить несколько записей по ID
- `Clear-History -Id (начальный_id..конечный_id)` : удалить команды в диапазоне ID
- `Clear-History -WhatIf` : показать, что будет удалено
- `Clear-History -Confirm` : запросить подтверждение перед очисткой
