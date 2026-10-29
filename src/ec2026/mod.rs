use crate::Solver;

mod q01;

pub const YEAR: u16 = 2026;

pub fn available_quests() -> &'static [u8] {
    &[1]
}

pub fn solver_for(year: u16, quest: u8) -> Option<Box<dyn Solver>> {
    if year != YEAR {
        return None;
    }
    match quest {
        1 => Some(Box::new(q01::Q01)),
        _ => None,
    }
}
