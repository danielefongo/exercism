use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::HashMap;
use std::fmt;

enum MatchResult {
    Won,
    Drawn,
    Lost,
}

struct Tally(HashMap<String, Squad>);

impl Tally {
    fn new() -> Self {
        Tally(HashMap::new())
    }

    fn win(&mut self, winner: &str, loser: &str) {
        self.update_squad_result(winner, MatchResult::Won);
        self.update_squad_result(loser, MatchResult::Lost);
    }

    fn drawn(&mut self, first: &str, second: &str) {
        self.update_squad_result(first, MatchResult::Drawn);
        self.update_squad_result(second, MatchResult::Drawn);
    }

    fn update_squad_result(&mut self, name: &str, result: MatchResult) {
        if !self.0.contains_key(name) {
            self.0.insert(name.to_owned(), Squad::new(name.to_owned()));
        }
        if let Some(stats) = self.0.get_mut(name) {
            stats.process(result);
        }
    }
}

impl fmt::Display for Tally {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = "Team                           | MP |  W |  D |  L |  P".to_string();
        let mut squads: Vec<&Squad> = self.0.values().collect();
        squads.sort();
        squads
            .into_iter()
            .for_each(|squad| output.push_str(&format!("\n{}", squad)));
        write!(f, "{}", output)
    }
}

#[derive(Debug)]
struct Squad {
    name: String,
    won: u8,
    drawn: u8,
    lost: u8,
    matches: u8,
}

impl Squad {
    pub fn new(name: String) -> Squad {
        Squad {
            name,
            won: 0,
            drawn: 0,
            lost: 0,
            matches: 0,
        }
    }
    pub fn points(&self) -> u8 {
        self.won * 3 + self.drawn
    }
    pub fn process(&mut self, result: MatchResult) {
        self.matches += 1;
        match result {
            MatchResult::Won => self.won += 1,
            MatchResult::Drawn => self.drawn += 1,
            MatchResult::Lost => self.lost += 1,
        };
    }
}

impl Ord for Squad {
    fn cmp(&self, other: &Self) -> Ordering {
        (&other.points(), &self.name).cmp(&(&self.points(), &other.name))
    }
}

impl PartialOrd for Squad {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Squad {
    fn eq(&self, other: &Self) -> bool {
        (&self.name, &self.points()) == (&other.name, &other.points())
    }
}

impl Eq for Squad {}

impl fmt::Display for Squad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted = format!(
            "{:name$}|  {} |  {} |  {} |  {} |  {}",
            self.name,
            self.matches,
            self.won,
            self.drawn,
            self.lost,
            self.points(),
            name = 31
        );
        write!(f, "{}", formatted)
    }
}

pub fn tally(match_results: &str) -> String {
    let mut squads = Tally::new();

    match_results
        .split("\n")
        .map(parser)
        .filter_map(|row| row.ok())
        .for_each(|[first_squad, second_squad, result]| match result {
            "win" => squads.win(first_squad, second_squad),
            "loss" => squads.win(second_squad, first_squad),
            _ => squads.drawn(first_squad, second_squad),
        });
    format!("{}", squads)
}

fn parser<'a>(row: &'a str) -> Result<[&'a str; 3], &'static str> {
    let row: Vec<&str> = row.split(";").collect();
    if row.len() == 3 {
        let first_squad = row[0];
        let second_squad = row[1];
        let result = row[2];
        Ok([first_squad, second_squad, result])
    } else {
        Err("Invalid string")
    }
}
