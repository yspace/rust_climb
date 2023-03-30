
网格边界对外的访问点

可以共用 也可以按服务配置 如同城门那样

内部的虚拟服务可以绑定到gateway

按照流向分 ingress 和 egress

* ingress gateway:
控制外部服务访问内部服务 配合virtualService

* Egress gateway：
控制网格内部服务访问外部服务 配合VirtrualService DesinationRule ServiceEntry的使用

通过这个访问时因为内网服务对外的权限 或者为了服务治理功能

## 实现原理
 gateway 与普通sidecar 均通过envoy作为proxy 实行流量控制。pilot 为不同类型的proxy生成相应的配置
 前者类型为router 后者类型为sidecar

 ## 访问外部服务
 egressGateway

 指定 ipRange
 serviceEntry