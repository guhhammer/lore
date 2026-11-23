# kmutil

커널 확장(kexts) 및 디스크의 kext 컬렉션을 관리하는 유틸리티.

## Usage
- `kmutil find` : 운영 체제에서 사용 가능한 kext 찾기
- `kmutil log` : 커널 관리 하위 시스템에 대한 로깅 정보 표시
- `kmutil inspect 옵션들` : 제공된 옵션에 따라 kext 컬렉션의 내용 검사 및 표시
- `kmutil check` : kext 컬렉션 간의 일관성 검사
- `sudo kmutil dumpstate` : 디버깅을 위한 kernelmanagerd 상태 덤프
- `kmutil load --bundle-path 경로/대상/extension.kext` : 결과의 이 경로에 지정된 번들을 기반으로 하나 이상의 확장 로드

## Links
- More information: https://keith.github.io/xcode-man-pages/kmutil.8.html
