# colab

Command-line interface for Google Colab. Some subcommands such as `run` and `exec` have their own usage documentation.

## Usage
- `colab new -s session_name --gpu T4|L4|G4|A100|H100` : allocate a new named session with a specific GPU
- `colab sessions` : list all active sessions
- `colab run --gpu T4|L4|A100|H100 path/to/script.py` : run a local script on a fresh GPU VM and automatically release it upon completion
- `colab exec -s session_name -f path/to/file.py|path/to/notebook.ipynb` : execute a local Python script or Jupyter notebook in a specific session
- `colab console -s session_name` : connect to an interactive TTY shell on the remote VM
- `colab drivemount -s session_name` : mount Google Drive in a specific session
- `colab download -s session_name path/to/remote_file path/to/local_destination` : download a file from the remote session
- `colab stop -s session_name` : stop and terminate a specific session

## Categories
files, network, hardware

## Links
- More information: https://github.com/googlecolab/google-colab-cli#command-index
- tldr page: https://tldr.inbrowser.app/pages/common/colab
