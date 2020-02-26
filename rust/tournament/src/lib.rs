use std::collections::HashMap;
use std::fmt::Write;

#[derive(Default)]
struct Score {
    wins: usize,
    losses: usize,
    draws: usize,
}

impl Score {
    fn points(&self) -> usize {
        self.wins * 3 + self.draws
    }
}

pub fn tally(match_results: &str) -> String {
    let mut scores: HashMap<&str, Score> = HashMap::new();
    for result in match_results.lines() {
        let mut iter = result.split(';');
        let (team_1, team_2, state) = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );
        match state {
            "win" => {
                scores.entry(team_1).or_default().wins += 1;
                scores.entry(team_2).or_default().losses += 1;
            }
            "loss" => {
                scores.entry(team_1).or_default().losses += 1;
                scores.entry(team_2).or_default().wins += 1;
            }
            "draw" => {
                scores.entry(team_1).or_default().draws += 1;
                scores.entry(team_2).or_default().draws += 1;
            }
            _ => unreachable!(),
        }
    }
    let mut scores: Vec<_> = scores.iter().collect();
    scores.sort_by(
        |a, b| match (b.1.points().cmp(&a.1.points()), a.0.cmp(&b.0)) {
            (std::cmp::Ordering::Equal, r @ _) => r,
            (r @ _, _) => r,
        },
    );
    let mut result = String::new();
    write!(
        &mut result,
        "{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
        "Team", "MP", "W", "D", "L", "P",
    )
    .unwrap();
    for (name, score) in scores {
        let mp = score.wins + score.losses + score.draws;
        let p = score.wins * 3 + score.draws;
        write!(
            &mut result,
            "\n{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            name, mp, score.wins, score.draws, score.losses, p
        )
        .unwrap();
    }
    result
}
