# par2

PAR 2.0 호환 패리티 아카이브(`.par2` 파일)를 사용한 파일 검증 및 복구.

## Usage
- `par2 create -r1..100 -- 경로/대상/파일` : 설정된 비율의 여유를 사용하여 패리티 아카이브 생성
- `par2 create -n1..32768 -- 경로/대상/파일` : 선택한 수의 볼륨 파일(색인 파일 추가)을 사용하여 패리티 아카이브 생성
- `par2 verify -- 경로/대상/파일.par2` : 패리티 아카이브로 파일 검증
- `par2 repair -- 경로/대상/파일.par2` : 패리티 아카이브로 파일 복구

## Links
- More information: https://github.com/Parchive/par2cmdline/
