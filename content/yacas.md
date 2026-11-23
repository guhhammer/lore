# yacas

Yet Another Computer Algebra System.

## Usage
- `yacas` : start an interactive `yacas` session
- `Integrate(x)Cos(x);` : [Interactive] While in a `yacas` session, execute a statement
- `Example();` : [Interactive] While in a `yacas` session, display an example
- `quit` : [Interactive] Quit from a `yacas` session
- `yacas -p -c path/to/script1 path/to/script2 ...` : execute one or more `yacas` scripts (without terminal or prompts), then exit
- `echo "Echo( Deriv(x)Cos(1/x) );" | yacas -p -c /dev/stdin` : execute and print the result of one statement, then exit

## Links
- More information: https://www.yacas.org/
- tldr page: https://tldr.inbrowser.app/pages/common/yacas
