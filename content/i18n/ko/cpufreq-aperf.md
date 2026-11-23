# cpufreq-aperf

일정 시간 동안 평균 CPU 주파수를 계산합니다.

## Usage
- `sudo cpufreq-aperf` : 모든 CPU 코어와 1초 새로고침 간격으로 계산 시작
- `sudo cpufreq-aperf -c 1` : CPU 1만 계산 시작
- `sudo cpufreq-aperf -i 3` : 모든 CPU 코어에 대해 3초 새로고침 간격으로 계산 시작
- `sudo cpufreq-aperf -o` : 한 번만 계산

## Links
- More information: https://manned.org/cpufreq-aperf
