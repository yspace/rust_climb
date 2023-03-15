https://www.arvintian.cn/post/web%E7%BD%91%E5%85%B3/%E4%BB%A3%E7%90%86%E6%9C%8D%E5%8A%A1%E5%99%A8%E4%B9%8Btreafik/


traefik(https://traefik.io/) 
go实现的代理服务器
支持代理http、tcp、udp流量代理
开源的反向代理与负载均衡工具。
适合与微服务系统结合，可实现自动化动态配置。

支持 Docker, Swarm, Mesos/Marathon, Mesos, Kubernetes, Consul, Etcd, Zookeeper, BoltDB, Rest-API 等后端模型

应用集成
traefik通过providers支持了各种服务发现，我们可以用在各种场景下作为我们的网关入口，尤其是容器部署的无状态服务，服务更新后providers可以动态加载服务地址更新service。
目前支持的provider：

    Docker
    Kubernetes
    Consul
    KV(etcd、redis)
    等等



daocloud