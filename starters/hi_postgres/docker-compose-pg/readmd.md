https://renzolucioni.com/postgres-docker-compose/

> postgres/postgres.conf is a Postgres config file. I’m mounting it as a volume so the container can read it when Postgres is started by command. To make the Postgres server listen for connections from clients on all available IP interfaces - useful when the server is running as a container and you want to connect to it from your host - put the following in your postgres/postgres.conf:

listen_addresses = '*'
postgres/data/ is a directory that will be created on your host when you start Postgres. I’m mounting it as a volume to persist data written by the container, meaning that data written to Postgres will survive if the container is removed. The :delegated suffix means that writes by the container to this volume may not be immediately reflected on the host file system (i.e., the container’s view of the volume is authoritative). Delegating write-heavy mounts like this one reduces Docker’s resource usage and gives you better performance than other volume mount configurations. Giving up some consistency guarantees like this is usually acceptable when working locally, especially when the data written is ephemeral or can be easily reproduced.

## 进命令行
-U 后面是角色名称 

进入容器 docker exec -it postgresql（postgesql的容器name） /bin/bash
连接数据库 psql -U postgres(role) postgres(数据库名字)

> psql -U yiqing

    现在可以使用db命令了

    列出所有数据库 \l
    查看数据库|链接到特定db \c <db-name>
    显示数据表 \dt
    显示表对应的字段信息 \d <table-name>
    显示引用表 \det
