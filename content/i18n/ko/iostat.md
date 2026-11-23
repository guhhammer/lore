# iostat

장치 및 파티션에 대한 통계 보고.

## Usage
- `iostat` : 시스템 시작 이후의 CPU 및 디스크 통계 보고서 표시
- `iostat -m` : 단위를 메가바이트로 변환한 CPU 및 디스크 통계 보고서 표시
- `iostat -c` : CPU 통계 표시
- `iostat -N` : 디스크 이름(및 LVM 포함)을 사용한 디스크 통계 표시
- `iostat -xN sda` : 장치 "sda"에 대한 디스크 이름을 포함한 확장 디스크 통계 표시
- `iostat 2` : 2초마다 CPU 및 디스크 통계의 증분 보고서 표시

## Links
- More information: https://manned.org/iostat
