# qmake

从 Qt 项目文件生成 Makefile。

## Usage
- `qmake` : 从当前目录中的项目文件生成一个 `Makefile`
- `qmake -o 路径/到/Makefile 路径/到/项目文件.pro` : 指定 `Makefile` 和项目文件的位置
- `qmake -project` : 生成一个默认的项目文件
- `qmake && make` : 编译一个项目
- `qmake -d` : 启用调试模式
- `qmake -help` : 显示帮助

## Links
- More information: https://doc.qt.io/qt-6/qmake-running.html
