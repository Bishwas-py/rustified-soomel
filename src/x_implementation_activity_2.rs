struct ShippingBox {
    dimensions: Dimensions,
    weight: f32,
    color: BoxColor,
}

struct Dimensions {
    height: i32,
    length: i32,
    width: i32,
}

enum BoxColor {
    Blue,
    Red,
    Yellow,
}

impl BoxColor {
    fn display(&self) {
        print!("Color: ");
        match self {
            BoxColor::Blue => { println!("blue") }
            BoxColor::Red => { println!("red") }
            BoxColor::Yellow => { println!("yellow") }
        }
    }
}

impl Dimensions {
    fn display(&self) {
        println!("Height:  {:?}", self.height);
        println!("Width:  {:?}", self.width);
        println!("Length:  {:?}", self.length);
    }
}

impl ShippingBox {
    fn create_3d_yellow_box() -> Self {
        Self {
            dimensions: Dimensions {
                height: 23,
                length: 32,
                width: 23
            },
            weight: 26.3,
            color: BoxColor::Yellow,
        }
    }

    fn display_box_weight(&self) {
        println!("Weight: {:?}", self.weight);
    }
}

fn main() {
    let shipping_box = ShippingBox::create_3d_yellow_box();
    shipping_box.color.display();
    shipping_box.dimensions.display();
    shipping_box.display_box_weight();
}