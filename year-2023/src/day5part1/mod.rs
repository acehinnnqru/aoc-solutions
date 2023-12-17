type RangeItem = ((u64, u64), u64, u64);

#[derive(Default, Debug)]
pub struct KVMap {
    list: Vec<RangeItem>,
}

impl KVMap {
    pub fn new(ranges: Vec<RangeItem>) -> Self {
        let mut s = Self::default();
        s.parse(ranges);
        s
    }

    pub fn get(&self, k: u64) -> u64 {
        for ((start, end), target_start, _) in &self.list {
            if k >= *start && k <= *end {
                return target_start + k - start;
            }
        }

        k
    }

    fn parse(&mut self, ranges: Vec<RangeItem>) {
        self.list = ranges;
        self.list.sort_by_key(|((start, _), _, _)| *start);
    }
}

pub fn get_ranges(line: &str) -> RangeItem {
    let mut splits = line.split_whitespace();

    let dest_start = splits.next().unwrap().parse::<u64>().unwrap();
    let src_start = splits.next().unwrap().parse::<u64>().unwrap();
    let length = splits.next().unwrap().parse::<u64>().unwrap();

    ((src_start, src_start + length - 1), dest_start, length)
}

pub fn read_seeds(line: &str) -> Vec<u64> {
    let mut splits = line.split(':');
    splits
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect()
}

pub fn read_data(lines: &[String]) -> Vec<KVMap> {
    let mut ret: Vec<KVMap> = vec![];

    let mut carry: Vec<RangeItem> = vec![];
    for line in lines {
        if line.is_empty() {
            ret.push(KVMap::new(carry.clone()));
            carry = vec![];
            continue;
        }

        if line.contains(':') {
            continue;
        }

        carry.push(get_ranges(line))
    }

    ret.push(KVMap::new(carry.clone()));

    ret
}

pub fn check_seed_location(seed: u64, kvmap: &[KVMap]) -> u64 {
    let mut carry = seed;
    for m in kvmap {
        carry = m.get(carry);
    }

    carry
}
