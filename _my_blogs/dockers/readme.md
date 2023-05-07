
## 网络

> docker network ls

可以查看已经建立的网络

host 访问容器内部的网络 一般做法就是端口映射

容器内部想访问宿主机的网络 
docker版本高于v20.10（2020/12/4 更新）

可在启动docker容器时 时加入
> --add-host=host.docker.internal:host-gateway

容器内部直接通过 **host.docker.internal:PORT** 来访问宿主机的服务

对于docker-compose
在container声明中加入：
> extra_hosts:
  - "host.docker.internal:host-gateway"
  
  
更低版本需要用老方法访问