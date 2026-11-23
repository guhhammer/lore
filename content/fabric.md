# fabric

An open-source framework for augmenting humans using AI. Provides a modular framework for solving specific problems using a crowdsourced set of AI prompts.

## Usage
- `fabric -S` : run the setup to configure fabric
- `fabric -l` : list all available patterns
- `fabric <path/to/input_file -p pattern_name` : run a pattern with input from a file
- `fabric -y "https://www.youtube.com/watch?v=video_id" -p pattern_name` : run a pattern on a YouTube video URL
- `fabric -p pattern1 | fabric -p pattern2` : chain patterns together by piping output from one to another
- `fabric -p custom_pattern_name` : run a custom user-defined pattern
- `fabric -p pattern_name -o path/to/output_file` : run a pattern and save the output to a file
- `fabric -p pattern_name -v "variable_name:value"` : run a pattern with the specified variables

## Categories
text, files, network

## Links
- More information: https://github.com/danielmiessler/fabric#usage
- tldr page: https://tldr.inbrowser.app/pages/common/fabric
