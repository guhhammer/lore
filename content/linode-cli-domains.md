# linode-cli domains

Manage Linode Domains and DNS configuration.

## Usage
- `linode-cli domains list` : list all managed domains
- `linode-cli domains create --domain domain_name --type master|slave --soa-email email` : create a new managed domain
- `linode-cli domains view domain_id` : view details of a specific domain
- `linode-cli domains delete domain_id` : delete a managed domain
- `linode-cli domains records-list domain_id` : list records for a specific domain
- `linode-cli domains records-create domain_id --type A|AAAA|CNAME|MX|... --name subdomain --target target_value` : add a DNS record to a domain
- `linode-cli domains records-update domain_id record_id --target new_target_value` : update a DNS record for a domain
- `linode-cli domains records-delete domain_id record_id` : delete a DNS record from a domain

## Categories
network, media

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/linode-cli-domains
