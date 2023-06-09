// This file is already in the front_of_house module simply by its name.
pub mod hosting;

pub mod serving {
    fn take_order(order: u8) {
        println!("Ordered {order}");
    }

    pub fn serve_order() {}

    fn take_payment() {}
}
