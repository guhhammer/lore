# fstrim

마운트된 파일 시스템에서 사용되지 않는 블록을 삭제합니다. SSD 및 microSD 카드와 같은 플래시 메모리 장치에서만 지원됩니다.

## Usage
- `sudo fstrim -a` : 지원되는 모든 마운트된 파티션의 사용되지 않는 블록 삭제
- `sudo fstrim /` : 지정된 파티션의 사용되지 않는 블록 삭제
- `sudo fstrim -v /` : 삭제 후 통계 표시

## Links
- More information: https://manned.org/fstrim
