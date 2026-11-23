# pv

파이프를 통해 전달되는 데이터의 진행 상황을 모니터링.

## Usage
- `pv 경로/대상/파일` : 파일의 내용을 출력하고 진행 표시줄을 표시
- `command1 | pv --size eta_예상_데이터_양 | command2` : 파이프 사이의 데이터 흐름 속도와 양을 측정 (`--size`는 선택적)
- `pv -cN in 큰_텍스트_파일 | grep 패턴 | pv -cN out >필터된_파일` : 파일을 필터링하고 진행 상황과 출력 데이터 양을 확인
- `pv -d pid` : 이미 실행 중인 프로세스에 연결하여 파일 읽기 진행 상황 보기
- `pv -EE 경로/대상/오류_있는_미디어 >image.img` : 오류가 있는 파일을 읽고 `dd conv=sync,noerror`처럼 오류를 건너뛰기
- `pv -L 1K --stop-at --size 최대_읽기_파일_크기` : 지정된 양의 데이터를 읽은 후 읽기 중지하고, 1K/s로 속도 제한

## Links
- More information: https://manned.org/pv
