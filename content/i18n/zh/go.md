# go

管理 Go 源代码的工具。

## Usage
- `go get 路径/到/包` : 下载并安装由其路径指定的包
- `go run 文件.go` : 编译并运行一个源文件（它必须包含一个 `main` 包）
- `go build -o 可执行文件 文件.go` : 将源文件编译为当前命名的可执行文件
- `go build` : 编译当前目录中的包
- `go test` : 执行当前包中的所有测试用例（文件必须以 `_test.go` 结尾）
- `go install` : 编译并安装当前包
- `go mod init 模块` : 在当前目录下初始化一个新模块

## Links
- More information: https://pkg.go.dev/cmd/go
