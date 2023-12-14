pub type CubeMap = std::collections::HashMap<String, u32>;

pub fn read_game(line: String) -> Vec<CubeMap> {
    let line = line.split(':').nth(1).unwrap();

    let mut ret = Vec::<CubeMap>::new();
    line.split(';').for_each(|game| {
        let mut m = CubeMap::new();
        game.split(',').for_each(|cube| {
            let mut iter = cube.trim().split(' ');
            let v = iter.next().unwrap().parse::<u32>().unwrap();
            let k = iter.next().unwrap();

            m.insert(k.to_string(), v);
        });

        ret.push(m);
    });

    ret
}

pub fn check_cubes(target_maps: &[CubeMap], expect_map: &CubeMap) -> bool {
    target_maps.iter().all(|m| {
        for (k, v) in expect_map.iter() {
            if let Some(tv) = m.get(k) {
                if tv > v {
                    return false;
                }
            }
        }

        true
    })
}
