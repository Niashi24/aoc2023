use crate::day::Day;

pub struct Day14;

pub struct Data {
    rounded: Vec<(usize, usize)>,
    cube: Vec<(usize, usize)>
}

impl Day<Data> for Day14 {
    fn parse_file(&self, file_content: String) -> Data {
        Data {
            rounded: file_content.lines().enumerate()
                .flat_map(|(y, s)| s.chars().enumerate().filter_map(move |(x, c)| {
                    if c == 'O' { Some((x, y)) } else { None }
                })).collect(),
            cube: file_content.lines().enumerate()
                .flat_map(|(y, s)| s.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' { Some((x, y)) } else { None }
                })).collect()
        }
    }

    fn part_1(&self, data: &Data) -> i64 {
        0
    }

    fn part_2(&self, data: &Data) -> i64 {
        0
    }
}

fn slide_up(rounded: &mut Vec<(usize, usize)>, cube: &Vec<(usize, usize)>) {
    rounded.sort_unstable_by_key(|x| x.1);
    for i in 0..rounded.len() {
        let (x, mut ny) = rounded.get(i).unwrap();
        while rounded.iter().all(|(_, y)| ny != y + 1) {
            ny -= 1;
        }

        *rounded.get_mut(i).unwrap() = (*x, ny);
    }
}