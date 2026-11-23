# pbmtext

텍스트를 PBM 이미지로 렌더링.

## Usage
- `pbmtext "Hello World!" >경로/대상/출력.pbm` : 한 줄의 텍스트를 PBM 이미지로 렌더링
- `echo "Hello\nWorld!" | pbmtext >경로/대상/출력.pbm` : 여러 줄의 텍스트를 PBM 이미지로 렌더링
- `pbmtext -f 경로/대상/폰트.pbm "Hello World!" >경로/대상/출력.pbm` : PBM 파일로 제공된 사용자 지정 폰트를 사용하여 텍스트 렌더링
- `echo "Hello\nWorld!" | pbmtext -s 3 -ls 10 >경로/대상/출력.pbm` : 문자와 줄 사이의 픽셀 수 지정

## Links
- More information: https://netpbm.sourceforge.net/doc/pbmtext.html
