# urpmq

Mageia에서 패키지 및 미디어에 대한 정보를 조회합니다.

## Usage
- `urpmq -i 패키지` : 설치 가능한 패키지에 대한 정보 표시
- `urpmq --requires 패키지` : 패키지의 직접적인 의존성 표시
- `urpmq -d 패키지` : 패키지의 직접 및 간접 의존성 표시
- `sudo urpmq -d -m --sources 경로/대상/파일.rpm` : RPM f파일에 필요한 설치되지 않은 패키지 및 소스 나열
- `urpmq --list-media --list-url` : URL과 함께 모든 구성된 미디어 나열(비활성 미디어 포함)
- `urpmq -g -r -y 키워드` : 패키지 검색 시 g그룹, 버전 및 r릴리즈 출력
- `urpmq -g -r 패키지` : 패키지의 정확한 이름을 사용하여 검색

## Links
- More information: https://man.linuxreviews.org/man8/urpmq.8.html
