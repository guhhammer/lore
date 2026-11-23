# iw dev

Toon en manipuleer draadloze apparaten.

## Usage
- `sudo iw dev wlanX set type monitor` : zet het apparaat in monitor-modus (interface moet eerst uit staan. Zie ook: `ip link`)
- `sudo iw dev wlanX set type managed` : zet het apparaat in managed-modus (interface moet eerst uit staan)
- `sudo iw dev wlanX set channel kanaal_nummer` : stel het Wi-Fi-kanaal van het apparaat in (apparaat moet eerst in monitor-modus staan met de interface aan)
- `sudo iw dev wlanX set freq freq_in_mhz` : stel de Wi-Fi-frequentie van het apparaat in MHz in (apparaat moet eerst in monitor-modus staan met de interface aan)
- `iw dev wlanX station dump` : toon alle bekende station-informatie
- `sudo iw dev wlanX interface add "vif_naam" type monitor addr 12:34:56:aa:bb:cc` : maak een virtuele interface in monitor-modus met een specifiek MAC-adres
- `sudo iw dev "vif_naam" del` : verwijder een virtuele interface
