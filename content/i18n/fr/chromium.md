# chromium

Navigateur Web open source principalement développé et maintenu par Google. Remarque : Vous devrez peut-être remplacer la commande `chromium` par le navigateur web de votre choix, tel que `brave`, `google-chrome`, `opera`, ou `vivaldi`.

## Usage
- `chromium https://example.com|chemin/vers/fichier.html` : ouvre une URL ou un fichier spécifique
- `chromium --incognito example.com` : ouvre en mode navigation privée
- `chromium --new-window example.com` : ouvre dans une nouvelle fenêtre
- `chromium --app=https://example.com` : ouvre en mode application (sans barres d'outils, barre d'URL, boutons, etc)
- `chromium --proxy-server="://hostname:66" example.com` : utilise un serveur proxy
- `chromium --user-data-dir=chemin/vers/répertoire` : ouvre dans un répertoire de profil personnalisé
- `chromium --user-data-dir=chemin/vers/répertoire --disable-web-security` : ouvre sans validation CORS (utile pour tester une API)
- `chromium --auto-open-devtools-for-tabs` : ouvre avec une fenêtre outils de développement pour chaque onglet ouvert

## Links
- More information: https://www.chromium.org/developers/how-tos/run-chromium-with-flags/
