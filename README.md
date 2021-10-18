# maple\_jobs

A very smol [Rust](https://www.rust-lang.org/) library that I made specifically
for [(de)serialising](https://en.wikipedia.org/wiki/Serialization) a
special-sauce format of [RON](https://github.com/ron-rs/ron) that represents
pre-BB MapleStory jobs.

## sample usage

```rust
let ron_file_contents = r#"
{
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
"#;

// Deserialise from a string
let jobs = maple_jobs::from_str(ron_file_contents).unwrap();

// Construct the same in-memory structure, but explicitly
use maple_jobs::job::{
    Class,
    Job,
    Location,
    Stat,
    StatConstraint,
    Stats,
    Weaponry,
    WepSet,
    WepType,
};
use std::{collections::HashMap, convert::TryInto};

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

// Test for equality
assert_eq!(jobs, jobs_explicit);
```

## legal

[Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0), or
[MIT](https://mit-license.org/), at [your
option](https://en.wikipedia.org/wiki/Multi-licensing).
