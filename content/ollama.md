# ollama

A large language model runner. For a list of available models, see <https://ollama.com/library>.

## Usage
- `ollama serve` : start the daemon required to run other commands
- `ollama run model` : run a model and chat with it (will automatically download the model if it's not downloaded)
- `ollama run model --think=false "prompt"` : run a model with a single prompt and thinking turned off
- `ollama ls` : list downloaded models
- `ollama pull model` : pull a specific model
- `ollama ps` : list running models
- `ollama rm model` : delete a model
- `ollama create new_model_name -f path/to/Modelfile` : create a model from a `Modelfile`

## Categories
network, development

## Links
- More information: https://docs.ollama.com/cli
- tldr page: https://tldr.inbrowser.app/pages/common/ollama
- Wikipedia: https://en.wikipedia.org/wiki/Ollama
