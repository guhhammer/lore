# fsck

Controlla l'integrità di un filesystem o lo ripara. Il filesystem non dev'essere montato al momento in cui il comando viene eseguito.

## Usage
- `sudo fsck /dev/sdX` : controlla il filesystem `/dev/sdX`, riportando eventuali blocchi danneggiati
- `sudo fsck -r /dev/sdX` : controlla il filesystem `/dev/sdX`, riportando eventuali blocchi danneggiati e per ognuno consente all'utente di scegliere interattivamente se ripararlo
- `sudo fsck -a /dev/sdX` : controlla il filesystem `/dev/sdX`, riportando eventuali blocchi danneggiati e riparandoli automaticamente

## Links
- More information: https://manned.org/fsck
