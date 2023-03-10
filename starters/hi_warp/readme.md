
测试服务端在cors下的响应
~~~bash
curl -X OPTIONS localhost:8080/questions \
-H "Access-Control-Request-Method: PUT" \
-H "Access-Control-Request-Headers: content-type" \
-H "Origin: https:/ /not-origin.io" -verbose
~~~