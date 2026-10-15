use std::fs;
use std::path::Path;

pub fn scaffold_quest(year: u16, quest: u8) -> Result<(), String> {
    if !(1..=25).contains(&quest) {
        return Err(format!(
            "quest number must be between 1 and 25, got {quest}"
        ));
    }

    let quest_str = format!("q{quest:02}");
    let quest_upper = format!("Q{quest:02}");

    // 1. Create input files
    let input_dir = format!("inputs/ec{year}/{quest_str}");
    fs::create_dir_all(&input_dir)
        .map_err(|e| format!("failed to create directory '{input_dir}': {e}"))?;

    for part in 1..=3 {
        let input_file = format!("{input_dir}/{part}.txt");
        if !Path::new(&input_file).exists() {
            fs::write(&input_file, b"")
                .map_err(|e| format!("failed to create input file '{input_file}': {e}"))?;
            println!("  Created: {input_file}");
        } else {
            println!("  Exists:  {input_file}");
        }
    }

    // 2. Create quest solution file
    let quest_file = format!("src/ec{year}/{quest_str}.rs");
    if Path::new(&quest_file).exists() {
        println!("  Exists:  {quest_file} (skipping overwrite)");
    } else {
        let template = format!(
            r#"use crate::internal::Solver;

pub struct {quest_upper};

impl {quest_upper} {{
    pub fn part1(_input: &[u8]) -> usize {{
        0
    }}

    pub fn part2(_input: &[u8]) -> usize {{
        0
    }}

    pub fn part3(_input: &[u8]) -> usize {{
        0
    }}
}}

impl Solver for {quest_upper} {{
    fn part1(&self, input: &[u8]) -> String {{
        Self::part1(input).to_string()
    }}

    fn part2(&self, input: &[u8]) -> String {{
        Self::part2(input).to_string()
    }}

    fn part3(&self, input: &[u8]) -> String {{
        Self::part3(input).to_string()
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, "")]
    fn test_part1(#[case] expected: usize, #[case] input: &str) {{
        assert_eq!({quest_upper}::part1(input.as_bytes()), expected);
    }}
}}
"#
        );
        fs::write(&quest_file, template)
            .map_err(|e| format!("failed to write quest template '{quest_file}': {e}"))?;
        println!("  Created: {quest_file}");
    }

    // 3. Update src/ec{year}/mod.rs
    let mod_file = format!("src/ec{year}/mod.rs");
    if Path::new(&mod_file).exists() {
        update_mod_rs(&mod_file, quest)?;
        println!("  Updated: {mod_file}");
    } else {
        eprintln!("  Notice:  '{mod_file}' not found. Please register {quest_str} manually.");
    }

    println!("\nQuest {quest} for year {year} is ready! Run with: cargo ec {year} {quest}");
    Ok(())
}

fn update_mod_rs(mod_path: &str, quest: u8) -> Result<(), String> {
    let content =
        fs::read_to_string(mod_path).map_err(|e| format!("failed to read '{mod_path}': {e}"))?;

    let updated = update_mod_rs_content(&content, quest)?;

    fs::write(mod_path, updated)
        .map_err(|e| format!("failed to write updated '{mod_path}': {e}"))?;

    Ok(())
}

pub fn update_mod_rs_content(content: &str, quest: u8) -> Result<String, String> {
    let mod_decl = format!("mod q{quest:02};");
    if content.contains(&mod_decl) {
        return Ok(content.to_string());
    }

    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    // 1. Insert `mod qXX;` after last `mod q...;`
    let last_mod_idx = lines
        .iter()
        .rposition(|line| line.trim_start().starts_with("mod q"));
    let insert_pos = match last_mod_idx {
        Some(idx) => idx + 1,
        None => 0,
    };
    lines.insert(insert_pos, mod_decl);

    // Reconstruct and update `available_quests()`
    let mut updated = lines.join("\n");
    let fn_signature = "pub fn available_quests() -> &'static [u8] {";
    if let Some(fn_start) = updated.find(fn_signature) {
        let body_start = fn_start + fn_signature.len();
        if let Some(slice_offset) = updated[body_start..].find("&[") {
            let actual_slice_start = body_start + slice_offset;
            if let Some(bracket_end) = updated[actual_slice_start..].find(']') {
                let actual_bracket_end = actual_slice_start + bracket_end;
                let slice_text = &updated[actual_slice_start..=actual_bracket_end]; // e.g. &[1, 2, 3, 4]
                let inner = &slice_text[2..slice_text.len() - 1]; // "1, 2, 3, 4"
                let mut nums: Vec<u8> = inner
                    .split(',')
                    .filter_map(|s| s.trim().parse::<u8>().ok())
                    .collect();
                if !nums.contains(&quest) {
                    nums.push(quest);
                    nums.sort_unstable();
                    let new_slice = format!(
                        "&[{}]",
                        nums.iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                    updated.replace_range(actual_slice_start..=actual_bracket_end, &new_slice);
                }
            }
        }
    }

    // 2. Insert match arm in `solver_for`: `{quest} => Some(Box::new(q{quest:02}::Q{quest:02})),`
    let quest_arm = format!("        {quest} => Some(Box::new(q{quest:02}::Q{quest:02})),");
    if !updated.contains(quest_arm.trim()) {
        let pattern = "_ => None,";
        if let Some(wildcard_pos) = updated.find(pattern) {
            updated.insert_str(wildcard_pos, &format!("{quest_arm}\n        "));
        }
    }

    Ok(updated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_mod_rs_content() {
        let original = r#"use crate::Solver;

mod q01;
mod q02;

pub const YEAR: u16 = 2024;

pub fn available_quests() -> &'static [u8] {
    &[1, 2]
}

pub fn solver_for(year: u16, quest: u8) -> Option<Box<dyn Solver>> {
    if year != YEAR {
        return None;
    }
    match quest {
        1 => Some(Box::new(q01::Q01)),
        2 => Some(Box::new(q02::Q02)),
        _ => None,
    }
}
"#;
        let updated = update_mod_rs_content(original, 3).unwrap();
        assert!(updated.contains("mod q03;"));
        assert!(updated.contains("&[1, 2, 3]"));
        assert!(updated.contains("3 => Some(Box::new(q03::Q03)),"));
    }
}
