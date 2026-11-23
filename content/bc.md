# bc

An arbitrary precision calculator language.

## Usage
- `bc` : start an interactive session
- `bc -i -l` : start an interactive session with the standard math library enabled
- `echo '5 / 3' | bc` : calculate an expression
- `bc path/to/script.bc` : execute a script
- `echo 'scale = 10; 5 / 3' | bc` : calculate an expression with the specified scale
- `echo 's|c|a|l|e(1)' | bc -l` : calculate a sine/cosine/arctangent/natural logarithm/exponential function using `mathlib`
- `echo "define factorial(n) { if (n <= 1) return 1; return n*factorial(n-1); }; factorial(10)" | bc` : execute an inline factorial script

## See also
- dc
- qalc

## Categories
development

## Links
- More information: https://manned.org/bc
- tldr page: https://tldr.inbrowser.app/pages/common/bc
