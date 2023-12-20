use std::ops::Range;

pub(crate) fn intersect(a: &Range<usize>, b: &Range<usize>) -> Option<Range<usize>> {
    if !intersects(a, b) { return None; }
    else { Some(a.start.max(b.start)..a.end.min(b.end)) }
}

pub(crate) fn intersects(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.contains(&b.start) || b.contains(&a.start)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RangeD<const N: usize> {
    pub(crate) start: [usize; N],
    pub(crate) end: [usize; N]
}

impl<const N: usize> RangeD<N> {
    pub fn from_range_1d(ranges: [Range<usize>; N]) -> Self {
        Self {
            start: ranges.clone().map(|x| x.start),
            end: ranges.clone().map(|x| x.end),
        }
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
}

pub struct RangeDIterator<'a, const N: usize> {
    range: &'a RangeD<N>,
    indices: [usize; N]
}

impl<'a, const N: usize> Iterator for RangeDIterator<'a, N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

