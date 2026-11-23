# virt-clone

libvirt 가상 머신 복제.

## Usage
- `virt-clone -o 가상머신_이름 --auto-clone` : 가상 머신을 복제하고 새 이름, 저장 경로 및 MAC 주소를 자동으로 생성
- `virt-clone -o 가상머신_이름 -n 새_가상머신_이름 -f 경로/대상/새_저장소 -m ff:ff:ff:ff:ff:ff|RANDOM` : 가상 머신을 복제하고 새 이름, 저장 경로 및 MAC 주소를 지정

## Links
- More information: https://manned.org/virt-clone
