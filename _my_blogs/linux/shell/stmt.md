条件语句
---


## 单分支


~~~sh

if [ 条件 ] 
	then
		指令
fi

if [ 条件 ]; then
		指令
fi		


~~~
分号相当于命令换行

## 双分支条件语句

~~~sh

if [ 条件 ] 
    then
	 指令
	else
	 指令
fi

~~~

## 多分枝

~~~sh

if [ 条件1 ]；then

    指令1

elif [ 条件2 ]；then

    指令2

elif [ 条件3 ]；then

    指令3

elif [ 条件4 ]；then

    指令4

else

    指令n

fi
~~~
