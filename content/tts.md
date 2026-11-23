# tts

Synthesize speech.

## Usage
- `tts --text "text"` : run text-to-speech with the default models, writing the output to `tts_output.wav`
- `tts --list_models` : list provided models
- `tts --model_info_by_idx model_type/model_query_idx` : query info for a model by idx
- `tts --model_info_by_name model_type/language/dataset/model_name` : query info for a model by name
- `tts --text "text" --model_name model_type/language/dataset/model_name` : run a text-to-speech model with its default vocoder model
- `tts --text "text" --model_path path/to/model.pth --config_path path/to/config.json --out_path path/to/file.wav` : run your own text-to-speech model (using the Griffin-Lim vocoder)

## Categories
text, files

## Links
- More information: https://github.com/coqui-ai/TTS#command-line-tts
- tldr page: https://tldr.inbrowser.app/pages/common/tts
