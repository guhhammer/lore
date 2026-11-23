# slmgr.vbs

Installeer, activeer en beheer Windows-licenties. Gebruik `cscript` om dit programma als CLI uit te voeren, of `wscript` voor een GUI. Opmerking: dit commando kan uw huidige Windows-licentie overschrijven, deactiveren en/of verwijderen, ga dus met voorzichtigheid verder.

## Usage
- `cscript slmgr.vbs /dli` : toon (d) de huidige Windows-licentie-informatie
- `cscript slmgr.vbs /dti` : toon (d) de installatie-iD voor het huidige apparaat. Nuttig voor offline licentieactivatie
- `cscript slmgr.vbs /xpr` : toon de verloopdatum ([xpr]) en -tijd van de huidige licentie
- `cscript slmgr.vbs /ipk product_sleutel` : installeer een nieuwe productsleutel (k) voor een Windows-licentie. Vereist beheerdersrechten en zal de bestaande licentie overschrijven
- `cscript slmgr.vbs /ato` : activeer de Windows-productlicentie online. Vereist beheerdersrechten
- `cscript slmgr.vbs /atp bevestigings_id` : activeer de Windows-productlicentie offline. Vereist beheerdersrechten en een bevestigings-ID verstrekt door Microsoft Product Activation Center
- `cscript slmgr.vbs /cpky` : wis (c) de productsleutel ([ky]) van de huidige licentie uit het Windows-register. Dit zal de huidige licentie niet deactiveren of verwijderen, maar voorkomt dat de sleutel in de toekomst wordt gestolen door kwaadaardige programma's
- `cscript slmgr.vbs /upk` : deïnstalleer (u) de huidige licentie (door zijn productsleutel (k))
