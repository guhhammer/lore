# kaggle

Official CLI for Kaggle implemented in Python 3.

## Usage
- `kaggle config view` : view current configuration values
- `kaggle c download competition -f path/to/file` : download a specific file from a competition dataset
- `kaggle c list -s search_term` : list competitions matching a search term
- `kaggle c files competition` : list files available for a specific competition
- `kaggle c submit competition -f path/to/submission.csv -m "message"` : submit a file to a competition with a message
- `kaggle d list -s search_term` : list datasets matching a search term
- `kaggle d download owner/dataset_name` : download all files from a dataset
- `kaggle k list -s search_term` : list kernels (notebooks) matching a search term

## Categories
network, files

## Links
- More information: https://github.com/Kaggle/kaggle-api/blob/main/docs/README.md
- tldr page: https://tldr.inbrowser.app/pages/common/kaggle
