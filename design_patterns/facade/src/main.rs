// 零件
trait Part {
    fn run(&self);
}

struct Horn; // 喇叭
struct Screen; // 屏幕
struct Net; // 网络
struct Transformer; // 变压器

impl Part for Screen {
    fn run(&self) {
        println!("Screen is ready")
    }
}

impl Part for Horn {
    fn run(&self) {
        println!("Horn is ready")
    }
}

impl Part for Net {
    fn run(&self) {
        println!("Net is ready")
    }
}

impl Part for Transformer {
    fn run(&self) {
        println!("Transformer is ready")
    }
}


// 创建外观
struct TvFacade {
    horn: Horn,
    screen: Screen,
    net: Net,
    transformer: Transformer, 
}

impl TvFacade {
    fn init(&self) {
        self.horn.run();
        self.screen.run();
        self.net.run();
        self.transformer.run();
        println!("tv is open")
    }
}


fn main() {
    let tv = TvFacade {
        net: Net,
        screen: Screen,
        horn: Horn,
        transformer: Transformer,
    };
    tv.init()
}
