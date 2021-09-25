mod hidden_single;
mod naked_single;
mod naked_subset;
mod hidden_subset;
mod locked_candidate;
mod pointing_tuple;
mod xwing;

pub(crate) use hidden_single::HiddenSingle;
pub(crate) use hidden_subset::HiddenSubset;
pub(crate) use naked_single::NakedSingle;
pub(crate) use naked_subset::NakedSubset;
pub(crate) use locked_candidate::LockedCandidate;
pub(crate) use pointing_tuple::PointingTuple;
pub(crate) use swordfish::Swordfish;
pub(crate) use xwing::XWing;

use itertools::Itertools;
use crate::{Cell, strategy::step::Step};

/// Find the list of locked candidates
pub(crate) fn find_locked<'a, F>(lines: &[&[&Cell]; 3], get_house: F) -> Option<Step>
where
    F: Fn(usize) -> Box<dyn Iterator<Item = &'a Cell> + 'a>,
{
    for candidate in 1_u8..=9 {
        let lines = lines
            .iter()
            .map(|&group| {
                group
                    .iter()
                    .filter(|&&cell| cell.has_candidate(candidate))
                    .map(|&cell| cell.index())
                    .collect_vec()
            })
            .filter(|group| !group.is_empty())
            .collect_vec();

        for i in 0..=lines.len() {
            if let Some((&indexes, others)) = lines
                .iter()
                .cycle()
                .skip(i)
                .take(lines.len())
                .collect_vec()
                .split_first()
            {
                if indexes.len() >= 2 && others.len() == 0_usize {
                    let eliminates = get_house(indexes[0])
                        .filter(|&cell| {
                            cell.has_candidate(candidate) && !indexes.contains(&cell.index())
                        })
                        .collect_vec();

                    if eliminates.len() > 0 {
                        let mut step = Step::new();

                        for index in indexes {
                            step.lock_candidate(*index, candidate)
                        }
                        for neighbor in eliminates {
                            step.eliminate_candidate(neighbor.index(), candidate)
                        }

                        return Some(step);
                    }
                }
            }
        }
    }

    None
}
