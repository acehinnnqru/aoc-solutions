pub fn read_seeds(line: &str) -> Vec<(u64, u64)> {
    let mut splits = line.split(':');
    splits
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect()
}

#[derive(Clone, Debug)]
pub struct Interval(u64, u64);

impl Interval {
    pub fn new(start: u64, length: u64) -> Self {
        Self(start, start + length - 1)
    }

    pub fn start(&self) -> u64 {
        self.0
    }


    pub fn length(&self) -> u64 {
        self.1 - self.0 + 1
    }

    pub fn end(&self) -> u64 {
        self.1
    }

    pub fn contains(&self, x: u64) -> bool {
        self.0 <= x && x <= self.end()
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || other.contains(self.start())
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.overlaps(other) {
            let start = self.start().max(other.start());
            let end = self.end().min(other.end());
            Some(Self::new(start, end - start))
        } else {
            None
        }
    }

    pub fn merge(&self, other: &Self) -> Self {
        assert!(self.overlaps(other));
        let start = self.start().min(other.start());
        let end = self.end().max(other.end());
        Self::new(start, end - start)
    }
}

#[derive(Debug, Default)]
pub struct VecInterval(Vec<Interval>);

impl VecInterval {
    pub fn new() -> Self {
        Self(Vec::default())
    }

    pub fn add(&mut self, interval: Interval) {
        let mut i = 0;
        while i < self.0.len() {
            if self.0[i].overlaps(&interval) {
                let merged = self.0[i].merge(&interval);
                self.0.remove(i);
                self.add(merged);
                return;
            } else if self.0[i].start() > interval.start() {
                break;
            }
            i += 1;
        }
        self.0.insert(i, interval);
    }

    pub fn add_all(&mut self, intervals: Vec<Interval>) {
        for interval in intervals {
            self.add(interval);
        }

        self.0.sort_by_key(|a| a.start());
    }

    pub fn remove(&mut self, interval: &Interval) {
        let mut i = 0;
        while i < self.0.len() {
            if let Some(intersection) = self.0[i].intersection(interval) {
                let mut left = self.0[i].clone();
                let mut right = self.0[i].clone();
                left.1 = intersection.start() - 1;
                right.0 = intersection.end() + 1;
                self.0.remove(i);
                if left.length() > 0 {
                    self.add(left);
                }
                if right.length() > 0 {
                    self.add(right);
                }
                return;
            } else if self.0[i].start() > interval.start() {
                break;
            }
            i += 1;
        }
    }

    pub fn remove_all(&mut self, intervals: Vec<Interval>) {
        for interval in intervals {
            self.remove(&interval);
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Interval> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
