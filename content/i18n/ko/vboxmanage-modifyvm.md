# vboxmanage-modifyvm

중지된 가상 machine 설정 변경.

## Usage
- `VBoxManage modifyvm uuid|vm_name --name new_name` : VM(가상머신) 이름 변경
- `VBoxManage modifyvm uuid|vm_name --memory 2048 --cpus 2` : 메모리와 CPU 설정 조정
- `VBoxManage modifyvm uuid|vm_name --vrde on` : 원격 디스플레이(VRDE) 활성화
- `VBoxManage modifyvm uuid|vm_name --recording on` : 세션 녹화 활성화

## Links
- More information: https://www.virtualbox.org/manual/ch08.html#vboxmanage-modifyvm
