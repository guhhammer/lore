# hf

Interactie met Hugging Face Hub. Log in, beheer de lokale cache, download of upload bestanden.

## Usage
- `hf auth login` : log in bij Hugging Face Hub
- `hf auth whoami` : toon de naam van de ingelogde gebruiker
- `hf auth logout` : log uit
- `hf env` : print informatie over de omgeving
- `hf download --repo-type repo_type repo_id bestandsnaam1 bestandsnaam2 ...` : download bestanden uit een repository en print het pad (laat bestandsnamen weg om de volledige repository te downloaden)
- `hf upload --repo-type repo_type repo_id pad/naar/lokaal_bestand_of_map pad/naar/repo_bestand_of_map` : upload een volledige map of een bestand naar Hugging Face
- `hf cache ls` : scan de cache om gedownloade repositories en hun schijfgebruik te bekijken

## Links
- More information: https://huggingface.co/docs/huggingface_hub/guides/cli
