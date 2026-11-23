# debconf-apt-progress

Install packages using debconf to display a progress bar.

## Usage
- `debconf-apt-progress [--] command [args ...]` : the general form, as the manual gives it
- `debconf-apt-progress --config` : another form of the same command
- `debconf-apt-progress --start` : another form of the same command
- `debconf-apt-progress --from waypoint --to waypoint [--] command [args ...]` : another form of the same command
- `debconf-apt-progress --stop` : another form of the same command
- `debconf-apt-progress --start` : start up a progress bar, running from 0 to 100 by default
- `debconf-apt-progress --stop` : stop a running progress bar

## Categories
packages, desktop
