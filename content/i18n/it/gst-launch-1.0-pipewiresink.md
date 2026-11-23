# gst-launch-1.0 pipewiresink

Crea un nodo PipeWire da cui inviare dati.

## Usage
- `gst-launch-1.0 videotestsrc ! pipewiresink mode=provide` : invia video di test
- `gst-launch-1.0 videotestsrc ! pipewiresink mode=provide client-name=nome_nodo` : assegna un nome al nodo PipeWire

## Links
- More information: https://github.com/PipeWire/pipewire/tree/master/src/gst
