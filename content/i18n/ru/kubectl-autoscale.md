# kubectl autoscale

Создавать автомасштабировщик для интеллектуального масштабирования количества подов на основе потребностей кластера Kubernetes.

## Usage
- `kubectl autoscale deploy имя_развёртывания --min мин_реплик --max макс_реплик` : настроить автомасштабирование развёртывания без указания целевой загрузки CPU
- `kubectl autoscale deploy имя_развёртывания --max макс_реплик --cpu-percent целевой_cpu` : настроить автомасштабирование развёртывания с целевой загрузкой CPU

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_autoscale/
