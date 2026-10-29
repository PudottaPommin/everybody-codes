extern crate core;

use crate::internal::ec_solver::{Part, Solver, input_path};
use clap::{Parser, Subcommand};

mod ec2024;
mod ec2026;
#[allow(dead_code)]
mod internal;
mod scaffold;

const DEFAULT_YEAR: u16 = 2026;

/// Run Everybody Codes solvers.
#[derive(Parser)]
#[command(
    name = "everybody-codes",
    about = "Runner for Everybody Codes solutions"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Event year or quest number (e.g. 2024, or pass quest directly like 4)
    #[arg(value_name = "YEAR_OR_QUEST")]
    arg1: Option<u16>,

    /// Quest number or part number
    #[arg(value_name = "QUEST_OR_PART")]
    arg2: Option<u8>,

    /// Part 1-3; omit to run all parts
    #[arg(value_name = "PART", value_parser = clap::value_parser!(u8).range(1..=3))]
    arg3: Option<u8>,

    /// Run all available quests for the year with summary benchmarks
    #[arg(short = 'a', long = "all")]
    all: bool,

    /// Override the puzzle input file path
    #[arg(short = 'i', long = "input")]
    input: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold a new quest template and input files
    New {
        /// Quest number (or year if two arguments provided)
        first: Option<u16>,
        /// Quest number (if first was year)
        second: Option<u8>,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(Commands::New { first, second }) = cli.command {
        let (year, quest) = match (first, second) {
            (Some(f), Some(s)) => (f, s),
            (Some(f), None) => {
                if f > 1000 {
                    eprintln!(
                        "Error: please specify a quest number, e.g. 'cargo ec new {} 1'",
                        f
                    );
                    std::process::exit(1);
                }
                (DEFAULT_YEAR, f as u8)
            }
            (None, _) => match prompt_for_scaffold() {
                Some((y, q)) => (y, q),
                None => {
                    println!("Scaffolding cancelled.");
                    return;
                }
            },
        };
        if let Err(e) = scaffold::scaffold_quest(year, quest) {
            eprintln!("Error scaffolding quest: {e}");
            std::process::exit(1);
        }
        return;
    }

    // Resolve arguments
    let (year, quest, part) = resolve_args(cli.arg1, cli.arg2, cli.arg3);

    if cli.all {
        run_all_quests(year);
        return;
    }

    let Some(q) = quest else {
        eprintln!("No quests registered for year {year}.");
        std::process::exit(1);
    };

    let Some(solver) = solver_for(year, q) else {
        eprintln!("No solver registered for year {year} quest {q}.");
        std::process::exit(2);
    };

    println!("Running Everybody Codes {year} - Quest {q:02}:");

    let parts: Vec<Part> = match part {
        Some(n) => vec![Part::from_number(n).expect("valid part 1-3")],
        None => Part::ALL.to_vec(),
    };

    let mut total_time = std::time::Duration::ZERO;
    for p in parts {
        let result = if let Some(custom_path) = cli.input.as_deref() {
            solver.solve_from_path(p, custom_path)
        } else if let Some(embedded) = solver.input(p) {
            if embedded.trim().is_empty() {
                eprintln!("[P{p:}]: skipped (input is empty)");
                None
            } else {
                let (answer, elapsed) = solver.solve_timed(p, embedded);
                println!("[P{p:}]: {answer} in {:?}", elapsed);
                Some((answer, elapsed))
            }
        } else {
            let path = input_path(year, q, p);
            solver.solve_from_path(p, &path)
        };

        if let Some((_, elapsed)) = result {
            total_time += elapsed;
        }
    }

    if cfg!(debug_assertions) && total_time.as_millis() > 50 {
        println!("(debug build - run with 'cargo ecr' for release mode)");
    }
}

fn resolve_args(
    arg1: Option<u16>,
    arg2: Option<u8>,
    arg3: Option<u8>,
) -> (u16, Option<u8>, Option<u8>) {
    match (arg1, arg2, arg3) {
        // No args: run latest quest of default year
        (None, None, None) => {
            let latest = available_quests_for(DEFAULT_YEAR).last().copied();
            (DEFAULT_YEAR, latest, None)
        }
        // One arg:
        // If arg1 <= 50: it's quest number for default year! E.g. `cargo ec 4`
        // If arg1 > 1000: it's a year! E.g. `cargo ec 2024` -> run latest of that year
        (Some(n), None, None) => {
            if n <= 50 {
                (DEFAULT_YEAR, Some(n as u8), None)
            } else {
                let latest = available_quests_for(n).last().copied();
                (n, latest, None)
            }
        }
        // Two args:
        // If arg1 <= 50: quest and part for default year! E.g. `cargo ec 4 2`
        // If arg1 > 1000: year and quest! E.g. `cargo ec 2024 4`
        (Some(a1), Some(a2), None) => {
            if a1 <= 50 {
                (DEFAULT_YEAR, Some(a1 as u8), Some(a2))
            } else {
                (a1, Some(a2), None)
            }
        }
        // Three args: explicit year, quest, part: `cargo ec 2024 4 2`
        (Some(y), Some(q), Some(p)) => (y, Some(q), Some(p)),
        _ => (DEFAULT_YEAR, None, None),
    }
}

pub fn available_quests_for(year: u16) -> &'static [u8] {
    match year {
        2024 => ec2024::available_quests(),
        2026 => ec2026::available_quests(),
        _ => &[],
    }
}

pub fn solver_for(year: u16, quest: u8) -> Option<Box<dyn Solver>> {
    match year {
        2024 => ec2024::solver_for(year, quest),
        2026 => ec2026::solver_for(year, quest),
        _ => None,
    }
}

fn format_duration(d: std::time::Duration) -> String {
    let micros = d.as_micros();
    if micros < 1_000 {
        format!("{micros}µs")
    } else if micros < 1_000_000 {
        format!("{:.2}ms", d.as_secs_f64() * 1000.0)
    } else {
        format!("{:.2}s", d.as_secs_f64())
    }
}

fn run_all_quests(year: u16) {
    let quests = available_quests_for(year);
    if quests.is_empty() {
        eprintln!("No quests registered for year {year}.");
        return;
    }

    println!("\n{:=^76}", format!(" Everybody Codes {year} Summary "));
    println!(
        " {:<5} | {:<16} | {:<16} | {:<16} | {:<10}",
        "Quest", "Part 1", "Part 2", "Part 3", "Total"
    );
    println!("{:-<76}", "");

    let mut grand_total = std::time::Duration::ZERO;
    let mut total_parts_solved = 0;

    for &q in quests {
        let Some(solver) = solver_for(year, q) else {
            continue;
        };

        let mut row_durations = [None, None, None];
        let mut row_total = std::time::Duration::ZERO;

        for (idx, &p) in Part::ALL.iter().enumerate() {
            let result = if let Some(embedded) = solver.input(p).filter(|s| !s.trim().is_empty()) {
                Some(solver.solve_timed(p, embedded))
            } else {
                let path = input_path(year, q, p);
                internal::read_input(&path)
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .map(|disk_input| solver.solve_timed(p, &disk_input))
            };

            if let Some((_ans, elapsed)) = result {
                row_durations[idx] = Some(elapsed);
                row_total += elapsed;
                total_parts_solved += 1;
            }
        }

        grand_total += row_total;

        let p1_str = row_durations[0]
            .map(format_duration)
            .unwrap_or_else(|| "-".to_string());
        let p2_str = row_durations[1]
            .map(format_duration)
            .unwrap_or_else(|| "-".to_string());
        let p3_str = row_durations[2]
            .map(format_duration)
            .unwrap_or_else(|| "-".to_string());
        let total_str = format_duration(row_total);

        println!(
            " Q{:<4} | {:<16} | {:<16} | {:<16} | {:<10}",
            format!("{q:02}"),
            p1_str,
            p2_str,
            p3_str,
            total_str
        );
    }

    println!("{:-<76}", "");
    println!(
        " Total | {} quests | {} parts solved {:<15} | {}",
        quests.len(),
        total_parts_solved,
        "",
        format_duration(grand_total)
    );
    println!("{:=^76}\n", "");

    if cfg!(debug_assertions) {
        println!(
            "Note: Ran in debug mode. Run with 'cargo allr' (or 'cargo ecr --all') for release benchmarks.\n"
        );
    }
}

fn prompt_for_scaffold() -> Option<(u16, u8)> {
    let next_quest = available_quests_for(DEFAULT_YEAR)
        .last()
        .copied()
        .unwrap_or(0)
        + 1;

    #[cfg(target_os = "windows")]
    {
        if let Some((y, q)) = prompt_gui(next_quest) {
            return Some((y, q));
        }
    }

    prompt_console(next_quest)
}

#[cfg(target_os = "windows")]
fn prompt_gui(default_quest: u8) -> Option<(u16, u8)> {
    let script = format!(
        "[void][Reflection.Assembly]::LoadWithPartialName('Microsoft.VisualBasic'); [Microsoft.VisualBasic.Interaction]::InputBox('Enter quest number (or year and quest, e.g. 2024 5):', 'Scaffold Everybody Codes', '{default_quest}')"
    );
    let output = std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if text.is_empty() {
        return None;
    }

    parse_year_and_quest(&text)
}

fn parse_year_and_quest(s: &str) -> Option<(u16, u8)> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    match parts.as_slice() {
        [q_str] => {
            let q = q_str.parse::<u8>().ok()?;
            if (1..=25).contains(&q) {
                Some((DEFAULT_YEAR, q))
            } else {
                None
            }
        }
        [y_str, q_str] => {
            let y = y_str.parse::<u16>().ok()?;
            let q = q_str.parse::<u8>().ok()?;
            if (1..=25).contains(&q) {
                Some((y, q))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn prompt_console(default_quest: u8) -> Option<(u16, u8)> {
    use std::io::{self, Write};

    println!("\n--- Scaffold New Everybody Codes Quest ---");
    print!("Enter event year [default: {DEFAULT_YEAR}]: ");
    let _ = io::stdout().flush();
    let mut year_input = String::new();
    if io::stdin().read_line(&mut year_input).is_err() {
        return None;
    }
    let trimmed_year = year_input.trim();
    let year = if trimmed_year.is_empty() {
        DEFAULT_YEAR
    } else {
        trimmed_year.parse::<u16>().unwrap_or(DEFAULT_YEAR)
    };

    for _ in 0..3 {
        print!("Enter quest / day number (1-25) [default: {default_quest}]: ");
        let _ = io::stdout().flush();
        let mut quest_input = String::new();
        if io::stdin().read_line(&mut quest_input).is_err() {
            return None;
        }
        let trimmed = quest_input.trim();
        let q = if trimmed.is_empty() {
            default_quest
        } else if let Ok(n) = trimmed.parse::<u8>() {
            n
        } else {
            println!("Invalid number.");
            continue;
        };

        if (1..=25).contains(&q) {
            return Some((year, q));
        }
        println!("Please enter a quest number between 1 and 25.");
    }
    None
}
