后期数据服务使用了rust！

早期用cassandra 大概200个节点

后期切换到scyllaDB 只用大概1/4节点数

延迟也大概提升五六十倍
cassandra{ 5-70ms }
scyllaDb{ 5ms }