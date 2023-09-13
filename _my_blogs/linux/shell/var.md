
/etc/profile
中定义的是全局变量 

也有针对当前用户的profile 一般在用户家目录下
~/.bash_profile 此文件会去找~/.bash_rc

> export SOME_VAR=SOME_VALUE

导出为环境变量

子shell 中可以看到父shell导出的环境变量

也可以后期在export为环境变量

unset 可以取消环境变量 

脚本语言中 变量基本都是不需要定义的 直接赋值

每个变量的值都是字符串形式存储的 不区分变量类型

特殊场景可以使用*shell declare* 显式定义类型

双引号字符串里面的变量会被解析替换 如同php中那样。*Interpolation* 也较单引号更常用

## 查看shell变量

指令
> env

环境变量 


想查看所有变量 包括环境变量

> set

看看自己的home目录
> echo $HOME

## 变量类型

shell是弱类型语言 默认为字符串类型 

### 只读变量
~~~bash

#!/bin/bash
url="http://baidu.com"
readonly url
url="new-url.com" # 会触发错误报警

~~~