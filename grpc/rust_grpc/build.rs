// 处理 protocol buffers 
// 
// 1. prost_build
// 2. tonic_build
// 
// 区别
//   1. 功能范围
//      - `prost_build`: 主要专注从 .proto 文件生成 rust 代码，用于序列化和反序列化 `protobuf` 环境消息
//      - `tonic_build`: 不仅生成消息类型，还生成 gRPC 服务代码，它实际上在内部使用 `prost_build` 来处理消息类型
//
//   2. gRPC 支持
//       - `prost_build`: 不直接支持 gRPC 。它只处理消息定义
//       - `tonic_build`: 专门设计用于 gRPC 服务。它生成客户端和服务器端的 gRPC 代码
//
//   3. 使用场景
//       - `prost_build`: 适用于只需要 protobuf 消息序列化/反序列化的项目
//       - `tonic_build`: 适用于需要实现 gRPC 服务的项目
// 
//   4. 依赖关系
//       - `prost_build`: 是 `prost` 包的一部分，专注于 protobuf
//       - `tonic_build`: 是 `tonic` 包的一部分， `tonic` 是一个 gRPC 框架，它依赖于 `prost`
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/greeter.proto")?;
    Ok(())
}
