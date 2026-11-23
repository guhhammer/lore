# apptainer build

Apptainer 컨테이너 이미지 빌드.

## Usage
- `apptainer build 경로/대상/이미지.sif 경로/대상/정의파일.def` : 정의 파일로부터 컨테이너 빌드
- `apptainer build 경로/대상/이미지.sif docker://이미지:태그` : docker Hub에서 컨테이너 빌드
- `apptainer build 경로/대상/이미지.sif library://user/collection/container:태그` : container Library에서 컨테이너 빌드
- `apptainer build -s 경로/대상/디렉토리 docker://이미지:태그` : 이미지 파일 대신 쓰기 가능한 샌드박스 디렉터리 생성
- `apptainer build --disable-cache 경로/대상/이미지.sif docker://이미지:태그` : 캐시를 사용하지 않고 컨테이너 빌드
- `apptainer build -F 경로/대상/이미지.sif 경로/대상/정의파일.def` : 기존 이미지 파일 강제 덮어쓰기
- `apptainer build -f 경로/대상/이미지.sif 경로/대상/정의파일.def` : 권한이 없는 환경에서 fakeroot를 사용해 빌드
- `apptainer build -h` : 도움말 표시

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_build.html
