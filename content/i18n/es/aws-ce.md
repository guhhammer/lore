# aws ce

Ejecuta operaciones de gestión de costos a través del servicio AWS Cost Explorer.

## Usage
- `aws ce create-anomaly-monitor --monitor nombre_monitor --monitor-type tipo_monitor` : crea monitor de anomalías
- `aws ce create-anomaly-subscription --subscription nombre_de_suscripción --monitor-arn monitor_arn --subscribers suscriptores` : crea suscripción de anomalías
- `aws ce get-anomalies --monitor-arn monitor_arn --start-time hora_de_inicio --end-time hora_final` : obtiene anomalías
- `aws ce get-cost-and-usage --time-period fecha_inicio/fecha_final --granularity granularidad --metrics métricas` : obtiene coste y uso
- `aws ce get-cost-forecast --time-period fecha_inicio/fecha_final --granularity granularidad --metric métrica` : obtiene previsión de costes
- `aws ce get-reservation-utilization --time-period fecha_inicio/fecha_final --granularity granularidad` : obtiene la utilización de la reserva
- `aws ce list-cost-category-definitions` : lista de definiciones de categorías de costes
- `aws ce tag-resource --resource-arn recurso_arn --tags etiquetas` : recurso de etiquetas

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ce/
