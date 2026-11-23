# birdc

Bird remote control. Kommandozeilenwerkzeug zum Abrufen von Informationen wie Routen von bird und zur Durchführung von Konfigurationen während der Laufzeit.

## Usage
- `birdc` : öffne die remote control Konsole
- `birdc configure` : lade die Konfiguration neu, ohne Bird neu zu starten
- `birdc show status` : zeige den aktuellen Status von Bird an
- `birdc show protocols` : zeige alle konfigurierten Protokolle an
- `birdc show protocols upstream1 all` : zeige alle Details zu einem Protokoll an
- `birdc "show route where bgp_path ~ [4242120045]"` : zeige alle Routen an, die eine bestimmte AS-Nummer enthalten
- `birdc show route primary` : zeige alle besten Routen an
- `birdc show route for fd00:/8 all` : zeige alle Details zu allen Routen von einem bestimmten Präfix an

## Links
- More information: https://bird.network.cz/?get_doc&v=30&f=bird-4.html
