使用nvm
> export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

在：Initialize Tauri in Your App 步骤中
使用了`cargo tauri init`
期间需要填写一些问题的答案 
~~~
cargo tauri init
What is your app name?: hi_tauri
What should the window title be?: hi_tauri
Where are your web assets (HTML/CSS/JS) located, relative to the "<current dir>/src-tauri/tauri.conf.json" file that will be created?: src
What is the url of your dev server?: http://localhost:5000
~~~

不能跳过的问题是最后两个 
 一个是< current dir >    这个可能要填个public吧 先填成src了 后期在tauri.conf.json文件中改动为public了 根据意思和手册上推测应该是发布目录  "distDir": "public", 的意思
 另一个是dev server地址  
 最终根据当前文件夹的结构 以及官网的example/helloword 的项目结构改动：
 ~~~js
   "package": {
    "productName": "hi_tauri",
    "version": "0.1.0"
  },
  "build": {
    "distDir": "../build",
    "devPath": "../public",
    "beforeDevCommand": "",
    "beforeBuildCommand": ""
  },
 ~~~

 此后在public中写index.html 并改动其中的内容 再此编译就可以看结果了