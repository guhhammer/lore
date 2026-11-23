# singularity

Singularity 컨테이너 및 이미지 관리.

## Usage
- `singularity pull --name 이미지.sif library://godlovedc/funny/lolcow:latest` : sylabs Cloud에서 원격 이미지 다운로드
- `singularity build 이미지.sif docker://godlovedc/lolcow` : 최신 Singularity 이미지 형식으로 원격 이미지 재구축
- `singularity shell 이미지.sif` : 이미지에서 컨테이너를 시작하고 내부에서 셸 실행
- `singularity exec 이미지.sif 명령` : 이미지에서 컨테이너를 시작하고 명령 실행
- `singularity run 이미지.sif` : 이미지에서 컨테이너를 시작하고 내부 runscript 실행
- `sudo singularity build 이미지.sif 레시피` : 레시피 파일에서 Singularity 이미지 생성

## Links
- More information: https://singularity-docs.readthedocs.io/en/latest/#commands
