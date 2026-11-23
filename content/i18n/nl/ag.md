# ag

The Silver Searcher. Zoals `ack`, maar wil sneller zijn.

## Usage
- `ag string` : zoek bestanden die `string` bevatten en druk de regelovereenkomsten in context af
- `ag string pad/naar/map` : vind bestanden die `string` bevatten in een specifieke map
- `ag -l string` : vind bestanden die `string` bevatten, maar vermeld alleen de bestandsnamen
- `ag -i -o STRING` : vind bestanden die `STRING` niet hoofdlettergevoelig bevatten en druk alleen de overeenkomst af in plaats van de hele regel
- `ag string -G file_name` : zoek `string` in bestanden met een naam die overeenkomt met `file_name`
- `ag '^ka(t|r)$'` : vind bestanden waarvan de inhoud overeenkomt met een `regex`
- `ag -g string` : zoek bestanden met een naam die overeenkomt met `string`

## Links
- More information: https://manned.org/ag
