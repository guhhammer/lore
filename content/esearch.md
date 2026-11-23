# esearch

Perform a new Entrez search using terms in indexed fields. It is part of the `edirect` package.

## Usage
- `esearch -db pubmed -query "selective serotonin reuptake inhibitor"` : search the pubmed database for selective serotonin reuptake inhibitor
- `esearch -db protein -query 'Escherichia*'` : search the protein database using a query and `regex`
- `esearch -db nuccore -query "insulin [PROT] AND rodents [ORGN]"` : search the nucleotide database for sequences whose metadata contain insulin and rodents
- `esearch -h` : display help

## Categories
databases, packages

## Links
- More information: https://www.ncbi.nlm.nih.gov/books/NBK179288/
- tldr page: https://tldr.inbrowser.app/pages/linux/esearch
