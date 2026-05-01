#[cxx::bridge(namespace = "cxx_shared_test_v3")]
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
        fn get_points(self: &Engine) -> Vec<Point>;
        fn report(self: &Engine) -> String;
        fn report_with_label(self: &Engine, label: &str) -> String;
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

    fn report_with_label(&self, label: &str) -> String {
        format!(
            "[{}] Engine has {} points.",
            label,
            self.points.len()
        )
    }
}
