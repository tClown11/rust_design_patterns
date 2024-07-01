# 该demo 为使用 grpc demo

## build grpc 编解码代码

```shell
    protoc --go_out=. --go_opt=paths=source_relative --go-grpc_out=. --go-grpc_opt=paths=source_relative  proto/greeter.proto
```
