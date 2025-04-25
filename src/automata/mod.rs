use std::collections::HashSet;
use super::consts::{INIT_GRID_X, INIT_GRID_Y};


pub struct Rule3 {
    pub rule: [[u8; 3]; 3],
    result: u8
}

pub struct Rules3x3 {
    pub rules_by_type: Vec<Vec<Rule3>>,
    pub num_states: u8,
    pub colors: Vec<iced::Color>,
}

impl Default for Rules3x3 {
    fn default() -> Rules3x3 {
        Rules3x3 {
            rules_by_type: vec![],
            num_states: 2,
            colors: vec![
                iced::Color::new(0.0, 0.0, 0.0, 1.0),
                iced::Color::new(1.0, 1.0, 1.0, 1.0),
                iced::Color::new(1.0, 1.0, 1.0, 1.0)
            ],
        }
    }
}


pub fn RAND_RULES() -> Rules3x3 {
    let rules: Vec<Vec<Rule3>> = 
    vec![
        vec![ // 0
            Rule3 {
                rule: [
                    [1, 1, 1],
                    [1, 0, 1],
                    [1, 1, 1],
                ],
                result: 2,
            }
        ], 
        vec![ // 1
            Rule3 {
                rule: [
                    [0, 0, 0],
                    [0, 1, 0],
                    [0, 0, 0],
                ],
                result: 2,
            }
        ], 
        vec![ // 2
            Rule3 {
                rule: [
                    [0, 0, 0],
                    [0, 2, 0],
                    [0, 0, 0],
                ],
                result: 1,
            },
        ]
    ];

    let colors = vec![ 
        iced::Color::new(0.0, 0.0, 0.0, 1.0),
        iced::Color::new(1.0, 1.0, 1.0, 1.0),
        iced::Color::new(0.8, 0.5, 0.1, 1.0)
    ];

    Rules3x3{rules_by_type: rules, num_states: 3, colors}
}

pub fn DEF_RULES_LIFE() -> Rules3x3 {
    // B3/S23
    generate_rules_neighbours(vec![3], vec![0, 1, 4, 5, 6, 7, 8])
}

pub fn DEFAULT_GRID() -> Vec<Vec<u8>> {
    let mut real_grid: Vec<Vec<u8>> = vec![vec![0; INIT_GRID_X]; INIT_GRID_Y];

    let grid = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
    ];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            real_grid[i][j] = grid[i][j]
        }
    }

    real_grid
}


pub fn apply_rule(
        grid: &Vec<Vec<u8>>,
        new_grid: &Vec<Vec<u8>>,
        index: (usize, usize),
        rule: &Rule3
) -> u8 {
    
    for i in 0..3 {
        for j in 0..3 {
            let x = index.0 + i - 1;
            let y = index.1 + j - 1;

            if grid[x][y] != rule.rule[i][j] {
                return new_grid[index.0][index.1]
            }
        }
    }

    return rule.result
}


pub fn ca(grid: &mut Vec<Vec<u8>>, rules_all: &Rules3x3) {
    let mut new_grid = grid.clone();

    for rule in rules_all.rules_by_type.iter().flat_map(|r| r.iter()) {
        for i in 1..grid.len()-1 {
            for j in 1..grid[0].len()-1 {
                new_grid[i][j] = apply_rule(grid, &new_grid, (i, j), &rule)
            }
        }
    }

    *grid = new_grid;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_rules() {
        let prev_rule = [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
        ];

        let all_rules = recur_get_rules(&prev_rule, 0);

        let ar_len = all_rules.len();
        println!("{all_rules:#?}");
        println!("{ar_len}");
        assert_eq!(ar_len, 512);
    }

    #[test]
    fn test_ca_gol() {
        // let result = generate_rules_neighbours(vec![3], vec![0, 1, 4, 5, 6, 7, 8]);

        let prev_rule = [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
        ];

        let all_rules = recur_get_rules(&prev_rule, 0);

        // let r: Vec<[[u8; 3]; 3]> = 
        //         all_rules.rules.iter().map(|rule3| rule3.rule.clone()).collect();

        let ar_len = all_rules.len();
        println!("{all_rules:#?}");
        println!("{ar_len}");
        // assert_eq!(result, 4);
    }
}

fn generate_rules_neighbours(n_alive: Vec<u8>, n_dead: Vec<u8>) -> Rules3x3 {
    // Generate rules based on alive and dead members (only 2 cell states)
    //
    // let all_rules: Vec<Rule3> = get_all_rules3().iter().map(|r| Rule3{rule: *r, result: 1}).collect();
    let all_rules: Vec<[[u8; 3]; 3]> = get_all_rules3().iter().map(|r| *r).collect();
    let mut filtered_rules_0: Vec<Rule3> = vec![]; // dead cells -> ...
    let mut filtered_rules_1: Vec<Rule3> = vec![]; // alive cells -> ...

    for mut found_rule in all_rules {
        if found_rule[1][1] != 0 {
            continue
        }
        // for convenience, just check rules where [1][1] == 0
        let mut num = 0;
        for found_rule_row in found_rule {
            for found_rule_cell in found_rule_row {
                if found_rule_cell != 0 {
                    num += 1;
                }
            }
        }

        if n_alive.contains(&num) {
            filtered_rules_0.push( Rule3{rule: found_rule, result: 1} );
        }

        if n_dead.contains(&num) {
            found_rule[1][1] = 1;
            filtered_rules_1.push( Rule3{rule: found_rule, result: 0} );
        }
    }

    let filtered_rules_all = vec![filtered_rules_0, filtered_rules_1];

    Rules3x3{rules_by_type: filtered_rules_all, num_states: 2, ..Default::default()}
}

fn get_all_rules3() -> HashSet<[[u8; 3]; 3]> {
    let prev_rule = [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ];

    let all_rules = recur_get_rules(&prev_rule, 0);

    all_rules
}

fn recur_get_rules(prev_rule: &[[u8; 3]; 3], idx: usize)
-> HashSet<[[u8; 3]; 3]> {
    if idx > 8 {
        // Insert first (all zeroes) rule
        let mut hs = HashSet::new();
        hs.insert(*prev_rule);
        return hs
    }

    // Get x,y position in 3x3 grid (from idx 0-8)
    let idx_x: usize = idx % 3;
    let idx_y: usize = idx / 3;

    let mut prev_rule_alive = prev_rule.clone();
    prev_rule_alive[idx_x][idx_y] = 1;

    let mut res1 = recur_get_rules(&prev_rule, idx+1);
    let res2 = recur_get_rules(&prev_rule_alive, idx+1);

    res1.insert(prev_rule_alive);
    res1.extend(res2);

    return res1
}

