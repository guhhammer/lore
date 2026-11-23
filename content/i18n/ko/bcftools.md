# bcftools

VCF 및 BCF 파일을 조작하기 위한 도구 모음.

## Usage
- `bcftools view 경로/대상/입력파일.bcf -O v` : BCF 파일을 확인하고 `stdout`으로 vCF 형식으로 변환
- `bcftools sort 경로/대상/입력파일.vcf.gz -O b -o 경로/대상/sorted.bcf -W` : VCF 파일의 변이를 염색체와 위치 기준으로 정렬하고, bCF 파일로 출력 후 인덱스 생성
- `bcftools concat 경로/대상/chr1.vcf.gz 경로/대상/chr2.vcf.gz ... -O z` : 동일한 샘플을 공유하는 정렬된 VCF 파일들을 압축(zipped) VCF로 `stdout`에 결합
- `bcftools filter -e 'QUAL<20' -s LowQual 경로/대상/입력파일.vcf.gz` : 낮은 품질 변이를 필터링하고 FILTER 컬럼에 "LowQual" 태그 추가
- `bcftools annotate -a 경로/대상/annotations.tsv.gz -c CHROM,POS,REF,ALT,INFO/AF 경로/대상/입력파일.vcf.gz` : tabix 인덱스된 테이블에서 주석 컬럼을 추가해 `stdout`으로 출력
- `bcftools isec 경로/대상/a.vcf.gz 경로/대상/b.vcf.gz ... --threads 4 -o 경로/대상/intersection.vcf` : VCF 파일 간 변이 교집합(inter[sec]tion)을 4개 스레드로 계산하여 출력
- `bcftools merge 경로/대상/cohort1.vcf.gz 경로/대상/cohort2.vcf.gz --no-index` : 인덱스 없이 VCF 파일들의 비중복 샘플 병합 `stdout`
- `bcftools index 경로/대상/입력파일.vcf.gz` : bgzip으로 압축된 VCF 파일에 대한 인덱스 생성

## Links
- More information: https://samtools.github.io/bcftools/bcftools.html
