use fibers::{Fiber, FiberStack};

fn main() {
    let mut x = 21;
    let mut fiber = Fiber::spawn(FiberStack::new(4096).expect("FiberStack"), |_main| {
        x *= 2;
    });
    while fiber.is_alive() {
        if fiber.yield_to().is_some() {
            break;
        }
    }
    println!("{x}");
}
