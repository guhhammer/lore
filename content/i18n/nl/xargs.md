# xargs

Voer een commando uit met doorgegeven argumenten van een ander commando, een bestand, etc. De invoer wordt behandeld als een enkel tekstblok en gesplitst in afzonderlijke stukken op spaties, tabbladen, nieuwe regels en einde-van-bestand.

## Usage
- `argumenten_bron | xargs commando` : voer een commando uit met de invoergegevens als argumenten
- `argumenten_bron | xargs sh -c "commando1 && commando2 | commando3"` : voer meerdere gekoppelde commando's uit op de invoergegevens
- `argumenten_bron | xargs -n 1 commando` : voer een nieuw commando uit met elk argument
- `argumenten_bron | xargs -P 10 -n 1 commando` : verhoog het limiet voor parallelle processen naar 10 (standaard is 1; 0 betekent zoveel mogelijk processen)
- `argumenten_bron | xargs -I _ commando _ optionele_extra_argumenten` : voer het commando één keer uit voor elke invoerregel, waarbij elke plaatsaanduiding (hier gemarkeerd als `_`) wordt vervangen door de invoerregel
- `argumenten_bron | xargs -p commando` : vraag de gebruiker om bevestiging voordat de opdracht wordt uitgevoerd (bevestig met `y` of `Y`)
- `xargs -a pad/naar/bestand commando` : lees een bestand voor argumenten die worden doorgegeven aan een commando
- `argumenten_bron | xargs -o commando` : geef het commando toegang tot de terminal voor interactieve invoer

## Links
- More information: https://www.gnu.org/software/findutils/manual/find.html#xargs-options
