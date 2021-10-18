pub mod job;

use job::Job;
use std::collections::HashMap;

pub fn from_str<S: AsRef<str>>(
    s: S,
) -> ron::error::Result<HashMap<String, Job>> {
    ron::from_str(s.as_ref())
}

#[cfg(test)]
mod tests {
    use super::from_str;
    use crate::job::{
        Class, Job, Location, Stat, StatConstraint, Stats, Weaponry, WepSet,
        WepType,
    };
    use std::{collections::HashMap, convert::TryInto};

    #[test]
    fn not_a_real_job() {
        let jobs = from_str(
            r#"{
            "not a real job": Job(
                classes: [Beginner, EvanBeginner, Spearman, DualBlade],
                location: MapleIsland,
                stats: Stats(
                    primary: [STR, MAXMP],
                    secondary: [LUK],
                    constraints: [
                        [Less(DEX)],
                        [Ful(STR), Ful(LUK)],
                    ],
                ),
                weaponry: Weaponry(
                    allowed: WepTypes([Spear, OneHandedMace]),
                    canonical: WepIds([132_2003, 132_2006]),
                ),
                ammo: true,
                skills: Some([]),
            ),
        }
        "#,
        )
        .unwrap();

        let mut jobs_explicit = HashMap::with_capacity(1);
        jobs_explicit.insert(
            "not a real job".to_owned(),
            Job {
                classes: vec![
                    Class::Beginner,
                    2001.try_into().unwrap(),
                    Class::Spearman,
                    Class::DualBlade,
                ],
                location: Location::MapleIsland,
                stats: Stats {
                    primary: vec![Stat::STR, Stat::MAXMP],
                    secondary: vec![Stat::LUK],
                    constraints: vec![
                        vec![StatConstraint::Less(Stat::DEX)],
                        vec![
                            StatConstraint::Ful(Stat::STR),
                            StatConstraint::Ful(Stat::LUK),
                        ],
                    ],
                },
                weaponry: Weaponry {
                    allowed: WepSet::WepTypes(vec![
                        WepType::Spear,
                        132.try_into().unwrap(),
                    ]),
                    canonical: WepSet::WepIds(vec![132_2003, 132_2006]),
                },
                ammo: true,
                skills: Some(Vec::new()),
            },
        );

        assert_eq!(jobs, jobs_explicit);
    }
}
