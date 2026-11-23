# logwatch

Fasst viele verschiedene Logs für gängige Dienste (z.B. Apache, pam_unix, sshd, usw.) in einem einzelnen Bericht zusammen.

## Usage
- `logwatch --range yesterday|today|all|help --detail low|medium|others` : analysiere Logs für einen Zeitraum mit einer bestimmten Detailtiefe
- `logwatch --range all --service apache|pam_unix|etc` : beschränke den Bericht auf Informationen zu einem ausgewählten Dienst

## Links
- More information: https://manned.org/logwatch
