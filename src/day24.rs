use std::ops::Range;
use itertools::Itertools;
use nalgebra::{Matrix4, Matrix4x1};
use num::Bounded;
use crate::combinations::CombinationIterator;
use crate::day::Day;
use crate::ranges::intersect_n;
use crate::slope_descent::{find_minimum, gradient_descent};

pub struct Day24;

#[derive(Clone, PartialEq, Debug)]
pub struct Hail {
    pos: [f64; 3],
    vel: [f64; 3]
}

impl Hail {
    pub fn intersect_2d(&self, other: &Self) -> Option<(P2D, f64, f64)> {
        let v1 = [self.pos[0], self.pos[1]];
        let v2 = [self.vel[0], self.vel[1]];
        let v3 = [other.pos[0], other.pos[1]];
        let v4 = [other.vel[0], other.vel[1]];
        
        intersect_2d(v1, v2, v3, v4)
    }
    
    pub fn intersect_3d(&self, other: &Self) -> Option<([f64; 3], f64, f64)> {
        intersect_3d(self.pos, self.vel, other.pos, other.vel)
    }
    
    pub fn try_t(&self, other: &Self) -> bool {
        (0..3).all(|i| {
            (self.pos[i] - other.pos[i]).signum() == (other.vel[i] - self.vel[i]).signum()
        })
        
        // let x = 5..;
    }
    
    pub fn ranges(&self) -> (([Range<i64>; 3], [Range<i64>; 3]), ([Range<i64>; 3], [Range<i64>; 3])) {
        // j = self.pos.iter().zip(self.vel.iter())
        
        let v1_range_1 = self.pos.map(|s| (s as i64)..i64::max_value());
        let v2_range_1 = self.vel.map(|s| i64::min_value()..(s as i64));
        let v1_range_2 = self.pos.map(|s| i64::min_value()..(s as i64 + 1));
        let v2_range_2 = self.pos.map(|s| (s as i64 + 1)..(i64::max_value()));

        ((v1_range_1, v2_range_1), (v1_range_2, v2_range_2))
    }
    
    pub fn apply_ranges(&self, v1_range: [Range<i64>; 3], v2_range: [Range<i64>; 3])
        -> Vec<([Range<i64>; 3], [Range<i64>; 3])>{
        let ((v1_range_1, v2_range_1),
            (v1_range_2, v2_range_2)) = self.ranges();
        // dbg!(&v1_range_1, &v2_range_1, &v1_range_2, &v2_range_2);
        
        let v1_range_1 = intersect_n(&v1_range_1, &v1_range);
        let v2_range_1 = intersect_n(&v2_range_1, &v2_range);
        let v1_range_2 = intersect_n(&v1_range_2, &v1_range);
        let v2_range_2 = intersect_n(&v2_range_2, &v2_range);
        
        // dbg!(&v1_range_1, &v2_range_1, &v1_range_2, &v2_range_2);
        
        let mut ranges = vec![];
        
        if let Some(x) = v1_range_1
            .and_then(|x| v2_range_1.map(|y| (x, y))) {
            ranges.push(x);
        }
        
        if let Some(x) = v1_range_2
            .and_then(|x| v2_range_2.map(|y| (x, y))) {
            ranges.push(x);
        }
        
        ranges
    }
    
    fn to_v6(&self) -> [f64; 6] {
        let mut out = [0.0; 6];
        
        out[0..3].iter_mut().enumerate().for_each(|(i, x)| *x = self.pos[i]);
        out[3..6].iter_mut().enumerate().for_each(|(i, x)| *x = self.vel[i]);
        
        out
    }
    
    fn cost(&self, x: [f64; 6]) -> f64 {
        let [px, py, pz, vx, vy, vz] = x;
        
        let x = intersect_3d([px, py, pz], [vx, vy, vz], self.pos, self.vel);
        
        x.map(|(_, t, u)| (t - u).abs())
            .filter(|x| !x.is_nan())
            .unwrap_or(min_distance_3d([px, py, pz], [vx, vy, vz], self.pos, self.vel) * 10000.0)
        * 100.0
        
        // if let Some((_, t, u)) = x {
        //     (t - u).abs()
        // } else {
        //     100.0
        // }
    }
}

type P2D = [f64; 2];

fn intersect_2d(v1: P2D, v2: P2D, v3: P2D, v4: P2D) -> Option<(P2D, f64, f64)> {
    let determinant = v2[0] * v4[1] - v4[0] * v2[1];
    if determinant == 0.0 { return None; }
    
    let t = (v4[0] * (v1[1] - v3[1]) + v4[1] * (v3[0] - v1[0])) / determinant;
    let u = (t * v2[0] + v1[0] - v3[0]) / v4[0];
    Some(([v1[0] + v2[0] * t, v1[1] + v2[1] * t], t, u))
}

fn intersect_3d_(a: [f64; 6], b: [f64; 6]) -> Option<([f64; 3], f64, f64)> {
    let [px, py, pz, vx, vy, vz] = a;
    let [sx, sy, sz, ux, uy, uz] = b;
    intersect_3d([px, py, pz], [vx, vy, vz], [sx, sy, sz], [ux, uy, uz])
}

fn min_distance_3d(p: [f64; 3], v: [f64; 3], s: [f64; 3], u: [f64; 3]) -> f64 {
    let w0 = [p[0] - s[0], p[1] - s[1], p[2] - s[2]];

    let a = v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
    let b = v[0] * u[0] + v[1] * u[1] + v[2] * u[2];
    let c = u[0] * u[0] + u[1] * u[1] + u[2] * u[2];
    let d = v[0] * w0[0] + v[1] * w0[1] + v[2] * w0[2];
    let e = u[0] * w0[0] + u[1] * w0[1] + u[2] * w0[2];

    let det = a * c - b * b;

    if det.abs() > 1e-12 {
        let t = (b * e - c * d) / det;
        let x = (a * e - b * d) / det;

        let closest_point_a = [p[0] + t * v[0], p[1] + t * v[1], p[2] + t * v[2]];
        let closest_point_b = [s[0] + x * u[0], s[1] + x * u[1], s[2] + x * u[2]];

        let distance_vector = [
            closest_point_a[0] - closest_point_b[0],
            closest_point_a[1] - closest_point_b[1],
            closest_point_a[2] - closest_point_b[2],
        ];

        return f64::sqrt(
            distance_vector[0] * distance_vector[0]
                + distance_vector[1] * distance_vector[1]
                + distance_vector[2] * distance_vector[2],
        );
    }

    // Lines are parallel, return distance between initial points
    f64::sqrt(w0[0] * w0[0] + w0[1] * w0[1] + w0[2] * w0[2])
}

fn intersect_3d(v1: [f64; 3], v2: [f64; 3], v3: [f64; 3], v4: [f64; 3]) -> Option<([f64; 3], f64, f64)> {
    let epsilon = 1e-6; // A small value to handle floating-point imprecision

    // Calculate the vectors representing the lines
    let v = [v2[0] - v1[0], v2[1] - v1[1], v2[2] - v1[2]];
    let w = [v4[0] - v3[0], v4[1] - v3[1], v4[2] - v3[2]];

    // Calculate cross product of v and w
    let cross_product = [
        v[1] * w[2] - v[2] * w[1],
        v[2] * w[0] - v[0] * w[2],
        v[0] * w[1] - v[1] * w[0],
    ];

    // Check if the vectors are parallel (cross_product magnitude is very small)
    if cross_product.iter().all(|&x| x.abs() < epsilon) {
        // Vectors are parallel, check if they are collinear
        let collinear_check = |a: f64, b: f64, c: f64| a.abs() < epsilon || (b / a - c / a).abs() < epsilon;

        if collinear_check(v[0], w[0], v1[0] - v3[0])
            && collinear_check(v[1], w[1], v1[1] - v3[1])
            && collinear_check(v[2], w[2], v1[2] - v3[2])
        {
            // The lines are collinear, return the starting point of the second line along with parameters
            let t = (v1[0] - v3[0]) / v[0];
            let u = (v1[1] - v3[1]) / v[1];
            Some((v1, t, u))
        } else {
            // The lines are parallel but not collinear, they do not intersect
            None
        }
    } else {
        // Calculate parameters t and u for the intersection point
        let t = ((v3[0] - v1[0]) * w[1] - (v3[1] - v1[1]) * w[0]) / cross_product[2];
        let u = ((v3[0] - v1[0]) * v[1] - (v3[1] - v1[1]) * v[0]) / cross_product[2];

        // Calculate the intersection point
        let intersection_point = [
            v1[0] + t * v[0],
            v1[1] + t * v[1],
            v1[2] + t * v[2],
        ];

        Some((intersection_point, t, u))
    }
}


impl Day<Vec<Hail>> for Day24 {
    fn parse_file(&self, file_content: String) -> Vec<Hail> {
        file_content.lines()
            .map(|s| {
                let (pos, vel) = s.split_once(" @ ").unwrap();
                let pos = pos.split(", ")
                    .map(|x| x.parse().unwrap())
                    .collect_vec().try_into().unwrap();
                let vel = vel.split(", ")
                    .map(|x| x.parse().unwrap())
                    .collect_vec().try_into().unwrap();
                
                Hail {pos, vel}
            }).collect()
    }

    fn part_1(&self, data: &Vec<Hail>) -> i64 {
        let test_area = if data.len() == 5 { 7.0..=27.0} 
        else { 200000000000000.0..=400000000000000.0 };
        
        let mut count = 0;        
        for [a, b] in CombinationIterator::<_, 2>::new(data.as_slice()) {
            if let Some(([x, y], u, v)) = a.intersect_2d(b) {
                if u >= 0.0 && v >= 0.0 && test_area.contains(&x) && test_area.contains(&y) {
                    count += 1;
                }
            }
        }
        
        count
    }

    fn part_2(&self, data: &Vec<Hail>) -> i64 {
        let a = Hail {
            pos: [24.0,13.0,10.0],
            vel: [-3.0,1.0,3.0],
        };
        
        let av6 = a.to_v6();
        
        dbg!(cost(av6, data));
        
        // for hail in data {
        //     println!("{}", hail.cost(a.to_v6()));
        // }
        
        // for h in data {
        //     dbg!(h.cost(av6));
        // }
        
        // dbg!(b.cost(a.to_v6()));
        
        let v1_range = 
                [i64::min_value()..i64::max_value(),
                i64::min_value()..i64::max_value(),
                i64::min_value()..i64::max_value()];
        let v2_range =
                [i64::min_value()..i64::max_value(),
                i64::min_value()..i64::max_value(),
                i64::min_value()..i64::max_value()];
        
        let mut to_visit = vec![(v1_range, v2_range)];
        
        let x = data.iter().fold(to_visit, |x, h| {
            let mut next_round = vec![];
            for (v1, v2) in x {
                let mut x = h.apply_ranges(v1, v2);
                // dbg!(&x);
                next_round.append(&mut x);
            }
            next_round
        });
        
        // dbg!(x);
        
        // dbg!(a.intersect_2d(&b));
        
        // dbg!(a.intersect_3d(&b));
        // a.try_t(&b);
        
        // dbg!(data.iter().all(|b| a.try_t(b)));
        
        // solve(data.iter().take(4).cloned().collect_vec().try_into().unwrap());
        
        // let t = 1.1;
        // let x = t.signum();
        
        println!("{:?}", solve_2(data));
        1
    }
}

fn solve_2(hails: &Vec<Hail>) -> [f64; 6] {
    dbg!(find_minimum(|x| cost(*x, hails), None)).0
}


fn cost(x: [f64; 6], hails: &Vec<Hail>) -> f64 {
    hails.iter().map(|h| h.cost(x)).sum::<f64>()
}

fn solve(hails: [Hail; 4]) {
    let A = Matrix4::from_fn(|x, y| {
       match x {
           0 => hails[y].vel[1],
           1 => -hails[y].vel[0],
           2 => -hails[y].pos[1],
           3 => hails[y].pos[0],
           _ => panic!("{x}")
       } 
    });
    
    dbg!(&A);

    let test = Matrix4x1::from([24.0,13.0,-3.0,1.0]);
    let det = test[0] * test[3] - test[1] * test[2];
    dbg!(A * test);
    
    let A = A.try_inverse().unwrap();
    
    let B = Matrix4x1::from_fn(|x, _| {
        let h = &hails[x];
        h.pos[0] * h.vel[1] - h.pos[1] * h.vel[0]
    });
    dbg!(B);
    let t_O = B + Matrix4x1::from_element(det);
    
    dbg!(t_O);
    
    let t_h = &hails[0];
    // ()
    
    let mut X = Matrix4x1::from_element(1.0);
    
    for _ in 0..3 {
        let det = X[0]*X[3] - X[1] * X[2];
        let O = Matrix4x1::from_element(det);
        let O = B + O;
        
        dbg!(X);
        X = A * O;
    }
    
    dbg!(A);
    dbg!(X);
}