use rabbithole_macro::rabbithole;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    #[rabbithole]
    fn distance(&self, other: &Point) -> f32 {
        let y_delta = self.y - other.y;
        let x_delta = self.x - other.x;
        let dist = ((y_delta.pow(2) + x_delta.pow(2)) as f32).sqrt();
        return dist;
    }
}

fn main() {
    let point_1 = Point::new(1, 1);
    let point_2 = Point::new(2, 2);
    println!(
        "distance between the points! {:?}",
        point_1.distance(&point_2)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distance_q_1() {
        let point_1 = Point::new(1, 1);
        let point_2 = Point::new(1, 1);

        assert_eq!(0., point_1.distance(&point_2));
    }
}
