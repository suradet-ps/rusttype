#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_between_origin_and_point() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 4.0);
        assert!((a.distance(&b) - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn translate_moves_point() {
        let mut p = Point::new(1.0, 2.0);
        p.translate(3.0, 4.0);
        assert!((p.x - 4.0).abs() < f64::EPSILON);
        assert!((p.y - 6.0).abs() < f64::EPSILON);
    }
}
