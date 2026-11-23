# update-alternatives

심볼릭 링크를 편리하게 관리하여 기본 명령을 결정합니다.

## Usage
- `sudo update-alternatives --install 경로/대상/심볼릭링크 명령_이름 경로/대상/명령_바이너리 우선순위` : 심볼릭 링크 추가
- `sudo update-alternatives --config java` : `java`에 대한 심볼릭 링크 구성
- `sudo update-alternatives --remove java /opt/java/jdk1.8.0_102/bin/java` : 심볼릭 링크 제거
- `update-alternatives --display java` : 지정된 명령에 대한 정보 표시
- `update-alternatives --get-selections` : 모든 명령과 현재 선택된 항목 표시

## Links
- More information: https://manned.org/update-alternatives
