# valgrind

Wrapper for a set of expert tools for profiling, optimizing, and debugging programs. Commonly used tools include `memcheck`, `cachegrind`, `callgrind`, `massif`, `helgrind`, and `drd`.

## Usage
- `valgrind program` : use the (default) Memcheck tool to show a diagnostic of memory usage by `program`
- `valgrind --leak-check=full --show-leak-kinds=all program` : use Memcheck to report all possible memory leaks of `program` in full detail
- `valgrind --tool=cachegrind program` : use the Cachegrind tool to profile and log CPU cache operations of `program`
- `valgrind --tool=massif --stacks=yes program` : use the Massif tool to profile and log heap memory and stack usage of `program`

## Categories
monitoring

## Links
- More information: https://valgrind.org/docs/manual/manual-core.html#manual-core.options
- tldr page: https://tldr.inbrowser.app/pages/common/valgrind
- Wikipedia: https://en.wikipedia.org/wiki/Valgrind
