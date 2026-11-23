# where-object

Выбирать объекты из коллекции на основе значений их свойств. Примечание: эта команда доступна только в PowerShell.

## Usage
- `Get-Alias | Where-Object -Property Name -eq имя` : отфильтровать псевдонимы по имени
- `Get-Service | Where-Object {$_.Status -eq "Stopped"}` : вывести список всех остановленных служб. Автоматическая переменная `$_` представляет каждый объект, переданный командлету `Where-Object`
- `Get-Module -ListAvailable | Where-Object { $_.Name -NotLike "Microsoft*" -And $_.Name -NotLike "PS*" }` : использовать несколько условий
