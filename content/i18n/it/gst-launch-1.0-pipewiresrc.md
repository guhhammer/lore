# gst-launch-1.0 pipewiresrc

Legge dati da un nodo PipeWire.

## Usage
- `gst-launch-1.0 pipewiresrc ! autoaudiosink` : ascolta il microfono predefinito
- `gst-launch-1.0 pipewiresrc target-object=nome_nodo ! autovideosink` : specifica il nodo da registrare e visualizza il video in una finestra
- `gst-launch-1.0 -e pipewiresrc ! videoconvert ! x264enc ! h264parse ! matroskamux ! filesink location=percorso/al/file.mkv` : registra video in un file
- `gst-launch-1.0 pipewiresrc num-buffers=1 ! videoconvert ! pngenc ! filesink location=percorso/al/file.png` : cattura uno screenshot da un nodo video
- `gst-launch-1.0 -e pipewiresrc ! opusenc ! oggmux ! filesink location=percorso/al/file.ogg` : registra audio in un file
- `gst-launch-1.0 pipewiresrc target-object=nome_nodo stream-properties=props,stream.capture.sink=true ! audioconvert ! fakesink` : registra il monitor di un dispositivo
- `gst-launch-1.0 -e pipewiresrc do-timestamp=true ! videoconvert ! x264enc ! h264parse ! mux. pipewiresrc do-timestamp=true ! opusenc ! mux. matroskamux name=mux ! filesink location=percorso/al/file.mkv` : multiplexa audio e video in un file

## Links
- More information: https://github.com/PipeWire/pipewire/tree/master/src/gst
