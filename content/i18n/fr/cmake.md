# cmake

Système de construction logicielle multiplateforme, qui permet de générer des recettes de construction pour les systèmes de construction natifs.

## Usage
- `cmake chemin/vers/le/répertoire_du_projet` : génère une recette de construction `CMakeLists.txt` depuis le répertoire d'un projet
- `cmake chemin/vers/le/répertoire_du_projet -D CMAKE_BUILD_TYPE=Release` : génère une recette de construction, en définissant le type de construction à `Release` à l'aide d'une variable CMake
- `cmake --build chemin/vers/le/répertoire_de_construction` : utilise une recette déjà générée dans un répertoire donné pour construire les artefacts
- `cmake --install chemin/vers/le/répertoire_de_construction --strip` : installe les artefacts de construction dans `/usr/local/` et retirer les symboles de débogage
- `cmake --install chemin/vers/le/répertoire_de_construction --strip --prefix chemin/vers/le/répertoire` : installe les artefacts de construction en utilisant un préfixe personnalisé pour les chemins
- `cmake --build chemin/vers/le/répertoire_de_construction --target nom_de_la_cible` : lance une cible de construction personnalisée

## Links
- More information: https://cmake.org/cmake/help/latest/manual/cmake.1.html
