use itertools::Itertools;
use nalgebra::{Matrix2, Matrix2x1, Matrix3, Matrix3x1, Matrix4, Matrix4x1, Matrix5, Matrix5x1};
use crate::day::Day;

pub struct Day24;

#[derive(Clone, PartialEq, Debug)]
pub struct Hail {
    pos: [f64; 3],
    vel: [f64; 3]
}

impl Hail {
    pub fn intersect_2d(&self, other: &Self) -> Option<([f64; 2], f64, f64)> {
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

fn intersect_2d(v1: [f64; 2], v2: [f64; 2], v3: [f64; 2], v4: [f64; 2]) -> Option<([f64; 2], f64, f64)> {
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
        
        data.iter().tuple_combinations()
            .filter_map(|(a, b)| a.intersect_2d(b))
            .filter(|([x, y], u, v)|
                u >= &0.0 && v >= &0.0 && test_area.contains(x) && test_area.contains(y))
            .count() as i64
    }

    fn part_2(&self, data: &Vec<Hail>) -> i64 {
        // take 5 and solve
        solve(data.iter()
            .cloned()
            .take(5)
            .collect_vec()
            .try_into()
            .unwrap()
        ).unwrap() as i64
    }
}

fn solve(mut hails: [Hail; 5]) -> Option<f64> {
    // solve for px, py, vx, vy algebraically (thank you @weasel137 !!)
    let a = Matrix5::from([
        hails.clone().map(|r| r.uy()),
        hails.clone().map(|r| -r.ux()),
        hails.clone().map(|r| -r.sy()),
        hails.clone().map(|r| r.sx()),
        [1.0; 5]
    ]);
    
    let b = Matrix5x1::from(hails.clone().map(|h| h.pos[0] * h.vel[1] - h.pos[1] * h.vel[0]));

    let [px, py, vx, vy, _] = a.lu().solve(&b)?.data.0[0];
    
    // solve for pz, vz algebraically
    let [sx, _, sz1] = hails[0].pos;
    let [ux, _, uz1] = hails[0].vel;
    let [_, sy, sz2] = hails[1].pos;
    let [_, uy, uz2] = hails[1].vel;    
    
    let dpx = px - sx;
    let dpy = py - sy;
    let dvx = vx - ux;
    let dvy = vy - uy;

    let a = Matrix2::from([[dvy, dvx], [-dpy, -dpx]]);
    let b = Matrix2x1::from([dvy * sz2 - dpy * uz2, dvx * sz1 - dpx * uz1]);
    
    let [pz, vz] = a.lu().solve(&b)?.data.0[0];
    
    Some((px + py + pz).round())
}