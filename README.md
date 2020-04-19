# 使用rust写的一个web版的demo

> 旨在熟练rust的用法，对于web开发出身的来说，可以互相借鉴学习

> 欢迎在issue中提交问题

> 欢迎在分支上提交你的优秀的code

+ 使用actix-web作为基础的web框架
+ mysql使用的是sqlx
+ redis使用的是mobc_redis，一种redis的连接池封装
+ log使用的是log4rs

### 问题
+ actix-web的中间件，无法获取request的body以及response的body
+ actix-web的请求体，无法支持请求上下文kv存储，如request-id等
+ log4rs日志，目前不支持异步，而且不支持kv结构存储，json后不雅观
+ sqlx暂不支持orm，需要自己手动拼写sql
+ redis暂不支持集群Cluster
