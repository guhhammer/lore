# go clean

移除目标文件和缓存文件。

## Usage
- `go clean -n` : 只打印移除命令，而不会真正移除任何东西
- `go clean -cache` : 删除编译缓存
- `go clean -testcache` : 删除所有测试结果缓存
- `go clean -modcache` : 删除模块缓存

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Remove_object_files_and_cached_files
