pub fn cal_copies(winnings: &[Vec<u32>]) -> Vec<u32> {
    let length = winnings.len();
    let mut counter = vec![1_u32; length];

    winnings.iter().enumerate().for_each(|(i, v)| {
        if v.is_empty() {
        } else {
            ((i + 1)..=(i + v.len()).min(length)).for_each(|ind| counter[ind] += counter[i])
        }
    });
    counter
}
