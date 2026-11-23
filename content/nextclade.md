# nextclade

Bioinformatics tool for virus genome alignment, clade assignment, and qc checks.

## Usage
- `nextclade run path/to/sequences.fa -r path/to/reference.fa -o path/to/alignment.fa` : align sequences to user provided reference, outputting the alignment to a file
- `nextclade run path/to/fasta -d dataset_name -t path/to/report.tsv` : create a TSV report, auto-downloading the latest dataset
- `nextclade dataset list` : list all available datasets
- `nextclade dataset get -n sars-cov-2 -o path/to/directory` : download the latest SARS-CoV-2 dataset
- `nextclade run -D path/to/dataset_directory -O path/to/output_directory path/to/sequences.fasta` : use a downloaded dataset, producing all outputs
- `nextclade run -d dataset_name -t path/to/output_tsv -- path/to/input_fasta_1 path/to/input_fasta_2 ...` : run on multiple files
- `nextclade run --retry-reverse-complement -d dataset_name -t path/to/output_tsv path/to/input_fasta` : try reverse complement if sequence does not align

## Categories
files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/nextclade
