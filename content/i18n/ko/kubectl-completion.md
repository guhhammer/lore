# kubectl completion

`kubectl` 명령어용 쉘 자동 완성 코드 생성.

## Usage
- `kubectl completion bash|zsh|fish|powershell` : bash, Zsh, fish, 또는 PowerShell용 자동 완성 스크립트 출력
- `source <(kubectl completion bash|zsh)` : bash 또는 Zsh 자동 완성을 현재 쉘 세션에 로드
- `kubectl completion bash >>~/.bashrc` : bash 자동 완성 스크립트를 `~/.bashrc`에 추가
- `kubectl completion zsh >"${fpath[1]}/_kubectl"` : zsh 자동 완성 스크립트를 `fpath` 디렉터리에 저장
- `kubectl completion fish | source` : fish 자동 완성을 현재 쉘 세션에 로드
- `kubectl completion fish >~/.config/fish/completions/kubectl.fish` : fish 자동완성을 영구적으로 저장
- `kubectl completion powershell | Out-String | Invoke-Expression` : powerShell 자동 완성을 현재 세션에 로드
- `kubectl completion powershell >>$PROFILE` : powerShell 자동완성을 영구 저장

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_completion/
