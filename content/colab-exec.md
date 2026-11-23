# colab exec

Execute code in an active Google Colab session.

## Usage
- `echo "print('Hello from Colab')" | colab exec` : execute code provided via `stdin`
- `colab exec -f path/to/script.py` : execute a local Python script in the active session
- `colab exec -s session_name -f path/to/notebook.ipynb` : execute a local Jupyter notebook in a specific named session
- `colab exec -f path/to/script.py --output-image path/to/plot.png` : execute a script and save generated plots to an image file
- `colab exec -f path/to/script.py --timeout 60.0` : execute code with a custom timeout in seconds

## Categories
files, development

## Links
- More information: https://github.com/googlecolab/google-colab-cli#execution
- tldr page: https://tldr.inbrowser.app/pages/common/colab-exec
