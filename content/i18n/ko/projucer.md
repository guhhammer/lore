# projucer

JUCE 프레임워크 애플리케이션을 위한 프로젝트 관리자.

## Usage
- `Projucer --status 경로/대상/프로젝트_파일` : 프로젝트에 대한 정보 표시
- `Projucer --resave 경로/대상/프로젝트_파일` : 프로젝트의 모든 파일 및 리소스 다시 저장
- `Projucer --set-version 버전_번호 경로/대상/프로젝트_파일` : 프로젝트의 버전 번호 업데이트
- `Projucer --create-project-from-pip 경로/대상/PIP 경로/대상/출력` : PIP 파일에서 JUCE 프로젝트 생성
- `Projucer --tidy-divider-comments 경로/대상/대상_폴더` : 모든 JUCE 스타일 주석 (`//=====`, `//-----` 또는 `///////`) 제거
- `Projucer --help` : 도움말 표시
