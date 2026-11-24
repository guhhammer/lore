# turbo

High-performance build system for JavaScript and TypeScript codebases.

## Usage
- `turbo login` : log in using the default web browser with a Vercel account
- `turbo link` : link the current directory to a Vercel organization and enable remote caching
- `turbo run build` : build the current project
- `turbo run task_name --concurrency=1` : run a task without concurrency
- `turbo run task_name --force` : run a task ignoring cached artifacts and forcibly re-execute all tasks
- `turbo run task_name --parallel --no-cache` : run a task in parallel across packages
- `turbo unlink` : unlink the current directory from your Vercel organization and disable Remote Caching
- `turbo run task_name --graph=path/to/file.html|jpg|json|pdf|png|svg` : generate a Dot graph of a specific task execution (the output file format can be controlled with the filename)

## See also
- nx

## Categories
files, development, monitoring

## Links
- More information: https://turborepo.com/docs/reference
- tldr page: https://tldr.inbrowser.app/pages/common/turbo
- Wikipedia: https://en.wikipedia.org/wiki/Turbo_(software)
