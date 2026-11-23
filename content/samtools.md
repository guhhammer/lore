# samtools

Tools for handling high-throughput sequencing (genomics) data. Used for reading/writing/editing/indexing/viewing of data in SAM/BAM/CRAM format.

## Usage
- `samtools view -S -b input.sam >output.bam` : convert a SAM input file to BAM stream and save to file
- `other_command | samtools view -h - chromosome:start-end` : take input from `stdin` (-) and print the SAM header and any reads overlapping a specific region to `stdout`
- `samtools sort input -o output.bam` : sort file and save to BAM (the output format is automatically determined from the output file's extension)
- `samtools index sorted_input.bam` : index a sorted BAM file (creates `sorted_input.bam.bai`)
- `samtools flagstat sorted_input` : print alignment statistics about a file
- `samtools idxstats sorted_indexed_input` : count alignments to each index (chromosome/contig)
- `samtools merge output input1 input2 ...` : merge multiple files
- `samtools split merged_input` : split input file according to read groups

## Categories
files

## Links
- More information: https://www.htslib.org/doc/samtools.html
- tldr page: https://tldr.inbrowser.app/pages/common/samtools
