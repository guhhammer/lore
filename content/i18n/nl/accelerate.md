# accelerate

Accelerate is een bibliotheek waarmee dezelfde PyTorch-code kan worden uitgevoerd op elke gedistribueerde configuratie.

## Usage
- `accelerate env` : toon informatie over de omgeving
- `accelerate config` : maak interactief een configuratiebestand
- `accelerate estimate-memory name/model` : toon de geschatte GPU-geheugenkosten van het uitvoeren van een Hugging Face model met verschillende gegevenstypen
- `accelerate test --config_file pad/naar/config.yaml` : test een Accelerate configuratiebestand
- `accelerate launch pad/naar/script.py --cpu` : voer een model uit op CPU met Accelerate
- `accelerate launch pad/naar/script.py --multi_gpu --num_machines 2` : voer een model uit op multi-GPU met Accelerate, met 2 machines

## Links
- More information: https://huggingface.co/docs/accelerate/index
