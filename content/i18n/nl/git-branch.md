# git branch

Hoofd Git-commando voor het werken met branches.

## Usage
- `git branch -a` : toon alle branches (lokaal en extern; de huidige branch is aangegeven met `*`)
- `git branch -a --contains commit_hash` : toon welke branches een specifieke Git commit in hun geschiedenis hebben
- `git branch --show-current` : toon de naam van de huidige branch
- `git branch branch_naam` : creëer een nieuwe branch gebaseerd op de huidige commit
- `git branch branch_naam commit_hash` : creëer een nieuwe branch gebaseerd op een specifieke commit
- `git branch -m oude_branch_naam nieuwe_branch_naam` : hernoem een branch
- `git branch -d branch_naam` : verwijder een lokale branch (je moet eerst wisselen naar een andere branch)
- `git push externe_naam -d externe_branch_naam` : verwijder een externe branch

## Links
- More information: https://git-scm.com/docs/git-branch
