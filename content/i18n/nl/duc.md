# duc

Een verzameling van tools voor het indexeren, inspecteren en visualiseren van schijfgebruik. Duc onderhoudt een database van geaccumuleerde groottes van mappen van het bestandssysteem, waardoor je deze database kunt raadplegen of mooie grafieken kunt maken om te laten zien waar de data zich bevindt.

## Usage
- `duc index /usr` : indexeer de `/usr` map en schrijf naar de standaard databaselocatie `~/.duc.db`
- `duc ls -Fg /usr/local` : toon alle bestanden en mappen onder `/usr/local` en toon relatieve bestandsgroottes in een grafiek
- `duc ls -Fg -R /usr/local` : toon alle bestanden en mappen onder `/usr/local` recursief met behulp van boomweergave
- `duc gui /usr` : start de grafische interface om het bestandssysteem te verkennen met behulp van zonnestraalgrafieken
- `duc ui /usr` : start de ncurses-consoleinterface om het bestandssysteem te verkennen
- `duc info` : dump databaseinformatie
