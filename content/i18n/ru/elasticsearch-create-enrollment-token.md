# elasticsearch-create-enrollment-token

Создавать токены регистрации для узлов Elasticsearch и экземпляров Kibana.

## Usage
- `elasticsearch-create-enrollment-token -s node` : создать токен регистрации для добавления нового узла Elasticsearch
- `elasticsearch-create-enrollment-token -s kibana` : создать токен регистрации для добавления нового экземпляра Kibana
- `elasticsearch-create-enrollment-token -s node --verbose` : создать токен регистрации и показать подробный вывод
- `elasticsearch-create-enrollment-token -s kibana --url "ip_адрес"` : создать токен регистрации для экземпляра Kibana с пользовательским URL-адресом Elasticsearch
- `elasticsearch-create-enrollment-token -h` : показать справку
