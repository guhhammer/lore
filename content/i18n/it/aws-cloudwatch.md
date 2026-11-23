# aws cloudwatch

Monitora le risorse AWS per ottenere visibilità a livello di sistema sull'utilizzo delle risorse, le prestazioni delle applicazioni e la salute operativa.

## Usage
- `aws cloudwatch list-dashboards` : elenca le dashboard per il tuo account
- `aws cloudwatch get-dashboard --dashboard-name nome_dashboard` : visualizza i dettagli per la dashboard specificata
- `aws cloudwatch list-metrics` : elenca le metriche
- `aws cloudwatch describe-alarms` : elenca gli allarmi
- `aws cloudwatch put-metric-alarm --alarm-name nome_allarme --evaluation-periods evaluation_periods --comparison-operator comparison_operator` : crea o aggiorna un allarme e lo associa a una metrica
- `aws cloudwatch delete-alarms --alarm-names nomi_allarme` : elimina gli allarmi specificati
- `aws cloudwatch delete-dashboards --dashboard-names nome_dashboard` : elimina le dashboard specificate

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cloudwatch/
