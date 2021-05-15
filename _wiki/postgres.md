## 参考：

- https://blog.csdn.net/helloword4217/article/details/90217603
- [Postgres远程访问配置 ](https://wenku.baidu.com/view/5a315325ce84b9d528ea81c758f5f61fb73628c9.html)
- [Postgresql允许远程访问配置修改 ](https://www.cnblogs.com/sage-blog/p/10170188.html)
- [PostgreSQL访问控制，允许&禁止指定IP访问 ](https://blog.csdn.net/zxb4221v/article/details/104581154)

由于是默认安装所以 ubuntu下并没有开启防火墙 
对于防火墙相关的设置 先忽略

## Postgresql允许远程访问配置修改



    修改监听

sudo vi /etc/postgresql/10/main/postgresql.conf 

  

将 #listen_addresses = ‘localhost’ 的注释去掉并改为 listen_addresses = '*'

    修改可访问用户的IP段

sudo vi /etc/postgresql/10/main/pg_hba.conf 



在文件末尾添加： host all all 0.0.0.0/0 md5 ，表示允许任何IP连接

    重启数据库

sudo /etc/init.d/postgresql restart