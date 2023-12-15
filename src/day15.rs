use pathfinding::num_traits::AsPrimitive;
use crate::day::Day;

pub struct Day15;

pub struct Data {
    operations: Vec<String>
}

impl Day<Data> for Day15 {
    fn parse_file(&self, file_content: String) -> Data {
        Data {
            operations: file_content.lines().next().unwrap()
                .split(",")
                .map(|x| x.to_owned())
                .collect()
        }
    }

    fn part_1(&self, data: &Data) -> i64 {
        data.operations.iter().map(|x| hash(&x)).sum::<usize>() as i64
    }

    fn part_2(&self, data: &Data) -> i64 {
        struct Lens(String, usize);
        // slightly jank thing you have to do to use [vec![]; 256]
        const fn new_vec<T>() -> Vec<T> {vec![]}
        const VEC: Vec<Lens> = new_vec();

        data.operations.iter().map(|x| {
            if let Some(i) = x.find("=") {
                Operation::Equals(x[..i].to_owned(), x[i+1..].parse().unwrap())
            } else {
                Operation::Dash(x[..x.len()-1].to_owned())
            }
        }).fold([VEC; 256], |mut boxes, op| {
            let boxed = op.label();
            match op {
                Operation::Equals(label, value) => {
                    let mut boxed = &mut boxes[boxed];
                    if let Some(pos) = boxed.iter().position(|Lens(s, _)| s == &label) {
                        boxed.get_mut(pos).unwrap().1 = value;
                    } else {
                        boxed.push(Lens(label, value));
                    }
                }
                Operation::Dash(label) => {
                    let mut boxed = &mut boxes[boxed];
                    if let Some(pos) = boxed.iter().position(|Lens(s, _)| s == &label) {
                        boxed.remove(pos);
                    }
                }
            };
            boxes
        }).into_iter().enumerate().map(|(i, x)| {
            x.into_iter().enumerate().map(|(ii, x)| {
                (i + 1) * (ii + 1) * x.1
            }).sum::<usize>()
        }).sum::<usize>() as i64
    }
}

fn hash(str: &str) -> usize {
    let mut value = 0;
    for c in str.chars() {
        let i: usize = c.as_();
        value += i;
        value = (value * 17) & 255;
    }
    value
}

enum Operation {
    Equals(String, usize),
    Dash(String)
}

impl Operation {
    pub fn label(&self) -> usize {
        hash(match self {
            Operation::Equals(x, _) => x,
            Operation::Dash(x) => x
        })
    }
}