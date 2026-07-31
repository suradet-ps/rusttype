enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn describe(&self) -> String {
        match self {
            Shape::Circle(r) => format!("Circle with radius {r}"),
            Shape::Rectangle(w, h) => format!("Rectangle {w}x{h}"),
            Shape::Triangle { base, height } => {
                format!("Triangle with base {base} and height {height}")
            }
        }
    }
}

fn describe_number(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1..=9 => "single digit",
        10..=99 => "double digit",
        _ => "large",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_area() {
        let s = Shape::Circle(5.0);
        assert!((s.area() - 25.0 * std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn rectangle_area() {
        let s = Shape::Rectangle(3.0, 4.0);
        assert!((s.area() - 12.0).abs() < f64::EPSILON);
    }

    #[test]
    fn triangle_area() {
        let s = Shape::Triangle {
            base: 6.0,
            height: 4.0,
        };
        assert!((s.area() - 12.0).abs() < f64::EPSILON);
    }

    #[test]
    fn number_categories() {
        assert_eq!(describe_number(0), "zero");
        assert_eq!(describe_number(5), "single digit");
        assert_eq!(describe_number(50), "double digit");
        assert_eq!(describe_number(100), "large");
    }
}
