
using postgres

using docker pull cmd ,you can download the image
docker pull <image-id>[:tag]

if you don't specify the `tag` ,docker will use the default tag: latest version of the image.

~~~cmd

docker pull postgres:12-alpine
docker run  -e POSTGRES_PASSWORD=postgres postgres:12-alpine

~~~
you can ignore the pull progress , just run the image it will firstly find the local image ,if not exist then it will download frome dock hub and run it .


contianer is a running image instance and with the specifying enviroments (files ,ports etc.)

history container

~~~cmd
docker ps -a
~~~

using docker cmd start/stop to control the container's lifetime ,by specifying the container-id

~~~
    docker start|stop xxxxxx<container-id>
~~~

docker run will create a new container 
docker start will restart the old container

### ports
we can bind the default port to specifying one . it will be useful for running two images which have different versions .

~~~
docker run -p6000:6379  reids
~~~

-p<host-port>:<container-port>

### -d
deamon mode | detached mode

as backgroud service 

### logs
docker logs <container-id>|<container-name>

to view the specifying container logs .

### --name for container name
running a image with specifying name

~~~
docker run -d -p:6000:6379 --name my-redis redis
~~~

### exec
enter the docker container in iterectively(交互式) mode

docker exec -it <container-id>|<contianer-name> /bin/bash

then give us a virtual file tree system .

we can use some restrict(受限的一些命令) cmd to view the content , ls  env  exit ... etc.


## playing with mongodb

~~~cmd

docker pull mongo
docker pull mongo-express
~~~

list networks 
> docker network ls

create a network 
> docker network create mongo-network

running a mongodb container 
> docker run -p 27017:27017 -d  -e MONGO_INITDB_ROOT_USERNAME=admin -e MONGO_INITDB_ROOT_PASSWORD=password --name mongodb --net mongo-network  mongo

view the logs
> docker logs mongodb

### mongo web admin
 https://hub.docker.com/_/mongo-express

~~~
    docker run   \
    -d \
    -p 8081:8081 \
    --network mongo-network \
    --name mongo-express \
    -e ME_CONFIG_MONGODB_ADMINUSERNAME="admin" \
    -e ME_CONFIG_MONGODB_ADMINPASSWORD="password" \
    -e ME_CONFIG_MONGODB_SERVER="mongodb" \
    mongo-express

~~~

view the logs
>  docker logs mongo-express

then go to the url [http://localhost:8081/](http://localhost:8081/)