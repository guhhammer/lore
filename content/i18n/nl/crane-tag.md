# crane tag

Efficiënt taggen van een remote image zonder het te downloaden, wat verschilt van het `copy` commando. Het slaat de controles van laagbestaan over omdat we weten dat het manifest al bestaat, wat het iets sneller maakt.

## Usage
- `crane tag image_naam tag_naam` : tag een remote image
- `crane tag -h` : toon de help
