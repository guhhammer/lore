# crane registry

이 명령은 자동으로 선택된 포트(:0), `$PORT` 또는 `--address`에서 레지스트리 구현을 제공. 서버가 push 및 pull을 수락하는 동안 명령 블록과 내용을 메모리 및 디스크에 저장할 . 수 있음.

## Usage
- `crane registry serve` : 레지스트리 구현 제공
- `crane registry serve --address 주소_이름` : 서버 준비 상태의 주소
- `crane registry serve --disk 경로/대상/store_dir` : blob이 저장될 디렉터리의 경로
- `crane registry -h` : 도움말 표시
- `crane registry serve -h` : 도움말 표시
