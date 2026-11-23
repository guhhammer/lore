# kill

Bir sürece, genellikle süreci durdurmakla ilgili bir sinyal gönder. `SIGKILL` ve `SIGSTOP` dışındaki tüm sinyaller, düzgün bir çıkış yapabilmesi için süreç tarafından yakalanabilir.

## Usage
- `kill süreç_kimliği` : bir programı varsayılan `SIGTERM` (sonlandır) sinyaliyle sonlandır
- `kill -l` : kullanılabilir sinyal adlarını listele (`SIG` öneki olmadan kullanılır)
- `kill -1 süreç_kimliği` : bir programı `SIGHUP` (hat kapandı) sinyaliyle sonlandır. Birçok arka plan programı sonlanmak yerine yeniden yüklenir
- `kill -2 süreç_kimliği` : bir programı `SIGINT` (kesme) sinyaliyle sonlandır. Bu sinyal genellikle kullanıcının `<Ctrl c>` tuşlarına basmasıyla gönderilir
- `kill -9 süreç_kimliği` : i̇şletim sistemine bir programı hemen sonlandırması için sinyal gönder (program sinyali yakalama fırsatı bulamaz)
- `kill -STOP süreç_kimliği` : i̇şletim sistemine, bir `SIGCONT` ("devam et") sinyali alınana kadar programı duraklatması için sinyal gönder
- `kill -USR1 -grup_kimliği` : verilen GID'ye (grup kimliği) sahip tüm süreçlere bir `SIGUSR1` sinyali gönder

## Links
- More information: https://manned.org/kill.1posix
