#[cxx::bridge(namespace = "cxx_noexport_test")]
mod ffi {
    #[derive(Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    extern "Rust" {
        type Engine;

        fn new_engine() -> Box<Engine>;
        fn add_point(self: &mut Engine, p: Point);
        fn report(self: &Engine) -> String;
    }
}

pub struct Engine {
    points: Vec<ffi::Point>,
}

#[must_use] 
pub fn new_engine() -> Box<Engine> {
    Box::new(Engine { points: Vec::new() })
}

impl Engine {
    fn add_point(&mut self, p: ffi::Point) {
        self.points.push(p);
    }

    fn report(&self) -> String {
        format!("Engine has {} points.", self.points.len())
    }
}
