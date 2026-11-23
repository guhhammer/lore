# bwa

Burrows-Wheeler Alignment tool. Short, low-divergent DNA sequences mapper against a large reference genome, such as the human genome.

## Usage
- `bwa index path/to/reference.fa` : index the reference genome
- `bwa mem -t 32 path/to/reference.fa path/to/read_single_end.fq.gz | gzip >path/to/alignment_single_end.sam.gz` : map single-end reads (sequences) to indexed genome using 32 threads and compress the result to save space
- `bwa mem -t 32 path/to/reference.fa path/to/read_pair_end_1.fq.gz path/to/read_pair_end_2.fq.gz | gzip >path/to/alignment_pair_end.sam.gz` : map pair-end reads (sequences) to the indexed genome using 32 threads and compress the result to save space
- `bwa mem -M -t 32 path/to/reference.fa path/to/read_pair_end_1.fq.gz path/to/read_pair_end_2.fq.gz | gzip >path/to/alignment_pair_end.sam.gz` : map pair-end reads (sequences) to the indexed genome using 32 threads with Marking shorter split hits as secondary for output SAM file compatibility in Picard software and compress the result
- `bwa mem -C -t 32 path/to/reference.fa path/to/read_pair_end_1.fq.gz path/to/read_pair_end_2.fq.gz | gzip >path/to/alignment_pair_end.sam.gz` : map pair-end reads (sequences) to indexed genome using 32 threads with FASTA/Q Comments (e.g. BC:Z:CGTAC) appending to a compressed result

## Categories
files, archives

## Links
- More information: https://manned.org/bwa
- tldr page: https://tldr.inbrowser.app/pages/linux/bwa
