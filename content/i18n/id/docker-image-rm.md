# docker image rm

Hapus kumpulan citra yang dikelola Docker.

## Usage
- `docker rmi citra1 citra2 ...` : hapus satu atau beberapa citra berdasarkan nama masing-masing
- `docker rmi -f citra` : hapus suatu citra secara paksa
- `docker rmi --no-prune citra` : hapus suatu citra tanpa menghapus citra induk yang tak diberi tag
- `docker rmi --help` : tampilkan bantuan

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/rm/
