# gemini

Launch an interactive prompt with Gemini AI.

## Usage
- `gemini` : start a REPL session to chat interactively
- `echo "Summarize the history of Rome" | gemini -p -` : send the output of another command to Gemini and exit immediately
- `gemini -m model_name` : use a specific model (default: auto-gemini-3)
- `gemini -s` : run inside a sandbox container
- `gemini -i "Give me an example of recursion in Python"` : execute a prompt then stay in interactive mode
- `gemini --approval-mode default|auto_edit|yolo|plan` : set the approval mode for tool calls
- `gemini -r latest|index|session_id` : resume a session (defaults to "latest"; accepts an index number or UUID)

## Categories
shell

## Links
- More information: https://geminicli.com/docs/cli/cli-reference/
- tldr page: https://tldr.inbrowser.app/pages/common/gemini
