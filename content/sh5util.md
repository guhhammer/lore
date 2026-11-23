# sh5util

Merge HDF5 files produced by the `sacct_gather_profile` plugin.

## Usage
- `sh5util -j job_id|job_id.step_id` : merge HDF5 files produced on each allocated node for the specified job or step
- `sh5util -j job_id|job_id.step_id -E -i path/to/file.h5 -s Energy|Filesystem|Network|Task` : extract one or more data series from a merged job file
- `sh5util -j job_id|job_id.step_id -I -s Energy|Filesystem|Network|Task -d data_item` : extract one data item from all nodes in a merged job file

## Categories
files, network, version-control

## Links
- More information: https://slurm.schedmd.com/sh5util.html
- tldr page: https://tldr.inbrowser.app/pages/linux/sh5util
