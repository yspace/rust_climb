
hax申请vps 搭建v2ray 
===

## 参考
- [纯ipv6搭建v2ray，hax、euserv v2ray BBR教程]( https://iweec.com/94.html)

## 申请vps

先要能访问外网^-^  还要有个telegram的号(没有就申请吧 老外很多用这个 好像等价国内微信的意思？要用手机号激活 中国区的好像锁你号码)

去 [hax](https://hax.co.id) 在每天8点的时候可以申请到（每天8点放机器 都是前面有人不要的或者没来得及续期的就被放回机器池了)
https://hax.co.id/create-vps/ 

选机器时候 数据中心选 kvm的 尽量不要选openvz（安装有些加速软件 不支持哦）
地下的选项框都勾上

## 安装 x-ui 
在浏览器端 用它的web terminal 登陆后台（ssh的）
就用这个：
> wget -N https://gitlab.com/rwkgyg/x-ui-yg/raw/main/install.sh && bash install.sh

官方那个好像有githubapi权限请求问题 会失败！（也可能有的不会）

## 安装warp
用这个可以顺利安装x-ui 但此时浏览器还是不能通过ipv6访问x-ui后台 此时需要安装 warp｜CFWarp
> x-ui 
可以选择 17 （以后标号可能就变了）按照CFWarp 协议栈支持就好 想从外部以ipv4｜ipv6 访问就安装哪个 也可以双栈支持

或者试试这个：
> bash <(curl -fsSL https://raw.githubusercontents.com/crazypeace/warp.sh/main/warp.sh) 4
有说 Github相关域名解析失败的原因 或许跟ipv4 的访问有关

## CloudFlare

做ipv6 代理（也有只做dns解析用 ）
- 添加ip跟域名的绑定 （要有自己的域名 如norm 或者eu.org 的免费域名就行了）用CF做代理 小黄云朵☁️要打开哦！
- 然后 SSL/TSL 菜单下 概述端到端加密 要选择完全
- 申请客户端证书 用来跟服务器vps做端对端通讯用
默认创建证书就好 10年证书

公钥：
~~~
-----BEGIN CERTIFICATE-----
MIIEFTCCAv2gAwIBAgIULUlZgKFafpxj5lFcfFPMPQtLVZswDQYJKoZIhvcNAQEL
BQAwgagxCzAJBgNVBAYTAlVTMRMwEQYDVQQIEwpDYWxpZm9ybmlhMRYwFAYDVQQH
Ew1TYW4gRnJhbmNpc2NvMRkwFwYDVQQKExBDbG91ZGZsYXJlLCBJbmMuMRswGQYD
VQQLExJ3d3cuY2xvdWRmbGFyZS5jb20xNDAyBgNVBAMTK01hbmFnZWQgQ0EgNmY3
MWVhOWVjN2U4ODdjYWMzZGZjOTQ4ZmYzYzNiMTUwHhcNMjIxMDAzMTM0NDAwWhcN
MzIwOTMwMTM0NDAwWjAiMQswCQYDVQQGEwJVUzETMBEGA1UEAxMKQ2xvdWRmbGFy
ZTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAMD48dD0jiLes+lzPBfo
zte/TnVlq9VGnuoBqTRd5+lQh675jLGtVFdkn60uAGvywUwuNHHCXVO8JHxdtRJ2
1LNlzZVQkBACDSp/ie2ivEUYuHQF/hm9en2hbkJR164Cu/mcFR2M5f1ccRMnvxHi
ig6uZTIAMiJlewpM0GbuTVoAd8h3Modd3MVDKCDmKIWuFLweb1UmrK0nOtVzHoD+
2cY4/0+EqLQdgWebTcb4GFJ+tNbhf7l4hzCsKj6RLYj/GQsP2BrB+eahYXcB/dcF
/xCEG4KyStbIxGoIsAYrNS/Adjf/rAP9EKvtBtx8d1nzb+zZdccga6xqlb/0pm9m
x7MCAwEAAaOBuzCBuDATBgNVHSUEDDAKBggrBgEFBQcDAjAMBgNVHRMBAf8EAjAA
MB0GA1UdDgQWBBSpW/Bsg0QaZGU2hqmJ9uFBwn0RNzAfBgNVHSMEGDAWgBQp27Ig
N3jK/cByrjsdIha6/EyWaTBTBgNVHR8ETDBKMEigRqBEhkJodHRwOi8vY3JsLmNs
b3VkZmxhcmUuY29tLzg4MTMzNjRkLTY4NGEtNDI2NS05MmRkLTc4ZGU2ODY0NzUw
Ni5jcmwwDQYJKoZIhvcNAQELBQADggEBAKGte7oadpgiGicB47ITcG2NCH9I3GI2
xPY6wbQgLSmSOvfXAv7s2rKV3E8X/oQVbsrJ7QGD9rX294nsRDVksRZd+8lTpvcn
p5oxWyjr223L1yjnORAvWhV3qFO4xTsbme3Qa1eQ9CKAagNce8z69HBXs91NGbEY
DXn2V0+M7Q992Ul/JVGZB3lc1c8BHmI1NniQu0FajbQSlMKiKxeYxrhFUiXWxlxq
AbNjrSUq/SpSLJzbS9S5ht/EJg/CfVEhL3gT+EpJ0nXHeL3W1dt/25ZtOl6GuwNS
4hUSR0iYBOhhluMzSuL2rOuR2VLtNl0WTLmlkll7wNG93oY6+JnPvSs=
-----END CERTIFICATE-----

~~~

私钥：
～～～

-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDA+PHQ9I4i3rPp
czwX6M7Xv051ZavVRp7qAak0XefpUIeu+YyxrVRXZJ+tLgBr8sFMLjRxwl1TvCR8
XbUSdtSzZc2VUJAQAg0qf4ntorxFGLh0Bf4ZvXp9oW5CUdeuArv5nBUdjOX9XHET
J78R4ooOrmUyADIiZXsKTNBm7k1aAHfIdzKHXdzFQygg5iiFrhS8Hm9VJqytJzrV
cx6A/tnGOP9PhKi0HYFnm03G+BhSfrTW4X+5eIcwrCo+kS2I/xkLD9gawfnmoWF3
Af3XBf8QhBuCskrWyMRqCLAGKzUvwHY3/6wD/RCr7QbcfHdZ82/s2XXHIGusapW/
9KZvZsezAgMBAAECggEAXpst4qWi8ZGBXthyA77NeZR+4Kth3QPscQkzbUsEoawr
eSs7QPibOuVIK108M7Al+DVG3ObPXngE4zwFw/GiQfmNGsEOf+qhjc5NTsXTy4Rd
GfS+KNxuKx1zsuvw84y/n5ZX+87SCq7BeQSpdhk8WgpB7ouOCLL76YbTPUNjhNWO
DrhJ2+UxgKWL48kbglEIJz6m7EccdDWPbdt40qTax31EB24b655cieK6ZXpAPajO
gDx+GUdxmBfZONLk536V1sZVgllnDfQPBv6b4LII4bD+FwFoktNJ++gMP1l1Ykx/
vAShurjcrX0KwjldfHQ7I18qmZ0m+yC+qNWMDil0oQKBgQD9mLYudodbFEXGY2tJ
s5daiPELef1C9JB89IlHBv+SoZqJGcMSmA7m1YI13JT+bjPqx0cInqNRR+5xDcgs
RoA6HjyH0DfXT1HP7r9d17NldoJRNR4ffvrIDzkCjAM61d0CYtf53KJlYxpAMQqu
hG/JWIAegKmONmS7d1ZtSsftmwKBgQDCzSTQVqRUzZG8HUZad4pRaEyO37gPQxDv
qjt9EytkLKJSafFt4A78KAHDDwRiWaGONzn5fSxKMsv1hCAsSCgbzYsLglZuEMra
D6pC0EAmfMhgkpUc40l+WFdNK1OmkMEo1lC/Rgpwl5PgqnzEJogJQPkn6NCTbG6Y
ywfVxee7yQKBgQCKXrvxCC18zcfwoqiUs1A/Tv8vAuXklasM8yTAQ5pJuLGoYw0k
ZSGkZBVDLFODrD/Zt9gTZPO50uTQfPOe7OzkSUE/3aXKbnY2XY+/NYWmNA6RHsmL
5+4QIPY7ZIgSn0STG7m6lN9rMLLPGbM5W/vewdJOUmexuA4ol8npomcKRwKBgGk8
AYM7CxN06iHBftwob+SCf7k/SmOUS3XtdK2gyXQYWKhE61P8EpxN0IXbevbWjNwT
IZKR+Hcs8dn0Qp0pbeF3LIJPKvnSwIMEq2nkroMcLfNdvIgakP1cI+1gVpRI5tB0
1rI92C3y9MQN1dDxL9Qv8WmJ8AN0NFr+c4EcpI9hAoGBAK2nLv4w0+tit3oQCmA1
oLk170jC4l9TYxb6bEJtK1R+Zo/Ad89fy7wP5K/di9gpZHOzxxDNbWanzwznEB/H
4nspxyb2TQlzj9VgIiXu2RoR1DO6yR7uW4khdlG/W+UYRmRm3CgcYvymuod441PD
I6XQe3qbcik7vJZealsQ4Flp
-----END PRIVATE KEY-----


～～～

## 创建入站
x-ui 后台 在入站列表中创建入站 注意端口选择443
传输 ws
域名：<cluodflare中跟ipv6绑定用的那个> 
证书：选择文件内容 
公钥：上面创建的内容
私钥: ...

创建记录成功后 `操作`或者`详情` 菜单就可以看到链接 或者二维码了 用手机或者电脑扫描或者url导入就行了

### cf 优选ip
https://github.com/XIU2/CloudflareSpeedTest

上面步骤经过后 是用域名访问的 可能速度慢 
可以使用 ip优选 做伪装 加速下