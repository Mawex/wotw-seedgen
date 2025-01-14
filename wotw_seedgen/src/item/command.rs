use std::fmt;

use decorum::R32;
use num_enum::TryFromPrimitive;
use wotw_seedgen_derive::{Display, FromStr, VVariant};

use super::{Item, Resource, VItem};
use crate::header::{vdisplay, CodeDisplay, VString};
use crate::uber_state::UberIdentifier;
use crate::util::{NumericBool, Position, Spell, VPosition};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, VVariant)]
pub enum Command {
    Autosave,
    Resource {
        resource: Resource,
        #[VWrap]
        amount: i32,
    },
    Checkpoint,
    Magic,
    StopEqual {
        uber_identifier: UberIdentifier,
        #[VWrap]
        value: R32,
    },
    StopGreater {
        uber_identifier: UberIdentifier,
        #[VWrap]
        value: R32,
    },
    StopLess {
        uber_identifier: UberIdentifier,
        #[VWrap]
        value: R32,
    },
    Toggle {
        target: ToggleCommand,
        #[VWrap]
        on: NumericBool,
    },
    Warp {
        #[VType]
        position: Position,
    },
    StartTimer {
        identifier: UberIdentifier,
    },
    StopTimer {
        identifier: UberIdentifier,
    },
    StateRedirect {
        intercept: i32,
        set: i32,
    },
    SetHealth {
        #[VWrap]
        amount: i32,
    },
    SetEnergy {
        #[VWrap]
        amount: i32,
    },
    SetSpiritLight {
        #[VWrap]
        amount: i32,
    },
    Equip {
        #[VWrap]
        slot: EquipSlot,
        ability: Spell,
    },
    AhkSignal {
        signal: String,
    },
    IfEqual {
        uber_identifier: UberIdentifier,
        #[VWrap]
        value: R32,
        #[VType]
        item: Box<Item>,
    },
    IfGreater {
        uber_identifier: UberIdentifier,
        #[VWrap]
        value: R32,
        #[VType]
        item: Box<Item>,
    },
    IfLess {
        uber_identifier: UberIdentifier,
        #[VWrap]
        value: R32,
        #[VType]
        item: Box<Item>,
    },
    DisableSync {
        uber_identifier: UberIdentifier,
    },
    EnableSync {
        uber_identifier: UberIdentifier,
    },
    CreateWarp {
        id: i32,
        #[VType]
        position: Position,
        label: Option<String>,
    },
    DestroyWarp {
        id: i32,
    },
    IfBox {
        #[VType]
        position1: Position,
        #[VType]
        position2: Position,
        #[VType]
        item: Box<Item>,
    },
    IfSelfEqual {
        #[VWrap]
        value: String,
        #[VType]
        item: Box<Item>,
    },
    IfSelfGreater {
        #[VWrap]
        value: String,
        #[VType]
        item: Box<Item>,
    },
    IfSelfLess {
        #[VWrap]
        value: String,
        #[VType]
        item: Box<Item>,
    },
    UnEquip {
        ability: Spell,
    },
    SaveString {
        id: i32,
        #[VType]
        string: String,
    },
    AppendString {
        id: i32,
        #[VType]
        string: String,
    },
}
impl Command {
    pub fn code(&self) -> CodeDisplay<Command> {
        CodeDisplay::new(self, |s, f| match s {
            Command::Autosave => write!(f, "0"),
            Command::Resource { resource, amount } => write!(f, "1|{}|{}", *resource as u8, amount),
            Command::Checkpoint => write!(f, "2"),
            Command::Magic => write!(f, "3"),
            Command::StopEqual {
                uber_identifier,
                value,
            } => write!(f, "4|{}|{}", uber_identifier.code(), value),
            Command::StopGreater {
                uber_identifier,
                value,
            } => write!(f, "5|{}|{}", uber_identifier.code(), value),
            Command::StopLess {
                uber_identifier,
                value,
            } => write!(f, "6|{}|{}", uber_identifier.code(), value),
            Command::Toggle { target, on } => write!(f, "7|{}|{}", *target as u8, *on as u8),
            Command::Warp { position } => write!(f, "8|{}", position.code()),
            Command::StartTimer { identifier } => write!(f, "9|{}", identifier.code()),
            Command::StopTimer { identifier } => write!(f, "10|{}", identifier.code()),
            Command::StateRedirect { intercept, set } => write!(f, "11|{}|{}", intercept, set),
            Command::SetHealth { amount } => write!(f, "12|{}", amount),
            Command::SetEnergy { amount } => write!(f, "13|{}", amount),
            Command::SetSpiritLight { amount } => write!(f, "14|{}", amount),
            Command::Equip { slot, ability } => write!(f, "15|{}|{}", *slot as u8, *ability as u16),
            Command::AhkSignal { signal } => write!(f, "16|{}", signal),
            Command::IfEqual {
                uber_identifier,
                value,
                item,
            } => write!(f, "17|{}|{}|{}", uber_identifier.code(), value, item.code()),
            Command::IfGreater {
                uber_identifier,
                value,
                item,
            } => write!(f, "18|{}|{}|{}", uber_identifier.code(), value, item.code()),
            Command::IfLess {
                uber_identifier,
                value,
                item,
            } => write!(f, "19|{}|{}|{}", uber_identifier.code(), value, item.code()),
            Command::DisableSync { uber_identifier } => write!(f, "20|{}", uber_identifier.code()),
            Command::EnableSync { uber_identifier } => write!(f, "21|{}", uber_identifier.code()),
            Command::CreateWarp {
                id,
                position,
                label,
            } => {
                write!(f, "22|{}|{}", id, position.code())?;
                match label {
                    Some(label) => write!(f, "|{label}"),
                    None => Ok(()),
                }
            }
            Command::DestroyWarp { id } => write!(f, "23|{}", id),
            Command::IfBox {
                position1,
                position2,
                item,
            } => write!(
                f,
                "24|{}|{}|{}",
                position1.code(),
                position2.code(),
                item.code()
            ),
            Command::IfSelfEqual { value, item } => write!(f, "25|{}|{}", value, item.code()),
            Command::IfSelfGreater { value, item } => write!(f, "26|{}|{}", value, item.code()),
            Command::IfSelfLess { value, item } => write!(f, "27|{}|{}", value, item.code()),
            Command::UnEquip { ability } => write!(f, "28|{}", *ability as u16),
            Command::SaveString { id, string } => write!(f, "29|{}|{}", id, string),
            Command::AppendString { id, string } => write!(f, "30|{}|{}", id, string),
        })
    }
}
vdisplay! {
    VCommand,
    impl fmt::Display for Command {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Autosave => write!(f, "Autosave"),
                Self::Resource { resource, amount } => write!(f, "Set {resource} to {amount}"),
                Self::Checkpoint => write!(f, "Checkpoint"),
                Self::Magic => write!(f, "✨Magic✨"),
                Self::StopEqual { uber_identifier, value } => write!(f, "Stop the multipickup if {} is {}", uber_identifier, value),
                Self::StopGreater { uber_identifier, value } => write!(f, "Stop the multipickup if {} is greater than {}", uber_identifier, value),
                Self::StopLess { uber_identifier, value } => write!(f, "Stop the multipickup if {} is less than {}", uber_identifier, value),
                Self::Toggle { target, on } => write!(f, "Sets the {target} to {on}"),
                Self::Warp { position } => write!(f, "Warp to {position}"),
                Self::StartTimer { identifier } => write!(f, "Start a timer on {identifier}"),
                Self::StopTimer { identifier } => write!(f, "Stop any timer on {identifier}"),
                Self::StateRedirect { intercept, set } => write!(f, "Override state applier {intercept} with {set}"),
                Self::SetHealth { amount } => write!(f, "Set current health to {amount}"),
                Self::SetEnergy { amount } => write!(f, "Set current energy to {amount}"),
                Self::SetSpiritLight { amount } => write!(f, "Set current spirit light to {amount}"),
                Self::Equip { slot, ability } => write!(f, "Equip {ability} to slot {slot}"),
                Self::AhkSignal { signal } => write!(f, "Trigger the \"{signal}\" keybind"),
                Self::IfEqual { uber_identifier, value, item } => write!(f, "Grant this item if {} is {}: {}", uber_identifier, value, item),
                Self::IfGreater { uber_identifier, value, item } => write!(f, "Grant this item if {} is greater than {}: {}", uber_identifier, value, item),
                Self::IfLess { uber_identifier, value, item } => write!(f, "Grant this item if {} is less than {}: {}", uber_identifier, value, item),
                Self::DisableSync { uber_identifier } => write!(f, "Disable multiplayer sync for {uber_identifier}"),
                Self::EnableSync { uber_identifier } => write!(f, "Enable multiplayer sync for {uber_identifier}"),
                Self::CreateWarp { id, position, label } => {
                    write!(f, "Create a warp icon with identifier {id} at {position}")?;
                    match label {
                        Some(label) => write!(f, " labelled \"{label}\""),
                        None => Ok(()),
                    }
                },
                Self::DestroyWarp { id } => write!(f, "Destroy the warp icon with identifier {id}"),
                Self::IfBox { position1, position2, item } => write!(f, "Grant this item if Ori is within the rectangle defined by {position1}/{position2}: {item}"),
                Self::IfSelfEqual { value, item } => write!(f, "Grant this item if the trigger state's value is {value}: {item}"),
                Self::IfSelfGreater { value, item } => write!(f, "Grant this item if the trigger state's value is greater than {value}: {item}"),
                Self::IfSelfLess { value, item } => write!(f, "Grant this item if the trigger state's value is less than {value}: {item}"),
                Self::UnEquip { ability } => write!(f, "Unequip {ability}"),
                Self::SaveString { id, string } => write!(f, "Stores the string \"{string}\" under the identifier {id}"),
                Self::AppendString { id, string } => write!(f, "Appends the string \"{string}\" to the current value stored under the identifier {id}"),
            }
        }
    }
}

#[derive(
    Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, TryFromPrimitive, FromStr,
)]
#[repr(u8)]
pub enum ToggleCommand {
    KwolokDoor = 0,
    Rain = 1,
    Howl = 2,
}

#[derive(
    Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, TryFromPrimitive, FromStr,
)]
#[repr(u8)]
pub enum EquipSlot {
    Ability1,
    Ability2,
    Ability3,
}
