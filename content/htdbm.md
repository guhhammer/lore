# htdbm

Manipulate DBM password databases.

## Usage
- `htdbm [ -TDBTYPE ] [ -i ] [ -c ] [ -m | -B | -d | -s | -p ] [ -C cost ] [ -t ] [ -v ] filename username` : the general form, as the manual gives it
- `htdbm -b [ -TDBTYPE ] [ -c ] [ -m | -B | -d | -s | -p ] [ -C cost ] [ -t ] [ -v ] filename username password` : another form of the same command
- `htdbm -n [ -i ] [ -c ] [ -m | -B | -d | -s | -p ] [ -C cost ] [ -t ] [ -v ] username` : another form of the same command
- `htdbm -nb [ -c ] [ -m | -B | -d | -s | -p ] [ -C cost ] [ -t ] [ -v ] username password` : another form of the same command
- `htdbm -v [ -TDBTYPE ] [ -i ] [ -c ] [ -m | -B | -d | -s | -p ] [ -C cost ] [ -t ] [ -v ] filename username` : another form of the same command
- `htdbm -i` : read the password from stdin without verification (for script usage)
- `htdbm -c` : create the passwdfile

## Categories
users
