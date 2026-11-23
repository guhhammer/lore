# pprof

Visualize and analyze profiling data.

## Usage
- `pprof -top ./fibbo ./fibbo-profile.pb.gz` : generate a text report from a specific profiling file, on fibbo binary
- `pprof -svg ./fibbo ./fibbo-profile.pb.gz` : generate a graph and open it on a web browser
- `pprof ./fibbo ./fibbo-profile.pb.gz` : run pprof in interactive mode to be able to manually launch `pprof` on a file
- `pprof -http=localhost:8080 ./fibbo ./fibbo-profile.pb.gz` : run a web server that serves a web interface on top of `pprof`
- `pprof http://localhost:8080/debug/pprof` : fetch a profile from an HTTP server and generate a report

## Categories
network

## Links
- More information: https://github.com/google/pprof/tree/main/doc#pprof
- tldr page: https://tldr.inbrowser.app/pages/common/pprof
