# toolbox create

새 Toolbx 컨테이너 생성.

## Usage
- `toolbox create -d 배포판` : 특정 배포판에 대한 Toolbx 컨테이너 생성
- `toolbox create -r 릴리스` : 현재 배포판의 특정 릴리스에 대한 Toolbx 컨테이너 생성
- `toolbox create -i 이름` : 사용자 지정 이미지로 Toolbx 컨테이너 생성
- `toolbox create -i registry.fedoraproject.org/fedora-toolbox:39` : 사용자 지정 Fedora 이미지에서 Toolbx 컨테이너 생성
- `toolbox create -d fedora -r f39` : fedora 39의 기본 이미지를 사용하여 Toolbx 컨테이너 생성

## Links
- More information: https://manned.org/toolbox-create
