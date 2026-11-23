# znew

`.Z` 파일을 gzip 형식으로 다시 압축.

## Usage
- `znew 경로/대상/파일1.Z` : 파일을 `.Z`에서 gzip 형식으로 다시 압축
- `znew -v 경로/대상/파일1.Z 경로/대상/파일2.Z ...` : 여러 파일을 다시 압축하고 각 파일의 크기 감소율을 표시
- `znew -9 경로/대상/파일1.Z` : 가장 느린 압축 방법을 사용하여 파일을 다시 압축 (최적의 압축을 위해)
- `znew -K 경로/대상/파일1.Z` : gzip 파일보다 작으면 `.Z` 파일을 Keeping하여 파일을 다시 압축

## Links
- More information: https://manned.org/znew
