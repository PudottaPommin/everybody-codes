use crate::Solver;

mod q01;
mod q02;
mod q03;
mod q04;

pub const YEAR: u16 = 2024;

pub fn available_quests() -> &'static [u8] {
    &[1, 2, 3, 4]
}

pub fn solver_for(year: u16, quest: u8) -> Option<Box<dyn Solver>> {
    if year != YEAR {
        return None;
    }
    match quest {
        1 => Some(Box::new(q01::Q01)),
        2 => Some(Box::new(q02::Q02)),
        3 => Some(Box::new(q03::Q03)),
        4 => Some(Box::new(q04::Q04)),
        _ => None,
    }
}