# bcftools

Tools for manipulating VCF and BCF files.

## Usage
- `bcftools view path/to/input.bcf -O v` : view BCF file and convert to vCF on `stdout`
- `bcftools sort path/to/input.vcf.gz -O b -o path/to/sorted.bcf -W` : sort a VCF file variants by chromosome and position, output to a bCF file, and index the sorted output
- `bcftools concat path/to/chr1.vcf.gz path/to/chr2.vcf.gz ... -O z` : concatenate sorted VCF files that share the same samples to zipped VCF on `stdout`
- `bcftools filter -e 'QUAL<20' -s LowQual path/to/input.vcf.gz` : filter for low quality variants and annotate with "LowQual" tag in the FILTER column
- `bcftools annotate -a path/to/annotations.tsv.gz -c CHROM,POS,REF,ALT,INFO/AF path/to/input.vcf.gz` : add annotated columns from a tabix-indexed table on `stdout`
- `bcftools isec path/to/a.vcf.gz path/to/b.vcf.gz ... --threads 4 -o path/to/intersection.vcf` : output variant inter[sec]tion between VCF files using 4 threads
- `bcftools merge path/to/cohort1.vcf.gz path/to/cohort2.vcf.gz --no-index` : merge non-overlapping samples from VCF files without indices on `stdout`
- `bcftools index path/to/input.vcf.gz` : create index for a bgzipped VCF file

## Categories
files

## Links
- More information: https://samtools.github.io/bcftools/bcftools.html
- tldr page: https://tldr.inbrowser.app/pages/common/bcftools
