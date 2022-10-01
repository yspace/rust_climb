https://github.com/fscarmen/warp

[八合一共存脚本+伪装站点](https://github.com/mack-a/v2ray-agent)
可能存在nginx开启不了的情况 有时候是需要卸载apache2的 不然80端口被它们占用了跟ngnix就冲突了
1.
~~~shell

 sudo apt-get --purge remove apache*

~~~

- wget -P /root -N --no-check-certificate "https://raw.githubusercontent.com/mack-a/v2ray-agent/master/install.sh" && chmod 700 /root/install.sh && /root/install.sh


[使用Hax VPS搭建V2ray](http://xiaokuqwq.hk3.20212021.ml.cdn.cloudflare.net/index.php/2022/01/20/66/)
https://iweec.com/94.html


很详细的教程[hax搭建xray 面板（x-ui）教程，支持的协议：vmess、vless、trojan、shadowsocks、dokodemo-door、socks、http](https://www.youtube.com/watch?v=-5F1KixFDbU#0&ab_channel=%E5%86%B0%E6%B2%B3%E5%9C%88%E5%AD%90#0)
2.
~~~ubuntu
sudo apt update
sudo apt install curl

bash <(curl -Ls https://raw.githubusercontent.com/vaxilu/x-ui/master/install.sh)
~~~
3.
~~~hax.helloqing.ga
-----BEGIN CERTIFICATE-----
MIIEFTCCAv2gAwIBAgIUIr5iBJSvVDAtiXLn93gCx3QhUVUwDQYJKoZIhvcNAQEL
BQAwgagxCzAJBgNVBAYTAlVTMRMwEQYDVQQIEwpDYWxpZm9ybmlhMRYwFAYDVQQH
Ew1TYW4gRnJhbmNpc2NvMRkwFwYDVQQKExBDbG91ZGZsYXJlLCBJbmMuMRswGQYD
VQQLExJ3d3cuY2xvdWRmbGFyZS5jb20xNDAyBgNVBAMTK01hbmFnZWQgQ0EgNmY3
MWVhOWVjN2U4ODdjYWMzZGZjOTQ4ZmYzYzNiMTUwHhcNMjIwNjAzMDAxNTAwWhcN
MzIwNTMxMDAxNTAwWjAiMQswCQYDVQQGEwJVUzETMBEGA1UEAxMKQ2xvdWRmbGFy
ZTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAINWB6EYddwzv6x4g315
eGFSvS2+YXEYnA8ZbDXR2Ydz0NtGUjEACJe035/Sf9IGRGeuXHU/rG7cv+d5Wk+L
TogNDb0Fu68OzPNqn7B+hH80KYbmXf77p69qRdRna0nyMSA3mj0dL5cFlvyy+G+c
k/0NA8V038y6c0liEBgI85kgxfvf5QPbUz5HsZxuV/L04IVfUeRXN3bdGaIBGbIZ
7nLEXADhbcQmYKR2gDqbLj31AaMxKg9p4ADG2g5J0huQBiFB4kOSL0hFPPy3ZMjw
+FP4k4Qy9DvtlzRqIHtJeVN5JKYU9gmguDjoScYkGZ5+7+gStjFFeIdlkcEE4hjX
b/UCAwEAAaOBuzCBuDATBgNVHSUEDDAKBggrBgEFBQcDAjAMBgNVHRMBAf8EAjAA
MB0GA1UdDgQWBBRg+GiPBrvD5ibTDv0uXqENliw6ADAfBgNVHSMEGDAWgBQp27Ig
N3jK/cByrjsdIha6/EyWaTBTBgNVHR8ETDBKMEigRqBEhkJodHRwOi8vY3JsLmNs
b3VkZmxhcmUuY29tLzg4MTMzNjRkLTY4NGEtNDI2NS05MmRkLTc4ZGU2ODY0NzUw
Ni5jcmwwDQYJKoZIhvcNAQELBQADggEBAKuDWUaAr6ANngjvnyp0KYxqAz0mFJzp
HaNwOyWrhAt72de4e4g8RZBRk+GSUDst9o+SF+DtIN9BJueLoXsnf5wVZ/QJupnr
RSHdpBbK0MkYxHO9tqrAeUU0Xw+rnIVHXEm+VCPL2ZWIuGr+NShdrISBJR0N7gYY
d+d8UnkayC9UFooKlLdTReHco9UAcxSzRGEdtKUDkiU2rr9Wm20x+jE+NKiFok+Z
hqBsuxmIwY5Y93GwlFqxU4MTziCtTIOs8UPcxoY0j8o9yMnZ7lF6yptuIldMPrh4
8jWklRqeVPvsmbJu5NdD7V2/HDeuIC9HHQwIzLZZ4lb0+5cKC3J0EM4=
-----END CERTIFICATE-----

-----BEGIN PRIVATE KEY-----
MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQCDVgehGHXcM7+s
eIN9eXhhUr0tvmFxGJwPGWw10dmHc9DbRlIxAAiXtN+f0n/SBkRnrlx1P6xu3L/n
eVpPi06IDQ29BbuvDszzap+wfoR/NCmG5l3++6evakXUZ2tJ8jEgN5o9HS+XBZb8
svhvnJP9DQPFdN/MunNJYhAYCPOZIMX73+UD21M+R7Gcblfy9OCFX1HkVzd23Rmi
ARmyGe5yxFwA4W3EJmCkdoA6my499QGjMSoPaeAAxtoOSdIbkAYhQeJDki9IRTz8
t2TI8PhT+JOEMvQ77Zc0aiB7SXlTeSSmFPYJoLg46EnGJBmefu/oErYxRXiHZZHB
BOIY12/1AgMBAAECggEAChRhcAPuUbjYAk7aepgGvONqePcGR/WFTqRbGXNSUFK7
agTG3JNd1XTQf9XaMP6Bo/puBqKdI4IGKWf3Hik5HPhxgDsPKTOH17usZ1GCbjfM
4xlNh/r85tnPY6qPQaTYj9osmHmjtCG98YVzyy8XbYGg8zfR0EbT/8MKrKmNNoiA
/CCZxb+WXeIARQIge8lM2kzfaEKoSqDe3t+HXWJFdxHkeGjnZk674c0OGxd9RQuu
G64X05XIM28iDoDMSmHPdQuv887LvTuK9HkONm35T+GX8OP/SIzG+dnLriI/imGt
pymbs+o5lth/ciRhFAnTlm/pS4j92zY5n0jTqafdsQKBgQC3rM1t6HGPmftetiir
H4hJHshWbY36sYKOP1bvbzCCMwr0l2A0AbAqgXIYHnmZjbuGedJJY1g+kKKiOp91
9cQjBwNkxAtdkAu+FRpoVrAM2KIJu0i45z4NFO3eUTetA3B7lAZUhH0zeAYLIPa9
E2jExbBP07rPtUvihDu14/S2UQKBgQC3DT65trUf1K7ATW7qOZk4nMVj0mXOoKyN
dbSS/JpppgWlVq3ihhqJOYEVglCq8P5k9v9k61qdhUw2A1aK9r/lc3wjtz8TyF6+
Loz08JIjsT4xZxgBQTbmM8ELXUnW+A9GemVQzwhJ0tvXn0t0uUFSp79qU0v1rYOL
3QnDjNjiZQKBgDdBoTujZiaRaHIB0xJIFjQJMrPtmX0F0gUBQSIfDTTeLjmPIL4E
k/71mmFvKZ3sabH3DDBP1shbstGjWT2fhjYTcg3qfJDVOPMiXiNtkOEMexL6aNJX
nu9IkJcFT/6YvkovKghbI19MiGTosdIH+MjZwEUDqXu7Su0GYwBBZ+lBAoGATu7j
MWnbyJIJmNrLDS0xKPxREa5UQDmArq8m4osSeqQgox8xdBCnuKyXQRMkfdHVoOvx
TS8/r9Ue+9uMofes3+Bgk3YdOQaZ9CBWn/hsy/9N9jeiXkgkyDdg8umTQaNN0vJM
ZgAgbtUB/4StIPQRevbiz5KDmrjrlJAH+c7uZxECgYBMR8puxZwT6/U2tgdXO3fj
bVjvfJlWostfg/Zx0lANRIRFKZVMssM9xrh6A1KkqMhpkD8zeP9UHwa5HEGXU1/O
JrVUzSAO513UWLA+YBzc0mTqhXnpzCFKo8VOZpBUxT5EUxJwt2q3YoMnYb8rRMDU
jTWZUnVbB7wr+bu5/QFo1g==
-----END PRIVATE KEY-----


~~~


https://zelikk.blogspot.com/2022/04/woiden-ipv6-vps-v2ray-vless-websocket-tls.html

[小白入门课-免费vps hax搭建八合一v2ray/xray/trojan节点](https://www.youtube.com/watch?v=rN1fwWblxyg&ab_channel=%E4%BA%91%E7%99%BD%E7%A7%91%E6%8A%80)

## ssh
https://github.com/electerm/electerm/releases

## DNS
he.net DNS 域名解析　https://boke123.net/184.html
https://www.dynu.com/en-US/ControlPanel/DDNS 添加域名成功后通知在email里面看域名服务器的地址：
～～～
Thank you for choosing Dynu!

Dynamic DNS service for gspace.eu.org has been activated. Please change name servers for your domain name at your current registrar to the ones listed below:

NS1.DYNU.COM
NS2.DYNU.COM
NS3.DYNU.COM
NS4.DYNU.COM
NS5.DYNU.COM
NS6.DYNU.COM

～～～

##

- [【甬哥YouTube教程】纯IPV6免费VPS系列二：Hax IPV6 free/Woiden NAT VPS连接SSH多样设置跳板EUserv，Hax相关设置说明，warp电脑客户端中转，Hax密码设置TUN永久开启脚本](https://ygkkk.blogspot.com/2022/04/youtubeipv6vpshax-ipv6-freewoiden-nat.html)

使自己机器具有双ip栈

神奇的warp cloudflare出品
浏览器下载地址: 1.1.1.1      这地址真厉害 跟110似的 一般人没资格用^-^

安上这个 然后点击大按钮🔘开启私密internet功能 浏览器就能用ipv6 地址访问vps啦： [ipv6]:<port>   如：http://[2001:41d0:203:698:1234:1294:ad9:1]:54320/     这样就可以访问x-ui后台啦 端口是x-ui安装后 给你的

然后 ssh客户端 electerm 也可以通过 ipv6 + 22端口 访问到vps啦 不开warp 是无法访问的！！！

hax机器申请下来后 安装os系统
vps-info 里面 有个 Nat VPS 这个就是开ipv4端口的   点击这个开通后 就给你个ipv4 访问地址 然后用ssh客户端就可以访问啦 不用使用那个难用的
`Web Base Terminal` 不想用ipv6 或者没装warp时 就可以用这个开启端口转发功能
