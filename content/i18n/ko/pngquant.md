# pngquant

PNG 변환기 및 손실 이미지 압축기.

## Usage
- `pngquant 경로/대상/파일.png` : 특정 PNG를 최대한 압축하고 결과를 새 파일로 저장
- `pngquant --ext .png -f 경로/대상/파일.png` : 특정 PNG를 압축하고 원본을 덮어쓰기
- `pngquant -Q 0-100 경로/대상/파일.png` : 사용자 지정 품질로 특정 PNG를 압축 시도 (최소 값보다 낮으면 건너뜀)
- `pngquant 64 경로/대상/파일.png` : 색상이 64개로 줄어진 특정 PNG를 압축
- `pngquant --skip-if-larger 경로/대상/파일.png` : 특정 PNG를 압축하고 파일이 원본보다 큰 경우 건너뜀
- `pngquant --strip 경로/대상/파일.png` : 특정 PNG를 압축하고 메타데이터 제거
- `pngquant 경로/대상/파일.png -o 경로/대상/파일.png` : 특정 PNG를 압축하고 지정된 경로에 저장
- `pngquant -v 경로/대상/파일.png` : 특정 PNG를 압축하고 진행 상황 표시

## Links
- More information: https://manned.org/pngquant
