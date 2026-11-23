# systemd-run

Uruchamiaj programy w przejściowych jednostkach zakresu, jednostkach usługowych lub jednostkach usługowych uruchamianych przez ścieżkę, gniazdo lub timer.

## Usage
- `sudo systemd-run komenda argument1 argument2 ...` : uruchom przejściową usługę
- `systemd-run --user komenda argument1 argument2 ...` : uruchom przejściową usługę pod menedżerem usług aktualnego użytkownika (bez uprawnień)
- `sudo systemd-run -u nazwa --description string komenda argument1 argument2 ...` : uruchom przejściową usługę z podaną nazwą jednostki i opisem
- `sudo systemd-run -r -E nazwa=wartość komenda argument1 argument2 ...` : uruchom przejściową usługę, która nie jest czyszczona po jej zakończeniu z podaną zmienną środowiskową
- `sudo systemd-run --on-calendar=wydarzenie_kalendarza komenda argument1 argument2 ...` : uruchom przejściowy timer, który okresowo uruchamia swoją przejściową usługę (zobacz `man systemd.time`, aby zapoznać się z formatem wydarzeń kalendarza)
- `systemd-run -r --pty komenda` : udostępnij terminal programowi (umożliwiając interaktywne wejście/wyjście) i zapewnij, że szczegóły wykonania pozostaną po zakończeniu programu
- `systemd-run -p MemoryMax=pamięć_w_bajtach -p CPUQuota=procent_czasu_cpu% --wait komenda` : ustaw właściwości (np. CPUQuota, MemoryMax) procesu i poczekaj, aż się zakończy
- `komenda1 | systemd-run -P komenda2 | komenda3` : użyj programu w potoku powłoki

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-run.html
