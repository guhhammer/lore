# fabric

AI를 활용해 인간의 작업을 확장하는 오픈소스 프레임워크. 크라우드 소싱된 AI 프롬프트 집합을 활용하여 특정 문제를 해결하는 모듈형 구조를 제공.

## Usage
- `fabric -S` : fabric 설정 실행
- `fabric -l` : 사용 가능한 모든 패턴 목록 조회
- `fabric <경로/대상/입력_파일 -p 패턴_이름` : 파일 입력을 사용하여 패턴 실행
- `fabric -y "https://www.youtube.com/watch?v=video_id" -p 패턴_이름` : youTube 영상 URL을 대상으로 패턴 실행
- `fabric -p 패턴1 | fabric -p 패턴2` : 여러 패턴을 파이프로 연결해 실행
- `fabric -p 사용자정의_패턴_이름` : 사용자 정의 패턴 실행
- `fabric -p 패턴_이름 -o 경로/대상/출력_파일` : 패턴 실행 결과를 파일로 저장
- `fabric -p 패턴_이름 -v "변수_이름:값"` : 변수 값을 지정하여 패턴 실행

## Links
- More information: https://github.com/danielmiessler/fabric#usage
