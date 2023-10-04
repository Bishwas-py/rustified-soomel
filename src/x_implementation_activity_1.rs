use crate::BoxColor::{Blue, Yellow};

struct ShippingBox {
    dimensions: i8,
    weight: f32,
    color: BoxColor,
}

enum BoxColor {
    Blue,
    Red,
    Yellow,
}

impl ShippingBox {
    fn create_3d_yellow_box() -> Self {
        Self {
            dimensions: 3,
            weight: 26.3,
            color: Yellow,
        }
    }

    fn display_box_dimensions(&self) {
        println!("Dimensions: {:?}", self.dimensions);
    }
    fn display_box_weight(&self) {
        println!("Weight: {:?}", self.weight);
    }

    fn display_box_color(&self) {
        match self.color {
            BoxColor::Blue => {
                println!("Box color: blue");
            }
            BoxColor::Red => {
                println!("Box color: red");
            }
            BoxColor::Yellow => {
                println!("Box color: yellow");
            }
        }
    }
}

fn main() {
    let shipping_box = ShippingBox::create_3d_yellow_box();
    shipping_box.display_box_color();
    shipping_box.display_box_dimensions();
    shipping_box.display_box_weight();
}