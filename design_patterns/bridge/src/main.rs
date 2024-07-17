
// 定义实现接口
trait Implementor {
    fn op_impl(&self);
}

// 定义具体的实现类
struct ConcreteImplA;
impl Implementor for ConcreteImplA {
    fn op_impl(&self) {
        println!("实现类 ConcreteImplA op ...")
    }
}

struct ConcreteImplB;
impl Implementor for ConcreteImplB {
    fn op_impl(&self) {
        println!("实现类 ConcreteImplB op ...")
    }
}

// 定义抽象接口
trait Abstraction {
    fn op(&self);
}

// 实现具体的抽象类
struct RefinedAbstraction {
    // 有一个实现累的引用
    implementor: Box<dyn Implementor>,
}

impl Abstraction for RefinedAbstraction {
    fn op(&self) {
        self.implementor.op_impl();
    }
}

impl RefinedAbstraction {
    fn new(implementor: impl Implementor + 'static) -> Self {
        Self {
            implementor: Box::new(implementor),
        }
    }
}

fn main() {
      // 创建具体的实现类
      let impl_a = ConcreteImplA;
      let impl_b = ConcreteImplB;
  
      // 创建具体的抽象类
      let abs_a = RefinedAbstraction::new(impl_a);
      let abs_b = RefinedAbstraction::new(impl_b);
  
      // 调用抽象类的操作方法
      abs_a.op();
      abs_b.op();
}
