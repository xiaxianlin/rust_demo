#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Bounds {
    top_left: Point,
    bottom_right: Point,
}

trait Drawable {
    fn bounds(&self) -> Bounds;
}

struct Square(i64, i64, i64);

impl Square {
    fn new(a: i64, b: i64, c: i64) -> Self {
        Square(a, b, c)
    }
}

impl Drawable for Square {
    fn bounds(&self) -> Bounds {
        Bounds {
            top_left: Point {
                x: self.0,
                y: self.1,
            },
            bottom_right: Point {
                x: self.1,
                y: self.2,
            },
        }
    }
}

#[derive(Debug)]
struct Circle(i64, i64, i64);

impl Circle {
    fn new(a: i64, b: i64, c: i64) -> Self {
        Circle(a, b, c)
    }
}

impl Drawable for Circle {
    fn bounds(&self) -> Bounds {
        Bounds {
            top_left: Point {
                x: self.0,
                y: self.1,
            },
            bottom_right: Point {
                x: self.1,
                y: self.2,
            },
        }
    }
}

struct Container<T>(T);
impl<T: Drawable> Container<T> {
    fn area(&self) -> i64 {
        let bounds = self.0.bounds();
        (bounds.bottom_right.x - bounds.top_left.x) * (bounds.bottom_right.y - bounds.top_left.y)
    }
}

impl<T: Drawable + std::fmt::Debug> Container<T> {
    fn show(&self) {
        println!("{:?} has bounds {:?}", self.0, self.0.bounds());
    }
}

trait Shape: Drawable {
    fn render_in(&self, bounds: Bounds);
    fn render(&self) {
        // self.render_in(overlap(SCREEN_BOUNDS, self.bounds()));
    }
}

pub fn test() {
    let square = Container(Square::new(1, 2, 2)); // Square is not Debug
    let circle = Container(Circle::new(3, 4, 1)); // Circle is Debug

    println!("area(square) = {}", square.area());
    println!("area(circle) = {}", circle.area());
    circle.show();
}
