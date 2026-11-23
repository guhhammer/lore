# elink

Look up precomputed neighbors within a database, or find associated records in other databases. It is part of the `edirect` package.

## Usage
- `esearch -db pubmed -query "selective serotonin reuptake inhibitor" | elink -target nuccore` : search pubmed then find related sequences
- `esearch -db nuccore -query "insulin [PROT] AND rodents [ORGN]" | elink -target biosample` : search nucleotide then find related biosamples

## Categories
databases, packages

## Links
- More information: https://www.ncbi.nlm.nih.gov/books/NBK179288/
- tldr page: https://tldr.inbrowser.app/pages/linux/elink
