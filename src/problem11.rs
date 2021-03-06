use itertools::iproduct;

const INPUT: &str = r#"08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48"#;

const MAX_SIZE: i8 = 20;

#[derive(PartialEq, Debug)]
struct Pos {
    row: i8,
    col: i8,
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|line| line.split_whitespace()
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .collect())
        .collect()
}

impl Pos {
    const fn new(row: i8, col: i8) -> Pos {
        Pos { row, col }
    }
}

fn build_coords(start_pos: &Pos, dr: i8, dc: i8) -> Option<Vec<Pos>> {
    let mut tmp_vec = Vec::<Pos>::with_capacity(4);
    (0..4_i8).for_each(|i|
        tmp_vec.push(
            Pos::new(
                start_pos.row + dr * i,
                start_pos.col + dc * i,
            )
        )
    );

    if tmp_vec.iter()
        .find(|item| item.row < 0 || item.col < 0
            || item.row > MAX_SIZE - 1 || item.col > MAX_SIZE - 1)
        .is_some() {
        None
    } else {
        Some(tmp_vec)
    }
}

fn multiply(input: &Vec<Vec<usize>>, coords: &Vec<Pos>) -> usize {
    dbg!(coords,

    coords.iter()
        .map(|&Pos { row, col }| input[row as usize][col as usize])
        .product()).1
}

fn find_answer() -> Option<usize> {
    let input = parse_input(INPUT);
    iproduct!((0..MAX_SIZE),(0..MAX_SIZE))
        .map(|(row, col)| Pos::new(row, col))
        .map(|pos|
            vec![
                build_coords(&pos, 1, 0),
                build_coords(&pos, 0, 1),
                build_coords(&pos, -1, 0),
                build_coords(&pos, 0, -1),
                build_coords(&pos, 1, 1),
                build_coords(&pos, -1, -1),
                build_coords(&pos, 1, -1),
                build_coords(&pos, -1, 1),
            ].into_iter()
                .filter_map(|coords| coords)
                .map(|coords| multiply(&input, &coords))
                .max())
        .filter_map(|coords| coords)
        .max()
}

fn main() {
    println!("Problem 11. Answer is {}", find_answer().unwrap_or_default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1 2\n3 4";
        let result = parse_input(input);

        assert_eq!(result, [[1, 2], [3, 4]]);
    }

    #[test]
    fn test_build_coords() {
        const START_POS_1: Pos = Pos::new(0, 0);
        assert_eq!(build_coords(&START_POS_1, 0, 1), Some(vec![Pos::new(0, 0), Pos::new(0, 1), Pos::new(0, 2), Pos::new(0, 3)]));
        assert_eq!(build_coords(&START_POS_1, 0, -1), None);
        assert_eq!(build_coords(&START_POS_1, -1, 0), None);
        assert_eq!(build_coords(&START_POS_1, 1, -1), None);
        assert_eq!(build_coords(&START_POS_1, 1, 0), Some(vec![Pos::new(0, 0), Pos::new(1, 0), Pos::new(2, 0), Pos::new(3, 0)]));
        assert_eq!(build_coords(&START_POS_1, 1, 1), Some(vec![Pos::new(0, 0), Pos::new(1, 1), Pos::new(2, 2), Pos::new(3, 3)]));

        const START_POS_2: Pos = Pos::new(MAX_SIZE - 1, MAX_SIZE - 1);
        assert_eq!(build_coords(&START_POS_2, 0, 1), None);
        assert_eq!(build_coords(&START_POS_2, -1, -1), Some(vec![Pos::new(MAX_SIZE - 1, MAX_SIZE - 1), Pos::new(MAX_SIZE - 2, MAX_SIZE - 2), Pos::new(MAX_SIZE - 3, MAX_SIZE - 3), Pos::new(MAX_SIZE - 4, MAX_SIZE - 4)]));
    }

    #[test]
    fn test_multiply() {
        const START_POS: Pos = Pos::new(0, 0);
        let coords: Vec<Pos> = build_coords(&START_POS, 0, 1).unwrap();
        let input = parse_input(INPUT);

        assert_eq!(multiply(&input, &coords), 8 * 02 * 22 * 97);
    }
}