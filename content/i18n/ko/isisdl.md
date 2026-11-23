# isisdl

TU-Berlin의 ISIS에서 파일과 영상을 다운로드하는 유틸리티.

## Usage
- `isisdl` : 동기화 프로세스 시작
- `isisdl --download-rate 20 --max-num-threads 5` : 다운로드 속도를 20 MiB/s로 제한하고 5개의 스레드로 다운로드
- `isisdl --init` : 초기 설정 마법사 실행
- `isisdl --config` : 추가 설정 마법사 실행
- `isisdl --sync` : 데이터베이스 전체 동기화 초기화 및 모든 파일의 체크섬 계산
- `isisdl --compress` : 다운로드된 영상 압축을 위해 ffmpeg 실행

## Links
- More information: https://github.com/Emily3403/isisdl
