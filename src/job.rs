use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt, io::Read};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Job {
    pub classes: Vec<Class>,
    pub location: Location,
    pub stats: Stats,
    pub weaponry: Weaponry,
    pub ammo: bool,
    pub skills: Option<Vec<u32>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Class {
    Beginner,
    Swordman,
    Fighter,
    Page,
    Spearman,
    Magician,
    FP,
    IL,
    Cleric,
    Archer,
    Hunter,
    Crossbowman,
    Rogue,
    Assassin,
    Bandit,
    DualBlade,
    Pirate,
    Brawler,
    Gunslinger,
    Noblesse,
    DawnWarrior1st,
    DawnWarrior,
    BlazeWizard1st,
    BlazeWizard,
    WindArcher1st,
    WindArcher,
    NightWalker1st,
    NightWalker,
    ThunderBreaker1st,
    ThunderBreaker,
    AranBeginner,
    EvanBeginner,
    Aran1st,
    Aran,
    Evan1st,
    Evan,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Beginner => f.write_str("beginner"),
            Self::Swordman => f.write_str("sword(wo)man"),
            Self::Fighter => f.write_str("fighter"),
            Self::Page => f.write_str("page"),
            Self::Spearman => f.write_str("spear(wo)man"),
            Self::Magician => f.write_str("magician"),
            Self::FP => f.write_str("F/P"),
            Self::IL => f.write_str("I/L"),
            Self::Cleric => f.write_str("cleric"),
            Self::Archer => f.write_str("archer"),
            Self::Hunter => f.write_str("hunter"),
            Self::Crossbowman => f.write_str("crossbow(o)man"),
            Self::Rogue => f.write_str("rogue"),
            Self::Assassin => f.write_str("assassin"),
            Self::Bandit => f.write_str("bandit"),
            Self::DualBlade => f.write_str("dual blade"),
            Self::Pirate => f.write_str("pirate"),
            Self::Brawler => f.write_str("brawler"),
            Self::Gunslinger => f.write_str("gunslinger"),
            Self::Noblesse => f.write_str("noblesse"),
            Self::DawnWarrior1st => f.write_str("dawn warrior (1st grade)"),
            Self::DawnWarrior => f.write_str("dawn warrior"),
            Self::BlazeWizard1st => f.write_str("blaze wizard (1st grade)"),
            Self::BlazeWizard => f.write_str("blaze wizard"),
            Self::WindArcher1st => f.write_str("wind archer (1st grade)"),
            Self::WindArcher => f.write_str("wind archer"),
            Self::NightWalker1st => f.write_str("night walker (1st grade)"),
            Self::NightWalker => f.write_str("night walker"),
            Self::ThunderBreaker1st => {
                f.write_str("thunder breaker (1st grade)")
            }
            Self::ThunderBreaker => f.write_str("thunder breaker"),
            Self::AranBeginner => f.write_str("aran (beginner)"),
            Self::EvanBeginner => f.write_str("evan (beginner)"),
            Self::Aran1st => f.write_str("aran (1st grade)"),
            Self::Aran => f.write_str("aran"),
            Self::Evan1st => f.write_str("evan (1st grade)"),
            Self::Evan => f.write_str("evan"),
        }
    }
}

impl From<Class> for u16 {
    fn from(value: Class) -> Self {
        match value {
            Class::Beginner => 0,
            Class::Swordman => 100,
            Class::Fighter => 110,
            Class::Page => 120,
            Class::Spearman => 130,
            Class::Magician => 200,
            Class::FP => 210,
            Class::IL => 220,
            Class::Cleric => 230,
            Class::Archer => 300,
            Class::Hunter => 310,
            Class::Crossbowman => 320,
            Class::Rogue => 400,
            Class::Assassin => 410,
            Class::Bandit => 420,
            Class::DualBlade => 430,
            Class::Pirate => 500,
            Class::Brawler => 510,
            Class::Gunslinger => 520,
            Class::Noblesse => 1000,
            Class::DawnWarrior1st => 1100,
            Class::DawnWarrior => 1110,
            Class::BlazeWizard1st => 1200,
            Class::BlazeWizard => 1210,
            Class::WindArcher1st => 1300,
            Class::WindArcher => 1310,
            Class::NightWalker1st => 1400,
            Class::NightWalker => 1410,
            Class::ThunderBreaker1st => 1500,
            Class::ThunderBreaker => 1510,
            Class::AranBeginner => 2000,
            Class::EvanBeginner => 2001,
            Class::Aran1st => 2100,
            Class::Aran => 2110,
            Class::Evan1st => 2200,
            Class::Evan => 2210,
        }
    }
}

impl TryFrom<u16> for Class {
    type Error = ();

    /// Only accepts values of `value` that represent second-grade-or-lower
    /// classes. For example, `111` represents "crusader", but will not be
    /// accepted by this function (you'll get `Err(())`). Instead, you need to
    /// provide `110` (in which case, you'll get `Ok(Class::Fighter)`).
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Beginner,
            100 => Self::Swordman,
            110 => Self::Fighter,
            120 => Self::Page,
            130 => Self::Spearman,
            200 => Self::Magician,
            210 => Self::FP,
            220 => Self::IL,
            230 => Self::Cleric,
            300 => Self::Archer,
            310 => Self::Hunter,
            320 => Self::Crossbowman,
            400 => Self::Rogue,
            410 => Self::Assassin,
            420 => Self::Bandit,
            430 => Self::DualBlade,
            500 => Self::Pirate,
            510 => Self::Brawler,
            520 => Self::Gunslinger,
            1000 => Self::Noblesse,
            1100 => Self::DawnWarrior1st,
            1110 => Self::DawnWarrior,
            1200 => Self::BlazeWizard1st,
            1210 => Self::BlazeWizard,
            1300 => Self::WindArcher1st,
            1310 => Self::WindArcher,
            1400 => Self::NightWalker1st,
            1410 => Self::NightWalker,
            1500 => Self::ThunderBreaker1st,
            1510 => Self::ThunderBreaker,
            2000 => Self::AranBeginner,
            2001 => Self::EvanBeginner,
            2100 => Self::Aran1st,
            2110 => Self::Aran,
            2200 => Self::Evan1st,
            2210 => Self::Evan,
            _ => return Err(()),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Location {
    Camp,
    MapleIsland,
    Outland,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Camp => f.write_str("Camp"),
            Self::MapleIsland => f.write_str("Maple Island"),
            Self::Outland => f.write_str("outland"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Stats {
    pub primary: Vec<Stat>,
    pub secondary: Vec<Stat>,
    pub constraints: Vec<Vec<StatConstraint>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Stat {
    STR,
    DEX,
    INT,
    LUK,
    MAXHP,
    MAXMP,
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::STR => f.write_str("STR"),
            Self::DEX => f.write_str("DEX"),
            Self::INT => f.write_str("INT"),
            Self::LUK => f.write_str("LUK"),
            Self::MAXHP => f.write_str("MAXHP"),
            Self::MAXMP => f.write_str("MAXMP"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum StatConstraint {
    Less(Stat),
    Ful(Stat),
    Pure(Stat),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Weaponry {
    pub allowed: WepSet,
    pub canonical: WepSet,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum WepSet {
    All,
    WepTypes(Vec<WepType>),
    WepIds(Vec<u32>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[repr(u8)]
pub enum WepType {
    OneHandedSword = 130,
    OneHandedAxe = 131,
    OneHandedMace = 132,
    Dagger = 133,
    Wand = 137,
    Staff = 138,
    TwoHandedSword = 140,
    TwoHandedAxe = 141,
    TwoHandedMace = 142,
    Spear = 143,
    Polearm = 144,
    Bow = 145,
    Crossbow = 146,
    Claw = 147,
    Knuckler = 148,
    Gun = 149,
}

impl WepType {
    /// "Staves"!!!
    pub fn plural(&self) -> &'static str {
        match self {
            Self::OneHandedSword => "one-handed swords",
            Self::OneHandedAxe => "one-handed axes",
            Self::OneHandedMace => "one-handed BWs",
            Self::Dagger => "daggers",
            Self::Wand => "wands",
            Self::Staff => "staves",
            Self::TwoHandedSword => "two-handed swords",
            Self::TwoHandedAxe => "two-handed axes",
            Self::TwoHandedMace => "two-handed BWs",
            Self::Spear => "spears",
            Self::Polearm => "polearms",
            Self::Bow => "bows",
            Self::Crossbow => "crossbows",
            Self::Claw => "claws",
            Self::Knuckler => "knucklers",
            Self::Gun => "guns",
        }
    }
}

impl fmt::Display for WepType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneHandedSword => f.write_str("one-handed sword"),
            Self::OneHandedAxe => f.write_str("one-handed axe"),
            Self::OneHandedMace => f.write_str("one-handed BW"),
            Self::Dagger => f.write_str("dagger"),
            Self::Wand => f.write_str("wand"),
            Self::Staff => f.write_str("staff"),
            Self::TwoHandedSword => f.write_str("two-handed sword"),
            Self::TwoHandedAxe => f.write_str("two-handed axe"),
            Self::TwoHandedMace => f.write_str("two-handed BW"),
            Self::Spear => f.write_str("spear"),
            Self::Polearm => f.write_str("polearm"),
            Self::Bow => f.write_str("bow"),
            Self::Crossbow => f.write_str("crossbow"),
            Self::Claw => f.write_str("claw"),
            Self::Knuckler => f.write_str("knuckler"),
            Self::Gun => f.write_str("gun"),
        }
    }
}

impl From<WepType> for u8 {
    fn from(value: WepType) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for WepType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            130 => Self::OneHandedSword,
            131 => Self::OneHandedAxe,
            132 => Self::OneHandedMace,
            133 => Self::Dagger,
            137 => Self::Wand,
            138 => Self::Staff,
            140 => Self::TwoHandedSword,
            141 => Self::TwoHandedAxe,
            142 => Self::TwoHandedMace,
            143 => Self::Spear,
            144 => Self::Polearm,
            145 => Self::Bow,
            146 => Self::Crossbow,
            147 => Self::Claw,
            148 => Self::Knuckler,
            149 => Self::Gun,
            _ => return Err(()),
        })
    }
}

pub fn from_str<S: AsRef<str>>(s: S) -> ron::error::Result<Job> {
    ron::from_str(s.as_ref())
}

pub fn from_reader<R: Read>(rdr: R) -> ron::error::Result<Job> {
    ron::de::from_reader(rdr)
}

#[cfg(test)]
mod tests {
    use super::{
        from_str, Class, Job, Location, Stat, StatConstraint, Stats, Weaponry,
        WepSet, WepType,
    };
    use std::convert::TryInto;

    #[test]
    fn not_a_real_job() {
        let job = from_str(
            r#"Job(
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
            )
            "#,
        )
        .unwrap();

        let job_explicit = Job {
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
        };

        assert_eq!(job, job_explicit);
    }
}
