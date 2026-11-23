# pw-metadata

PipeWire 객체의 메타데이터를 모니터링, 설정 및 삭제.

## Usage
- `pw-metadata` : `default` 이름의 메타데이터 표시
- `pw-metadata -n settings 0` : `settings`에서 ID 0의 메타데이터 표시
- `pw-metadata -l` : 사용 가능한 모든 메타데이터 객체 나열
- `pw-metadata -m` : 실행을 유지하며 메타데이터 변경 사항 기록
- `pw-metadata -d` : 모든 메타데이터 삭제
- `pw-metadata -n settings 0 log.level 1` : `settings`에서 `log.level`을 1로 설정
- `pw-metadata -h` : 도움말 표시

## Links
- More information: https://docs.pipewire.org/page_man_pw-metadata_1.html
