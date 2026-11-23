# pkgctl diff

패키지 파일을 다양한 모드로 비교.

## Usage
- `pkgctl diff -l 경로/대상/파일|패키지명` : tar 콘텐츠 l리스트 비교 모드(기본값)로 패키지 파일 비교
- `pkgctl diff -d 경로/대상/파일|패키지명` : diffoscope 비교 모드로 패키지 파일 비교
- `pkgctl diff -p 경로/대상/파일|패키지명` : `.PKGINFO` 비교 모드로 패키지 파일 비교
- `pkgctl diff -b 경로/대상/파일|패키지명` : `.BUILDINFO` 비교 모드로 패키지 파일 비교

## Links
- More information: https://manned.org/pkgctl-diff
