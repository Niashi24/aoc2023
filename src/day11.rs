use crate::day::Day;

pub struct Day11;

pub struct Data {
    galaxies: Vec<(usize, usize)>
}

impl Day<Data> for Day11 {
    fn parse_file(&self, file_content: String) -> Data {
        Data {
            galaxies: file_content.lines().enumerate()
                .flat_map(|(y, s)| s.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' { Some((x, y)) } else { None }
                })).collect()
        }
    }

    fn part_1(&self, data: &Data) -> i64 {
        solve(&data.galaxies, 1)
    }

    fn part_2(&self, data: &Data) -> i64 {
        solve(&data.galaxies, 999_999)
    }
}

fn solve(galaxies: &Vec<(usize, usize)>, multiplier: usize) -> i64 {
    let (mx, my) = galaxies.iter().fold((0, 0), |(mx, my), (x, y)| {
        (mx.max(*x), my.max(*y))
    });

    let xs = (0..mx)
        .filter(|i| !galaxies.iter().any(|(x, _)| x == i))
        .collect::<Vec<_>>();
    let ys = (0..my)
        .filter(|i| !galaxies.iter().any(|(_, y)| y == i))
        .collect::<Vec<_>>();
    let new_stars = galaxies.iter().cloned().map(|(x, y)| {
        (x + multiplier * xs.iter().filter(|i| &x > i).count(),
         y + multiplier * ys.iter().filter(|i| &y > i).count())
    }).collect::<Vec<_>>();

    // iterate through pairs and get total manhattan distance
    new_stars.iter().enumerate().map(|(i, (ax, ay))| new_stars.iter()
        .skip(i + 1)
        .map(|(bx, by)| bx.abs_diff(*ax) + by.abs_diff(*ay))
        .sum::<usize>())
        .sum::<usize>() as i64
}