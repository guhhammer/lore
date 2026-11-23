# elasticsearch-node

Управлять низкоуровневыми операциями узла Elasticsearch, такими как завершение работы, переназначение роли или просмотр информации.

## Usage
- `elasticsearch-node info` : показать информацию о текущем узле
- `elasticsearch-node unsafe-bootstrap` : подготовить узел к полной перезагрузке кластера (например, после обновления)
- `elasticsearch-node repurpose` : переназначить роль узла (например, с master на data)
- `elasticsearch-node roles` : вывести список ролей, назначенных узлу
- `elasticsearch-node diagnostics` : показать установленную версию JVM, путь к домашнему каталогу Elasticsearch и другую диагностическую информацию
- `elasticsearch-node -h` : показать справку
