# pg_upgradecluster

Upgrade an existing PostgreSQL cluster to a new major version.

## Usage
- `pg_upgradecluster [-v newversion] oldversion name [newdatadir]` : the general form, as the manual gives it
- `pg_upgradecluster -v newversion` : set the version to upgrade to (default: latest available)
- `pg_upgradecluster --check` : check if all packages required for the upgrade are installed
- `pg_upgradecluster --logfile filel` : set a custom log file path for the upgraded database cluster
- `pg_upgradecluster --locale=locale` : set the default locale for the upgraded database cluster

## Categories
packages, containers, system
