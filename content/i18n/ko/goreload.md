# goreload

Go 프로그램용 라이브 리로드 유틸리티.

## Usage
- `goreload -b 경로/대상/바이너리 경로/대상/파일.go` : 바이너리 파일 보기 (기본값은 `.goreload`)
- `goreload --logPrefix prefix 경로/대상/파일.go` : 사용자 정의 로그 접두사를 설정 (기본값은 `goreload`)
- `goreload --all` : 파일이 변경될 때마다 다시 로드

## Links
- More information: https://github.com/acoshift/goreload#basic-usage
