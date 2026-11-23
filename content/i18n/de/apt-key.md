# apt-key

Schlüssel-Management-Tool für den APT-Paket-Manager auf Debian und Ubuntu.

## Usage
- `apt-key list` : liste alle vertrauten Schlüssel auf
- `apt-key add public_key_file.asc` : füge einen Schlüssel hinzu
- `apt-key del key_id` : lösche einen Schlüssel
- `wget -qO - https://host.tld/filename.key | apt-key add -` : füge einen Remote-Schlüssel hinzu
- `apt-key adv --keyserver pgp.mit.edu --recv key_id` : füge einen Schlüssel von einem Schlüsselserver hinzu nur mit der Schlüssel-ID

## Links
- More information: https://manned.org/apt-key
