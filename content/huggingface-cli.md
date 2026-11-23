# huggingface-cli

This command is an alias of `hf`.

## Usage
- `hf auth login` : log in to Hugging Face Hub
- `hf auth whoami` : display the name of the logged in user
- `hf auth logout` : log out
- `hf env` : print information about the environment
- `hf download --repo-type repo_type repo_id filename1 filename2 ...` : download files from a repository and print out the path (omit filenames to download entire repository)
- `hf upload --repo-type repo_type repo_id path/to/local_file_or_directory path/to/repo_file_or_directory` : upload an entire folder or a file to Hugging Face
- `hf cache ls` : scan cache to see downloaded repositories and their disk usage

## Categories
network, files, monitoring

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/huggingface-cli
