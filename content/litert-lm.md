# litert-lm

Run and manage LiteRT-LM models.

## Usage
- `litert-lm list` : list all imported models
- `litert-lm run path/to/model.litertlm|model_id` : run a local or imported model interactively
- `litert-lm run --from-huggingface-repo owner/repository model.litertlm --prompt "What is the capital of France?"` : run a model directly from a Hugging Face repository with a single prompt
- `litert-lm run path/to/model.litertlm|model_id --backend gpu` : run a model using GPU acceleration
- `litert-lm run path/to/model.litertlm|model_id --vision-backend gpu|cpu --attachment path/to/image.jpg --prompt "Describe this image."` : run a multimodal model with an image attachment
- `litert-lm run path/to/model.litertlm|model_id --preset path/to/preset.py` : run a model with Python function calling tools via a preset file
- `litert-lm benchmark path/to/model.litertlm|model_id` : benchmark a model's performance
- `litert-lm serve --port 9379` : start an OpenAI-compatible API server on a specific port

## Categories
files, hardware, monitoring

## Links
- More information: https://developers.google.com/edge/litert-lm/cli/usage
- tldr page: https://tldr.inbrowser.app/pages/common/litert-lm
