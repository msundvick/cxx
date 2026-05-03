#[cxx::bridge]
mod ffi {
    // 1. A shared struct (data layout matches exactly in C++ and Rust)
    #[derive(Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    // 2. An opaque Rust type (C++ knows it exists, but Rust manages the memory/layout)
    extern "Rust" {
        type Engine;

        // Constructor
        fn new_engine() -> Box<Engine>;

        // Methods (note the self references)
        fn add_point(self: &mut Engine, p: Point);
        fn get_points(self: &Engine) -> Vec<Point>;
        fn report(self: &Engine) -> String;
    }
}

// 3. The actual Rust implementation
pub struct Engine {
    points: Vec<ffi::Point>,
}

pub fn new_engine() -> Box<Engine> {
    Box::new(Engine { points: Vec::new() })
}

impl Engine {
    fn add_point(&mut self, p: ffi::Point) {
        println!("Rust: Added point ({}, {})", p.x, p.y);
        self.points.push(p);
    }

    fn get_points(&self) -> Vec<ffi::Point> {
        self.points.clone()
    }

    fn report(&self) -> String {
        format!(
            "Engine successfully processed {} points.",
            self.points.len()
        )
    }
}
