# docker login

Bei einer Docker Registry einloggen.

## Usage
- `docker login` : interaktives Einloggen bei einer Registry
- `docker login -u benutzername` : einloggen mit einem angegebenen Benutzernamen (fragt nach dem Passwort)
- `docker login -u benutzername -p passwort server` : einloggen mit einem angegebenen Benutzernamen und Passwort
- `echo "passwort" | docker login -u benutzername --password-stdin` : einloggen mit einem Passwort, welches von `stdin` gelesen wird

## Links
- More information: https://docs.docker.com/reference/cli/docker/login/
