# cpufreq-set

CPU 주파수 설정을 수정하는 도구. 주파수 값은 `cpufreq-info -l` 명령의 출력 범위 내에 있어야 합니다.

## Usage
- `sudo cpufreq-set -c 1 -g userspace` : CPU 1의 CPU 주파수 정책을 "userspace"로 설정
- `sudo cpufreq-set -c 1 -d 최소_주파수` : CPU 1의 현재 최소 CPU 주파수 설정
- `sudo cpufreq-set -c 1 -u 최대_주파수` : CPU 1의 현재 최대 CPU 주파수 설정
- `sudo cpufreq-set -c 1 -f 작업_주파수` : CPU 1의 현재 작업 주파수 설정

## Links
- More information: https://manned.org/cpufreq-set
