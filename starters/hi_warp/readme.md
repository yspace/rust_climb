
测试服务端在cors下的响应
~~~bash
curl -X OPTIONS localhost:8080/questions \
-H "Access-Control-Request-Method: PUT" \
-H "Access-Control-Request-Headers: content-type" \
-H "Origin: https:/ /not-origin.io" -verbose
~~~

压测工具：

wrk 基于时间的压测 指定的时间到了就强行关闭线程
> wrk http://localhost:8080 -c 128 -t 4 -d 1
128并发 4个线程 持续一秒

ab  基于数量的