pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![];
    }
    let mut result = vec![vec![0; size as usize]; size as usize];
    let mut i = 0;
    let mut j = 0;
    result[0][0] = 1;
    let movement_patter: [i32; 4] = [0, 1, 0, -1];
    let mut movement_idx = 0;
    let mut iterator = (2..=size * size).into_iter();
    let mut steps = size - 1;
    while steps > 0 {
        let iter = if steps == size - 1 { 3 } else { 2 };
        for _ in 0..iter {
            for _ in 0..steps {
                i += movement_patter[movement_idx];
                j += movement_patter[(movement_idx + 1) % movement_patter.len()];
                result[i as usize][j as usize] = iterator.next().unwrap();
            }
            movement_idx = (movement_idx + 1) % movement_patter.len();
        }
        steps -= 1;
    }
    return result;
}
