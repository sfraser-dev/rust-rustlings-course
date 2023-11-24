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


use std::collections::HashMap;

fn main() {
    let res1 = "".to_string()
        + "England,France,4,2\n"
        + "France,Italy,3,1\n"
        + "Poland,Spain,2,0\n"
        + "Germany,England,2,1\n";

    my_test_build_scores_table(res1);

    // just to prevent "unused" warnings
    let res2 = "".to_string()
        + "England,France,4,2\n"
        + "France,Italy,3,1\n"
        + "Poland,Spain,2,0\n"
        + "Germany,England,2,1\n";
    let _temp: HashMap<String, Team> = build_scores_table(res2);
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

        update_hashtable(&mut the_scores, &team_1_name, &team_1_score, &team_2_score);
        update_hashtable(&mut the_scores, &team_2_name, &team_2_score, &team_1_score);

    }
    the_scores
}

fn update_hashtable(the_scores: &mut HashMap<String, Team>, team_name: &String, goals_for: &u8, goals_against: &u8)  {
    // team 1 exists in the hashtable already
    if the_scores.contains_key(team_name) {
        // clone the name of team 1, use it as the key to get the Team struct value
        // hashtable.get() returns Option. if let pattern matching to decompose the Team struct into Some
        if let Some(the_team_struct_option) = the_scores.get(team_name) {
            // create a new Team struct and populate it with the exact same values (manual clone) as the "gotten" struct
            let the_team_struct_option_new = Team {
                goals_scored: the_team_struct_option.goals_scored + *goals_for,
                goals_conceded: the_team_struct_option.goals_conceded + *goals_against,
            };
            println!(
                "{}: existing team. GF {}, GA {}",
                team_name,
                the_team_struct_option_new.goals_scored,
                the_team_struct_option_new.goals_conceded,
            );

            // update the hashtable with the Team struct containing the new goals scored/conceded total
            // team_name.clone() used to go from borrowed (&String) to owned (String)
            the_scores.insert(team_name.clone(), the_team_struct_option_new);
        } else {
            println!("error");
        }

    // team 1 isn't currently in the hashtable
    } else {
        println!(
            "{}: new team. GF {}, GA {}",
            team_name, goals_for, goals_against,
        );
        // update the hashtable with the Team struct containing the goals scored/conceded in their first game
        the_scores.insert(
            // team_name.clone() used to go from borrowed (&String) to owned (String). cannot use dereferencing on a &String as string is not a primitive type
            team_name.clone(),
            Team {
                // .clone() used to go from &u8 to u8 (could have used * dereferencing here to as u8 is a primitive type)
                goals_scored: goals_for.clone(),
                goals_conceded: goals_against.clone(),
            },
        );
    }
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

        update_hashtable(&mut scores, &team_1_name, &team_1_score, &team_2_score);
        update_hashtable(&mut scores, &team_2_name, &team_2_score, &team_1_score);
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
