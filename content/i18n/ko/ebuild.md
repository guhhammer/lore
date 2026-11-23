# ebuild

Gentoo Portage 시스템에 대한 저수준 인터페이스.

## Usage
- `ebuild 경로/대상/파일.ebuild manifest` : 패키지 매니페스트 생성 또는 업데이트
- `ebuild 경로/대상/파일.ebuild clean` : 빌드 파일의 임시 빌드 디렉터리 정리
- `ebuild 경로/대상/파일.ebuild fetch` : 소스가 존재하지 않을 경우 소스 가져오기
- `ebuild 경로/대상/파일.ebuild unpack` : 소스를 임시 빌드 디렉터리에 추출
- `ebuild 경로/대상/파일.ebuild compile` : 추출된 소스 컴파일
- `ebuild 경로/대상/파일.ebuild install` : 임시 설치 디렉터리에 패키지 설치
- `ebuild 경로/대상/파일.ebuild qmerge` : 라이브 파일 시스템에 임시 파일 설치
- `ebuild 경로/대상/파일.ebuild merge` : 지정된 ebuild 파일의 소스 가져오기, 추출, 컴파일, 설치 및 qmerge 수행

## Links
- More information: https://wiki.gentoo.org/wiki/Ebuild
