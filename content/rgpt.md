# rgpt

An automated code review tool that uses GPT you can use straight from your terminal.

## Usage
- `rgpt --i "$(git diff path/to/file)"` : ask GPT to improve the code with no extra options
- `rgpt --v --i "$(git diff path/to/file)"` : get a more detailed verbose output from `rgpt` while reviewing the code
- `rgpt --max 300 --i "$(git diff path/to/file)"` : ask GPT to improve the code and limit it to a certain amount of GPT3 tokens
- `rgpt --pres 1.2 --i "$(git diff path/to/file)"` : ask GPT for a more unique result using a float value between 0 and 2. (higher = more unique)
- `rgpt --model davinci --i "$(git diff path/to/file)"` : ask GPT to review your code using a specific model
- `rgpt --json --i "$(git diff path/to/file)"` : make `rgpt` use a JSON output

## Categories
files, version-control, shell

## Links
- More information: https://github.com/vaibhav-venkat/review-gpt
- tldr page: https://tldr.inbrowser.app/pages/common/rgpt
