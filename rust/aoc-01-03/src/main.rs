use std::error::Error;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn zero() -> Point {
        Point { x: 0i32, y: 0i32 }
    }

    fn to_zero(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn move_by(&self, op: &str) -> Point {
        let direction = &op[0..1];
        let step = op[1..].parse::<i32>().unwrap();
        match direction {
            "U" => Point {
                x: self.x,
                y: self.y + step,
            },
            "D" => Point {
                x: self.x,
                y: self.y - step,
            },
            "L" => Point {
                x: self.x - step,
                y: self.y,
            },
            "R" => Point {
                x: self.x + step,
                y: self.y,
            },
            _ => panic!("invalid direction of operation"),
        }
    }
}

fn draw_route(ops: &Vec<&str>) -> Vec<Point> {
    let (_, route) = ops.iter().fold(
        (Point::zero(), Vec::new()),
        |(mut point, mut route), &op| {
            let direction = &op[0..1];
            let mut step = op[1..].parse::<i32>().unwrap();
            while step > 0 {
                point = point.move_by(format!("{}{}", direction, 1).as_ref());
                route.push(point);
                step -= 1;
            }
            (point, route)
        },
    );
    route
}

fn get_indexed_route_joints(ops_a: &Vec<&str>, ops_b: &Vec<&str>) -> Vec<(i32, i32, Point)> {
    let route_a = draw_route(ops_a);
    let route_b = draw_route(ops_b);
    let mut joints_with_index: Vec<(i32, i32, Point)> = Vec::new();
    route_a
        .iter()
        .enumerate()
        .map(|(i, &p_a)| {
            route_b
                .iter()
                .enumerate()
                .map(|(j, &p_b)| {
                    if p_a == p_b {
                        joints_with_index.push((i as i32 + 1, j as i32 + 1, p_a));
                    }
                })
                .for_each(drop);
        })
        .for_each(drop);
    joints_with_index
}

fn manhattan_distance(ops_a: &Vec<&str>, ops_b: &Vec<&str>) -> i32 {
    get_indexed_route_joints(ops_a, ops_b)
        .iter()
        .map(|&(_, _, point)| point.to_zero())
        .min()
        .unwrap()
}

fn fewest_combined_steps(ops_a: &Vec<&str>, ops_b: &Vec<&str>) -> i32 {
    get_indexed_route_joints(ops_a, ops_b)
        .iter()
        .map(|&(i, j, _)| i + j)
        .min()
        .unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string("input-part-1.txt")?;
        let ops: Vec<Vec<&str>> = input
            .lines()
            .map(|line| line.split(",").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        let ops_a = &ops[0];
        let ops_b = &ops[1];
        let answer = manhattan_distance(ops_a, ops_b);
        println!("answer to part-1 is {}", answer);

    let input = fs::read_to_string("input-part-2.txt")?;
    let ops: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let ops_a = &ops[0];
    let ops_b = &ops[1];
    let answer = fewest_combined_steps(ops_a, ops_b);
    println!("answer to part-2 is {}", answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::manhattan_distance;
    use crate::fewest_combined_steps;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(
            manhattan_distance(
                &vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
                &vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"]
            ),
            159
        );
        assert_eq!(
            manhattan_distance(
                &vec!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"],
                &vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"]
            ),
            135
        );
    }

    #[test]
    fn test_fewest_combined_steps() {
        assert_eq!(
            fewest_combined_steps(
                &vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
                &vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
            ),
            610
        );
        assert_eq!(
            fewest_combined_steps(
                &vec!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"],
                &vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"],
            ),
            410
        )
    }
}
