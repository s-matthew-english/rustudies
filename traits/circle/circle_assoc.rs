struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

/// Note that associated functions are called with the Struct::function() syntax, 
/// rather than the ref.method() syntax.
fn main() {
    let c = Circle::new(0.0, 0.0, 2.0);
    println!("{}", c.area());
}
