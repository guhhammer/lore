# esac

This shell keyword is used to end a `case` statement.

## Usage
- `case $COUNTRULE in words) wc --words README ;; lines) wc --lines README ;; esac` : match a variable against string literals to decide which command to run
- `case $COUNTRULE in [wW]|words) wc --words README ;; [lL]|lines) wc --lines README ;; *) echo "what?" ;; esac` : combine patterns with |, use * as a fallback pattern
- `case $ANIMAL in cat) echo "It's a cat" ;;& cat|dog) echo "It's a cat or a dog" ;;& *) echo "Fallback" ;; esac` : allow matching multiple patterns
- `case $ANIMAL in cat) echo "It's a cat" ;& dog) echo "It's either a dog or cat fell through" ;& *) echo "Fallback" ;; esac` : continue to the next pattern's commands without checking the pattern
- `help case` : display help

## Categories
text, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/esac
