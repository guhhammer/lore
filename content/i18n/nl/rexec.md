# rexec

Voer een commando uit op een externe host. Opmerking: gebruik `rexec` met voorzichtigheid, omdat het gegevens in platte tekst verzendt. Overweeg veilige alternatieven zoals SSH voor versleutelde communicatie.

## Usage
- `rexec -h remote_host ls -l` : voer een commando uit op een externe host
- `rexec -u gebruikersnaam -h remote_host ps aux` : specificeer de externe gebruikersnaam op een externe host
- `rexec -n -h remote_host ls -l` : redirect `stdin` van `/dev/null` op een externe host
- `rexec -P 1234 -h remote_host ls -l` : specificeer de externe Poort op een externe host
