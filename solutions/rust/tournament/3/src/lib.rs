use std::collections::BTreeMap;

struct Points {
    matches: usize,
    wins: usize,
    draws: usize,
    losses: usize,
    points: usize,
}

pub fn tally(match_results: &str) -> String {

    let mut ss = format!("{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}\n", "Team", "MP", "W", "D", "L", "P");
    
    if match_results.is_empty() { return ss.trim().to_string() }

    let mut points: BTreeMap<&str, Points> = BTreeMap::new();

    for game in match_results.split('\n') {
        let s: Vec<&str> = game.split(';').collect();

        let ((a_win, a_draw, a_loss), (b_win, b_draw, b_loss)) = match s[2] {
            "win" => ((1, 0, 0), (0, 0, 1)),
            "loss" => ((0, 0, 1), (1, 0, 0)),
            _ => ((0, 1, 0), (0, 1, 0)),
        };

        points
            .entry(s[0])
            .and_modify(|x| {
                x.wins += a_win;
                x.draws += a_draw;
                x.losses += a_loss
            })
            .or_insert(Points {
                matches: 0,
                wins: a_win,
                draws: a_draw,
                losses: a_loss,
                points: 0,
            });

        points
            .entry(s[1])
            .and_modify(|x| {
                x.wins += b_win;
                x.draws += b_draw;
                x.losses += b_loss
            })
            .or_insert(Points {
                matches: 0,
                wins: b_win,
                draws: b_draw,
                losses: b_loss,
                points: 0,
            });
    }

    let mut v:Vec<(&str, usize, usize, usize, usize, usize)> = Vec::new();

    for p in points.iter_mut() {
        p.1.matches = p.1.wins + p.1.draws + p.1.losses;
        p.1.points = p.1.wins * 3 + p.1.draws;
        v.push((p.0, p.1.matches, p.1.wins, p.1.draws, p.1.losses, p.1.points));
    }

    v.sort_by(|a, b| (b.5.cmp(&a.5)));
    
    for line in v {
        ss += &format!("{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}\n", line.0, line.1, line.2, line.3, line.4, line.5);
    }

    ss.trim().to_string()
}
