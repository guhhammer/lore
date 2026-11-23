# gpm

Linux 가상 콘솔에서 마우스 지원을 활성화.

## Usage
- `sudo gpm -m /dev/input/mice -t ps2` : PS/2 타입(type) 마우스(mouse)를 사용하여 gpm 시작
- `sudo gpm -m /dev/ttyS0 -t ms` : microsoft 타입(type) 직렬(serial) 마우스(mouse)를 사용하여 gpm 시작
- `sudo gpm -m 경로/대상/마우스_장치 -t 마우스_타입 -D` : 디버깅(Debugging)을 위해 포그라운드에서 지정한 마우스(mouse)와 유형(type)으로 gpm 시작
- `sudo gpm -k` : 실행 중인 gpm 종료(kill)
- `sudo gpm -m 경로/대상/마우스_장치 -t 마우스_타입 -R` : 서버 호환성을 위한 리피터(Repeater) 모드로 마우스(mouse)와 유형(type)으로 gpm 시작
- `gpm -t help` : 사용 가능한 마우스 유형(types) 목록 표시

## Links
- More information: https://manned.org/gpm
