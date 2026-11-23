# mashtree

유전체로부터 빠르게 트리를 생성합니다. 계통수를 생성하지 않습니다.

## Usage
- `mashtree --numcpus 12 *.fastq.gz *.fasta >mashtree.dnd` : fastq 및/또는 fasta 파일로부터 여러 스레드를 사용하여 가장 빠르게 트리를 생성하고, newick 파일로 출력
- `mashtree --mindepth 0 --numcpus 12 *.fastq.gz *.fasta >mashtree.dnd` : fastq 및/또는 fasta 파일로부터 여러 스레드를 사용하여 가장 정확하게 트리를 생성하고, newick 파일로 출력
- `mashtree_bootstrap.pl --reps 100 --numcpus 12 *.fastq.gz -- --min-depth 0 >mashtree.bootstrap.dnd` : 신뢰값을 포함하여 트리를 가장 정확하게 생성 (참고: `mashtree` 자체의 옵션은 `--` 오른쪽에 위치해야 함)

## Links
- More information: https://github.com/lskatz/mashtree#usage
