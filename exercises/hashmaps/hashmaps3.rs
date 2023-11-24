// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::collections::HashMap;

fn main() {
    let res = "".to_string()
        + "England,France,4,2\n"
        + "France,Italy,3,1\n"
        + "Poland,Spain,2,0\n"
        + "Germany,England,2,1\n";

    my_test_build_scores_table(res);
}

fn my_test_build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut the_scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.

        // team 1 exists in the hashtable already
        if the_scores.contains_key(&team_1_name) {
            // clone the name of team 1, use it as the key to get the Team struct value
            let team_1_name_clone: String = team_1_name.clone();
            // hashtable.get() returns Option. if let pattern matching to decompose the Team struct into Some 
            if let Some(the_team_struct_option) = the_scores.get(&team_1_name_clone) {
                // create a new Team struct and populate it with the exact same values (manual clone) as the "gotten" struct 
                let the_team_struct_option_new= Team {
                    goals_scored: the_team_struct_option.goals_scored + team_1_score,
                    goals_conceded: the_team_struct_option.goals_conceded + team_2_score,
                };
                println!(
                    "{}: existing team. GF {}, GA {}",
                    team_1_name_clone,
                    the_team_struct_option_new.goals_scored,
                    the_team_struct_option_new.goals_conceded,
                );

                // update the hashtable with the Team struct containing the new goals scored/conceded total
                the_scores.insert(team_1_name_clone, the_team_struct_option_new);
            } else {
                println!("error");
            }
        
        // team 1 isn't currently in the hashtable
        } else {
            println!(
                "{}: new team. GF {}, GA {}",
                team_1_name, team_1_score, team_2_score,
            );
            // update the hashtable with the Team struct containing the goals scored/conceded in their first game
            the_scores.insert(
                team_1_name,
                Team {
                    goals_scored: team_1_score,
                    goals_conceded: team_2_score,
                },
            );
        }
        
        // team 2 exists in the hashtable already
        if the_scores.contains_key(&team_2_name) {
            // clone the name of team 2, use it as the key to get the Team struct value
            let team_2_name_clone: String = team_2_name.clone();
            // hashtable.get() returns Option. if let pattern matching to decompose the Team struct into Some 
            if let Some(the_team_struct_option) = the_scores.get(&team_2_name_clone) {
                // create a new Team struct and populate it with the exact same values (manual clone) as the "gotten" struct 
                let the_team_struct_option_clone = Team {
                    goals_scored: the_team_struct_option.goals_scored + team_2_score,
                    goals_conceded: the_team_struct_option.goals_conceded + team_1_score,
                };
                println!(
                    "{}: existing team. GF {}, GA {}",
                    team_2_name_clone,
                    the_team_struct_option_clone.goals_scored,
                    the_team_struct_option_clone.goals_conceded,
                );
                // update the hashtable with the Team struct containing the new goals scored/conceded total
                the_scores.insert(team_2_name_clone, the_team_struct_option_clone);
            } else {
                println!("error");
            }
        
        // team 1 isn't currently in the hashtable
        } else {
            println!(
                "{}: new team. GF {}, GA {}",
                team_2_name, team_2_score, team_1_score,
            );
            // update the hashtable with the Team struct containing the goals scored/conceded in their first game
            the_scores.insert(
                team_2_name,
                Team {
                    goals_scored: team_2_score,
                    goals_conceded: team_1_score,
                },
            );
        }
    }
    // println!("{:?}",the_scores);
    the_scores
}

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
