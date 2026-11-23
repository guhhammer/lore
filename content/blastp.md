# blastp

Protein-Protein BLAST.

## Usage
- `blastp -query query.fa -subject subject.fa -evalue 1e-9` : align two or more sequences using blastp, with the e-value threshold of 1e-9, pairwise output format, output to screen
- `blastp -task blastp-fast -query query.fa -subject subject.fa` : align two or more sequences using blastp-fast
- `blastp -query query.fa -subject subject.fa -outfmt '6 qseqid qlen qstart qend sseqid slen sstart send bitscore evalue pident' -out output.tsv` : align two or more sequences, custom tabular output format, output to file
- `blastp -query query.fa -db blast_database_name -num_threads 16 -max_target_seqs 10` : search protein databases using a protein query, 16 threads to use in the BLAST search, with a maximum number of 10 aligned sequences to keep
- `blastp -query query.fa -db nr -remote` : search the remote non-redundant protein database using a protein query
- `blastp -h` : display help (use `-help` for detailed help)

## Categories
databases

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/blastp
