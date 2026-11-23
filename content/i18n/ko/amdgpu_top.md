# amdgpu_top

AMDGPU 드라이버를 사용하여 AMD GPU 사용률과 하드웨어 지표를 표시하는 도구.

## Usage
- `amdgpu_top --list` : AMDGPU 장치 목록 출력
- `amdgpu_top -p` : 모든 GPU 프로세스 및 프로세스 별 메모리 사용량 출력
- `amdgpu_top --pci "0000:01:00.0"` : PCI 버스로 특정 GPU 선택
- `amdgpu_top` : 대화형 TUI 모니터 실행
- `amdgpu_top --gui` : GUI 모니터 실행
- `amdgpu_top --smi` : 간단한 SMI 스타일 TUI 뷰 실행

## Links
- More information: https://github.com/Umio-Yasuno/amdgpu_top#usage
