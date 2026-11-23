# orca-c

ORCA 라이브 프로그래밍 환경의 C 포트. ORCA는 절차적 시퀀서를 생성하기 위한 난해한 프로그래밍 언어입니다.

## Usage
- `orca-c` : 빈 작업 공간으로 ORCA 시작
- `orca-c 경로/대상/파일.orca` : 특정 파일을 열며 ORCA 시작
- `orca-c --bpm 분당_비트수` : 특정 템포로 ORCA 시작 (기본값은 120)
- `orca-c --initial-size 열x행` : 그리드 크기를 설정하여 ORCA 시작
- `orca-c --undo-limit 제한` : 최대 실행 취소 단계 수를 설정하여 ORCA 시작 (기본값은 100)
- `<F1>` : ORCA 내부에서 메인 메뉴 표시
- `<?>` : ORCA 내부에서 모든 단축키 표시
- `<Ctrl g>` : ORCA 내부에서 모든 ORCA 연산자 표시

## Links
- More information: https://github.com/hundredrabbits/Orca-c
