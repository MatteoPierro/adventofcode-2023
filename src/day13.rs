use itertools::Itertools;
use num::range;

use crate::input_reader::read_lines;

fn summarize_notes(notes: &Vec<Vec<String>>, maximum_smudge: usize) -> usize {
    notes.iter().map(|note| summarize_note(note, maximum_smudge)).sum::<usize>()
}

fn summarize_note(note: &Vec<String>, maximum_smudge: usize) -> usize {
    if let Some(reflection_row) = finds_reflection_row(note, maximum_smudge) {
        return reflection_row * 100;
    }

    if let Some(reflection_column) = finds_reflection_column(note, maximum_smudge) {
        return reflection_column;
    }

    panic!("Should have find something!")
}

fn finds_reflection_column(note: &Vec<String>, maximum_smudge: usize) -> Option<usize> {
    finds_reflection_row(&flip_note(note), maximum_smudge)
}

fn finds_reflection_row(note: &Vec<String>, maximum_smudge: usize) -> Option<usize> {
    let candidates = finds_candidate_reflections(note);
    for candidate in candidates {
        if let Some(winning_candidate) = try_candidate(candidate, note, maximum_smudge) {
            return Some(winning_candidate);
        }
    }

    None
}

fn try_candidate(candidate: usize, note: &Vec<String>, maximum_smudge: usize) -> Option<usize> {
    let note_len = note.len() as isize;
    let mut current: isize = candidate as isize;
    let mut next: isize = (candidate + 1) as isize;

    let mut total_smudge: usize = 0;
    while current >= 0 && next < note_len {
        total_smudge += smudge(&note[current as usize], &note[next as usize]);
        next += 1;
        current -= 1;
    }

    if total_smudge != maximum_smudge {
        return None;
    }

    Some(candidate + 1)
}

fn finds_candidate_reflections(note: &Vec<String>) -> Vec<usize> {
    let mut candidates = vec![];

    for (prev, next) in (0..note.len()).zip(1..note.len()) {
        if smudge(&note[prev], &note[next]) <= 1 {
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

fn smudge(s1: &str, s2: &str) -> usize {
    let mut diff: usize = 0;
    for (index, c1) in s1.chars().enumerate() {
        if s2.chars().nth(index).unwrap() != c1 {
            diff += 1;
        }
    }
    diff
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day13::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_part() {
        let input = &read_input_file("input_day13.txt");

        assert_eq!(30802, summarize_notes(&parse_notes(input), 0));
        assert_eq!(37876, summarize_notes(&parse_notes(input), 1));
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
        assert_eq!(5, summarize_note(&notes[0], 0));
        assert_eq!(400, summarize_note(&notes[1], 0));
        assert_eq!(405, summarize_notes(&notes, 0));
        assert_eq!(400, summarize_notes(&notes, 1));
    }
}