use std::collections::BTreeMap;

#[derive(Debug)]
struct Team<'a> {
    name: &'a str,
    matches: usize,
    wins: usize,
    draws: usize,
    losses: usize,
    points: usize,
}

pub fn tally(match_results: &str) -> String {

    let mut ss = format!("{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}\n", "Team", "MP", "W", "D", "L", "P");
    
    if match_results.is_empty() { return ss.trim().to_string() }

    let mut teams: BTreeMap<&str, Team> = BTreeMap::new();

    for game in match_results.split('\n') {
        let s: Vec<&str> = game.split(';').collect();

        let ((a_win, a_draw, a_loss), (b_win, b_draw, b_loss)) = match s[2] {
            "win" => ((1, 0, 0), (0, 0, 1)),
            "loss" => ((0, 0, 1), (1, 0, 0)),
            _ => ((0, 1, 0), (0, 1, 0)),
        };

        teams
            .entry(s[0])
            .and_modify(|x| {
                x.wins += a_win;
                x.draws += a_draw;
                x.losses += a_loss
            })
            .or_insert(Team {
                name: s[0],
                matches: 0,
                wins: a_win,
                draws: a_draw,
                losses: a_loss,
                points: 0,
            });

        teams
            .entry(s[1])
            .and_modify(|x| {
                x.wins += b_win;
                x.draws += b_draw;
                x.losses += b_loss
            })
            .or_insert(Team {
                name: s[0],
                matches: 0,
                wins: b_win,
                draws: b_draw,
                losses: b_loss,
                points: 0,
            });
    }

    let mut v:Vec<(&str, usize, usize, usize, usize, usize)> = Vec::new();

    for team in teams.iter_mut() {
        team.1.matches = team.1.wins + team.1.draws + team.1.losses;
        team.1.points = team.1.wins * 3 + team.1.draws;
        v.push((team.0, team.1.matches, team.1.wins, team.1.draws, team.1.losses, team.1.points));
    }

    v.sort_by(|a, b| (b.5.cmp(&a.5)));
    
    for line in v {
        ss += &format!("{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}\n", line.0, line.1, line.2, line.3, line.4, line.5);
    }

    ss.trim().to_string()
}
