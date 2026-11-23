# cmake

Cross-platform bouwautomatiseringssysteem dat recepten genereert voor native bouwsystemen.

## Usage
- `cmake pad/naar/projectmap` : genereer een bouwrecept in de huidige map met `CMakeLists.txt` van een projectmap
- `cmake --build pad/naar/bouwmap` : gebruik een gegenereerd recept in een bepaalde map om artefacten te bouwen
- `cmake --install pad/naar/bouwmap --strip` : installeer de bouwartefacten in `/usr/local/` en verwijder debug-symbolen
- `cmake pad/naar/projectmap -D CMAKE_BUILD_TYPE=Release` : genereer een bouwrecept met bouwtype ingesteld naar `Release` met CMake-variabele
- `cmake -G generator_naam pad/naar/projectmap` : genereer een bouwrecept met `generator_naam` als onderliggend bouwsysteem
- `cmake --install pad/naar/bouwmap --strip --prefix pad/naar/map` : installeer de bouwartefacten met een aangepast voorvoegsel voor paden
- `cmake --build pad/naar/bouwmap -t doelnaam` : voer een aangepast bouwdoel uit
- `cmake -h` : toon de help

## Links
- More information: https://cmake.org/cmake/help/latest/manual/cmake.1.html
