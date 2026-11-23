# pmap

프로세스 또는 프로세스들의 메모리 맵을 보고.

## Usage
- `pmap pid` : 특정 프로세스 ID (PID)의 메모리 맵 출력
- `pmap --extended pid` : 확장된 형식 표시
- `pmap --device pid` : 장치 형식 표시
- `pmap --range low,high` : `low`와 `high`로 지정된 메모리 주소 범위로 결과 제한
- `pmap pid1 pid2 ...` : 여러 프로세스의 메모리 맵 출력

## Links
- More information: https://manned.org/pmap
