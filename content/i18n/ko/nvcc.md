# nvcc

NVIDIA CUDA 컴파일러 드라이버.

## Usage
- `nvcc 경로/대상/소스.cu -o 경로/대상/실행파일` : CUDA 프로그램 컴파일
- `nvcc 경로/대상/소스.cu -o 경로/대상/실행파일 -g -G` : 디버그 정보 생성
- `nvcc 경로/대상/소스.cu -o 경로/대상/실행파일 -I 경로/대상/포함대상 -L 경로/대상/라이브러리 -l 라이브러리_이름` : 다른 경로에서 라이브러리 포함
- `nvcc 경로/대상/소스.cu -o 경로/대상/실행파일 -gencode arch=아키텍처_이름,code=gpu_코드_이름` : 특정 GPU 아키텍처에 대한 컴퓨팅 능력 지정

## Links
- More information: https://docs.nvidia.com/cuda/cuda-compiler-driver-nvcc/
