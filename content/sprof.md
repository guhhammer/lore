# sprof

Read and display shared object profiling data. Note: Requires profile data generated via the `$LD_PROFILE` environment variable.

## Usage
- `sprof path/to/library.so path/to/library.so.profile` : generate a flat profile and call graph (default output)
- `sprof -p path/to/library.so path/to/library.so.profile` : generate a flat profile with counts and ticks
- `sprof -q path/to/library.so path/to/library.so.profile` : generate a call graph
- `sprof -c path/to/library.so path/to/library.so.profile` : print call pairs and their usage counts
- `sprof path/to/library.so` : use profile data from current directory (auto-detected by soname)

## Categories
files, development, desktop

## Links
- More information: https://manned.org/sprof
- tldr page: https://tldr.inbrowser.app/pages/linux/sprof
