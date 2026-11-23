# ospp.vbs

Installeer, activeer en beheer volumelicentieversies van Microsoft Office-producten. Gebruik `cscript` om dit programma via de CLI uit te voeren, of `wscript` voor de GUI. Opmerking: dit commando kan je huidige volume aan gelicentieerde Office-productversies overschrijven, deactiveren en/of verwijderen, ga dus voorzichtig te werk.

## Usage
- `cscript ospp.vbs /inpkey:productcode` : installeer een productcode (Opmerking: dit vervangt de bestaande code)
- `cscript ospp.vbs /unpkey:productcode_cijfers` : deïnstalleer een geïnstalleerde productcode met de laatste vijf cijfers van de productcode
- `cscript ospp.vbs /sethst:ip|hostnaam` : stel een KMS-hostnaam in
- `cscript ospp.vbs /setprt:poort` : stel een KMS-poort in
- `cscript ospp.vbs /act` : activeer geïnstalleerde Office-productcodes
- `cscript ospp.vbs /dstatus` : toon licentie-informatie voor geïnstalleerde productcodes
