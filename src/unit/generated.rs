// AUTO-GENERATED FILE, DO NOT EDIT
use crate::unit::{UnitTypeData, NavalTypeData, Traits};

pub static UNIT_TYPE_DATA: &[UnitTypeData] = &[
    UnitTypeData { name: "Archer", cost: 3, max_hp: 10.0, attack: 2.0, defense: 1.0, range: 2, traits: Traits::DASH.union(Traits::FORTIFY) },
    UnitTypeData { name: "BabyDragon", cost: 10, max_hp: 15.0, attack: 3.0, defense: 3.0, range: 1, traits: Traits::DASH.union(Traits::ESCAPE).union(Traits::GROW).union(Traits::SCOUT).union(Traits::STATIC) },
    UnitTypeData { name: "BattleSled", cost: 5, max_hp: 15.0, attack: 3.0, defense: 2.0, range: 1, traits: Traits::DASH.union(Traits::ESCAPE).union(Traits::SKATE) },
    UnitTypeData { name: "Catapult", cost: 8, max_hp: 10.0, attack: 4.0, defense: 0.0, range: 3, traits: Traits::STIFF },
    UnitTypeData { name: "Centipede", cost: 10, max_hp: 20.0, attack: 4.0, defense: 3.0, range: 1, traits: Traits::CREEP.union(Traits::DASH).union(Traits::EAT).union(Traits::STATIC) },
    UnitTypeData { name: "Cloak", cost: 8, max_hp: 5.0, attack: 2.0, defense: 0.5, range: 1, traits: Traits::CREEP.union(Traits::DASH).union(Traits::HIDE).union(Traits::INFILTRATE).union(Traits::SCOUT).union(Traits::STATIC).union(Traits::STIFF) },
    UnitTypeData { name: "Crab", cost: 10, max_hp: 40.0, attack: 4.0, defense: 4.0, range: 1, traits: Traits::AUTOFLOOD.union(Traits::ESCAPE).union(Traits::STATIC) },
    UnitTypeData { name: "Dagger", cost: 2, max_hp: 10.0, attack: 2.0, defense: 2.0, range: 1, traits: Traits::DASH.union(Traits::INDEPENDENT).union(Traits::STATIC).union(Traits::SURPRISE) },
    UnitTypeData { name: "DefaultWarrior", cost: 2, max_hp: 10.0, attack: 2.0, defense: 2.0, range: 1, traits: Traits::DASH.union(Traits::FORTIFY) },
    UnitTypeData { name: "Defender", cost: 3, max_hp: 15.0, attack: 1.0, defense: 3.0, range: 1, traits: Traits::FORTIFY },
    UnitTypeData { name: "Doomux", cost: 10, max_hp: 20.0, attack: 4.0, defense: 2.0, range: 1, traits: Traits::CREEP.union(Traits::DASH).union(Traits::EXPLODE) },
    UnitTypeData { name: "Egg", cost: 10, max_hp: 10.0, attack: 0.0, defense: 2.0, range: 1, traits: Traits::FORTIFY.union(Traits::GROW).union(Traits::STATIC).union(Traits::STIFF) },
    UnitTypeData { name: "Exida", cost: 8, max_hp: 10.0, attack: 3.0, defense: 1.0, range: 3, traits: Traits::POISON },
    UnitTypeData { name: "FireDragon", cost: 10, max_hp: 20.0, attack: 4.0, defense: 3.0, range: 2, traits: Traits::DASH.union(Traits::SCOUT).union(Traits::SPLASH).union(Traits::STATIC) },
    UnitTypeData { name: "Gaami", cost: 10, max_hp: 30.0, attack: 4.0, defense: 3.0, range: 1, traits: Traits::AUTO_FREEZE.union(Traits::FREEZE_AREA).union(Traits::STATIC) },
    UnitTypeData { name: "Giant", cost: 10, max_hp: 40.0, attack: 5.0, defense: 4.0, range: 1, traits: Traits::STATIC },
    UnitTypeData { name: "Hexapod", cost: 3, max_hp: 5.0, attack: 3.0, defense: 1.0, range: 1, traits: Traits::CREEP.union(Traits::DASH).union(Traits::ESCAPE).union(Traits::SNEAK) },
    UnitTypeData { name: "IceArcher", cost: 3, max_hp: 10.0, attack: 0.0, defense: 1.0, range: 2, traits: Traits::DASH.union(Traits::FORTIFY).union(Traits::FREEZE).union(Traits::STIFF) },
    UnitTypeData { name: "IceFortress", cost: 15, max_hp: 20.0, attack: 4.0, defense: 3.0, range: 2, traits: Traits::SCOUT.union(Traits::SKATE) },
    UnitTypeData { name: "Jelly", cost: 8, max_hp: 20.0, attack: 2.0, defense: 2.0, range: 1, traits: Traits::STATIC.union(Traits::STIFF).union(Traits::TENTACLES) },
    UnitTypeData { name: "Juggernaut", cost: 10, max_hp: 40.0, attack: 4.0, defense: 4.0, range: 1, traits: Traits::CARRY.union(Traits::STATIC).union(Traits::STIFF).union(Traits::STOMP) },
    UnitTypeData { name: "Kiton", cost: 3, max_hp: 15.0, attack: 1.0, defense: 3.0, range: 1, traits: Traits::POISON },
    UnitTypeData { name: "Knight", cost: 8, max_hp: 10.0, attack: 3.5, defense: 1.0, range: 1, traits: Traits::DASH.union(Traits::PERSIST) },
    UnitTypeData { name: "MindBender", cost: 5, max_hp: 10.0, attack: 0.0, defense: 1.0, range: 1, traits: Traits::CONVERT.union(Traits::HEAL).union(Traits::STIFF) },
    UnitTypeData { name: "Mooni", cost: 5, max_hp: 10.0, attack: 0.0, defense: 1.0, range: 1, traits: Traits::AUTO_FREEZE.union(Traits::SKATE).union(Traits::STATIC).union(Traits::STIFF) },
    UnitTypeData { name: "Phychi", cost: 3, max_hp: 5.0, attack: 1.0, defense: 1.0, range: 2, traits: Traits::DASH.union(Traits::POISON).union(Traits::SURPRISE) },
    UnitTypeData { name: "Pirate", cost: 2, max_hp: 10.0, attack: 2.0, defense: 1.0, range: 1, traits: Traits::DASH.union(Traits::INDEPENDENT).union(Traits::STATIC).union(Traits::SURPRISE) },
    UnitTypeData { name: "Polytaur", cost: 3, max_hp: 15.0, attack: 3.0, defense: 1.0, range: 1, traits: Traits::DASH.union(Traits::FORTIFY).union(Traits::INDEPENDENT).union(Traits::STATIC) },
    UnitTypeData { name: "Puffer", cost: 8, max_hp: 10.0, attack: 4.0, defense: 0.0, range: 3, traits: Traits::DRENCH },
    UnitTypeData { name: "Raychi", cost: 8, max_hp: 15.0, attack: 3.0, defense: 2.0, range: 1, traits: Traits::CREEP.union(Traits::DASH).union(Traits::EXPLODE).union(Traits::NAVIGATE) },
    UnitTypeData { name: "Rider", cost: 3, max_hp: 10.0, attack: 2.0, defense: 1.0, range: 1, traits: Traits::DASH.union(Traits::ESCAPE).union(Traits::FORTIFY) },
    UnitTypeData { name: "Segment", cost: 1, max_hp: 5.0, attack: 2.0, defense: 1.5, range: 1, traits: Traits::EXPLODE.union(Traits::STATIC).union(Traits::STIFF) },
    UnitTypeData { name: "Shaman", cost: 5, max_hp: 10.0, attack: 1.0, defense: 1.0, range: 1, traits: Traits::BOOST.union(Traits::CONVERT).union(Traits::STATIC) },
    UnitTypeData { name: "Shark", cost: 8, max_hp: 10.0, attack: 3.5, defense: 2.0, range: 1, traits: Traits::DASH.union(Traits::SURPRISE) },
    UnitTypeData { name: "Swordsman", cost: 5, max_hp: 15.0, attack: 3.0, defense: 3.0, range: 1, traits: Traits::DASH },
    UnitTypeData { name: "Tridention", cost: 8, max_hp: 10.0, attack: 2.5, defense: 1.0, range: 2, traits: Traits::DASH.union(Traits::PERSIST) },
    UnitTypeData { name: "Warrior", cost: 2, max_hp: 10.0, attack: 2.0, defense: 2.0, range: 1, traits: Traits::DASH.union(Traits::FORTIFY) },
];

pub static NAVAL_TYPE_DATA: &[NavalTypeData] = &[
    NavalTypeData { name: "Bomber", cost: 15, attack: 3.0, defense: 2.0, range: 3, traits: Traits::CARRY.union(Traits::SPLASH).union(Traits::STATIC).union(Traits::STIFF) },
    NavalTypeData { name: "Raft", cost: 0, attack: 0.0, defense: 2.0, range: 0, traits: Traits::CARRY.union(Traits::STATIC).union(Traits::STIFF) },
    NavalTypeData { name: "Rammer", cost: 5, attack: 3.0, defense: 3.0, range: 1, traits: Traits::CARRY.union(Traits::DASH).union(Traits::STATIC) },
    NavalTypeData { name: "Scout", cost: 5, attack: 2.0, defense: 1.0, range: 2, traits: Traits::CARRY.union(Traits::DASH).union(Traits::SCOUT).union(Traits::STATIC) },
];
