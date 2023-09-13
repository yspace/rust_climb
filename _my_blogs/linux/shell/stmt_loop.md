循环语句
---


## while

~~~sh

while 条件
    do
        指令
done
~~~

有个util 循环 类似while 循环 但是取反逻辑

~~~sh

echo -e "current-time \t has-running \t logged-users \t avgload-last-1min \t 5min \t 15   "

while true 
	do 
	uptime
	sleep 2 # 每隔两秒执行上面👆的命令	
done

~~~

uptime命令等效top命令的第一行输出结果

将负载情况追加到日志文件

~~~sh

while [ 1 ]
do
	uptime  >> /tmp/uptime.log
	sleep 2
done
~~~

## for


~~~sh

for 变量名 in 变量取值列表
    do
        指令...
done

~~~

循环输出用户传递给脚本的参数 后面也可以省略掉"$@"
~~~sh

for param in "$@"
do
	echo $param
done

~~~

形式二 c风格


~~~sh

for ((exp1;exp2;exp3))
    do
        指令...
done

~~~

双括号中空格有无是不影响的
~~~sh

for ((i=1;i<3;i++))
do
	echo $i
	
done
~~~


### 循环退出

break

continue 

同其他语言 

exit

会退出整个脚本的！！！

## 其他循环

- util

- select