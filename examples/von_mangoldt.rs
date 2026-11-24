use shor_grover_exploration::riemann::von_mangoldt::{
    von_mangoldt_report, von_mangoldt_deep_report,
};

fn main() {
    println!("{}", von_mangoldt_report(5000));
    println!("\n{}", von_mangoldt_deep_report(1000));
}
