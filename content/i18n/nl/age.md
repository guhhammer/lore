# age

Een eenvoudige, moderne en veilige tool voor het versleutelen van bestanden.

## Usage
- `age -p -o pad/naar/versleuteld_bestand pad/naar/niet-versleuteld_bestand` : genereer een versleuteld bestand dat kan worden ontsleuteld met een wachtwoordzin
- `age -r openbare_sleutel -o pad/naar/versleuteld_bestand pad/naar/niet-versleuteld_bestand` : versleutel een bestand met een of meer openbare sleutels die als letterlijke waarden worden ingevoerd (herhaal de `--recipient` flag om meerdere openbare sleutels op te geven)
- `age -R pad/naar/ontvangers_bestand -o pad/naar/versleuteld_bestand pad/naar/niet-versleuteld_bestand` : versleutel een bestand voor een of meer ontvangers van wie de openbare sleutels zijn opgegeven in een bestand (één per regel)
- `age -d -o pad/naar/ontsleuteld_bestand pad/naar/versleuteld_bestand` : ontsleutel een bestand met een wachtwoordzin
- `age -d -i pad/naar/privé_sleutel_bestand -o pad/naar/ontsleuteld_bestand pad/naar/versleuteld_bestand` : ontsleutel een bestand met een privésleutelbestand

## Links
- More information: https://github.com/FiloSottile/age#usage
