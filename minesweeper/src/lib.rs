pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let n = minefield.len();
    let mut result: Vec<String> = vec![];
    for i in 0..n {
        let mut res_line: String = String::with_capacity(n);
        let line = minefield[i].as_bytes();
        let m = line.len();
        for j in 0..m {
            let char = line[j] as char;
            if char == ' ' {
                let mut count = 0;
                let left = if j > 0 { j - 1 } else { 0 };
                let right = if j < m - 1 { j + 1 } else { j };
                count += count_mines(&line[left..=right]);
                if i > 0 {
                    let prev = minefield[i - 1].as_bytes();
                    count += count_mines(&prev[left..=right]);
                }
                if i < n - 1 {
                    let next = minefield[i + 1].as_bytes();
                    count += count_mines(&next[left..=right]);
                }

                if count != 0 {
                    res_line.insert(j, char::from_digit(count, 10).unwrap());
                } else {
                    res_line.insert(j, char);
                }
            } else {
                res_line.insert(j, char);
            }
        }
        result.push(res_line)
    }
    result
}

fn count_mines(line: &[u8]) -> u32 {
    let mut result = 0;
    for i in 0..line.len() {
        if line[i] as char == '*' {
            result += 1;
        }
    }
    result
}
