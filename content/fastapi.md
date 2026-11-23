# fastapi

Run FastAPI apps which uses Uvicorn under the hood.

## Usage
- `fastapi run path/to/file.py --reload` : run a FastAPI app with automatic reload (for development)
- `fastapi dev path/to/file.py` : serve your app in both development mode
- `fastapi run path/to/file.py --host host_address --port port` : specify the host and port to run on
- `fastapi run path/to/file.py --app-dir path/to/app --app custom_app_name` : set the app variable name (if not `app`) or specify a custom app directory
- `fastapi --help` : display help
- `fastapi subcommand --help` : display help for a subcommand

## Categories
files, network

## Links
- More information: https://manned.org/fastapi
- tldr page: https://tldr.inbrowser.app/pages/common/fastapi
