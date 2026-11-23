# lpq

프린터 대기열 상태 표시.

## Usage
- `lpq` : 기본 대상의 대기 중인 작업 표시
- `lpq -a -E` : 모든 프린터의 대기 중인 작업을 암호화 적용하여 표시
- `lpq -l` : 대기 중인 작업을 자세한 형식으로 표시
- `lpq -P 대상/인스턴스` : 특정 프린터나 클래스의 대기 중인 작업 표시
- `lpq +간격` : 대기열이 비어질 때까지 n초마다 한 번씩 대기 중인 작업 표시

## Links
- More information: https://openprinting.github.io/cups/doc/man-lpq.html
