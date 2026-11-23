# git bisect

Gebruik binair zoeken om de commit te vinden die een bug heeft geïntroduceerd. Git springt automatisch heen en weer in de commitgrafiek om de foutieve commit te vinden.

## Usage
- `git bisect start foutieve_commit schone_commit` : start een bisect op een commitbereik dat is begrensd door een bekende, foutieve commit en een bekende, schone (meestal oudere) commit
- `git bisect good|bad` : voor elke commit die `git bisect` selecteert, markeer het als "bad" of "good" na het testen voor het probleem
- `git bisect reset` : sluit de bisect-sessie en keer terug naar de vorige branch
- `git bisect skip` : sla een commit over tijdens een bisect (b.v. een commit die de tests faalt vanwege een ander probleem)
- `git bisect start foutieve_commit schone_commit -- pad/naar/bestand_of_map` : start een bisect-sessie en houd hierbij alleen rekening met commits die een bepaald bestand of een bepaalde map aanpassen
- `git bisect run pad/naar/testscript optionele_script_argumenten` : automatiseer het bisect-proces met behulp van een testscript dat afsluit via `exit` met 0 voor "good" en met een niet-nul exitcode voor "bad"
- `git bisect log` : toon een log van wat er tot nu toe is gedaan
- `git bisect visualize` : toon de resterende kandidaat-commits die nog gecontroleerd moeten worden

## Links
- More information: https://git-scm.com/docs/git-bisect
