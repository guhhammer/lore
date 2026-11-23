# go

Outil de gestion du code source Go. Certaines sous-commandes telles que `build` ont leur propre documentation d'utilisation.

## Usage
- `go get chemin_du_paquet` : télécharge et installe un paquet, spécifié par son chemin d'importation
- `go run fichier.go` : compile et exécute un fichier source (il doit contenir un paquet `main`)
- `go build -o executable fichier.go` : compile un fichier source dans un exécutable nommé
- `go build` : compile le paquet présent dans le répertoire courant
- `go test` : exécute tous les cas de test du paquet courant (les fichiers doivent se terminer par `_test.go`)
- `go install` : compile et installe le paquet actuel
- `go mod init nom_du_module` : initialise un nouveau module dans le répertoire courant

## Links
- More information: https://pkg.go.dev/cmd/go
