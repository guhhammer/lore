# aws ses

CLI for AWS Simple Email Service. High-scale inbound and outbound cloud email service.

## Usage
- `aws ses create-receipt-rule-set --rule-set-name rule_set_name --generate-cli-skeleton` : create a new receipt rule set
- `aws ses describe-active-receipt-rule-set --generate-cli-skeleton` : describe the active receipt rule set
- `aws ses describe-receipt-rule --rule-set-name rule_set_name --rule-name rule_name --generate-cli-skeleton` : describe a specific receipt rule
- `aws ses list-receipt-rule-sets --starting-token token_string --max-items integer --generate-cli-skeleton` : list all receipt rule sets
- `aws ses delete-receipt-rule-set --rule-set-name rule_set_name --generate-cli-skeleton` : delete a specific receipt rule set (the currently active rule set cannot be deleted)
- `aws ses delete-receipt-rule --rule-set-name rule_set_name --rule-name rule_name --generate-cli-skeleton` : delete a specific receipt rule
- `aws ses send-email --from from_address --destination "ToAddresses=addresses" --message "Subject={Data=subject_text,Charset=utf8},Body={Text={Data=body_text,Charset=utf8},Html={Data=message_body_containing_html,Charset=utf8\}\}"` : send an email
- `aws ses subcommand help` : display help for a specific SES subcommand

## Categories
cloud, system

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ses/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-ses
