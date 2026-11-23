# todo

Um gerenciador de tarefas simples, de interface de linha de comando e em conformidade com os padrões.

## Usage
- `todo list --startable` : lista tarefas iniciáveis
- `todo new coisas_para_fazer -l trabalho` : adiciona uma nova tarefa à lista de trabalho
- `todo edit --location nome_local id_tarefa` : adiciona um local para uma tarefa com um ID provido
- `todo show id_tarefa` : mostra detalhes sobre uma tarefa
- `todo done id_tarefa1 id_tarefa2 ...` : marca tarefas com os IDs especificados como concluídas
- `todo delete task_id` : exclui uma tarefa
- `todo flush` : exclui tarefas concluídas e redefine os IDs das tarefas restantes

## Links
- More information: https://todoman.readthedocs.io/en/stable/man.html
