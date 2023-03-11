pub fn verse(n: u32) -> String {
    fn sign_number(n: u32, capital: bool) -> String {
        match (n, capital) {
            (0, true) => String::from("No more bottles"),
            (0, false) => String::from("no more bottles"),
            (1, _) => String::from("1 bottle"),
            (_, _) => format!("{} bottles", n),
        }
    }

    fn first(n: u32) -> String {
        format!(
            "{} of beer on the wall, {} of beer.\n",
            sign_number(n, true),
            sign_number(n, false)
        )
    }

    fn second(n: u32) -> String {
        match n {
            0 => {
                String::from("Go to the store and buy some more, 99 bottles of beer on the wall.\n")
            }
            1 => format!(
                "Take it down and pass it around, {} of beer on the wall.\n",
                sign_number(n - 1, false)
            ),
            _ => format!(
                "Take one down and pass it around, {} of beer on the wall.\n",
                sign_number(n - 1, false)
            ),
        }
    }

    first(n) + second(n).as_str()
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
