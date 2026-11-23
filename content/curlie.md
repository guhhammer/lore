# curlie

A frontend to `curl` that adds the ease of use of `httpie`.

## Usage
- `curlie httpbin.org/get` : send a GET request
- `curlie post httpbin.org/post name=john age:=25` : send a POST request
- `curlie get httpbin.org/get first_param==5 second_param==true` : send a GET request with query parameters (e.g. `first_param=5&second_param=true`)
- `curlie get httpbin.org/get header-name:header-value` : send a GET request with a custom header

## Links
- More information: https://github.com/rs/curlie#usage
- tldr page: https://tldr.inbrowser.app/pages/linux/curlie
