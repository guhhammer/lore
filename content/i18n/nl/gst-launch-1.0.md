# gst-launch-1.0

Bouw en voer een GStreamer-pipeline uit.

## Usage
- `gst-launch-1.0 videotestsrc ! autovideosink` : speel een testvideo af in een venster
- `gst-launch-1.0 audiotestsrc -v ! autoaudiosink` : speel testaudio af en schakel uitgebreide uitvoer in
- `gst-launch-1.0 playbin uri=protocol://host/pad/naar/bestand` : speel een mediabestand af in een venster
- `gst-launch-1.0 filesrc location=pad/naar/bestand ! bestand_typedemux ! codec_typedec ! codec_typeenc ! bestand_typemux ! filesink location=pad/naar/bestand` : codeer een mediabestand opnieuw
- `gst-launch-1.0 filesrc location=pad/naar/bestand ! rtspclientsink location=rtsp://host_ip/pad/naar/bestand` : stream een bestand naar een RTSP-server
- `gst-launch-1.0 -e videotestsrc ! x264enc ! mp4mux ! filesink location=pad/naar/bestand.mp4` : forceer een End Of Stream-gebeurtenis als de pipeline wordt afgesloten met `<Ctrl c>`, voor containers die finalisatie vereisen zoals `mp4`
- `gst-launch-1.0 -e videotestsrc ! x264enc ! element_naam. audiotestsrc ! opusenc ! element_naam. matroskamux name=element_naam ! filesink location=pad/naar/bestand.mkv` : multiplex testvideo en testaudio samen in één bestand
- `GST_DEBUG=1..5 GST_DEBUG_DUMP_DOT_DIR=pad/naar/map gst-launch-1.0 pipeline` : schakel debug-uitvoer in en dump een pipeline naar een `.dot`-bestand dat vervolgens gerenderd kan worden met tools zoals `dot`

## Links
- More information: https://gstreamer.freedesktop.org/documentation/tools/gst-launch.html
