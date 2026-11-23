# cwebp

이미지 파일을 WebP 파일로 압축.

## Usage
- `cwebp 경로/대상/이미지_파일 -o 경로/대상/출력파일.webp` : 기본 설정(손실 압축, q = 75)으로 WebP 파일을 압축해 출력(output) 파일로 저장
- `cwebp 경로/대상/이미지_파일 -o 경로/대상/출력파일.webp -q 100` : 최고 품질(quality)과 최대 파일 크기로 WebP 파일을 압축
- `cwebp 경로/대상/이미지_파일 -o 경로/대상/출력파일.webp -q 0` : 최저 품질(quality)과 최소 파일 크기로 WebP 파일을 압축
- `cwebp 경로/대상/이미지_파일 -o 경로/대상/출력파일.webp -z 9` : 무손실 압축으로 가능한 한 작은 파일 크기로 WebP 파일을 생성
- `cwebp 경로/대상/이미지_파일 -o 경로/대상/출력파일.webp -resize 너비 높이` : webP 파일을 압축하고 이미지 크기를 조정 (너비 또는 높이가 0이면 종횡비 유지)
- `cwebp 경로/대상/이미지_파일 -o 경로/대상/출력파일.webp -noalpha` : webP 파일을 압축하고 알파 채널 정보를 삭제

## Links
- More information: https://developers.google.com/speed/webp/docs/cwebp
