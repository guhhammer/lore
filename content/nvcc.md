# nvcc

The NVIDIA CUDA Compiler Driver.

## Usage
- `nvcc path/to/source.cu -o path/to/executable` : compile a CUDA program
- `nvcc path/to/source.cu -o path/to/executable -g -G` : generate debug information
- `nvcc path/to/source.cu -o path/to/executable -I path/to/includes -L path/to/library -l library_name` : include libraries from a different path
- `nvcc path/to/source.cu -o path/to/executable -gencode arch=arch_name,code=gpu_code_name` : specify the compute capability for a specific GPU architecture

## Categories
files, development

## Links
- More information: https://docs.nvidia.com/cuda/cuda-compiler-driver-nvcc/
- tldr page: https://tldr.inbrowser.app/pages/common/nvcc
