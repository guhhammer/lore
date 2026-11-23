# apptainer push

컨테이너 이미지를 원격 레지스트리에 업로드하는 명령어.

## Usage
- `apptainer push 경로/대상/이미지.sif library://user/collection/container:태그` : container Library로 컨테이너 업로드
- `apptainer push 경로/대상/이미지.sif oras://registry/namespace/image:태그` : OCI 레지스트리로 컨테이너 업로드
- `apptainer push -U 경로/대상/이미지.sif library://user/collection/container:태그` : 서명 검증을 건너뛰고 컨테이너 업로드
- `apptainer push -D "description" 경로/대상/이미지.sif library://user/collection/container:태그` : 설명을 포함해 컨테이너 업로드 (library 전용)
- `apptainer push -h` : 도움말 표시

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_push.html
