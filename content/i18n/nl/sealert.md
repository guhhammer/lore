# sealert

Analyseer en leg SELinux AVC-weigeringsberichten uit. Onderdeel van het `setroubleshoot-server` pakket.

## Usage
- `sudo sealert -a /var/log/audit/audit.log` : analyseer alle recente SELinux-weigeringen
- `sudo sealert -l waarschuwings_id` : analyseer een specifieke waarschuwings-ID uit systeemlogs
- `sudo sealert -b` : toon een samenvatting van recente SELinux-waarschuwingen
- `sudo tail -f /var/log/audit/audit.log | sealert -l -` : monitor de auditlog in real-time op nieuwe waarschuwingen

## Links
- More information: https://manned.org/sealert
