# apptainer exec

Apptainer container 내 명령어 실행하는 도구.

## Usage
- `apptainer exec 경로/대상/이미지.sif 명령어` : 컨테이너 내부에서 명령 실행
- `apptainer exec 경로/대상/이미지.sif 명령어 인자1 인자2 ...` : 인자를 포함하여 명령 실행
- `apptainer exec -B path/to/source:path/to/destination 경로/대상/이미지.sif 명령어` : 호스트 디렉터리를 컨테이너에 bind 마운트하여 실행
- `apptainer exec --env 변수=값 경로/대상/이미지.sif 명령어` : 환경 변수를 설정하여 명령 실행
- `apptainer exec -C 경로/대상/이미지.sif 명령어` : 완전 격리 모드에서 명령어 실행 (filesystem, PID, IPC, 환경 분리)
- `apptainer exec --writable-tmpfs 경로/대상/이미지.sif 명령어` : 쓰기 가능한 임시 파일시스템 오버레이 사용
- `apptainer exec --nv 경로/대상/이미지.sif 명령어` : NVIDIA GPU 지원 활성화하여 실행
- `apptainer exec -h` : 도움말 표시

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_exec.html
