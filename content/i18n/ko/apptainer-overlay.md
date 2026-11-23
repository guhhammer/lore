# apptainer overlay

Apptainer 컨테이너를 위해 EXT3 쓰기 가능한 오버레이 이미지를 관리하는 도구.

## Usage
- `apptainer overlay create -s 크기 경로/대상/이미지.sif` : 기존 SIF 이미지에 쓰기 가능한 오버레이 추가
- `apptainer overlay create -s 크기 경로/대상/오버레이.img` : 독립적인 EXT3 오버레이 이미지 생성
- `apptainer overlay create -s 크기 -S 경로/대상/오버레이.img` : 희소 오버레이 이미지 생성
- `apptainer overlay create -f -s 크기 경로/대상/오버레이.img` : fakeroot용 오버레이 생성
- `apptainer overlay create --create-dir 경로/대상/디렉터리 경로/대상/오버레이.img` : 레이아웃 내 특정 디렉터리를 포함하는 오버레이 생성
- `apptainer overlay -h` : 도움말 표시

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_overlay.html
