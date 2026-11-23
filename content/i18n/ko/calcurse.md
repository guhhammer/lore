# calcurse

명령줄에서 사용할 수 있는 텍스트 기반의 일정 관리 및 달력 애플리케이션.

## Usage
- `calcurse` : 인터랙티브 모드로 `calcurse` 시작
- `calcurse -a` : 오늘의 약속 및 이벤트를 출력하고 종료
- `calcurse-caldav --init=keep-remote` : 모든 로컬 calcurse 항목을 제거하고 원격 객체 가져오기
- `calcurse-caldav --init=keep-local` : 모든 원격 객체 제거하고 로컬 calcurse 항목 푸시
- `calcurse-caldav --init=two-way` : 로컬 객체를 CalDAV 서버에 복사하고 그 반대도 수행

## Links
- More information: https://github.com/lfos/calcurse/blob/pu/doc/calcurse.1.txt
