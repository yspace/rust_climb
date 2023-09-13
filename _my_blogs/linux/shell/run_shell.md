
## 类型 

unix linux 系统都自带了不同的shell
通常都是软链

查看可用的shell
> cat /etc/shells

查看当前登陆用户默认的shell
> echo $SHELL

查看当前shell
> echo $0

### 脚本

组合简单命令 复用

胶水

可以调用其他命令行来完成任务

### 指定解释器

文件的前几个字符串 *shebang* 表示该文件由什么程序执行

~~~bash
#!/bin/bash
~~~

### 执行
一切皆文件：脚本 命令 程序

- sh <some-bash.sh> 
- bash <some-bash.sh> 
把脚本喂给 sh

上面👆的方式不存在权限问题 脚本中并未指定*shebang*时推荐使用的方式 

- ./script_name.sh
chmod +x script_name.sh 拥有执行权限后 可以直接运行

- ./script_name.sh &
后台执行  但会随shell的退出而终止

> nohup ./script.sh & 
就不会了 stdout 和 stderror 重定向到nohup.out文件

jobs 命令可以查看后台运行的进程

- source script_name.sh 

- . script_name.sh

类似php语言 include 功能么？


重定向写入 这种方式很少使用
> bash < some_script.sh
## 后台运行

> bash some-script.sh &
或者
> nohup some-script.sh &

查看后台进程
> jobs

切换后台第n个进程为前台
> fg <#n>

> CTRL+Z 
可以暂停程序

暂停后 可以用bg 把程序切到后台运行状态

> kill % <num>

杀死任务

## 父子关系

> bash 

上面的命令会在当前shell下 开启一个子shell进程

> ps -f

通过上面命令可以查看 shell关系 

注意父子进程间的变量通常都是各自私有的

## 命令结果赋值

~~~bash

var1=$(ls)

var2=`ls`

echo $var1
echo $var2

~~~
> echo ${var}  

花括号可以用来区分变量的边界   如果变量后面还想拼接一些字符串时 如: $varsome_suffix_here  加花括号就可以区分这种情况${var}some_suffix_here



花括号还能当range用 {0..10} {a..z }
> echo {0..10}

*$*是有特殊含义的 可以用转义符 *\$*
> echo \${1..10}
$1 $2 $3 $4 $5 $6 $7 $8 $9 $10
注意 数字大于9 以后就会出现歧义 $10 最好写成*${10}* 不然会被解释为$1 后跟0

小括号里面是命令

## 变量

### 环境变量
@see https://blog.51cto.com/hujiangtao/1925768

亦称全局变量 可在创建他们的shell中极其子shell中使用

又可分为自定义 和 内置 环境变量

* 环境变量用于定义Shell的运行环境，保证Shell命令的正确执行，Shell通过环境变量来确定登录用户名，命令路径，终端类型，登录目录等，所有的环境变量都是全局变量，可用于所有子进程中
对crond计划任务 一般需要重新定义环境变量

* 可在命令行设置 用户退出后变量丢失
用户家目录下的.bash_profile 及全局配置/etc/bashrc, /etc/profile /etc/prifile.d/ 目录下定义的环境变量每次都会预先被初始化

* 大写是惯例

* 有一些环境变量，如HOME，PATH，SHELL，UID，USER等，在用户登录前就已被/bin/login

程序设置好了。通常环境变量定义并保存在用户家目录下的.bash_profile或/etc/profile文件中

### 特殊变量
- $0    当前脚本名称 
- $1 $2 ... $n 代表脚本名后紧跟的变量1 2 ...
- $#  变量的个数
- $*  所有所传的参数 加双引号"$*"  就相当于"$1 $2 $3 ...$n" 整体作为字符串
- $@  有点特殊 不加双引号 则同$* 加双引号 "$@" 相当于 "$1" "$2" ... "$n"

~~~sh

set -- var1 var2 var3  # -- 清除所有参数变量 重新设置后面的三个参数
echo $#
echo $1 $2 $3

echo $*
echo $@

for i in $@ ; do echo $i ; done

for i in $* ; do echo $i ; done

for i in "$*" ; do echo $i ; done

for i in "$@" ; do echo $i ; done

~~~

对比js 或者php之类
里面 每个函数 内部 可以有 func_num_args()  func_get_args()获取当前函数所传的参数个数 所传的参数数组

$0 脚本名称 类似php中的 __FUNCTION__ 获取当前函数名称

~~~sh

echo $(dirname $0)
echo $(basename $0)

~~~

- $?     上一次指令是否成功 有点get_last_error 的意味 

比如创建两次一样的目录 第一次成功 那么第二次因为存在 则会失败

~~~bash

 cd /xx 
 [[ $? == 0 ]] && echo "成功 " || echo "失败"
~~~

$? 返回值参考
0  表示运行成功
2  权限拒绝
1～125 表示运行失败，脚本命令，系统命令错误或参数传递错误
126 找到该命令，但无法执行
127 未找到要运行的命令
128 命令被系统强制结束

生产环境：
1）用于判断命令，脚本或函数等程序是否执行成功
2）若在脚本中调用执行“exit数字”，则会返回这个数字给“$?”变量
3）如果在函数中使用“return 数字”，则会以函数返回值的形式传给“$?”


## 命令
两百个基本命令随os发行  扩充后达700-1000个

## 内置命令

不在 bin 目录下

用户登陆系统后，内置命令就被载入系统内存，一直在运行

如：
- echo
- eval 
- cd
- pwd 
- exit

## 一般命令

如 ls    磁盘上的程序文件 需要调入内存 再执行


## 常用命令

- tail            

>	tail --help

tail -f some-file.log 
持续显示文件内容 

