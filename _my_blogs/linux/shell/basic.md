每个进程启动 会打开三个文本流端口： stdin stdout stderr

对应标准输入 输出 错误输出

输出重定向 *>*

> ls > dir_log

追加 *>>*

> ls >> dir_log

输入重定向类似
> wc -l < /dev/null

### 内联输入重定向
cmd << marker

~~~bash
less <<<EOF
obj 1
obj 2
obj 3
EOF

~~~

cmd <<<EOF ....  EOF >file 
	 同时将输入存为文件
	 
	 
	 
## 管道

cmd1 | cmd2 | cmd3 .......

> ls /bin/ | less

管道是一种进程间通讯的方式 IPC

## 字符 变量 运算
 字符
 # 注释
 
 '...' 字符串
 
 \ 转义
 
 / 路径分割符
 
！ 逻辑反     如 > ls a[!0-9]

## 输入输出

- echo 做输出
- read 读取用户｜程序 输入

> help read 
看看帮助文档

~~~sh

read -t 10 -p "请输入一个数" num
echo $num

read  -p "请输入两个数" num1 num2
if [ $num1 -gt $num2 ]; then
	echo " $num1 > $num2  "
elif [ $num1 -eq $num2 ]; then
	echo " $num1 == $num2 "
else
		echo "$num1 < $num2 "
fi
~~~
 