# aws ce

Esegue operazioni di gestione dei costi tramite il servizio AWS Cost Explorer.

## Usage
- `aws ce create-anomaly-monitor --monitor nome_monitor --monitor-type tipo_monitor` : crea monitor anomalie
- `aws ce create-anomaly-subscription --subscription nome_subscription --monitor-arn arn_monitor --subscribers subscribers` : crea sottoscrizione anomalie
- `aws ce get-anomalies --monitor-arn arn_monitor --start-time orario_inizio --end-time orario_fine` : ottieni anomalie
- `aws ce get-cost-and-usage --time-period data_inizio/data_fine --granularity granularita' --metrics metriche` : ottieni costi e utilizzo
- `aws ce get-cost-forecast --time-period data_inizio/data_fine --granularity granularita' --metric metrica` : ottieni previsione costi
- `aws ce get-reservation-utilization --time-period data_inizio/data_fine --granularity granularita'` : ottieni utilizzo prenotazioni
- `aws ce list-cost-category-definitions` : elenca definizioni categorie costi
- `aws ce tag-resource --resource-arn arn_risorsa --tags tags` : aggiungi tag a risorsa

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ce/
