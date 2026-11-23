# licensor

Write licenses to `stdout`.

## Usage
- `licensor MIT >LICENSE` : write the MIT license to a file named `LICENSE`
- `licensor -p MIT >LICENSE` : write the MIT license with a placeholder copyright notice to a file named `LICENSE`
- `licensor MIT "Bobby Tables" >LICENSE` : specify a copyright holder named Bobby Tables
- `licensor "Apache-2.0 WITH LLVM-exception" >LICENSE` : specify license exceptions with a WITH expression
- `licensor -l` : list all available licenses
- `licensor -e` : list all available exceptions

## Links
- More information: https://github.com/raftario/licensor
- tldr page: https://tldr.inbrowser.app/pages/common/licensor
