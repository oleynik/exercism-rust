/// Only for `0 <= num <= 99`. Otherwise – panic!
fn units(num: u64) -> String {
    match num {
        0 => String::new(),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        15 => String::from("fifteen"),
        18 => String::from("eighteen"),
        13..=19 => units(num % 10) + "teen",
        20..=29 => {
            String::from("twenty") + if num % 10 > 0 { "-" } else { "" } + units(num % 10).as_str()
        }
        30..=39 => {
            String::from("thirty") + if num % 10 > 0 { "-" } else { "" } + units(num % 10).as_str()
        }
        40..=49 => {
            String::from("forty") + if num % 10 > 0 { "-" } else { "" } + units(num % 10).as_str()
        }
        50..=59 => {
            String::from("fifty") + if num % 10 > 0 { "-" } else { "" } + units(num % 10).as_str()
        }
        60..=69 => {
            String::from("sixty") + if num % 10 > 0 { "-" } else { "" } + units(num % 10).as_str()
        }
        70..=79 => {
            String::from("seventy") + if num % 10 > 0 { "-" } else { "" } + units(num % 10).as_str()
        }
        80..=89 => {
            String::from("eighty") + if num % 10 > 0 { "-" } else { "" } + units(num % 10).as_str()
        }
        90..=99 => {
            String::from("ninety") + if num % 10 > 0 { "-" } else { "" } + units(num % 10).as_str()
        }
        _ => panic!("Unexpected number: {}", num),
    }
}

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let mut result = vec![];
    let tens: Vec<(u32, String)> = Vec::from([
        (18, String::from("quintillion")),
        (15, String::from("quadrillion")),
        (12, String::from("trillion")),
        (9, String::from("billion")),
        (6, String::from("million")),
        (3, String::from("thousand")),
        (2, String::from("hundred")),
        (0, String::new()),
    ]);
    for (base, name) in tens {
        let ten_power = 10u64.pow(base);
        if n < ten_power {
            continue;
        }
        let dig3 = n / ten_power;
        if dig3 / 100 > 0 {
            result.push(units(dig3 / 100));
            result.push("hundred".to_string());
        }
        result.push(units(dig3 % 100));
        result.push(name);
        n %= ten_power;
    }
    result.join(" ").trim().to_string()
}
