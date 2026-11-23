# git branch

Comanda principală Git pentru lucrul cu ramuri.

## Usage
- `git branch -a` : listează toate ramurile (locale și la distanță; ramura curentă este evidențiată cu un `*`)
- `git branch -a --contains hash_commit` : listează care ramuri includ un commit Git specific în istoricul lor
- `git branch --show-current` : afișează numele ramurii curente
- `git branch nume_ramură` : creează o ramură nouă pornind de la commit-ul curent
- `git branch nume_ramură hash_commit` : creează o ramură nouă pornind de la commit-ul specificat
- `git branch -m nume_ramură_veche nume_ramură_nouă` : redenumește o ramură (trebuie să comuti pe o altă ramură înainte de a face asta)
- `git branch -d nume_ramură` : șterge o ramură locală (trebuie să comuti pe o altă ramură înainte de a face asta)
- `git push nume_la_distanță -d nume_ramură_la_distanță` : șterge o ramură la distanță

## Links
- More information: https://git-scm.com/docs/git-branch
