# prctl

Lees of configureer de resource controls van lopende processen, taken en projecten.

## Usage
- `prctl proces_id` : bekijk de proceslimieten en -rechten
- `prctl -P proces_id` : bekijk de proceslimieten en -rechten in een machineleesbaar formaat
- `prctl -n process.max-file-descriptor proces_id` : verkrijg een specifieke limiet voor een lopend proces

## Links
- More information: https://www.unix.com/man-page/sunos/1/prctl
