use ndarray::*;
use simplex_method::Table;

fn main() {
    env_logger::init();
    let constr_coeff =
        array![[4f64, 1f64, 1f64],
         [1f64, 1f64, 0f64],
         [0f64, 0.5f64, 4f64]];
    let func_coeff =
         array![1f64, 5f64, 5f64];
    let constr_val =
         array![6f64, 5f64, 5f64];
    let mut table = Table::new(constr_coeff, constr_val, func_coeff, false);
    table.optimise().expect("Something's gone wrong");
    println!("{}", table);
}
