use fibers::{Fiber, FiberStack};

fn main() {
    let mut x = 21;
    let mut fiber = Fiber::spawn(FiberStack::new(4096).expect("FiberStack"), |main| {
        for _ in 0..10 {
            println!("yield");
            main.yield_to();
        }
        x *= 2;
    });
    while fiber.is_alive() {
        fiber.yield_to();
    }
    println!("{x}");
}
