use crate::Solver;

mod q01;
mod q02;
mod q03;
mod q04;

pub fn solver_for(year: u16, quest: u8) -> Option<Box<dyn Solver>> {
    use crate::ec2024::*;
    match (year, quest) {
        (2024, 1) => Some(Box::new(q01::Q01)),
        (2024, 2) => Some(Box::new(q02::Q02)),
        (2024, 3) => Some(Box::new(q03::Q03)),
        (2024, 4) => Some(Box::new(q04::Q04)),
        _ => None,
    }
}
