# snakefmt

Snakemake 파일을 포맷합니다.

## Usage
- `snakefmt 경로/대상/snakefile` : 특정 Snakefile 포맷
- `snakefmt 경로/대상/폴더` : 특정 디렉토리 내 모든 Snakefile을 재귀적으로 포맷
- `snakefmt --config 경로/대상/구성.toml 경로/대상/snakefile` : 특정 구성 파일을 사용하여 파일 포맷
- `snakefmt --line-length 100 경로/대상/snakefile` : 특정 최대 줄 길이를 사용하여 파일 포맷
- `snakefmt --diff 경로/대상/snakefile` : 변경 사항을 수행하지 않고 표시만 하기 (드라이런)

## Links
- More information: https://github.com/snakemake/snakefmt#usage
