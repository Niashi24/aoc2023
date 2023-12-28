use std::collections::hash_map::RandomState;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Range};
use itertools::Itertools;
use num::{Bounded, One};

pub(crate) fn intersect(a: &Range<usize>, b: &Range<usize>) -> Option<Range<usize>> {
    if !intersects(a, b) { return None; }
    else { Some(a.start.max(b.start)..a.end.min(b.end)) }
}

pub(crate) fn intersects(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.contains(&b.start) || b.contains(&a.start)
}

pub(crate) fn intersect_n<const N: usize>(a: &[Range<i64>; N], b: &[Range<i64>; N]) -> Option<[Range<i64>; N]> {
    let mut out = a.clone();
    for (a, b) in out.iter_mut().zip(b.iter()) {
        *a = a.start.max(b.start)..a.end.min(b.end);
    }
    
    // dbg!(&out);
    
    if out.iter().all(|r| {
        r.end > r.start
    }) {
        Some(out)
    } else { None }
}

pub(crate) fn min_max_xy<Iter, Idx>(iter: Iter) -> Option<(Range<Idx>, Range<Idx>)>
where
    Iter: Iterator<Item=(Idx, Idx)>,
    Idx: Copy + Ord + Bounded + One + Add<Output=Idx>
{
    let mut min_x = Idx::max_value();
    let mut max_x = Idx::min_value();
    let mut min_y = Idx::max_value();
    let mut max_y = Idx::min_value();
    let mut any = false;
    
    for (x, y) in iter {
        any = true;
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    
    if any { Some((min_x..max_x + Idx::one(), min_y..max_y + Idx::one())) } else { None }
}

pub(crate) fn min_max_comp<Iter, Idx, const N: usize>(iter: Iter) -> Option<([Idx; N], [Idx; N])>
    where   Iter: Iterator<Item=[Idx; N]>,
            Idx: Copy + Ord + Bounded + One + Add<Output=Idx>
{
    let mut mins = [Idx::max_value(); N];
    let mut maxes = [Idx::min_value(); N];
    let mut any = false;
    for nums in iter {
        any = true;
        for (i, x) in nums.into_iter().enumerate() {
            mins[i] = mins[i].min(x);
            maxes[i] = maxes[i].max(x);
        }
    }

    if any { Some((mins, maxes)) } else { None }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RangeD<const N: usize> {
    pub(crate) start: [usize; N],
    pub(crate) end: [usize; N]
}

impl<'a, const N: usize> IntoIterator for &'a RangeD<N> {
    type Item = [usize; N];
    type IntoIter = RangeDIterator<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const N: usize> Display for RangeD<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.start.iter().zip(self.end.iter()).map(|(s,e)| {
            format!("{s}..{e}")
        }).join(", "))
    }
}

impl<const N: usize> RangeD<N> {
    pub fn from_range_1d(ranges: [Range<usize>; N]) -> Self {
        Self {
            start: ranges.clone().map(|x| x.start),
            end: ranges.clone().map(|x| x.end),
        }
    }
    
    pub fn offset(&mut self, offset: usize) {
        self.start.iter_mut().for_each(|i| *i += offset);
        self.end.iter_mut().for_each(|i| *i += offset);
    }
    
    pub fn offset_neg(&mut self, offset: usize) {
        self.start.iter_mut().for_each(|i| *i -= offset);
        self.end.iter_mut().for_each(|i| *i -= offset);
    }
    
    pub fn offset_component(&mut self, i: usize, offset: usize) {
        self.start[i] += offset;
        self.end[i] += offset;
    }

    pub fn offset_component_neg(&mut self, i: usize, offset: usize) {
        self.start[i] -= offset;
        self.end[i] -= offset;
    }
    
    pub fn volume(&self) -> usize {
        self.start.iter().zip(self.end.iter()).map(|(s, e)| e - s).product()
    }
    
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        // if !self.intersects(other) { return None; }
        
        let mut start = [0; N];
        let mut end = [0; N];
        for i in 0..N {
            start[i] = self.start[i].max(other.start[i]);
            end[i] = self.end[i].min(other.end[i]);
        }
        
        if start.iter().zip(end.iter()).all(|(s, e)| s < e) {
            Some(Self {
                start,
                end
            })
        } else {
            None
        }
        
        
    }
    
    pub fn intersects(&self, other: &Self) -> bool {
        fn contains_simple<const N: usize>(a: &RangeD<N>, b: &RangeD<N>) -> bool {
            (0..N).all(|i| {
                a.start[i] <= b.start[i] && b.start[i] < a.end[i]
            })
        }
        
        contains_simple::<N>(self, other) || contains_simple::<N>(other, self)
    }
    
    pub fn len_d(&self, i: usize) -> usize {
        self.end[i] - self.start[i]
    }
    
    pub fn iter(&self) -> RangeDIterator<N> {
        RangeDIterator::new(self)
    }
}

pub struct RangeDIterator<'a, const N: usize> {
    ranges: &'a RangeD<N>,
    values: [usize; N]
}

impl<'a, const N: usize> RangeDIterator<'a, N> {
    pub fn new(range: &'a RangeD<N>) -> Self {
        Self {
            ranges: range,
            values: range.start
        }
    }
}

impl<'a, const N: usize> Iterator for RangeDIterator<'a, N> {
    type Item = [usize; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.values[0] == self.ranges.end[0] {
            return None;
        }
        
        fn increment<const N: usize>(indices: &mut [usize; N], ranges: &RangeD<N>, i: usize) {
            indices[i] += 1;
            if ranges.end[i] == indices[i] && i != 0 {
                indices[i] = ranges.start[i];
                increment(indices, ranges, i - 1);
            }
        }
        let item = self.values;
        
        let mut i = N - 1;
        self.values[i] += 1;
        while i != 0 && self.values[i] == self.ranges.end[i] {
            self.values[i] = self.ranges.start[i];
            self.values[i - 1] += 1;
            i -= 1;
        }

        Some(item)
    }
}

