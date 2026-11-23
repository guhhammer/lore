# xmake

A cross-platform C & C++ build utility based on Lua.

## Usage
- `xmake create -l c project_name` : create an Xmake C project, consisting of a hello world and `xmake.lua`
- `xmake b` : build an Xmake project
- `xmake r` : run an Xmake project
- `xmake r target_name` : run a compiled Xmake target directly
- `xmake f -p macosx|linux|iphoneos|... -a x86_64|i386|arm64|... -m debug|release` : configure a project's build targets
- `xmake i -o path/to/directory` : install the compiled target to a directory

## Categories
development, files, network

## Links
- More information: https://github.com/xmake-io/xmake#command-line-interface-reference
- tldr page: https://tldr.inbrowser.app/pages/common/xmake
