use plotters::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn plot_trajectory(filename: &str, coordinates: Vec<(i32, i32)>) {
    let root_drawing_area = BitMapBackend::new(filename, (800, 800)).into_drawing_area();
    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .caption("Submarine trajectory", ("Arial", 30))
        .margin(5)
        .build_cartesian_2d(0..1000, 0..1000)
        .unwrap();

    chart
        .draw_series(PointSeries::of_element(
            coordinates,
            3,
            &BLACK,
            &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))
        .unwrap();

    chart.configure_mesh().draw().unwrap();
}

struct Position {
    x: i32, // horizontal movement
    y: i32, // depth movement
    aim: i32,
}

impl Position {
    pub fn new() -> Self {
        Position { x: 0, y: 0, aim: 0 }
    }

    fn star_one(&mut self, direction: &str, distance: i32) {
        match direction {
            "forward" => self.x += distance,
            "down" => self.y += distance,
            "up" => self.y -= distance,
            _ => panic!("Unknown direction: {}", direction),
        }
    }

    fn star_two(&mut self, direction: &str, distance: i32) {
        match direction {
            "forward" => {
                self.x += distance;
                self.y += self.aim * distance;
            }
            "down" => self.aim += distance,
            "up" => self.aim -= distance,
            _ => panic!("Unknown direction: {}", direction),
        }
    }
}

fn main() {
    let file = File::open("input").expect("File not found");
    let buf_reader = BufReader::new(file);

    let movements: Vec<(String, i32)> = buf_reader
        .lines()
        .map(|line| {
            let mut split = line.as_ref().unwrap().split(' ');
            (
                split.next().unwrap().to_string(),
                split.next().unwrap().parse::<i32>().expect("Not a number"),
            )
        })
        .collect();

    // star_one
    let mut position = Position::new();
    let mut coordinates = Vec::new();
    for movement in &movements {
        let (direction, distance) = movement;
        position.star_one(direction, *distance);
        coordinates.push((position.x, position.y));
    }
    plot_trajectory("star_one.png", coordinates.clone());
    println!("result: {}", position.x * position.y);

    // star_two
    let mut position = Position::new();
    for movement in movements {
        let (direction, distance) = movement;
        position.star_two(&direction, distance);
        coordinates.push((position.x, position.y));
    }
    plot_trajectory("star_two.png", coordinates);
    println!("result: {}", position.x * position.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        let mut position = Position::new();

        for (direction, distance) in [
            ("forward", 5),
            ("down", 5),
            ("forward", 8),
            ("up", 3),
            ("down", 8),
            ("forward", 2),
        ] {
            position.star_one(direction, distance);
        }
        assert_eq!(position.x, 15);
        assert_eq!(position.y, 10);
    }

    #[test]
    fn test_star_two() {
        let mut position = Position::new();

        for (direction, distance) in [
            ("forward", 5),
            ("down", 5),
            ("forward", 8),
            ("up", 3),
            ("down", 8),
            ("forward", 2),
        ] {
            position.star_two(direction, distance);
        }
        assert_eq!(position.x, 15);
        assert_eq!(position.y, 60);
    }
}
