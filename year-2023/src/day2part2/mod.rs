use super::day2part1::CubeMap;

pub fn get_power(m: &CubeMap) -> u32 {
    m.values().product::<u32>()
}

pub fn cal_least_map(mut maps: Vec<CubeMap>) -> CubeMap {
    maps.iter_mut()
        .reduce(|left, right| {
            for (k, v) in left {
                right
                    .entry(k.clone())
                    .and_modify(|ov| {
                        *ov = (*ov).max(*v);
                    })
                    .or_insert(*v);
            }
            right
        })
        .unwrap()
        .to_owned()
}
