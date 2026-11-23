# kaggle kernels

Manage Kaggle kernels.

## Usage
- `kaggle k list` : list all kernels
- `kaggle k files kernel_name` : list kernel output files
- `kaggle k init -p path/to/directory` : initialize metadata file for a kernel (defaults to current directory)
- `kaggle k push -p path/to/directory` : push new code to a kernel and run the kernel
- `kaggle k pull kernel_name -p path/to/directory` : pull a kernel
- `kaggle k output kernel_name` : get data output from the latest kernel run
- `kaggle k status kernel_name` : display the status of the latest kernel run

## Categories
files, system

## Links
- More information: https://github.com/Kaggle/kaggle-api/blob/main/docs/README.md#kernels
- tldr page: https://tldr.inbrowser.app/pages/common/kaggle-kernels
