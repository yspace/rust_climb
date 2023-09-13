
netstat 可以查看网络状态 可以根据端口或者服务名称看到某个服务是否启动

~~~sh

result=`netstat -lntup | grep mysqld| wc -l`

if [ $result -gt 0];then
	echo "mysql is running"
	
else
	echo "mysql is stopped"
	systemctl start mysql
fi

~~~

## nmap

nmap 也可以扫描远程或者本地ip下面的端口打开状况 用来判断某些服务是否可用：


> nmap 192.168.0.100  -p 3306

如果没有这个工具可以安装下
> yum install nmap 



可以放定时任务下监控服务是否运行中