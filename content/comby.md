# comby

Tool for structural code search and replace that supports many languages.

## Usage
- `comby 'assert_eq!(:[a], :[b])' 'assert_eq!(:[b], :[a])' .rs` : match and rewrite templates, and print changes
- `comby 'assert_eq!(:[a], :[b])' 'assert_eq!(:[b].Capitalize, :[a])' .rs` : match and rewrite with rewrite properties
- `comby -in-place 'match_pattern' 'rewrite_pattern'` : match and rewrite in-place
- `comby -match-only 'match_pattern' ""` : only perform matching and print matches

## Links
- More information: https://github.com/comby-tools/comby
- tldr page: https://tldr.inbrowser.app/pages/common/comby
