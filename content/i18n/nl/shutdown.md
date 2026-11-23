# shutdown

Een tool om een machine af te sluiten, te herstarten of af te melden.

## Usage
- `shutdown /s` : sluit de huidige machine af
- `shutdown /s /f` : sluit de huidige machine geforceerd af, waarbij alle applicaties worden gesloten
- `shutdown /r /t 0` : herstart de huidige machine onmiddellijk
- `shutdown /h` : zet de huidige machine in slaapstand
- `shutdown /l` : meld de huidige machine af
- `shutdown /s /t 8` : zet een timer in aantal seconden voor het afsluiten van de huidige machine
- `shutdown /a` : breek een afsluitsequentie af voordat de timer is afgelopen
- `shutdown /m \\hostnaam` : sluit een machine af op afstand
