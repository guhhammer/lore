# xzdiff

`xz`, `lzma`, `gzip`, `bzip2`, `lzop`, 또는 `zstd`로 압축된 파일에 대해 `diff`를 실행. 지정된 모든 옵션은 `diff`에 직접 전달됩니다.

## Usage
- `xzdiff 경로/대상/파일1 경로/대상/파일2` : 두 파일 비교
- `xzdiff --side-by-side 경로/대상/파일1 경로/대상/파일2` : 두 파일을 비교하여 차이를 나란히 표시
- `xzdiff --brief 경로/대상/파일1 경로/대상/파일2` : 두 파일을 비교하고 다르다는 것만 보고(차이점 세부사항은 없음)
- `xzdiff --report-identical-files 경로/대상/파일1 경로/대상/파일2` : 두 파일을 비교하고 파일이 동일할 때 보고
- `xzdiff --paginate 경로/대상/파일1 경로/대상/파일2` : 페이지 단위로 결과를 보여주며 두 파일 비교

## Links
- More information: https://manned.org/xzdiff
