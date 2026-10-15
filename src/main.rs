extern crate core;

use crate::internal::ec_solver::{Part, Solver, input_path};
use clap::Parser;

mod ec2024;
mod internal;

/// Run Everybody Codes solvers.
#[derive(Parser)]
struct Args {
    /// Event year, e.g. 2024
    year: u16,
    /// Quest number, e.g. 1
    quest: u8,
    /// Part 1-3; omit to run all parts
    #[arg(value_parser = clap::value_parser!(u8).range(1..=3))]
    part: Option<u8>,
}

fn main() {
    let args = Args::parse();

    let Some(solver) = solver_for(args.year, args.quest) else {
        eprintln!(
            "no solver registered for {} quest {}",
            args.year, args.quest
        );
        std::process::exit(2);
    };

    let parts: Vec<Part> = match args.part {
        Some(n) => vec![Part::from_number(n).expect("range already validated")],
        None => Part::ALL.to_vec(),
    };

    for part in parts {
        solver.solve_from_path(part, &input_path(args.year, args.quest, part));
    }
}

/// The one place you touch when adding a new quest.
pub fn solver_for(year: u16, quest: u8) -> Option<Box<dyn Solver>> {
    ec2024::solver_for(year, quest)
}
