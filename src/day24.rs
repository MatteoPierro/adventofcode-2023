use itertools::Itertools;
use num::Zero;

use crate::input_reader::read_lines;

#[derive(Clone, Debug)]
struct Point(f64, f64, f64);

#[derive(Clone, Debug)]
struct Hail {
    start: Point,
    end: Point,
}

impl Hail {
    fn linear_equation_xy(&self) -> (f64, f64, f64) {
        let a = self.end.1 - self.start.1;
        let b = self.start.0 - self.end.0;
        let c = -1.0 * self.start.0 * (self.end.1 - self.start.1) + self.start.1 * (self.end.0 - self.start.0);
        (a, b, c)
    }
}

fn find_intersection((a1, b1, c1): (f64, f64, f64), (a2, b2, c2): (f64, f64, f64)) -> Option<(f64, f64)> {
    if (a2 * b1 - a1 * b2).is_zero() || (a1 * b2 - a2 * b1).is_zero() {
        return None;
    }

    let x0 = (b2 * c1 - b1 * c2) / (a2 * b1 - a1 * b2);
    let y0 = (a2 * c1 - a1 * c2) / (a1 * b2 - a2 * b1);
    Some((x0, y0))
}

fn calculate_distance_xy((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
    ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt()
}

fn parse_hails(input: &str) -> Vec<Hail> {
    read_lines(input).iter().map(|l| {
        let parts = l.split(" @ ").collect::<Vec<_>>();
        let initial_positions = parts[0].split(", ")
            .map(|v| v.parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        let speeds = parts[1].split(", ")
            .map(|v| v.trim().parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        Hail {
            start: Point(initial_positions[0],
                         initial_positions[1],
                         initial_positions[2]),
            end: Point(initial_positions[0] + speeds[0],
                       initial_positions[1] + speeds[1],
                       initial_positions[2] + speeds[2]),
        }
    }).collect::<Vec<_>>()
}

fn count_intersecting_hails(hails: Vec<Hail>, test_area: (f64, f64)) -> usize {
    let mut count: usize = 0;

    for combinations in hails.into_iter().combinations(2) {
        let hail = &combinations[0];
        let other = &combinations[1];
        if are_hails_intersecting(hail, other, test_area) {
            count += 1
        }
    }

    count
}

fn are_hails_intersecting(hail: &Hail, other: &Hail, test_area: (f64, f64)) -> bool {
    if let Some(intersection) = find_intersection(hail.linear_equation_xy(), other.linear_equation_xy()) {
        let d_1_in = calculate_distance_xy(intersection, (hail.start.0, hail.start.1));
        let d_1_f = calculate_distance_xy(intersection, (hail.end.0, hail.end.1));
        let d_2_in = calculate_distance_xy(intersection, (other.start.0, other.start.1));
        let d_2_f = calculate_distance_xy(intersection, (other.end.0, other.end.1));
        return intersection.0 > test_area.0 && intersection.0 < test_area.1 &&
            intersection.1 > test_area.0 && intersection.1 < test_area.1 &&
            d_1_in > d_1_f && d_2_in > d_2_f;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day24::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_part() {
        let input = &read_input_file("input_day24.txt");

        assert_eq!(12740, count_intersecting_hails(parse_hails(input), (200000000000000.0, 400000000000000.0)));
    }

    #[test]
    fn it_counts_intersecting_hails() {
        let input = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3"};

        assert_eq!(2, count_intersecting_hails(parse_hails(input), (7.0, 27.0)));
    }
}