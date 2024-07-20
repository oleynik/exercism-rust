use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

struct Team {
    name: String,
    wins: u32,
    draws: u32,
    lost: u32,
}

impl Team {
    fn new(name: &str) -> Self {
        Team {
            name: name.to_string(),
            wins: 0,
            draws: 0,
            lost: 0,
        }
    }

    fn matches(&self) -> u32 {
        self.wins + self.draws + self.lost
    }

    fn points(&self) -> u32 {
        3 * self.wins + self.draws
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name,
            self.matches(),
            self.wins,
            self.draws,
            self.lost,
            self.points(),
        )
    }
}

fn opposite_result(res: &str) -> &str {
    match res {
        "win" => "loss",
        "loss" => "win",
        _ => "draw",
    }
}

pub fn tally(match_results: &str) -> String {
    let mut result = format!(
        "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
        "Team", "MP", "W", "D", "L", "P"
    );
    if match_results.is_empty() {
        return result;
    }
    let mut map: HashMap<String, Team> = HashMap::new();
    match_results
        .lines()
        .map(|l| {
            let mut split = l.split(";");
            let host = split.next().unwrap();
            let guest = split.next().unwrap();
            let res = split.next().unwrap();
            vec![(host, res), (guest, opposite_result(res))]
        })
        .flatten()
        .for_each(|(name, res)| {
            let team = map.entry(String::from(name)).or_insert(Team::new(name));
            match res {
                "win" => team.wins += 1,
                "loss" => team.lost += 1,
                _ => team.draws += 1,
            }
        });
    let mut vec = map.values().collect::<Vec<&Team>>();
    vec.sort_by(|a, b| match a.points().cmp(&b.points()) {
        Ordering::Equal => a.name.cmp(&b.name).reverse(),
        cmp => cmp,
    });
    result.push('\n');
    result.push_str(
        vec.iter()
            .rev()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join("\n")
            .as_str(),
    );
    result
}
