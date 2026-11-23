# vegeta

Un utilitaire de ligne de commande et une bibliothèque pour les tests de charge HTTP.

## Usage
- `echo "GET https://example.com" | vegeta attack -duration=30s` : lance une attaque d'une durée de 30 secondes
- `echo "GET https://example.com" | vegeta attack -insecure -duration=30s` : lance une attaque sur un serveur avec un certificat HTTPS auto-signé
- `echo "GET https://example.com" | vegeta attack -duration=30s -rate=10` : lance une attaque avec un taux de 10 demandes par seconde
- `echo "GET https://example.com" | vegeta attack -duration=30s | vegeta report` : lance une attaque et affiche un rapport
- `echo "GET https://example.com" | vegeta attack -duration=30s | vegeta plot >chemin/au/results.html` : lance une attaque et reporte les résultats sur un graphique (latence en fonction du temps)
- `vegeta attack -duration=30s -targets=requetes.txt | vegeta report` : lance une attaque contre plusieurs URL à partir d'un fichier

## Links
- More information: https://github.com/tsenart/vegeta#usage-manual
