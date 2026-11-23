# modprobe

Linux 커널에 모듈을 추가하거나 제거합니다.

## Usage
- `modprobe -n 모듈_이름` : 모듈을 커널에 로드하는 것처럼 시뮬레이션하지만 실제로는 하지 않음
- `sudo modprobe 모듈_이름` : 모듈을 커널에 로드
- `sudo modprobe -r 모듈_이름` : 모듈을 커널에서 제거
- `sudo modprobe -r --remove-holders 모듈_이름` : 모듈과 해당 모듈에 의존하는 모듈을 커널에서 제거
- `sudo modprobe -D 모듈_이름` : 커널 모듈의 의존성 표시

## Links
- More information: https://manned.org/modprobe
