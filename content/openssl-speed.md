# openssl speed

Benchmark OpenSSL algorithms.

## Usage
- `openssl speed` : benchmark all available algorithms
- `openssl speed md5 sha1 sha256 aes rsa` : benchmark the most common algorithms
- `openssl speed -evp aes-256-cbc` : benchmark a specific EVP cipher
- `openssl speed -seconds 10` : benchmark algorithms for a specific duration (in seconds)
- `openssl speed -elapsed` : benchmark using wall-clock time instead of CPU user time
- `openssl speed -mr` : output in machine readable format
- `openssl speed -multi $(nproc)` : benchmark multiple algorithms in parallel using all available CPU cores

## Categories
monitoring

## Links
- More information: https://docs.openssl.org/master/man1/openssl-speed/
- tldr page: https://tldr.inbrowser.app/pages/common/openssl-speed
