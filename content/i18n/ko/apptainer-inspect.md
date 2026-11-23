# apptainer inspect

Apptainer 컨테이너 이미지 메타데이터를 출력하는 도구.

## Usage
- `apptainer inspect 경로/대상/이미지.sif` : 이미지의 라벨 정보 출력 (기본)
- `apptainer inspect -d 경로/대상/이미지.sif` : 이미지 생성에 사용된 정의 파일 출력
- `apptainer inspect -r 경로/대상/이미지.sif` : 이미지의 runscript 출력
- `apptainer inspect -e 경로/대상/이미지.sif` : 이미지의 환경 변수 출력
- `apptainer inspect --list-apps 경로/대상/이미지.sif` : 컨테이너 내 모든 애플리케이션 목록 출력
- `apptainer inspect --all 경로/대상/이미지.sif` : 모든 메타데이터를 JSON 형식으로 출력
- `apptainer inspect -h` : 도움말 표시

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_inspect.html
