# brotli

Brotli 압축을 사용하여 파일을 압축/압축 해제.

## Usage
- `brotli 경로/대상/파일` : 파일을 압축하여 파일 옆에 압축 버전을 생성
- `brotli -d 경로/대상/파일.br` : 파일을 압축 해제(decompress)하여, 파일 옆에 압축되지 않은 버전을 생성
- `brotli 경로/대상/파일 -o 경로/대상/압축된_출력_파일.br` : 출력(output) 파일 이름을 지정하여, 파일을 압축
- `brotli -d 경로/대상/압축된_파일.br -o 경로/대상/출력_파일` : 출력(output) 파일 이름을 지정하여 Brotli 파일을 압축 해제(decompress)ㄴ
- `brotli -q 11 경로/대상/파일 -o 경로/대상/압축된_출력_파일.br` : 압축 품질을 지정 (1=가장 빠름 (가장 안 좋은 품질), 11=가장 느림 (가장 좋은 품질))

## Links
- More information: https://manned.org/brotli
