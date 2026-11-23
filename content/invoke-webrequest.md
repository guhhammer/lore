# invoke-webrequest

Perform an HTTP/HTTPS request to the Web. Note: This command can only be used through PowerShell.

## Usage
- `Invoke-WebRequest http://example.com -OutFile path\to\file` : download the contents of a URL to a file
- `Invoke-WebRequest http://example.com -UseBasicParsing` : only return raw HTML data instead of parsing it under Internet Explorer (PowerShell 3.0-5.1 only)
- `Invoke-WebRequest -Method Post -Body @{ name='bob' } http://example.com/form` : send form-encoded data (POST request of type `application/x-www-form-urlencoded`)
- `Invoke-WebRequest -Headers @{ X-My-Header = '123' } -Method PUT http://example.com` : send a request with an extra header, using a custom HTTP method
- `Invoke-WebRequest -Body '{"name":"bob"}' -ContentType 'application/json' http://example.com/users/1234` : send data in JSON format, specifying the appropriate content-type header
- `Invoke-WebRequest -Headers @{ Authorization = "Basic "+ [System.Convert]::ToBase64String([System.Text.Encoding]::ASCII.GetBytes("myusername:mypassword")) } http://example.com` : pass a username and password for server authentication

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
network, files, text, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/invoke-webrequest
