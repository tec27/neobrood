use bevy::{ecs::component::Component, math::I16Vec2};
use bitflags::bitflags;
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::math::{bounds::IBounds, FixedPoint};

use super::{BwImage, Flingy};

/// A thing that can be constructed by a player (e.g. a unit, a building, etc.). Note that not all
/// of these are actually buildable by players in normal gameplay, it is just hard to come up with
/// a name to describe this group of stuff :)
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Construct {
    pub id: u16,
    pub flingy_id: u8,
    pub turret_type: Option<u16>,
    // NOTE(tec27): The unit data has a second sub unit type field, but none of the data actually
    // uses it so we discard it
    pub construction_image_id: u32,
    pub unit_direction: u8,
    pub shield_points: Option<FixedPoint>,
    pub hit_points: FixedPoint,
    pub elevation_level: u8,
    pub unknown_0: u8,
    pub sub_label: u8,
    pub computer_ai_idle: u8,
    pub human_ai_idle: u8,
    pub return_to_idle: u8,
    pub attack_unit: u8,
    pub attack_move: u8,
    pub ground_weapon: u8,
    pub max_ground_hits: u8,
    pub air_weapon: u8,
    pub max_air_hits: u8,
    pub ai_internal: u8,
    pub flags: ConstructFlags,
    pub target_acquisition_range: u8,
    pub sight_range: u8,
    pub armor_upgrade: u8,
    pub unit_size: u8,
    pub armor: u8,
    pub right_click_action: u8,
    pub what_sound_start: u16,
    pub what_sound_end: u16,
    pub placebox_size: I16Vec2,
    /// The space that this [Construct] takes up on the map, in logical pixels.
    pub bounds: IBounds,
    pub portrait: u16,
    pub mineral_cost: u16,
    pub vespene_cost: u16,
    pub build_time: u16,
    pub requirement_index: u16,
    pub star_edit_group_flags: u8,
    pub supply_provided: u8,
    pub supply_required: u8,
    pub space_required: u8,
    pub space_provided: u8,
    pub build_score: u16,
    pub destroy_score: u16,
    pub unit_map_string: u16,
    pub is_brood_war: bool,
    pub star_edit_availability_flag: u16,

    /// Contains data specific to the underlying kind of this [Construct] (e.g. data that pertains
    /// only to buildings).
    pub kind: ConstructKind,
}

/// Describes the kind of a [Construct] and any data specific to that kind.
#[derive(Copy, Clone, Debug)]
pub enum ConstructKind {
    Building(BuildingData),
    Unit(UnitData),
    Other,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct BuildingData {
    pub infestation: u16,
    pub addon_size: I16Vec2,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct UnitData {
    pub ready_sound: u16,
    pub piss_sound_start: u16,
    pub piss_sound_end: u16,
    pub yes_sound_start: u16,
    pub yes_sound_end: u16,
}

bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ConstructFlags: u32 {
        const BUILDING = 0x1;
        const ADDON = 0x2;
        const FLYER = 0x4;

        const WORKER = 0x8;
        const TURRET = 0x10;
        const FLYING_BUILDING = 0x20;
        const HERO = 0x40;
        const REGENERATES_HEALTH = 0x80;
        const UNKNOWN_100 = 0x100;

        const TWO_UNITS_PER_EGG = 0x400;
        const POWERUP = 0x800;
        const RESOURCE_DEPOT = 0x1_000;
        const RESOURCE = 0x2_000;
        const ROBOTIC = 0x4_000;
        const DETECTOR = 0x8_000;
        const ORGANIC = 0x10_000;
        const REQUIRES_CREEP = 0x20_000;

        const REQUIRES_PSI_POWER = 0x80_000;
        const CAN_BURROW = 0x100_000;
        const HAS_ENERGY = 0x200_000;
        const INITIALLY_CLOAKED = 0x400_000;

        const SPRITE_SIZE_MEDIUM = 0x2_000_000;
        const SPRITE_SIZE_LARGE = 0x4_000_000;

        const CAN_MOVE = 0x8_000_000;
        const CAN_TURN = 0x10_000_000;
        const INVINCIBLE = 0x20_000_000;
        const MECHANICAL = 0x40_000_000;
        // TODO(tec27): Figure out what this is for, seems like maybe PROVIDES_CREEP?
        const UNKNOWN_80000000 = 0x80_000_000;
    }
}

impl Construct {
    #[inline]
    pub const fn flingy(&self) -> &'static Flingy {
        &super::FLINGIES[self.flingy_id as usize]
    }

    #[inline]
    pub const fn construction_image(&self) -> &'static BwImage {
        &super::IMAGES[self.construction_image_id as usize]
    }

    #[inline]
    pub fn type_id(&self) -> ConstructTypeId {
        self.id.into()
    }

    /// Returns if this [Construct] is a unit.
    #[inline]
    pub const fn is_unit(&self) -> bool {
        matches!(self.kind, ConstructKind::Unit(_))
    }

    /// Returns if this [Construct] is a building.
    #[inline]
    pub const fn is_building(&self) -> bool {
        matches!(self.kind, ConstructKind::Building(_))
    }
}

/// Specifies the type (e.g. class) of a construct (i.e. marine, zergling, mineral field, etc.).
#[derive(
    Component,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    FromPrimitive,
    IntoPrimitive,
)]
#[repr(u16)]
pub enum ConstructTypeId {
    TerranMarine = 0,
    TerranGhost = 1,
    TerranVulture = 2,
    TerranGoliath = 3,
    TerranGoliathTurret = 4,
    TerranSiegeTank = 5,
    TerranSiegeTankTurret = 6,
    TerranScv = 7,
    TerranWraith = 8,
    TerranScienceVessel = 9,
    HeroGuiMontag = 10,
    TerranDropship = 11,
    TerranBattlecruiser = 12,
    TerranSpiderMine = 13,
    TerranNuclearMissile = 14,
    TerranCivilian = 15,
    HeroSarahKerrigan = 16,
    HeroAlanSchezar = 17,
    HeroAlanSchezarTurret = 18,
    HeroJimRaynorVulture = 19,
    HeroJimRaynorMarine = 20,
    HeroTomKazansky = 21,
    HeroMagellan = 22,
    HeroEdmundDuke = 23,
    HeroEdmundDukeTurret = 24,
    HeroEdmundDukeSieged = 25,
    HeroEdmundDukeSiegedTurret = 26,
    HeroArcturusMengsk = 27,
    HeroHyperion = 28,
    HeroNorad2 = 29,
    TerranSiegeTankSieged = 30,
    TerranSiegeTankSiegedTurret = 31,
    TerranFirebat = 32,
    SpellScannerSweep = 33,
    TerranMedic = 34,
    ZergLarva = 35,
    ZergEgg = 36,
    ZergZergling = 37,
    ZergHydralisk = 38,
    ZergUltralisk = 39,
    ZergBroodling = 40,
    ZergDrone = 41,
    ZergOverlord = 42,
    ZergMutalisk = 43,
    ZergGuardian = 44,
    ZergQueen = 45,
    ZergDefiler = 46,
    ZergScourge = 47,
    HeroTorrasque = 48,
    HeroMatriarch = 49,
    ZergInfestedTerran = 50,
    HeroInfestedKerrigan = 51,
    HeroUncleanOne = 52,
    HeroHunterKiller = 53,
    HeroDevouringOne = 54,
    HeroKukulzaMutalisk = 55,
    HeroKukulzaGuardian = 56,
    HeroYggdrasill = 57,
    TerranValkyrie = 58,
    ZergCocoon = 59,
    ProtossCorsair = 60,
    ProtossDarkTemplar = 61,
    ZergDevourer = 62,
    ProtossDarkArchon = 63,
    ProtossProbe = 64,
    ProtossZealot = 65,
    ProtossDragoon = 66,
    ProtossHighTemplar = 67,
    ProtossArchon = 68,
    ProtossShuttle = 69,
    ProtossScout = 70,
    ProtossArbiter = 71,
    ProtossCarrier = 72,
    ProtossInterceptor = 73,
    HeroDarkTemplar = 74,
    HeroZeratul = 75,
    HeroTassadarZeratulArchon = 76,
    HeroFenixZealot = 77,
    HeroFenixDragoon = 78,
    HeroTassadar = 79,
    HeroMojo = 80,
    HeroWarbringer = 81,
    HeroGanthritor = 82,
    ProtossReaver = 83,
    ProtossObserver = 84,
    ProtossScarab = 85,
    HeroDanimoth = 86,
    HeroAldaris = 87,
    HeroArtanis = 88,
    CritterRhynadon = 89,
    CritterBengalaas = 90,
    SpecialCargoShip = 91,
    SpecialMercenaryGunship = 92,
    CritterScantid = 93,
    CritterKakaru = 94,
    CritterRagnasaur = 95,
    CritterUrsadon = 96,
    ZergLurkerEgg = 97,
    HeroRaszagal = 98,
    HeroSamirDuran = 99,
    HeroAlexeiStukov = 100,
    SpecialMapRevealer = 101,
    HeroGerardDuGalle = 102,
    ZergLurker = 103,
    HeroInfestedDuran = 104,
    SpellDisruptionWeb = 105,
    TerranCommandCenter = 106,
    TerranComsatStation = 107,
    TerranNuclearSilo = 108,
    TerranSupplyDepot = 109,
    TerranRefinery = 110,
    TerranBarracks = 111,
    TerranAcademy = 112,
    TerranFactory = 113,
    TerranStarport = 114,
    TerranControlTower = 115,
    TerranScienceFacility = 116,
    TerranCovertOps = 117,
    TerranPhysicsLab = 118,
    UnusedTerran1 = 119,
    TerranMachineShop = 120,
    UnusedTerran2 = 121,
    TerranEngineeringBay = 122,
    TerranArmory = 123,
    TerranMissileTurret = 124,
    TerranBunker = 125,
    SpecialCrashedNorad2 = 126,
    SpecialIonCannon = 127,
    PowerupUrajCrystal = 128,
    PowerupKhalisCrystal = 129,
    ZergInfestedCommandCenter = 130,
    ZergHatchery = 131,
    ZergLair = 132,
    ZergHive = 133,
    ZergNydusCanal = 134,
    ZergHydraliskDen = 135,
    ZergDefilerMound = 136,
    ZergGreaterSpire = 137,
    ZergQueensNest = 138,
    ZergEvolutionChamber = 139,
    ZergUltraliskCavern = 140,
    ZergSpire = 141,
    ZergSpawningPool = 142,
    ZergCreepColony = 143,
    ZergSporeColony = 144,
    UnusedZerg1 = 145,
    ZergSunkenColony = 146,
    SpecialOvermindWithShell = 147,
    SpecialOvermind = 148,
    ZergExtractor = 149,
    SpecialMatureChrysalis = 150,
    SpecialCerebrate = 151,
    SpecialCerebrateDaggoth = 152,
    UnusedZerg2 = 153,
    ProtossNexus = 154,
    ProtossRoboticsFacility = 155,
    ProtossPylon = 156,
    ProtossAssimilator = 157,
    UnusedProtoss1 = 158,
    ProtossObservatory = 159,
    ProtossGateway = 160,
    UnusedProtoss2 = 161,
    ProtossPhotonCannon = 162,
    ProtossCitatdelOfAdun = 163,
    ProtossCyberneticsCore = 164,
    ProtossTemplarArchives = 165,
    ProtossForge = 166,
    ProtossStargate = 167,
    SpecialStasisCellPrison = 168,
    ProtossFleetBeacon = 169,
    ProtossArbiterTribunal = 170,
    ProtossRoboticsSupportBay = 171,
    ProtossShieldBattery = 172,
    SpecialKhaydarinCrystalForm = 173,
    SpecialProtossTemple = 174,
    SpecialXelNagaTemple = 175,
    ResourceMineralField1 = 176,
    ResourceMineralField2 = 177,
    ResourceMineralField3 = 178,
    UnusedCave = 179,
    UnusedCaveIn = 180,
    UnusedCantina = 181,
    UnusedMiningPlatform = 182,
    UnusedIndependentCommandCenter = 183,
    SpecialIndependentStarport = 184,
    UnusedIndependentJumpGate = 185,
    UnusedRuins = 186,
    UnusedKhaydarinCrystalFormation = 187,
    ResourceVespeneGeyser = 188,
    SpecialWarpGate = 189,
    SpecialPsiDisrupter = 190,
    UnusedZergMarker = 191,
    UnusedTerranMarker = 192,
    UnusedProtossMarker = 193,
    SpecialZergBeacon = 194,
    SpecialTerranBeacon = 195,
    SpecialProtossBeacon = 196,
    SpecialZergFlagBeacon = 197,
    SpecialTerranFlagBeacon = 198,
    SpecialProtossFlagBeacon = 199,
    SpecialPowerGenerator = 200,
    SpecialOvermindCocoon = 201,
    SpecialDarkSwarm = 202,
    SpecialFloorMissileTrap = 203,
    SpecialFloorHatch = 204,
    SpecialUpperLevelDoor = 205,
    SpecialRightUpperLevelDoor = 206,
    SpecialPitDoor = 207,
    SpecialRightPitDoor = 208,
    SpecialFloorGunTrap = 209,
    SpecialWallMissileTrap = 210,
    SpecialWallFlameTrap = 211,
    SpecialRightWallMissileTrap = 212,
    SpecialRightWallFlameTrap = 213,
    StartLocation = 214,
    PowerupFlag = 215,
    PowerupYoungChrysalis = 216,
    PowerupPsiEmitter = 217,
    PowerupDataDisk = 218,
    PowerupKhaydarinCrystal = 219,
    PowerupMineralCluster1 = 220,
    PowerupMineralCluster2 = 221,
    PowerupProtossGasOrb1 = 222,
    PowerupProtossGasOrb2 = 223,
    PowerupZergGasSac1 = 224,
    PowerupZergGasSac2 = 225,
    PowerupTerranGasTank1 = 226,
    PowerupTerranGasTank2 = 227,
    #[num_enum(catch_all)]
    Unknown(u16),
}

impl Default for ConstructTypeId {
    #[inline]
    fn default() -> Self {
        ConstructTypeId::Unknown(666)
    }
}
