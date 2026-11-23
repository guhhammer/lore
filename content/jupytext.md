# jupytext

Convert Jupyter notebooks to plain text documents, and back again.

## Usage
- `jupytext --set-formats ipynb,py path/to/notebook.ipynb` : turn a notebook into a paired `.ipynb`/`.py` notebook
- `jupytext --to py path/to/notebook.ipynb` : convert a notebook to a `.py` file
- `jupytext --to notebook path/to/notebook.py` : convert a `.py` file to a notebook with no outputs
- `jupytext --to notebook --execute path/to/notebook.md` : convert a `.md` file to a notebook and run it
- `jupytext --update --to notebook path/to/notebook.py` : update the input cells in a notebook and preserve outputs and metadata
- `jupytext -s path/to/notebook.ipynb` : update all paired representations of a notebook

## Categories
files, text

## Links
- More information: https://jupytext.readthedocs.io/en/latest/using-cli.html
- tldr page: https://tldr.inbrowser.app/pages/common/jupytext
