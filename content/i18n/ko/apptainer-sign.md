# apptainer sign

SIF 컨테이너 이미지에 디지털 서명을 추가하는 도구.

## Usage
- `apptainer sign 경로/대상/이미지.sif` : 기본 PGP 키를 사용하여 컨테이너 이미지 서명
- `apptainer sign --key 경로/대상/개인키.pem 경로/대상/이미지.sif` : 특정 개인 키 파일을 사용하여 서명
- `apptainer sign -k 키_인덱스 경로/대상/이미지.sif` : 특정 PGP 키 인덱스를 사용하여 서명
- `apptainer sign -g 그룹_아이디 경로/대상/이미지.sif` : 이미지 내 특정 객체 그룹에 서명
- `apptainer sign -i 객체_아이디 경로/대상/이미지.sif` : 이미지 내 특정 객체 ID에 서명
- `apptainer sign -h` : 도움말 표시

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_sign.html
