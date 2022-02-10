
官方步骤出错
cargo install tauri-cli --version ^1.0.0-beta

google到需要从github上安装：
> cargo install tauri-cli --git https://github.com/tauri-apps/tauri --branch next

## nvm 
命令行找不到nvm 
~~~shell
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash

export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion
~~~
先下载安装