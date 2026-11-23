# sf

A powerful command-line interface that simplifies development and build automation when working with your Salesforce org.

## Usage
- `sf force:auth:web:login --setalias organization --instanceurl organization_url` : authorize a Salesforce Organization
- `sf force:org:list` : list all authorized organizations
- `sf force:org:open --targetusername organization` : open a specific organization in the default web browser
- `sf force:org:display --targetusername organization` : display information about a specific organization
- `sf force:source:push --targetusername organization` : push source metadata to an Organization
- `sf force:source:pull --targetusername organization` : pull source metadata from an Organization
- `sf force:user:password:generate --targetusername organization` : generate a password for the organization's logged-in user
- `sf force:user:permset:assign --permsetname permission_set_name --targetusername organization` : assign a permission set for the organization's logged-in user

## Categories
users, network, development

## Links
- More information: https://developer.salesforce.com/tools/salesforcecli
- tldr page: https://tldr.inbrowser.app/pages/common/sf
