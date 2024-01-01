use itertools::Itertools;
use nalgebra::{Matrix1x4, Matrix2, Matrix2x1, Matrix4};
use crate::combinations::CombinationIterator;
use crate::day::Day;

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
    
    pub fn eval(&self, t: f64) -> [f64; 3] {
        let mut output = self.pos;
        output.iter_mut().zip(self.vel).for_each(|(s, v)| *s += v * t);
        output
    }
    
    pub fn sx(&self) -> f64 { self.pos[0] }
    pub fn sy(&self) -> f64 { self.pos[1] }
    pub fn sz(&self) -> f64 { self.pos[2] }
    pub fn ux(&self) -> f64 { self.vel[0] }
    pub fn uy(&self) -> f64 { self.vel[1] }
    pub fn uz(&self) -> f64 { self.vel[2] }
}

type P2D = [f64; 2];

fn intersect_2d(v1: P2D, v2: P2D, v3: P2D, v4: P2D) -> Option<(P2D, f64, f64)> {
    let determinant = v2[0] * v4[1] - v4[0] * v2[1];
    if determinant == 0.0 { return None; }
    
    let t = (v4[0] * (v1[1] - v3[1]) + v4[1] * (v3[0] - v1[0])) / determinant;
    let u = (t * v2[0] + v1[0] - v3[0]) / v4[0];
    Some(([v1[0] + v2[0] * t, v1[1] + v2[1] * t], t, u))
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
        // take 4 and solve
        solve(data.iter().skip(1).cloned().take(4).collect_vec().try_into().unwrap()).unwrap() as i64
    }
}

fn solve(mut hails: [Hail; 4]) -> Option<f64> {
    // Solve for px, py, vx, vy
    let A = Matrix4::from_fn(|x, y| {
       match x {
           0 => hails[y].vel[1],
           1 => -hails[y].vel[0],
           2 => -hails[y].pos[1],
           3 => hails[y].pos[0],
           _ => panic!("{x}")
       } 
    });
    let AINV = A.try_inverse().unwrap();
    
    let B = Matrix1x4::from(hails.clone().map(|h| h.pos[0] * h.vel[1] - h.pos[1] * h.vel[0]));
    
    // Initial guess
    let mut X = Matrix1x4::from_element(1.0);
    
    for _ in 0..1000 {
        let det = X[0]*X[3] - X[1] * X[2];
        let O = Matrix1x4::from_element(det);
        let O = B + O;
        
        X = O * AINV;
    }
    let [px, py, vx, vy] = X.data.0.map(|r| r[0].round());
    
    // solve for pz, vz algebraically    
    let [sx, _, sz1] = hails[0].pos;
    let [ux, _, uz1] = hails[0].vel;
    let [_, sy, sz2] = hails[1].pos;
    let [_, uy, uz2] = hails[1].vel;    
    
    let dpx = px - sx;
    let dpy = py - sy;
    let dvx = vx - ux;
    let dvy = vy - uy;

    let BZ = Matrix2::from([[dvy, dvx], [-dpy, -dpx]]);
    let CZ = Matrix2x1::from([dvy * sz2 - dpy * uz2, dvx * sz1 - dpx * uz1]);
    let BZInv = BZ.try_inverse()?;

    let PZVZ = BZInv * CZ;
    let [pz, vz] = PZVZ.data.0[0].map(|r| r.round());
    
    println!("{}, {}, {} @ {}, {}, {}", px, py, pz, vx, vy, vz);
    
    Some(px + py + pz)
}