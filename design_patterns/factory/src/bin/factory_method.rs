trait Product {
    fn weight(&self) -> f64;
}

trait Factory {
    type ProductType;
    fn create_product(&self) -> Self::ProductType;
}

struct Pen;

impl Product for Pen {
    fn weight(&self) -> f64 {
        println!("this pen weight is 150 g");
        150.0
    }
}

struct PenFactory;

impl Factory for PenFactory {
    type ProductType = Pen;
    fn create_product(&self) -> Self::ProductType {
        Pen
    }
}

struct Car;

impl Product for Car {
    fn weight(&self) -> f64 {
        println!("this car weight is 1.5 T");
        1500000.0
    }
}

struct CarFactory;
impl Factory for CarFactory {
    type ProductType = Car;
    fn create_product(&self) -> Self::ProductType {
        Car
    }
}

fn main() {
    CarFactory.create_product().weight();
    PenFactory.create_product().weight();
}