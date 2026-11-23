# krunvm

OCI 이미지를 사용하여 MicroVM을 생성.

## Usage
- `krunvm create docker.io/fedora --cpus vcpu_수 --mem 메모리_메가바이트 --name "이름"` : fedora 기반 MicroVM 생성
- `krunvm start "이미지_이름"` : 특정 이미지 시작
- `krunvm list` : 이미지 나열
- `krunvm changevm --cpus vcpu_수 --mem 메모리_메가바이트 --name "새_VM_이름" "현재_VM_이름"` : 특정 이미지 변경
- `krunvm delete "이미지_이름"` : 특정 이미지 삭제

## Links
- More information: https://github.com/containers/krunvm/blob/main/docs/krunvm.1.txt
