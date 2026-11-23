# instaloader

Download foto's, video's, bijschriften en andere metadata van Instagram. Opmerking: je moet Instagram-inloggegevens opgeven als je media in hoge kwaliteit wilt downloaden.

## Usage
- `instaloader profiel_naam` : download een profiel
- `instaloader --highlights profiel_naam` : download highlights
- `instaloader -q -G profiel_naam` : download berichten met geotags (indien beschikbaar), zonder gebruikersinteractie
- `instaloader --user-agent user_agent profiel_naam` : specificeer een user agent voor HTTP-verzoeken
- `instaloader -l gebruikersnaam -p wachtwoord profiel_naam` : specificeer inloggegevens en download berichten (handig voor privéprofielen)
- `instaloader -F profiel_naam` : sla een doel over als het eerst gedownloade bestand al gevonden is (handig om Instagram-archieven bij te werken)
- `instaloader -l gebruikersnaam -p wachtwoord -s --igtv profiel_naam` : download verhalen en IGTV-video's (inloggen vereist)
- `instaloader -l gebruikersnaam -p wachtwoord -s --igtv --highlights profiel_naam` : download alle soorten berichten (inloggen vereist)

## Links
- More information: https://instaloader.github.io/cli-options.html
