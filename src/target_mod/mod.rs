// Module for target configuration
//
// Target should be defined with size, position (at bulk, or at boundary)
// It provides function a 'check_find' which check whether a position vector v is near the target
//

use crate::prelude::*;

pub mod cont_bulk;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum TargetType{
    ContinuousInBulk,
    ContinuousAtBoundary,
    LatticeInBulk,
    LatticeAtBoundary,
    NetworkSingleNode,
}

impl_fmt_for_type!(TargetType,
    TargetType::ContinuousInBulk => "Target in Bulk of Continous System.",
    TargetType::ContinuousAtBoundary => "Target at Boundary of Continous System.",
    TargetType::LatticeInBulk => "Target in Bulk of Lattice System.",
    TargetType::LatticeAtBoundary => "Target at Boundary of Lattice System.",
    TargetType::NetworkSingleNode => "Target is a Single Node in Network.");

impl_fromstr_for_type!(TargetType,
    TargetType::ContinuousInBulk => "Target in Bulk of Continous System.",
    TargetType::ContinuousAtBoundary => "Target at Boundary of Continous System.",
    TargetType::LatticeInBulk => "Target in Bulk of Lattice System.",
    TargetType::LatticeAtBoundary => "Target at Boundary of Lattice System.",
    TargetType::NetworkSingleNode => "Target is a Single Node in Network.");

pub trait TargetCore<T>{
    // Return the type of target
    fn ttype(&self) -> TargetType;

    // Check whether a searcher finds the target
    fn check_find(&self, pos: &Position<T>) -> Result<bool, Error>;
}



#[cfg(test)]
mod tests{
    use super::*;
    use crate::{impl_fmt_test, impl_fromstr_test};

    impl_fmt_test!(test_fmt_targettype,
        TargetType::ContinuousInBulk => "Target in Bulk of Continous System.",
        TargetType::ContinuousAtBoundary => "Target at Boundary of Continous System.",
        TargetType::LatticeInBulk => "Target in Bulk of Lattice System.",
        TargetType::LatticeAtBoundary => "Target at Boundary of Lattice System.",
        TargetType::NetworkSingleNode => "Target is a Single Node in Network.");

    impl_fromstr_test!(test_fromstr_targettype,
        TargetType,
        TargetType::ContinuousInBulk => "Target in Bulk of Continous System.",
        TargetType::ContinuousAtBoundary => "Target at Boundary of Continous System.",
        TargetType::LatticeInBulk => "Target in Bulk of Lattice System.",
        TargetType::LatticeAtBoundary => "Target at Boundary of Lattice System.",
        TargetType::NetworkSingleNode => "Target is a Single Node in Network.");

}








