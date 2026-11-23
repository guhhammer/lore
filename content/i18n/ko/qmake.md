# qmake

Qt 프로젝트 파일로부터 Makefile을 생성.

## Usage
- `qmake` : 현재 디렉터리의 프로젝트 파일에서 `Makefile` 생성
- `qmake -o 경로/대상/Makefile 경로/대상/project_file.pro` : `Makefile`과 프로젝트 파일의 위치 지정
- `qmake -project` : 기본 프로젝트 파일 생성
- `qmake && make` : 프로젝트 컴파일
- `qmake -d` : 디버그 모드 활성화
- `qmake -help` : 도움말 표시

## Links
- More information: https://doc.qt.io/qt-6/qmake-running.html
