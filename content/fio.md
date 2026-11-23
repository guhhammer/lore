# fio

Flexible I/O tester: do an I/O action spawning multiple threads or processes.

## Usage
- `fio --filename=path/to/file --direct=1 --rw=randread --bs=4k --ioengine=libaio --iodepth=256 --runtime=120 --numjobs=4 --time_based --group_reporting --name=job_name --eta-newline=1 --readonly` : test random reads
- `fio --filename=path/to/file --direct=1 --rw=read --bs=4k --ioengine=libaio --iodepth=256 --runtime=120 --numjobs=4 --time_based --group_reporting --name=job_name --eta-newline=1 --readonly` : test sequential reads
- `fio --filename=path/to/file --direct=1 --rw=randrw --bs=4k --ioengine=libaio --iodepth=256 --runtime=120 --numjobs=4 --time_based --group_reporting --name=job_name --eta-newline=1` : test random read/write
- `fio path/to/job_file` : test with parameters from a job file
- `fio --showcmd path/to/job_file` : convert a specific job file to command-line options

## Categories
files, processes, development

## Links
- More information: https://fio.readthedocs.io/en/latest/fio_doc.html
- tldr page: https://tldr.inbrowser.app/pages/common/fio
