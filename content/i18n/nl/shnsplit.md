# shnsplit

Splitst audiobestanden volgens een `.cue` bestand. Meer informatie: <http://shnutils.freeshell.org/shntool/>.

## Usage
- `shnsplit -f pad/naar/bestand.cue pad/naar/bestand.wav` : splits een `.wav` + `.cue` bestand in meerdere bestanden
- `shnsplit -a` : toon ondersteunde formaten
- `shnsplit -f pad/naar/bestand.cue -o flac pad/naar/bestand.flac` : splits een `.flac` bestand in meerdere bestanden
- `shnsplit -f pad/naar/bestand.cue pad/naar/bestand.wav -t "%n - %a - %t"` : splits een `.wav` bestand in meerdere bestanden in de vorm van "track-nummer - album - titel"
