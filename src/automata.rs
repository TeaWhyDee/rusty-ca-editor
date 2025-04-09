use super::consts::{INIT_GRID_X, INIT_GRID_Y};


pub struct Rule3x3 {
    rule: [[u8; 3]; 3],
    result: u8
}

pub struct Rules3x3 {
    rules: Vec<Rule3x3>
}


pub fn DEF_RULES_LIFE() -> Rules3x3 {
    let rules: Vec<Rule3x3> = vec![
        Rule3x3 {
            rule: [
                [0, 0, 0],
                [1, 0, 0],
                [0, 0, 0],
            ],
            result: 1,
        },
        Rule3x3 {
            rule: [
                [0, 0, 0],
                [0, 1, 1],
                [0, 0, 0],
            ],
            result: 0,
        },
    ];

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
        rule: &Rule3x3
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
