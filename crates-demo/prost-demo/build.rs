use prost_build::Config;
fn main() {
    println!("cargo:rerun-if-changed=person.proto");
    println!("cargo:rerun-if-changed=build.rs");
    Config::new()
        .out_dir("src/pb")
        // 可修改 bytes 使用的数据类型
        // . bytes(&["."])
        // 对目标字段使用 btree_map 类型
        .btree_map(&["scores"])
        // 为数据结构增加 serialize 和 deserialize 功能
        .type_attribute(".",  "#[derive(serde::Serialize, serde::Deserialize)]")
        // 为字段增加 serialize 和 deserialize (这里为为 data 这个字段跳过 serialize 和 deserialize 功能，因为 bytes 不支持编解码)
        .field_attribute("data", "#[serde(skip_serializing_if = \"Vec::is_empty\")]")
        .compile_protos(&["proto/person.proto"], &["."])
        .unwrap();
}