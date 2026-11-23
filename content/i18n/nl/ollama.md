# ollama

Een groot taalmodel-runner.

## Usage
- `ollama serve` : start de daemon die vereist is om andere commando's uit te voeren
- `ollama run model` : voer een model uit en praat ermee (het model wordt automatisch gedownload als het nog niet is gedownload)
- `ollama run model --think=false "prompt"` : voer een model uit met één prompt en zet nadenken uit
- `ollama ls` : toon gedownloade modellen
- `ollama pull model` : pull een specifiek model
- `ollama ps` : toon actieve modellen
- `ollama rm model` : verwijder een model
- `ollama create nieuwe_model_naam -f pad/naar/Modelfile` : maak een model van een `Modelfile`

## Links
- More information: https://ollama.com/library
