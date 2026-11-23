# debconf-set-selections

Insert new values into the debconf database.

## Usage
- `debconf-set-selections file debconf-get-selections | ssh newhost debconf-set-selections` : the general form, as the manual gives it
- `debconf-set-selections --verbose` : verbose output
- `debconf-set-selections --checkonly` : only check the input file format, do not save changes to database

## Categories
databases
