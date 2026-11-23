# git notes

Add or inspect object notes.

## Usage
- `git notes list` : list all notes and the objects they are attached to
- `git notes list object` : list all notes attached to a given object (defaults to `HEAD`)
- `git notes show object` : show the notes attached to a given object (defaults to `HEAD`)
- `git notes append object` : append a note to a specified object (opens the default text editor)
- `git notes append --message="message_text"` : append a note to a specified object, specifying the message
- `git notes edit object` : edit an existing note (defaults to `HEAD`)
- `git notes copy source_object target_object` : copy a note from one object to another
- `git notes remove object` : remove all the notes added to a specified object

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-notes
- tldr page: https://tldr.inbrowser.app/pages/common/git-notes
