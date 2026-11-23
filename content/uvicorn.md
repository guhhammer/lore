# uvicorn

Python ASGI HTTP Server, for asynchronous projects.

## Usage
- `uvicorn import.path:app_object` : run Python web app
- `uvicorn --host localhost --port 8080 import.path:app_object` : listen on port 8080 on localhost
- `uvicorn --reload import.path:app_object` : turn on live reload
- `uvicorn --workers 4 import.path:app_object` : use 4 worker processes for handling requests
- `uvicorn --ssl-certfile cert.pem --ssl-keyfile key.pem import.path:app_object` : run app over HTTPS

## Categories
network, files

## Links
- More information: https://www.uvicorn.org/settings/
- tldr page: https://tldr.inbrowser.app/pages/common/uvicorn
