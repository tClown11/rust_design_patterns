trait File {
    fn read(&self);
}

struct TxFile {
    file_name: String,
}

impl TxFile {
    fn load_from_dist(&self) {
        println!("加载文件: {}", self.file_name);
    }

    fn new(file_name: String) -> Self {
        let file = TxFile{ file_name };
        file.load_from_dist();
        file
    }
}

impl File for TxFile {
    fn read(&self) {
        println!("读取文件: {}", self.file_name);
    }
}

struct ProxyFile {
    txt_file: TxFile,
    file_name: String,
}

impl ProxyFile {
    fn new(file_name: String) -> Self {
        Self {
            txt_file: TxFile::new(file_name.clone()),
            file_name,
        }
    }
}

impl File for ProxyFile {
    fn read(&self) {
        println!("------------ 通过代理读取文件 {} 开始 ----------------", self.file_name);
        self.txt_file.read();
        println!("------------ 通过代理读取文件 {} 结束 ----------------", self.file_name);
    }
}

fn main() {
    let proxy_file = ProxyFile::new("rust is future".to_string());
    proxy_file.read();
}
