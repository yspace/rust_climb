https://citizix.com/running-postgresql-14-with-docker-and-docker-compose/

~~~cmd

 docker run -d \
        --name my-postgres \
        -p 5432:5432 \
        -v ~/apps/postgres:/var/lib/postgresql/data \
        -e POSTGRES_PASSWORD=yiqing \
        -e POSTGRES_USER=qing \
        -e POSTGRES_DB=hello_pg \
         postgres:14-alpine
~~~

In the above command:

The -d instructs docker container to run as a detached process. It run container in background and print container ID
-p is for port mapping. We are instructing the container to expose the container port externally. Container port 5432 is mapped to host port 5432. That means the service can be accessed through localhost:5432
The -v directive is used to mount volumes. In our case we are mounting the container volume /var/lib/postgresql/data to host path ~/apps/postgres. Containers are ephemeral devices that will contain its data for the time it is running. Once a container is stopped, its data is lost. Mounting volumes ensures that the data is added to a host path that can be reused when the container is restarted.
The -e argument is for the environment variables. The supplied environment variables will be used to set up a Postgres user, password and a database.

To check that our container is running as expected, use the docker ps command:
> docker ps

We can login to the container using the docker exec command while executing /bin/bash interactively. Here we are also logging in to posgtres with the credentials we specified above and checking the version.

➜ docker exec -it my-postgres /bin/bash

> psql -U qing -d hello_pg;
> select version();

If you need to clean up the container when not in use, you can stop and remove the container using this command:

~~~shell

docker stop my-postgres

# Removing
docker rm my-postgres
~~~

## docker composer 

Here is how we would use docker-compose to achieve the functionality above. Save this as docker-compose.yaml:

~~~docker-compose

version: '3.9'

services:
  postgres:
    image: postgres:14-alpine
    ports:
      - 5432:5432
    volumes:
      - ~/apps/postgres:/var/lib/postgresql/data
    environment:
      - POSTGRES_PASSWORD=yiqing
      - POSTGRES_USER=qing
      - POSTGRES_DB=hello_pg

~~~

Now bring up the containers:

> docker-compose up -d

The commands:

up brings up the container
-d in a detached mode

Verify the container processes using the ps command:
>  docker-compose ps

To login to the container and login to postgres, use this:

> docker-compose exec postgres /bin/bash
> psql -U qing -d hello_pg;
> select version();


## 客户端
macos 本机 安装 psql 加入到path环境变量

> brew install libpq
> echo 'export PATH="/usr/local/opt/libpq/bin:$PATH"' >> ~/.zshrc; 

## 创建表

列类型： https://www.postgresql.org/docs/9.5/datatype.html
列类型： https://www.postgresql.org/docs/current/datatype.html

~~~psql

create table person(
id bigserial not null primary key,
name varchar(255) not null ,
gender varchar(7) not null,
birthday date not null ,
email varchar(255)
);
~~~

测试数据灌入：
https://mockaroo.com/

下载生成的数据 
在psql 下 \i 导入sql文件


## rust中使用pg

不错的例子: https://github.com/mvniekerk/tokio-cron-scheduler/blob/main/src/postgres/mod.rs