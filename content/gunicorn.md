# gunicorn

Python WSGI HTTP Server.

## Usage
- `gunicorn import.path:app_object` : run Python web app
- `gunicorn -b localhost:8080 import.path:app_object` : listen on port 8080 on localhost
- `gunicorn --reload import.path:app_object` : turn on live reload
- `gunicorn -w 4 import.path:app_object` : use 4 worker processes for handling requests
- `gunicorn --threads 4 import.path:app_object` : use 4 worker threads for handling requests
- `gunicorn --certfile cert.pem --keyfile key.pem import.path:app_object` : run app over HTTPS

## Categories
network, files

## Links
- More information: https://manned.org/gunicorn
- tldr page: https://tldr.inbrowser.app/pages/common/gunicorn
