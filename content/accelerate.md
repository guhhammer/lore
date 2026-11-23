# accelerate

A library that enables the same PyTorch code to be run across any distributed configuration.

## Usage
- `accelerate env` : print environment information
- `accelerate config` : interactively create a configuration file
- `accelerate estimate-memory name/model` : print the estimated GPU memory cost of running a Hugging Face model with different data types
- `accelerate test --config_file path/to/config.yaml` : test an Accelerate configuration file
- `accelerate launch path/to/script.py --cpu` : run a model on CPU with Accelerate
- `accelerate launch path/to/script.py --multi_gpu --num_machines 2` : run a model on multi-GPU with Accelerate, with 2 machines

## Categories
development, files, hardware

## Links
- More information: https://huggingface.co/docs/accelerate/index
- tldr page: https://tldr.inbrowser.app/pages/common/accelerate
