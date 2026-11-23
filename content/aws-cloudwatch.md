# aws cloudwatch

Monitor AWS resources to gain system-wide visibility into resource utilization, application performance, and operational health.

## Usage
- `aws cloudwatch list-dashboards` : list dashboards for your account
- `aws cloudwatch get-dashboard --dashboard-name dashboard_name` : display details for the specified dashboard
- `aws cloudwatch list-metrics` : list metrics
- `aws cloudwatch describe-alarms` : list alarms
- `aws cloudwatch put-metric-alarm --alarm-name alarm_name --evaluation-periods evaluation_periods --comparison-operator comparison_operator` : create or update an alarm and associate it with a metric
- `aws cloudwatch delete-alarms --alarm-names alarm_names` : delete the specified alarms
- `aws cloudwatch delete-dashboards --dashboard-names dashboard_names` : delete the specified dashboards

## Categories
cloud, monitoring

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cloudwatch/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-cloudwatch
