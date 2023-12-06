fn calculate_record_breaks(times: Vec<usize>, distance: Vec<usize>) -> usize {
    times.iter().zip(distance)
        .map(|(&t, d)| calculate_winning_combinations(t, d))
        .fold(1, |curr, acc| acc * curr)
}

fn calculate_winning_combinations(time: usize, distance: usize) -> usize {
    let time_range = 1..time;
    let distance_range = time_range.clone().rev();
    time_range.zip(distance_range)
        .map(|(t, d)| t * d).filter(|&s| s > distance)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day6::*;

    #[test]
    fn it_solves_first_part() {
        let times = vec![56, 71, 79, 99];
        let distance = vec![334, 1135, 1350, 2430];
        assert_eq!(211904, calculate_record_breaks(times, distance));
    }

    #[test]
    fn it_solves_second_part() {
        let times = vec![56717999];
        let distance = vec![334113513502430];
        assert_eq!(43364472, calculate_record_breaks(times, distance));
    }

    #[test]
    fn it_calculates_record_breaks() {
        let times = vec![7, 15, 30];
        let distance = vec![9, 40, 200];
        assert_eq!(288, calculate_record_breaks(times, distance));
    }

    #[test]
    fn it_calculates_calculate_winning_combinations() {
        assert_eq!(4, calculate_winning_combinations(7, 9));
        assert_eq!(8, calculate_winning_combinations(15, 40));
        assert_eq!(9, calculate_winning_combinations(30, 200));
    }
}