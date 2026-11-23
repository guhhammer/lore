# timeout

Voer een commando uit met een tijdslimiet.

## Usage
- `timeout 3s sleep 10` : voer `sleep 10` uit en beëindig het na 3 seconden
- `timeout -s INT|HUP|KILL|... 5s sleep 10` : stuur een signaal naar het commando nadat de tijdslimiet is verlopen (standaard `TERM`, `kill -l` om alle signalen te tonen)
- `timeout -v 0.5s|1m|1h|1d|... commando` : stuur verbose output naar `stderr` en laat het signaal zien dat is verzonden bij een timeout
- `timeout -p 1s|1m|1h|1d|... commando` : behoud de exit status van het commando ongeacht of er een timeout is
- `timeout -k 5m 30s commando` : stuur een krachtig `KILL`-signaal na een bepaalde tijd als het commando het initiële signaal negeert bij een timeout
