# neofetch-on-windows
The alternative of neofetch but on windows and writed on Rust! 

English:

For install this on your terminal, download rustup and in folder of project enter this command: cargo install --path . <- Dot require!

Yay! Now you can run neofetch anywhere and your neofetch will be displayed.
How can I make Neofetch automatically open when opening PowerShell or the Windows Terminal? To do this, enter these commands one by one in powershell!:
1.: if (!(Test-Path $PROFILE)) { New-Item -Type File -Path $PROFILE -Force }
2.: Add-Content -Path $PROFILE -Value "neofetch"

Russian:

Для установки в вашем терминале, установите RustUp с официального сайта и в папке с сурсами прокета введите эту команду: cargo install --path . <-Точка необходима

Тадам! Теперь введя где угодно neofetch у вас выведется ваш neofetch.
Как сделать так чтобы при открытии powershell / windows terminal автоматически открывался neofetch? Для этого введите эти команды поочереди в powershell!:
1.: if (!(Test-Path $PROFILE)) { New-Item -Type File -Path $PROFILE -Force }
2.: Add-Content -Path $PROFILE -Value "neofetch"

Пример вывода:

  ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄   OS: Windows 10 (19045)
  █████████████████   █████████████████   Host: DESKTOP-PVEGVN4
  █████████████████   █████████████████   Kernel: 19045
  █████████████████   █████████████████   Uptime: 541 min
  █████████████████   █████████████████   Resolution: 1920x1080
  █████████████████   █████████████████   CPU: Intel(R) Core(TM) i3-6300 CPU @ 3.80GHz
  █████████████████   █████████████████   CPU Arch: x86_64
                                          GPU: NVIDIA GeForce GTX 1650
  █████████████████   █████████████████   Memory available: 2.33 GB
  █████████████████   █████████████████   Memory total: 11.94 GB
  █████████████████   █████████████████
  █████████████████   █████████████████
  █████████████████   █████████████████
  █████████████████   █████████████████
  ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀   ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
