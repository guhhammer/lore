# xml canonic

Maak XML-documenten canoniek.

## Usage
- `xml c14n pad/naar/invoer.xml|uri >pad/naar/uitvoer.xml` : maak een XML-document canoniek met behoud van commentaar
- `xml c14n --without-comments pad/naar/invoer.xml|uri >pad/naar/uitvoer.xml` : maak een XML-document canoniek en verwijder het commentaar
- `xml c14n --exc-with-comments pad/naar/invoer.xml|uri pad/naar/c14n.xpath` : maak XML uitsluitend canoniek, met behulp van een XPATH vanuit een bestand, met behoud van commentaar
- `xml c14n --help` : toon de help

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139560880
