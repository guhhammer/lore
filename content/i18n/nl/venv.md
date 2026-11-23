# venv

Maak lichtgewicht virtuele omgevingen in Python.

## Usage
- `python -m venv pad/naar/virtuele_omgeving` : maak een Python virtuele omgeving aan
- `. pad/naar/virtuele_omgeving/bin/activate` : activeer de virtuele omgeving (Linux en macOS)
- `pad\naar\virtuele_omgeving\Scripts\activate.bat` : activeer de virtuele omgeving (Windows)
- `deactivate` : deactiveer de virtuele omgeving
- `alias venv='python -m venv .venv && source .venv/bin/activate|.venv\Scripts\activate.bat'` : maak een alias aan die een `venv`-map genereert en deze automatisch activeert

## Links
- More information: https://docs.python.org/library/venv.html
