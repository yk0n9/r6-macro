use clap::ValueEnum;
use serde::{Deserialize, Serialize};

pub trait ToStr {
    fn to_str(&self) -> &'static str;
}

// 武器
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Weapon {
    pub name: Name,
    pub sight: Sight,
    pub barrel: Barrel,
    pub grip: Grip,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            name: Name::L85A2,
            sight: Sight::None,
            barrel: Barrel::None,
            grip: Grip::None,
        }
    }
}

// 当前主手
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, ValueEnum)]
pub enum Name {
    _416C,
    _552Commando,
    _556XI,
    _6P41,
    _9mmC1,
    _9x19VSN,
    AK12,
    AK74M,
    ALDA556,
    ARX200,
    AugA2,
    AugA3,
    AR33,
    C7E,
    CARBINE,
    Commando9,
    DP27,
    F2,
    F90,
    FMG9,
    G36C,
    G8A1,
    K1A,
    LmgE,
    L85A2,
    M4,
    M762,
    MK17CQB,
    MP5,
    MP5K,
    MP5SD,
    MP7,
    MPX,
    M12,
    M249,
    M249SAW,
    Mx4Storm,
    POF9,
    P90,
    PDW9,
    PARA308,
    P10Roni,
    R4C,
    SPEAR308,
    ScorpionEvo3A1,
    SC3000K,
    T95LSW,
    TYPE89,
    T5SMG,
    UMP45,
    UZK50GI,
    V308,
    Vector45ACP,
}

impl ToStr for Name {
    fn to_str(&self) -> &'static str {
        match self {
            Name::L85A2 => "L85A2",
            Name::AR33 => "AR33",
            Name::_556XI => "556XI",
            Name::G36C => "G36C",
            Name::R4C => "R4-C",
            Name::F2 => "F2",
            Name::AK12 => "AK-12",
            Name::AugA2 => "AUG A2",
            Name::_552Commando => "552 Commando",
            Name::_416C => "416-C CARBINE",
            Name::CARBINE => "C8-SFW",
            Name::MK17CQB => "MK17 CQB",
            Name::PARA308 => "PARA-308",
            Name::TYPE89 => "TYPE-89",
            Name::C7E => "C7E",
            Name::M762 => "M762",
            Name::V308 => "V308",
            Name::SPEAR308 => "SPEAR .308",
            Name::M4 => "M4",
            Name::ARX200 => "ARX200",
            Name::AK74M => "AK-74M",
            Name::F90 => "F90",
            Name::Commando9 => "Commando 9",
            Name::SC3000K => "SC3000K",
            Name::POF9 => "POF-9",
            Name::PDW9 => "PDW9",
            Name::FMG9 => "FMG-9",
            Name::MP5K => "MP5K",
            Name::UMP45 => "UMP45",
            Name::MP5 => "MP5",
            Name::P90 => "P90",
            Name::_9x19VSN => "9x19VSN",
            Name::MP7 => "MP7",
            Name::_9mmC1 => "9mm C1",
            Name::MPX => "MPX",
            Name::M12 => "M12",
            Name::MP5SD => "MP5SD",
            Name::Vector45ACP => "Vector .45 ACP",
            Name::T5SMG => "T-5 SMG",
            Name::ScorpionEvo3A1 => "SCORPION EVO 3 A1",
            Name::K1A => "K1A",
            Name::Mx4Storm => "Mx4 Storm",
            Name::AugA3 => "AUG A3",
            Name::P10Roni => "P10 RONI",
            Name::UZK50GI => "UZK50GI",
            Name::_6P41 => "6P41",
            Name::DP27 => "DP27",
            Name::G8A1 => "G8A1",
            Name::M249 => "M249",
            Name::T95LSW => "T-95 LSW",
            Name::LmgE => "LMG-E",
            Name::ALDA556 => "ALDA 5.56",
            Name::M249SAW => "M249 SAW",
        }
    }
}

// 瞄准镜
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, ValueEnum)]
pub enum Sight {
    None,
    X1_0,
    X1_5,
    X2_0,
    X2_5,
    X3_0,
    X4_0,
}

impl ToStr for Sight {
    fn to_str(&self) -> &'static str {
        match self {
            Sight::None => "无",
            Sight::X1_0 => "1.0x",
            Sight::X1_5 => "1.5x",
            Sight::X2_0 => "2.0x",
            Sight::X2_5 => "2.5x",
            Sight::X3_0 => "3.0x",
            Sight::X4_0 => "4.0x",
        }
    }
}

// 枪管
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, ValueEnum)]
pub enum Barrel {
    None,
    // 消焰器
    FlashHider,
    // 补偿器
    Compensator,
    // 消音器
    Silencer,
    // 延长枪管
    ExtendedBarrel,
}

impl ToStr for Barrel {
    fn to_str(&self) -> &'static str {
        match self {
            Barrel::None => "无",
            Barrel::FlashHider => "消焰器",
            Barrel::Compensator => "补偿器",
            Barrel::Silencer => "消音器",
            Barrel::ExtendedBarrel => "延长枪管",
        }
    }
}

// 握把
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, ValueEnum)]
pub enum Grip {
    None,
    // 垂直
    Vertical,
    // 三角
    Angled,
}

impl ToStr for Grip {
    fn to_str(&self) -> &'static str {
        match self {
            Grip::None => "无",
            Grip::Vertical => "垂直握把",
            Grip::Angled => "拐角握把"
        }
    }
}
