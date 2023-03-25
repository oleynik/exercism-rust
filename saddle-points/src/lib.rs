pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // It's called a "saddle point" because it is greater than or equal to every
    // element in its row and less than or equal to every element in its column.
    if input.is_empty() {
        return vec![];
    }
    let mut result: Vec<(usize, usize)> = vec![];
    for i in 0..input.len() {
        let row = &input[i];
        if row.is_empty() {
            return vec![];
        }
        let max_value = row.iter().max();
        let max_idxs = row
            .iter()
            .enumerate()
            .filter(|(i, &e)| e == *max_value.unwrap())
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        for max_idx in max_idxs {
            let mut min_idx = i;
            for ii in 0..input.len() {
                if i == ii {
                    continue;
                }
                if input[ii][max_idx] < input[min_idx][max_idx] {
                    min_idx = ii;
                    break;
                }
            }
            if min_idx == i {
                result.push((min_idx, max_idx));
            }
        }
    }
    result
}
