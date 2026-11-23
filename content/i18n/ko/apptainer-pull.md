# apptainer pull

원격 소스에서 컨테이너 이미지를 다운로드하는 명령어.

## Usage
- `apptainer pull 경로/대상/이미지.sif docker://이미지:태그` : docker Hub에서 컨테이너 다운로드
- `apptainer pull 경로/대상/이미지.sif library://사용자/컬렉션/컨테이너:태그` : container Library에서 컨테이너 다운로드
- `apptainer pull 경로/대상/이미지.sif oras://레지스트리/네임스페이스/이미지:태그` : OCI 레지스트리에서 컨테이너 다운로드
- `apptainer pull --arch amd64|arm64|ppc64le 경로/대상/이미지.sif library://이미지:태그` : 특정 아키텍처용 컨테이너 다운로드
- `apptainer pull -F 경로/대상/이미지.sif docker://이미지:태그` : 기존 이미지 파일 강제 덮어쓰기
- `apptainer pull --sandbox 경로/대상/디렉터리 docker://이미지:태그` : 쓰기 가능한 sandbox 디렉터리로 컨테이너 다운로드
- `apptainer pull --disable-cache 경로/대상/이미지.sif docker://이미지:tag` : 캐시를 사용하지 않고 컨테이너 다운로드
- `apptainer pull -h` : 도움말 표시

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_pull.html
