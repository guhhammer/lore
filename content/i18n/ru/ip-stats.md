# ip stats

Показывать статистику интерфейсов и управлять ею.

## Usage
- `ip st` : показать статистику всех сетевых интерфейсов
- `ip st show dev сетевой_интерфейс` : показать статистику конкретного сетевого интерфейса
- `ip st show group link` : показать статистику канального уровня (аналог `ip -statistics link show`)
- `ip st show group offload` : показать статистику аппаратной разгрузки для всех устройств
- `ip st show dev сетевой_интерфейс group offload` : показать статистику разгрузки для конкретного интерфейса
- `ip st show dev сетевой_интерфейс group offload subgroup l3_stats|cpu_hit|hw_stats_info` : показать конкретную подгруппу разгрузки
- `ip st show group afstats subgroup mpls` : показать статистику по семейству адресов (например, MPLS)
- `ip st set dev сетевой_интерфейс l3_stats on` : включить сбор статистики аппаратного уровня L3 на устройстве

## Links
- More information: https://manned.org/ip-stats
