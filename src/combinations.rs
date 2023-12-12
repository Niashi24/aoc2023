
pub struct CombinationIterator<'a, T, const N: usize> {
    slice: &'a [T],
    counters: [usize; N]
}

impl<'a, T, const N: usize> Iterator for CombinationIterator<'a, T, N> {
    type Item = [&'a T; N];

    fn next(&mut self) -> Option<Self::Item> {

        if self.counters.iter().enumerate().any(|(i, x)| {
            x == &(self.slice.len() - (N - i - 1))
        }) { return None; }

        let out = Some(self.counters.map(|i| &self.slice[i]));

        for i in (1..N).rev() {
            self.counters[i] += 1;
            if self.counters.get(i).unwrap() != &(self.slice.len() - (N - i - 1)) {
                return out;
            } else {
                for x in i..N {
                    self.counters[x] = self.counters[i - 1] + x - i + 2;
                }
            }
        }

        self.counters[0] += 1;

        out
    }
}

impl<'a, T, const N: usize> CombinationIterator<'a, T, N> {
    pub(crate) fn new(slice: &'a [T]) -> Self {
        let mut counters = [0; N];
        counters.iter_mut()
            .enumerate()
            .for_each(|(i, x)| *x = i);

        CombinationIterator {
            slice,
            counters
        }
    }
}

