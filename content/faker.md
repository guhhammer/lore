# faker

A Python library and a tool to generate fake data.

## Usage
- `faker` : show all fake data providers along with examples
- `faker name|address|passport_full|credit_card_full|phone_number|email|company|date_time|user_name|password|job|...` : generate fake data of a specific type
- `faker -r number -l de_DE|de_CH|... address` : generate a number of fake addresses from a specific country (use `localectl list-locales | cut --delimiter . --fields 1` to get list of locales)
- `faker -r number -l en_AU|en_US|... city -o path/to/file.txt` : generate a number of cities in a specific country and output them to a file (use `localectl list-locales | cut --delimiter . --fields 1` to get list of locales)
- `faker -r number -v user_agent` : generate a number of random HTTP user-agents showing verbose output
- `faker -r number -s ',' domain_name` : generate a number of domain names and separate each using a specific separator

## Categories
development, files, network

## Links
- More information: https://faker.readthedocs.io/en/master/
- tldr page: https://tldr.inbrowser.app/pages/common/faker
