use super::consts::{INIT_GRID_X, INIT_GRID_Y};


pub struct Rule3 {
    rule: [[u8; 3]; 3],
    result: u8
}

pub struct Rules3x3 {
    rules: Vec<Rule3>
}

fn generate_rules_neighbours(n_alive: Vec<u8>, n_dead: Vec<u8>) -> Rules3x3 {
    let cur_rule = vec![
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ];

    let found_rules: Vec<Rule3> = get_all_rules3().iter().map(|r| Rule3{rule: *r, result: 1}).collect();

    Rules3x3{rules: found_rules}
}

fn get_all_rules3() -> Vec<[[u8; 3]; 3]> {
    // let mut all_rules: Vec<[[u8; 3]; 3]> = vec![];

    let prev_rule = [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ];

    let all_rules = recur_get_rules(&prev_rule, 0);

    all_rules
}

// fn recur_get_rules(mut all_rules: Vec<[[u8; 3]; 3]>, prev_rule: &[[u8; 3]; 3], idx: usize)
fn recur_get_rules(prev_rule: &[[u8; 3]; 3], idx: usize)
-> Vec<[[u8; 3]; 3]> {
    if idx > 8 {
        return vec![]
    }

    // idx 0-8
    let idx_x: usize = idx % 3;
    let idx_y: usize = idx / 3;

    let mut prev_rule_alive = prev_rule.clone();
    prev_rule_alive[idx_x][idx_y] = 1;

    let mut res1 = recur_get_rules(&prev_rule, idx+1);
    let res2 = recur_get_rules(&prev_rule_alive, idx+1);

    res1.push(prev_rule_alive);

    // println!("{prev_rule:?}");
    // println!("{prev_rule_alive:?}");

    res1.extend(res2);
    return res1
}

pub fn DEF_RULES_LIFE() -> Rules3x3 {
    // let rules: Vec<Rule3x3> = vec![
    //     Rule3x3 {
    //         rule: [
    //             [0, 0, 0],
    //             [1, 0, 0],
    //             [0, 0, 0],
    //         ],
    //         result: 1,
    //     },
    //     Rule3x3 {
    //         rule: [
    //             [0, 0, 0],
    //             [0, 1, 1],
    //             [0, 0, 0],
    //         ],
    //         result: 0,
    //     },
    // ];
    
    let rules: Vec<Rule3> = vec![ ];

    generate_rules_neighbours(vec![3,4], vec![1,2,5,6,7,8]);

    Rules3x3{rules}
}

pub fn DEFAULT_GRID() -> Vec<Vec<u8>> {
    let mut real_grid: Vec<Vec<u8>> = vec![vec![0; INIT_GRID_X]; INIT_GRID_Y];

    let grid = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 1],
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

    for rule in &rules_all.rules {
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
    fn it_works() {
        // let result = generate_rules_neighbours(vec![3,4], vec![1,2,5,6,7,8]);

        let prev_rule = [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
        ];

        let all_rules = recur_get_rules(&prev_rule, 0);

        // let r: Vec<[[u8; 3]; 3]> = 
        //         all_rules.rules.iter().map(|rule3| rule3.rule.clone()).collect();

        println!("{all_rules:?}");
        println!("{all_rules.len()}");
        // assert_eq!(result, 4);
    }
}
