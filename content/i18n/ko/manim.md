# manim

수학 설명 영상을 위한 애니메이션 엔진입니다.

## Usage
- `manim 경로/파일.py 장면이름` : 기본 설정으로 Python 스크립트의 장면을 렌더링
- `manim -pql 경로/파일.py 장면이름` : 실시간 미리보기로 렌더링 (렌더링 후 자동으로 영상 열기)
- `manim -pqh 경로/파일.py 장면이름` : 고화질(1080p 60fps)로 렌더링
- `manim -o 출력파일이름 경로/파일.py 장면이름` : 출력 파일 이름을 직접 지정
- `manim -r 1920,1080 --fps 60 경로/파일.py 장면이름` : 특정 해상도와 프레임 속도로 렌더링
- `manim --list_scenes 경로/파일.py` : 렌더링 없이 스크립트에 정의된 장면 목록 보기
- `manim --help` : 도움말 표시

## Links
- More information: https://docs.manim.community/en/stable/tutorials/quickstart.html
