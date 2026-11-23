# postconf

Postfix configuration utility. This command displays the values of the `main.cf` configuration parameters by default and warns about possible mistyped parameter names. It can also change the `main.cf` configuration parameter values.

## Usage
- `postconf -c path/to/configuration_directory` : specify the directory of the `main.cf` configuration file instead of the default configuration directory
- `postconf -e` : edit the `main.cf` configuration file and update parameter settings with the "name=value" pairs
- `postconf -d` : print the default parameter settings of the `main.cf` instead of the actual settings
- `postconf -C builtin|service|user|all` : display parameters only from the specified class
- `postconf -a` : list available SASL plug-in types for the Postfix SMTP server. The plug-in type is selected with the `smtpd_sasl_type` configuration parameter by specifying `cyrus` or `dovecot` as the name
- `postconf -m` : list the names of all supported lookup table types. Lookup tables are specified as `type:name` in configuration files where the type can be `btree`, `cdb`, `hash`, `mysql`, etc

## Categories
files

## Links
- More information: https://manned.org/postconf
- tldr page: https://tldr.inbrowser.app/pages/linux/postconf
