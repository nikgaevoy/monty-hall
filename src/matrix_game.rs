use good_lp::{clarabel, variable, variables, Expression, Solution, SolverModel, Variable};

#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]
pub struct GameSolution {
    cost: f64,
    distribution: Vec<f64>,
}

fn row_expression(variables: &[Variable], row: &[f64]) -> Expression {
    variables.iter().zip(row).map(|(x, c)| *c * *x).sum()
}

pub fn reverse_game(game: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut ans =
        vec![vec![0.; game.len()]; game.iter().map(|row| row.len()).max().unwrap_or_default()];

    for (i, row) in game.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            ans[j][i] = -val;
        }
    }

    ans
}

pub fn solve_game(game: &Vec<Vec<f64>>) -> GameSolution {
    if game.is_empty() {
        return Default::default();
    }

    variables! {problem: cost;}
    let cols: Vec<Variable> = problem.add_vector(
        variable().bounds(0..=1),
        game.iter().map(|row| row.len()).max().unwrap(),
    );
    let total_prob: Expression = cols.iter().sum();
    let mut model = problem
        .minimise(cost)
        .using(clarabel)
        .with(total_prob.eq(1.));

    for row in game {
        model = model.with(row_expression(&cols, row).leq(cost));
    }

    let solution = model.solve().unwrap();

    GameSolution {
        cost: solution.value(cost),
        distribution: cols.into_iter().map(|col| solution.value(col)).collect(),
    }
}
