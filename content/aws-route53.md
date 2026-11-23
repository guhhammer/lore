# aws route53

CLI for AWS Route53 - Route 53 is a highly available and scalable Domain Name System (DNS) web service.

## Usage
- `aws route53 list-hosted-zones` : list all hosted zones, private and public
- `aws route53 list-resource-record-sets --hosted-zone-id zone_id` : show all records in a zone
- `aws route53 create-hosted-zone --name name --caller-reference request_identifier` : create a new, public zone using a request identifier to retry the operation safely
- `aws route53 delete-hosted-zone --id zone_id` : delete a zone (if the zone has non-defaults SOA and NS records the command will fail)
- `aws route53 test-dns-answer --hosted-zone-id zone_id --record-name name --record-type type` : test DNS resolving by Amazon servers of a given zone

## Categories
cloud, network, media

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/route53/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-route53
