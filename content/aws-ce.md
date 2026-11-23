# aws ce

Run cost management operations through the AWS Cost Explorer service.

## Usage
- `aws ce create-anomaly-monitor --monitor monitor_name --monitor-type monitor_type` : create anomaly monitor
- `aws ce create-anomaly-subscription --subscription subscription_name --monitor-arn monitor_arn --subscribers subscribers` : create anomaly subscription
- `aws ce get-anomalies --monitor-arn monitor_arn --start-time start_time --end-time end_time` : get anomalies
- `aws ce get-cost-and-usage --time-period start_date/end_date --granularity granularity --metrics metrics` : get cost and usage
- `aws ce get-cost-forecast --time-period start_date/end_date --granularity granularity --metric metric` : get cost forecast
- `aws ce get-reservation-utilization --time-period start_date/end_date --granularity granularity` : get reservation utilization
- `aws ce list-cost-category-definitions` : list cost category definitions
- `aws ce tag-resource --resource-arn resource_arn --tags tags` : tag resource

## Categories
cloud, monitoring, system

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ce/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-ce
