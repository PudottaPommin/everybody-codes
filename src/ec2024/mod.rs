use crate::ec_solver::Solver;
use crate::ec2024::q02::Q02;
use crate::ec2024::q03::Q03;

pub mod q01;
pub mod q02;
pub mod q03;

pub fn solver_for(year: u16, quest: u8) -> Option<Box<dyn Solver>> {
    use crate::ec2024::q01::Q01;
    match (year, quest) {
        (2024, 1) => Some(Box::new(Q01)),
        (2024, 2) => Some(Box::new(Q02)),
        (2024, 3) => Some(Box::new(Q03)),
        _ => None,
    }
}
