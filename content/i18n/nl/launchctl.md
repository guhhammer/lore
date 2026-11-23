# launchctl

Beheer Apple's `launchd` manager voor launch daemons (systeembrede diensten) en launch agents (programma's per gebruiker). `launchd` laadt op XML gebaseerde `*.plist`-bestanden die op de juiste locaties zijn geplaatst, en voert de corresponderende commando's uit volgens hun gedefinieerde schema.

## Usage
- `launchctl load ~/Library/LaunchAgents/my_script.plist` : activeer een gebruikersspecifieke agent die in `launchd` moet worden geladen wanneer de gebruiker inlogt
- `sudo launchctl load /Library/LaunchAgents/root_script.plist` : activeer een agent die root-rechten vereist om te kunnen werken en/of moet worden geladen wanneer elke gebruiker inlogt (let op de afwezigheid van `~` in het pad)
- `sudo launchctl load /Library/LaunchDaemons/system_daemon.plist` : activeer een systeembrede daemon die moet worden geladen wanneer het systeem opstart (zelfs als er geen gebruiker inlogt)
- `launchctl list` : toon alle geladen agenten/daemons, met de PID als het proces dat ze specificeren momenteel actief is, en de afsluitcode die de laatste keer dat ze werden uitgevoerd is teruggegeven
- `launchctl unload ~/Library/LaunchAgents/my_script.plist` : ontlaad een momenteel geladen agent, b.v. om wijzigingen aan te brengen (Let op: het plist-bestand wordt automatisch in `launchd` geladen na een herstart en/of inloggen)
- `launchctl start script_bestand` : voer handmatig een bekende (geladen) agent/daemon uit, zelfs als dit niet het juiste moment is (Let op: deze opdracht gebruikt het label van de agent, in plaats van de bestandsnaam)
- `launchctl stop script_bestand` : beëindig handmatig het proces dat is gekoppeld aan een bekende agent/daemon, als deze actief is

## Links
- More information: https://keith.github.io/xcode-man-pages/launchctl.1.html
