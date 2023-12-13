use itertools::Itertools;
use num::range;

use crate::input_reader::read_lines;

fn summarize_notes(notes: Vec<Vec<String>>) -> usize {
    notes.iter().map(summarize_note).sum::<usize>()
}

fn summarize_note(note: &Vec<String>) -> usize {
    if let Some(reflection_row) = finds_reflection_row(note) {
        return reflection_row * 100;
    }

    if let Some(reflection_column) = finds_reflection_column(note) {
        return reflection_column;
    }

    panic!("Should have find something!")
}

fn finds_reflection_column(note: &Vec<String>) -> Option<usize> {
    finds_reflection_row(&flip_note(note))
}

fn finds_reflection_row(note: &Vec<String>) -> Option<usize> {
    let candidates = finds_candidate_reflections(note);
    for candidate in candidates {
        if let Some(winning_candidate) = try_candidate(candidate, note) {
            return Some(winning_candidate);
        }
    }

    None
}

fn try_candidate(candidate: usize, note: &Vec<String>) -> Option<usize> {
    let note_len = note.len() as isize;
    let mut current: isize = candidate as isize;
    let mut next: isize = (candidate + 1) as isize;
    while current >= 0 && next < note_len {
        if note[current as usize] != note[next as usize] {
            return None;
        }
        next += 1;
        current -= 1;
    }

    Some(candidate + 1)
}

fn finds_candidate_reflections(note: &Vec<String>) -> Vec<usize> {
    let mut candidates = vec![];

    for (prev, next) in (0..note.len()).zip(1..note.len()) {
        if note[prev] == note[next] {
            candidates.push(prev);
        }
    }

    candidates
}

fn flip_note(note: &Vec<String>) -> Vec<String> {
    let mut flipped_note = vec![];

    for y in range(0, note[0].len()) {
        let mut column: Vec<char> = vec![];
        for row in note {
            column.push(row.chars().nth(y).unwrap());
        }
        flipped_note.push(column.iter().join(""));
    }

    flipped_note
}

fn parse_notes(input: &str) -> Vec<Vec<String>> {
    read_lines(input).split(|l| l.is_empty())
        .map(|s| s.to_vec())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day13::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_part() {
        let input = &read_input_file("input_day13.txt");

        assert_eq!(30802, summarize_notes(parse_notes(input)));
    }

    #[test]
    fn it_summarize_notes() {
        let input = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"};

        let notes = parse_notes(input);
        assert_eq!(5, summarize_note(&notes[0]));
        assert_eq!(400, summarize_note(&notes[1]));
        assert_eq!(405, summarize_notes(notes));
    }
}