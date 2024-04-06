use crate::gamedata::{
    FlipState, FrameId, FrameIdByte, FrameSet, GasOverlay, ImageId, IscriptCollection,
    IscriptCommand as IC, IscriptId, IscriptLabel, OverlayId, SignalId, SoundId, Speed, SpriteId,
    WeaponId, WeaponType,
};
use std::num::NonZeroU16;
/// Contains all the iscript collections referenced by images in the game.
pub const ISCRIPTS: [IscriptCollection<'static>; 412usize] = unsafe {
    [
        IscriptCollection {
            id: 0u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(789u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(549u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(369u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(369u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(74u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1198u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(74u16))),
            ],
        },
        IscriptCollection {
            id: 1u16,
            scripts: &[],
        },
        IscriptCollection {
            id: 2u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(647u16)))],
        },
        IscriptCollection {
            id: 3u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(532u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 4u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1177u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(779u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(262u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(262u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(55u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(195u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(55u16))),
            ],
        },
        IscriptCollection {
            id: 5u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(1010u16)))],
        },
        IscriptCollection {
            id: 6u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(622u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1202u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(185u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(685u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(636u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(842u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(951u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(719u16))),
            ],
        },
        IscriptCollection {
            id: 7u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(712u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 8u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(974u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(375u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(703u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1197u16))),
            ],
        },
        IscriptCollection {
            id: 9u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(820u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1132u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(302u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(302u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(505u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(21u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(346u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(21u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(418u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1109u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1059u16))),
            ],
        },
        IscriptCollection {
            id: 10u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(642u16)))],
        },
        IscriptCollection {
            id: 11u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1037u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1076u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(269u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(269u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(348u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(499u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(355u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(348u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(743u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1187u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(965u16))),
            ],
        },
        IscriptCollection {
            id: 12u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(1073u16)))],
        },
        IscriptCollection {
            id: 13u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(686u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(785u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(765u16))),
            ],
        },
        IscriptCollection {
            id: 14u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(479u16)))],
        },
        IscriptCollection {
            id: 15u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(799u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1021u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(77u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(77u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(77u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(77u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(31u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(31u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(356u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(31u16))),
            ],
        },
        IscriptCollection {
            id: 16u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(480u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(375u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1173u16))),
            ],
        },
        IscriptCollection {
            id: 17u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(867u16)))],
        },
        IscriptCollection {
            id: 18u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1007u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(648u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(410u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(410u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(413u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(413u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(373u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(373u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(265u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(14u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(687u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(819u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(770u16))),
            ],
        },
        IscriptCollection {
            id: 19u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(711u16)))],
        },
        IscriptCollection {
            id: 20u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(793u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(769u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(327u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(327u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(221u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(221u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(761u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(377u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(377u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(218u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(56u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(856u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(921u16))),
            ],
        },
        IscriptCollection {
            id: 21u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(231u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(892u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(194u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(231u16))),
            ],
        },
        IscriptCollection {
            id: 22u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(491u16)))],
        },
        IscriptCollection {
            id: 23u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(800u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(751u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(72u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(72u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(72u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(72u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(30u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(30u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(233u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(30u16))),
            ],
        },
        IscriptCollection {
            id: 24u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(999u16)))],
        },
        IscriptCollection {
            id: 25u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(826u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(938u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1064u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(536u16))),
            ],
        },
        IscriptCollection {
            id: 26u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(946u16)))],
        },
        IscriptCollection {
            id: 27u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(774u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(997u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(85u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(85u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(85u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(85u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1003u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(20u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(20u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(307u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(20u16))),
            ],
        },
        IscriptCollection {
            id: 28u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(651u16)))],
        },
        IscriptCollection {
            id: 29u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(671u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(827u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1191u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1184u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(83u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(268u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(83u16))),
            ],
        },
        IscriptCollection {
            id: 30u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(662u16)))],
        },
        IscriptCollection {
            id: 31u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(567u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(450u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(212u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(212u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(73u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(259u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(73u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(833u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(804u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(602u16))),
            ],
        },
        IscriptCollection {
            id: 32u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(1062u16)))],
        },
        IscriptCollection {
            id: 33u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(816u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 34u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(987u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 35u16,
            scripts: &[],
        },
        IscriptCollection {
            id: 36u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(617u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1122u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(562u16))),
            ],
        },
        IscriptCollection {
            id: 37u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(560u16)))],
        },
        IscriptCollection {
            id: 38u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(896u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(835u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1189u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(45u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(45u16))),
            ],
        },
        IscriptCollection {
            id: 39u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1094u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(82u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1148u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(361u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(923u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1131u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(556u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(361u16))),
            ],
        },
        IscriptCollection {
            id: 40u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1186u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(839u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(184u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(79u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(79u16))),
            ],
        },
        IscriptCollection {
            id: 41u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1080u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(891u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(476u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(38u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(38u16))),
            ],
        },
        IscriptCollection {
            id: 42u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(658u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1101u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1170u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 43u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(590u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(19u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1136u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(633u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(48u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(48u16))),
            ],
        },
        IscriptCollection {
            id: 44u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(464u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(19u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1144u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(776u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(53u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(53u16))),
            ],
        },
        IscriptCollection {
            id: 45u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(731u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(19u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(539u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(817u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(59u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(59u16))),
            ],
        },
        IscriptCollection {
            id: 46u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(424u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(340u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(340u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(916u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(795u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(830u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(9u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(9u16))),
            ],
        },
        IscriptCollection {
            id: 47u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(983u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(516u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(220u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(63u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(63u16))),
            ],
        },
        IscriptCollection {
            id: 48u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(967u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(669u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(522u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(60u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(60u16))),
            ],
        },
        IscriptCollection {
            id: 49u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(913u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(959u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(846u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(46u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(46u16))),
            ],
        },
        IscriptCollection {
            id: 50u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(521u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(469u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1089u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(52u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(52u16))),
            ],
        },
        IscriptCollection {
            id: 51u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(640u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(436u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(752u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(54u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(54u16))),
            ],
        },
        IscriptCollection {
            id: 52u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(484u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(949u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(981u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(182u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(69u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(69u16))),
            ],
        },
        IscriptCollection {
            id: 53u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(877u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(19u16))),
            ],
        },
        IscriptCollection {
            id: 54u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(780u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(19u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(766u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(209u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(62u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(62u16))),
            ],
        },
        IscriptCollection {
            id: 55u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1154u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(504u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(966u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(36u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(36u16))),
            ],
        },
        IscriptCollection {
            id: 56u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(489u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1193u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(564u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1050u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(51u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(51u16))),
            ],
        },
        IscriptCollection {
            id: 57u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(525u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(943u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(470u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(58u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(58u16))),
            ],
        },
        IscriptCollection {
            id: 58u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(735u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(626u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(591u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(39u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(39u16))),
            ],
        },
        IscriptCollection {
            id: 59u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(631u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(41u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(41u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(28u16))),
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(726u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(864u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(28u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(28u16))),
            ],
        },
        IscriptCollection {
            id: 60u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(213u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(545u16))),
            ],
        },
        IscriptCollection {
            id: 61u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(677u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(76u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(76u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(434u16))),
            ],
        },
        IscriptCollection {
            id: 62u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(637u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(5u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(117u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(117u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(527u16))),
            ],
        },
        IscriptCollection {
            id: 63u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(478u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(19u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(365u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(40u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(232u16))),
            ],
        },
        IscriptCollection {
            id: 64u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(841u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1185u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(40u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(40u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(232u16))),
            ],
        },
        IscriptCollection {
            id: 65u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(518u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1119u16))),
            ],
        },
        IscriptCollection {
            id: 66u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(764u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(680u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(289u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(250u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(289u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(250u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(659u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(745u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(3u16))),
            ],
        },
        IscriptCollection {
            id: 67u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(477u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(995u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(206u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(242u16))),
            ],
        },
        IscriptCollection {
            id: 68u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(698u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(847u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(665u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(3u16))),
            ],
        },
        IscriptCollection {
            id: 69u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(960u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1009u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(548u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(873u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(614u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(306u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(27u16))),
            ],
        },
        IscriptCollection {
            id: 70u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1178u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(905u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(211u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(211u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(382u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(382u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(794u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(149u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(149u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(252u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(25u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1097u16))),
            ],
        },
        IscriptCollection {
            id: 71u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(1036u16)))],
        },
        IscriptCollection {
            id: 72u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(823u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 73u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(200u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(12u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(12u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(12u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(12u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(12u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(12u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(12u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(12u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(12u16))),
            ],
        },
        IscriptCollection {
            id: 74u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(191u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(572u16))),
            ],
        },
        IscriptCollection {
            id: 75u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(453u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1195u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(313u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(534u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(739u16))),
            ],
        },
        IscriptCollection {
            id: 76u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(34u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(214u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(283u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(214u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(283u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(34u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(34u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(370u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(34u16))),
            ],
        },
        IscriptCollection {
            id: 77u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1011u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1024u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(325u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(325u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(343u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(343u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(542u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(137u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(137u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(296u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(24u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(953u16))),
            ],
        },
        IscriptCollection {
            id: 78u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1176u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(493u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(173u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(173u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(317u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(317u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(277u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(277u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(229u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(26u16))),
            ],
        },
        IscriptCollection {
            id: 79u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(786u16)))],
        },
        IscriptCollection {
            id: 80u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1156u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 81u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1013u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1172u16))),
            ],
        },
        IscriptCollection {
            id: 82u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(937u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(844u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(11u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(41u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(11u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(41u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(3u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(3u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1069u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(3u16))),
            ],
        },
        IscriptCollection {
            id: 83u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(131u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(131u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 84u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(815u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1025u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(249u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(249u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(438u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(490u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(982u16))),
            ],
        },
        IscriptCollection {
            id: 85u16,
            scripts: &[],
        },
        IscriptCollection {
            id: 86u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1027u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(911u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(11u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(11u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1102u16))),
            ],
        },
        IscriptCollection {
            id: 87u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(519u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(840u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(401u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(303u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(612u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(947u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(577u16))),
            ],
        },
        IscriptCollection {
            id: 88u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(661u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(533u16))),
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1075u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 89u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(940u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 90u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1125u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1158u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(406u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1060u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1171u16))),
            ],
        },
        IscriptCollection {
            id: 91u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(471u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(372u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(372u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 92u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(834u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1159u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(783u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(635u16))),
            ],
        },
        IscriptCollection {
            id: 93u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(866u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(204u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(204u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(917u16))),
            ],
        },
        IscriptCollection {
            id: 94u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1194u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(748u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(287u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(509u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(287u16))),
            ],
        },
        IscriptCollection {
            id: 95u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1033u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(49u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(49u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(49u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(113u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(49u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(49u16))),
            ],
        },
        IscriptCollection {
            id: 96u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(416u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(82u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(797u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1079u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(320u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(858u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1126u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(314u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(320u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1099u16))),
            ],
        },
        IscriptCollection {
            id: 97u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(644u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(822u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(338u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(863u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(338u16))),
            ],
        },
        IscriptCollection {
            id: 98u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(114u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(43u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(43u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(43u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(114u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(43u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(43u16))),
            ],
        },
        IscriptCollection {
            id: 99u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(650u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(571u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(501u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(449u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(851u16))),
            ],
        },
        IscriptCollection {
            id: 100u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(432u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(225u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(225u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1150u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(634u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 101u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(713u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1166u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1045u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(89u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(89u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 102u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1093u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(82u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(517u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(749u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(345u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(554u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(883u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(831u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(345u16))),
            ],
        },
        IscriptCollection {
            id: 103u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(112u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(18u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(18u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(18u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(18u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(18u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(18u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(112u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(18u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(18u16))),
            ],
        },
        IscriptCollection {
            id: 104u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(872u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(859u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 105u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1082u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(771u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(454u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(744u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(727u16))),
            ],
        },
        IscriptCollection {
            id: 106u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(64u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1135u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(64u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(64u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 107u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(714u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(656u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(357u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1004u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1019u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(357u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1087u16))),
            ],
        },
        IscriptCollection {
            id: 108u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(944u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(196u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(196u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(428u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(318u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(318u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 109u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(628u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(44u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(44u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(44u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(135u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(44u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(44u16))),
            ],
        },
        IscriptCollection {
            id: 110u16,
            scripts: &[],
        },
        IscriptCollection {
            id: 111u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(927u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(968u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1138u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(166u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(879u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(422u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1175u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(166u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(497u16))),
            ],
        },
        IscriptCollection {
            id: 112u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(94u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(15u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(15u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(15u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(15u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(15u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(15u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(94u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(15u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(15u16))),
            ],
        },
        IscriptCollection {
            id: 113u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(608u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1095u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(321u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(605u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(540u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(321u16))),
            ],
        },
        IscriptCollection {
            id: 114u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(754u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(299u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(299u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(429u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(553u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 115u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(461u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(50u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(50u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(50u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(153u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(50u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(50u16))),
            ],
        },
        IscriptCollection {
            id: 116u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(285u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(285u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 117u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1005u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(417u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(189u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(613u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(362u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(189u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1055u16))),
            ],
        },
        IscriptCollection {
            id: 118u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(443u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(207u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(207u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1020u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(552u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 119u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(901u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(986u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(736u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1174u16))),
            ],
        },
        IscriptCollection {
            id: 120u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(814u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(906u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(68u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(68u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(68u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(68u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(245u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(245u16))),
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(23u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(23u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(23u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 121u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(918u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(583u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(129u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(962u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(129u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(172u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(129u16))),
            ],
        },
        IscriptCollection {
            id: 122u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(440u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(311u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(311u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(523u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(565u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 123u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(675u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(767u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1023u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 124u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(750u16)))],
        },
        IscriptCollection {
            id: 125u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1124u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(958u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(621u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(710u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(902u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(368u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(368u16))),
            ],
        },
        IscriptCollection {
            id: 126u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(730u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1084u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(909u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(332u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(919u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(441u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(692u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(332u16))),
            ],
        },
        IscriptCollection {
            id: 127u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1139u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(16u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(16u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(16u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(16u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(16u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(16u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(120u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(16u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(16u16))),
            ],
        },
        IscriptCollection {
            id: 128u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(609u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1103u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(412u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(758u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(660u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(412u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(586u16))),
            ],
        },
        IscriptCollection {
            id: 129u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(992u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(392u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(392u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1130u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1038u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 130u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(681u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(47u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(47u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(47u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(143u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(47u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(47u16))),
            ],
        },
        IscriptCollection {
            id: 131u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(715u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(632u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(806u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1040u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(520u16))),
            ],
        },
        IscriptCollection {
            id: 132u16,
            scripts: &[],
        },
        IscriptCollection {
            id: 133u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(875u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 134u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(528u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(82u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1057u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(588u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(183u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(603u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(452u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(832u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(183u16))),
            ],
        },
        IscriptCollection {
            id: 135u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(22u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(86u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(22u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(22u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(86u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(22u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(22u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(22u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(86u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(86u16))),
            ],
        },
        IscriptCollection {
            id: 136u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(683u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(625u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(511u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(915u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1006u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(558u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(482u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(865u16))),
            ],
        },
        IscriptCollection {
            id: 137u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(118u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(179u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(179u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(315u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(118u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(118u16))),
            ],
        },
        IscriptCollection {
            id: 138u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(934u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(595u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(796u16))),
            ],
        },
        IscriptCollection {
            id: 139u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(802u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(843u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1022u16))),
            ],
        },
        IscriptCollection {
            id: 140u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(403u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(708u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(585u16))),
            ],
        },
        IscriptCollection {
            id: 141u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(403u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 142u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(458u16)))],
        },
        IscriptCollection {
            id: 143u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(1047u16)))],
        },
        IscriptCollection {
            id: 144u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(1012u16)))],
        },
        IscriptCollection {
            id: 145u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(876u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 146u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1090u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(513u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(404u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(404u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(202u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(202u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(537u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(3u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(3u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(890u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(3u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(957u16))),
            ],
        },
        IscriptCollection {
            id: 147u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1152u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(597u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(8u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(8u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(8u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(8u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(8u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(8u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(8u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(8u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(600u16))),
            ],
        },
        IscriptCollection {
            id: 148u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1120u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(71u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(71u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(71u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(71u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(13u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(13u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(13u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(13u16))),
            ],
        },
        IscriptCollection {
            id: 149u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1155u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 150u16,
            scripts: &[],
        },
        IscriptCollection {
            id: 151u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(984u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(759u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1147u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(3u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(472u16))),
            ],
        },
        IscriptCollection {
            id: 152u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(88u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(529u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(216u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(216u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1041u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(257u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(88u16))),
            ],
        },
        IscriptCollection {
            id: 153u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(646u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(578u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(341u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(341u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(351u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(351u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(388u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(388u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(414u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(65u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1201u16))),
            ],
        },
        IscriptCollection {
            id: 154u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(952u16)))],
        },
        IscriptCollection {
            id: 155u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1058u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(926u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(91u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(91u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(91u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(91u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(704u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(3u16))),
            ],
        },
        IscriptCollection {
            id: 156u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(507u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(455u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(11u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(11u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(546u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(400u16))),
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(423u16))),
            ],
        },
        IscriptCollection {
            id: 157u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(907u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(467u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(550u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(973u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(526u16))),
            ],
        },
        IscriptCollection {
            id: 158u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(733u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(971u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(312u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(312u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(705u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(61u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(421u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(772u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(963u16))),
            ],
        },
        IscriptCollection {
            id: 159u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1083u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(641u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(90u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(90u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(90u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(90u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(364u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(364u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(248u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(98u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(538u16))),
            ],
        },
        IscriptCollection {
            id: 160u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(398u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(964u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(291u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(398u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1188u16))),
            ],
        },
        IscriptCollection {
            id: 161u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(781u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(746u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(324u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(336u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(324u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(336u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(294u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(294u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1098u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(3u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(463u16))),
            ],
        },
        IscriptCollection {
            id: 162u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(144u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(144u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 163u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(805u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(976u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(870u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(860u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1118u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(263u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(67u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(869u16))),
            ],
        },
        IscriptCollection {
            id: 164u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(998u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 165u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1061u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1149u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(601u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(3u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1181u16))),
            ],
        },
        IscriptCollection {
            id: 166u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(933u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(6u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(667u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 167u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(502u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1053u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(57u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(57u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(57u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(457u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 168u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1113u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(6u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1051u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 169u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(37u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(37u16))),
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(37u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(37u16))),
            ],
        },
        IscriptCollection {
            id: 170u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(994u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(6u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(707u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 171u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(611u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(6u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(721u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(678u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 172u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(630u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(208u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 173u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(884u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(6u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(747u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 174u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(643u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(6u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(445u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1043u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 175u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(809u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(227u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 176u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(360u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(360u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 177u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(868u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(80u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1052u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1129u16))),
            ],
        },
        IscriptCollection {
            id: 178u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(199u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(223u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(199u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 179u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1016u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(6u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(352u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(352u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(230u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(230u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(282u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(282u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(755u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 180u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(777u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(6u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(950u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 181u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1091u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(473u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1034u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1017u16))),
            ],
        },
        IscriptCollection {
            id: 182u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(825u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(6u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(690u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(888u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 183u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1063u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(845u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(390u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(697u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(390u16))),
            ],
        },
        IscriptCollection {
            id: 184u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(397u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(170u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(397u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 185u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1163u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(6u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(402u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1028u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(402u16))),
            ],
        },
        IscriptCollection {
            id: 186u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(174u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(354u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(174u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 187u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(376u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(6u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(376u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 188u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(810u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(6u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(596u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 189u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(236u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(80u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(236u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 190u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(679u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(6u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(775u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 191u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(494u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(563u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(980u16))),
            ],
        },
        IscriptCollection {
            id: 192u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(427u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(763u16))),
            ],
        },
        IscriptCollection {
            id: 193u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1165u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(459u16))),
            ],
        },
        IscriptCollection {
            id: 194u16,
            scripts: &[],
        },
        IscriptCollection {
            id: 195u16,
            scripts: &[],
        },
        IscriptCollection {
            id: 196u16,
            scripts: &[],
        },
        IscriptCollection {
            id: 197u16,
            scripts: &[],
        },
        IscriptCollection {
            id: 198u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1192u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(699u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(270u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(788u16))),
            ],
        },
        IscriptCollection {
            id: 199u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(594u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1002u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(333u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1035u16))),
            ],
        },
        IscriptCollection {
            id: 200u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(908u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(773u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(389u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(544u16))),
            ],
        },
        IscriptCollection {
            id: 201u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1140u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(276u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(33u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(854u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(33u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(33u16))),
            ],
        },
        IscriptCollection {
            id: 202u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(689u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(276u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 203u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(623u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 204u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(787u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(92u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(128u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(148u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(17u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(17u16))),
            ],
        },
        IscriptCollection {
            id: 205u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(569u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(92u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(128u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(148u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(17u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(17u16))),
            ],
        },
        IscriptCollection {
            id: 206u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1042u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(92u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(128u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(148u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(17u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(17u16))),
            ],
        },
        IscriptCollection {
            id: 207u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(316u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(818u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(316u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 208u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(264u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 209u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(286u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 210u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(244u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 211u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1066u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 212u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(850u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 213u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(955u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 214u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1065u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1190u16))),
            ],
        },
        IscriptCollection {
            id: 215u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(574u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 216u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(415u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(639u16))),
            ],
        },
        IscriptCollection {
            id: 217u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(475u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(474u16))),
            ],
        },
        IscriptCollection {
            id: 218u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1092u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(568u16))),
            ],
        },
        IscriptCollection {
            id: 219u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1133u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(737u16))),
            ],
        },
        IscriptCollection {
            id: 220u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(506u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1072u16))),
            ],
        },
        IscriptCollection {
            id: 221u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(186u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(203u16))),
            ],
        },
        IscriptCollection {
            id: 222u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(186u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(203u16))),
            ],
        },
        IscriptCollection {
            id: 223u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(580u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(239u16))),
            ],
        },
        IscriptCollection {
            id: 224u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(442u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(239u16))),
            ],
        },
        IscriptCollection {
            id: 225u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(861u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(386u16))),
            ],
        },
        IscriptCollection {
            id: 226u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(977u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(386u16))),
            ],
        },
        IscriptCollection {
            id: 227u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1014u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(384u16))),
            ],
        },
        IscriptCollection {
            id: 228u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1054u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(384u16))),
            ],
        },
        IscriptCollection {
            id: 229u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(620u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(295u16))),
            ],
        },
        IscriptCollection {
            id: 230u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1008u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(310u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(310u16))),
            ],
        },
        IscriptCollection {
            id: 231u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(606u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(378u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(378u16))),
            ],
        },
        IscriptCollection {
            id: 232u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(425u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(555u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(160u16))),
            ],
        },
        IscriptCollection {
            id: 233u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(673u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1128u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(124u16))),
            ],
        },
        IscriptCollection {
            id: 234u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(889u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(813u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(105u16))),
            ],
        },
        IscriptCollection {
            id: 235u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1127u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(465u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(104u16))),
            ],
        },
        IscriptCollection {
            id: 236u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(589u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(279u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(279u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(547u16))),
            ],
        },
        IscriptCollection {
            id: 237u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1077u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(178u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(178u16))),
            ],
        },
        IscriptCollection {
            id: 238u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(837u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(309u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(309u16))),
            ],
        },
        IscriptCollection {
            id: 239u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1111u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1169u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(152u16))),
            ],
        },
        IscriptCollection {
            id: 240u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(925u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(904u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(125u16))),
            ],
        },
        IscriptCollection {
            id: 241u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(897u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(177u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(177u16))),
            ],
        },
        IscriptCollection {
            id: 242u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(930u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(485u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(996u16))),
            ],
        },
        IscriptCollection {
            id: 243u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1078u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(717u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 244u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(790u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1071u16))),
            ],
        },
        IscriptCollection {
            id: 245u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(512u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(948u16))),
            ],
        },
        IscriptCollection {
            id: 246u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1110u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(394u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(394u16))),
            ],
        },
        IscriptCollection {
            id: 247u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(778u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(495u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(138u16))),
            ],
        },
        IscriptCollection {
            id: 248u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(676u16)))],
        },
        IscriptCollection {
            id: 249u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(616u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1108u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(156u16))),
            ],
        },
        IscriptCollection {
            id: 250u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(782u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 251u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1145u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 252u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(649u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(598u16))),
            ],
        },
        IscriptCollection {
            id: 253u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1134u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1115u16))),
            ],
        },
        IscriptCollection {
            id: 254u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(900u16)))],
        },
        IscriptCollection {
            id: 255u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(972u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(260u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(260u16))),
            ],
        },
        IscriptCollection {
            id: 256u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(672u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(405u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(405u16))),
            ],
        },
        IscriptCollection {
            id: 257u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(931u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(446u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 258u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1153u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(205u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(205u16))),
            ],
        },
        IscriptCollection {
            id: 259u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(801u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 260u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1112u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(910u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(99u16))),
            ],
        },
        IscriptCollection {
            id: 261u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1048u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 262u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(880u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 263u16,
            scripts: &[],
        },
        IscriptCollection {
            id: 264u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1074u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(593u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(84u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(784u16))),
            ],
        },
        IscriptCollection {
            id: 265u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(829u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 266u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(768u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1105u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(122u16))),
            ],
        },
        IscriptCollection {
            id: 267u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(895u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(881u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(807u16))),
            ],
        },
        IscriptCollection {
            id: 268u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(803u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(171u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(171u16))),
            ],
        },
        IscriptCollection {
            id: 269u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(824u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 270u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(836u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 271u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1056u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(939u16))),
            ],
        },
        IscriptCollection {
            id: 272u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(559u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(936u16))),
            ],
        },
        IscriptCollection {
            id: 273u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(969u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 274u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(460u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 275u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(393u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 276u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(756u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1039u16))),
            ],
        },
        IscriptCollection {
            id: 277u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1085u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 278u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(724u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 279u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(852u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1179u16))),
            ],
        },
        IscriptCollection {
            id: 280u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(488u16)))],
        },
        IscriptCollection {
            id: 281u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(945u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 282u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1168u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 283u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(853u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 284u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(448u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 285u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(508u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 286u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1146u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 287u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(515u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 288u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(514u16))),
            ],
        },
        IscriptCollection {
            id: 289u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 290u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(142u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(142u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 291u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(147u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(147u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 292u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(792u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(175u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(175u16))),
            ],
        },
        IscriptCollection {
            id: 293u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(688u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(961u16))),
            ],
        },
        IscriptCollection {
            id: 294u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1116u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(486u16))),
            ],
        },
        IscriptCollection {
            id: 295u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(684u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(903u16))),
            ],
        },
        IscriptCollection {
            id: 296u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1121u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(281u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(281u16))),
            ],
        },
        IscriptCollection {
            id: 297u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(928u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 298u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(492u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(293u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(293u16))),
            ],
        },
        IscriptCollection {
            id: 299u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1141u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(653u16))),
            ],
        },
        IscriptCollection {
            id: 300u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(366u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 301u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(709u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(738u16))),
            ],
        },
        IscriptCollection {
            id: 302u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(706u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 303u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1123u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 304u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(330u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 305u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1100u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 306u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(760u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(618u16))),
            ],
        },
        IscriptCollection {
            id: 307u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(808u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 308u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(524u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 309u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(411u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 310u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(922u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 311u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(942u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 312u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(693u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(371u16))),
            ],
        },
        IscriptCollection {
            id: 313u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(993u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 314u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1031u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(849u16))),
            ],
        },
        IscriptCollection {
            id: 315u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(975u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 316u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(664u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 317u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(541u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 318u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(855u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 319u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(722u16)))],
        },
        IscriptCollection {
            id: 320u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(535u16)))],
        },
        IscriptCollection {
            id: 321u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(1046u16)))],
        },
        IscriptCollection {
            id: 322u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(151u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(304u16))),
            ],
        },
        IscriptCollection {
            id: 323u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(543u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(304u16))),
            ],
        },
        IscriptCollection {
            id: 324u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(109u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(253u16))),
            ],
        },
        IscriptCollection {
            id: 325u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(439u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(253u16))),
            ],
        },
        IscriptCollection {
            id: 326u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(127u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(359u16))),
            ],
        },
        IscriptCollection {
            id: 327u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(932u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(359u16))),
            ],
        },
        IscriptCollection {
            id: 328u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(101u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(238u16))),
            ],
        },
        IscriptCollection {
            id: 329u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(573u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(238u16))),
            ],
        },
        IscriptCollection {
            id: 330u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(762u16)))],
        },
        IscriptCollection {
            id: 331u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(954u16)))],
        },
        IscriptCollection {
            id: 332u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1200u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(430u16))),
            ],
        },
        IscriptCollection {
            id: 333u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1199u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(627u16))),
            ],
        },
        IscriptCollection {
            id: 334u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(886u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(420u16))),
            ],
        },
        IscriptCollection {
            id: 335u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(663u16)))],
        },
        IscriptCollection {
            id: 336u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(1180u16)))],
        },
        IscriptCollection {
            id: 337u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(503u16)))],
        },
        IscriptCollection {
            id: 338u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(682u16)))],
        },
        IscriptCollection {
            id: 339u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(1032u16)))],
        },
        IscriptCollection {
            id: 340u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1088u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(935u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(240u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1030u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(240u16))),
            ],
        },
        IscriptCollection {
            id: 341u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(587u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(561u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(383u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(579u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(383u16))),
            ],
        },
        IscriptCollection {
            id: 342u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(353u16)))],
        },
        IscriptCollection {
            id: 343u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(1070u16)))],
        },
        IscriptCollection {
            id: 344u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(300u16)))],
        },
        IscriptCollection {
            id: 345u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(498u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(655u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(168u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(168u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(920u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(322u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(433u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(322u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 346u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(812u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(335u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(335u16))),
            ],
        },
        IscriptCollection {
            id: 347u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(462u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(666u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(190u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(190u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(728u16))),
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(339u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(198u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(339u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(198u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 348u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(798u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(694u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(246u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(246u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(222u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(290u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(222u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(290u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 349u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(610u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(718u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(374u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(374u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(298u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(358u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(298u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(358u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 350u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(551u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(991u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(188u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(188u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(344u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(254u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(344u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(254u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 351u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(435u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(734u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(280u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(280u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(396u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(305u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(396u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(305u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 352u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(912u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(742u16))),
            ],
        },
        IscriptCollection {
            id: 353u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(828u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(592u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1183u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1015u16))),
            ],
        },
        IscriptCollection {
            id: 354u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(874u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(970u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(334u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(334u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(385u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(385u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(180u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(180u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(399u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(75u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(885u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(468u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(500u16))),
            ],
        },
        IscriptCollection {
            id: 355u16,
            scripts: &[],
        },
        IscriptCollection {
            id: 356u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(557u16)))],
        },
        IscriptCollection {
            id: 357u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(871u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(956u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(78u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(78u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(78u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(78u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(32u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(32u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(241u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(32u16))),
            ],
        },
        IscriptCollection {
            id: 358u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1196u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(857u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1142u16))),
            ],
        },
        IscriptCollection {
            id: 359u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(437u16)))],
        },
        IscriptCollection {
            id: 360u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(811u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1018u16))),
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(716u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(255u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1151u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1106u16))),
            ],
        },
        IscriptCollection {
            id: 361u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(1107u16)))],
        },
        IscriptCollection {
            id: 362u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1029u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(624u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(11u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(267u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(11u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(267u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(3u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(3u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(893u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(691u16))),
            ],
        },
        IscriptCollection {
            id: 363u16,
            scripts: &[],
        },
        IscriptCollection {
            id: 364u16,
            scripts: &[],
        },
        IscriptCollection {
            id: 365u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(740u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(695u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(7u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(7u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(7u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(7u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(670u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(7u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(7u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(7u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(7u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(570u16))),
            ],
        },
        IscriptCollection {
            id: 366u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(821u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(87u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(87u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(87u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(87u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1162u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(10u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(10u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(10u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(10u16))),
            ],
        },
        IscriptCollection {
            id: 367u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(674u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 368u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(791u16)))],
        },
        IscriptCollection {
            id: 369u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(576u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(702u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(337u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(235u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(337u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(235u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(481u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(35u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(35u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(757u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(35u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(615u16))),
            ],
        },
        IscriptCollection {
            id: 370u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(119u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(119u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 371u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(451u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1157u16))),
            ],
        },
        IscriptCollection {
            id: 372u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(70u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(899u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(167u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(167u16))),
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1044u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(201u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(70u16))),
            ],
        },
        IscriptCollection {
            id: 373u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1137u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(326u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(326u16))),
            ],
        },
        IscriptCollection {
            id: 374u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(878u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(732u16))),
            ],
        },
        IscriptCollection {
            id: 375u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(426u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(741u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(217u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(607u16))),
            ],
        },
        IscriptCollection {
            id: 376u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(989u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(988u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(319u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(163u16))),
            ],
        },
        IscriptCollection {
            id: 377u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1182u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1081u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(165u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1143u16))),
            ],
        },
        IscriptCollection {
            id: 378u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(645u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(483u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(97u16))),
            ],
        },
        IscriptCollection {
            id: 379u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(701u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1161u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(582u16))),
            ],
        },
        IscriptCollection {
            id: 380u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(581u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 381u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1067u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(487u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(723u16))),
            ],
        },
        IscriptCollection {
            id: 382u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1096u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(530u16))),
            ],
        },
        IscriptCollection {
            id: 383u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(599u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 384u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(838u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 385u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(700u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 386u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(985u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 387u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(990u16)))],
        },
        IscriptCollection {
            id: 388u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(381u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 389u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1086u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 390u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1167u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 391u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1068u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 392u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(510u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 393u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(266u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 394u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(720u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 395u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1117u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1000u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(164u16))),
            ],
        },
        IscriptCollection {
            id: 396u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(848u16)))],
        },
        IscriptCollection {
            id: 397u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(753u16)))],
        },
        IscriptCollection {
            id: 398u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(729u16)))],
        },
        IscriptCollection {
            id: 399u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(898u16)))],
        },
        IscriptCollection {
            id: 400u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(725u16)))],
        },
        IscriptCollection {
            id: 401u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(1104u16)))],
        },
        IscriptCollection {
            id: 402u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(575u16)))],
        },
        IscriptCollection {
            id: 403u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(894u16)))],
        },
        IscriptCollection {
            id: 404u16,
            scripts: &[Some(IscriptId(NonZeroU16::new_unchecked(657u16)))],
        },
        IscriptCollection {
            id: 405u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(979u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(19u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(629u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(284u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(81u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(81u16))),
            ],
        },
        IscriptCollection {
            id: 406u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(654u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(80u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(882u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(349u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(126u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(126u16))),
            ],
        },
        IscriptCollection {
            id: 407u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(496u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1160u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 408u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(431u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(4u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(136u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(1u16))),
            ],
        },
        IscriptCollection {
            id: 409u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(66u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(66u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(66u16))),
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
        IscriptCollection {
            id: 410u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(924u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(80u16))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(1114u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(409u16))),
                None,
                None,
                Some(IscriptId(NonZeroU16::new_unchecked(157u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(157u16))),
            ],
        },
        IscriptCollection {
            id: 411u16,
            scripts: &[
                Some(IscriptId(NonZeroU16::new_unchecked(1026u16))),
                Some(IscriptId(NonZeroU16::new_unchecked(2u16))),
            ],
        },
    ]
};
pub const ISCRIPT_ANIMS: [&[IC]; 1203usize] = [
    &[],
    &[IC::Wait(125u8)],
    &[IC::Wait(1u8)],
    &[
        IC::SetVerticalPosition(1i8),
        IC::WaitRandom {
            min: 8u8,
            max: 10u8,
        },
        IC::SetVerticalPosition(2i8),
        IC::WaitRandom {
            min: 8u8,
            max: 10u8,
        },
        IC::SetVerticalPosition(1i8),
        IC::WaitRandom {
            min: 8u8,
            max: 10u8,
        },
        IC::SetVerticalPosition(0i8),
        IC::WaitRandom {
            min: 8u8,
            max: 10u8,
        },
    ],
    &[
        IC::PlaySound(SoundId(7u16)),
        IC::ImageOverlay {
            image: ImageId(334u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
        IC::LiftOffConditionalJump(IscriptLabel(2u16)),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(273u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(774u16)),
        IC::ImageOverlay {
            image: ImageId(60u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(186u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(7u16)),
        IC::ImageOverlay {
            image: ImageId(215u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(223u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::GotoRepeatAttack,
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(111u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(108u16),
        },
    ],
    &[IC::Wait(1u8)],
    &[IC::Wait(1u8)],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[IC::Wait(1u8)],
    &[
        IC::PlaySound(SoundId(774u16)),
        IC::ImageOverlay {
            image: ImageId(60u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(187u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::Wait(1u8), IC::TurnOnceClockwise],
    &[
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(141u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(150u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(155u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(154u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(107u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(130u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(159u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(158u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Move(8u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(8u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(8u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(8u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(8u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(8u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(8u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(1u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(1u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(119u16),
    }],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(22u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(23u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(24u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(25u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(26u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(27u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(26u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(25u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(24u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(23u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Air),
        IC::GotoRepeatAttack,
    ],
    &[
        IC::Wait(5u8),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(1u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(1u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
    ],
    &[IC::Wait(1u8)],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[IC::Wait(1u8)],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::WaitRandom {
            min: 25u8,
            max: 30u8,
        },
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::WaitRandom {
            min: 25u8,
            max: 30u8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(96u16),
        },
    ],
    &[
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(3u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(3u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::Wait(1u8),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(85u16),
    }],
    &[
        IC::Wait(1u8),
        IC::Attack,
        IC::GotoRepeatAttack,
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(13u8),
        IC::IgnoreRest,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::Wait(1u8),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(221u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::TriggerTargetRangeConditionalJump {
            distance: 48u16,
            label: IscriptLabel(419u16),
        },
        IC::ImageOverlayUseLo {
            image: ImageId(549u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlaySound(SoundId(58u16)),
        IC::Attack,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[
        IC::PlaySound(SoundId(113u16)),
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::GotoRepeatAttack,
    ],
    &[IC::PlayFrame {
        frame: FrameId(85u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SetVerticalPosition(0i8),
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::SetVerticalPosition(1i8),
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::SetVerticalPosition(2i8),
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::SetVerticalPosition(1i8),
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(123u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlaySound(SoundId(71u16)),
        IC::ImageOverlayUseLo {
            image: ImageId(518u16),
            x: 0i8,
            y: 0i8,
        },
        IC::AttackWith(WeaponType::Ground),
        IC::GotoRepeatAttack,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::PlaySound(SoundId(1094u16)),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::AttackWith(WeaponType::Air),
        IC::GotoRepeatAttack,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlaySound(SoundId(7u16)),
        IC::ImageOverlay {
            image: ImageId(215u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(224u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::Wait(1u8),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlaySound(SoundId(7u16)),
        IC::ImageOverlay {
            image: ImageId(334u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
        IC::LiftOffConditionalJump(IscriptLabel(2u16)),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(274u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(153u16),
    }],
    &[
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::SetVerticalPosition(0i8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::PlaySound(SoundId(90u16)),
        IC::AttackWith(WeaponType::Ground),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::GotoRepeatAttack,
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::TriggerTargetRangeConditionalJump {
            distance: 48u16,
            label: IscriptLabel(466u16),
        },
        IC::ImageOverlayUseLo {
            image: ImageId(549u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlaySound(SoundId(58u16)),
        IC::Attack,
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[IC::PlayFrame {
        frame: FrameId(221u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Wait(1u8),
        IC::PlaySoundBetween {
            min: SoundId(102u16),
            max: SoundId(103u16),
        },
        IC::AttackWith(WeaponType::Ground),
        IC::Wait(1u8),
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
    ],
    &[
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::GotoRepeatAttack,
    ],
    &[IC::PlayFrame {
        frame: FrameId(3u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
    ],
    &[
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(155u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(154u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::TurnRandom(3u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::SpriteOverlay {
            sprite: SpriteId(505u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::TurnRandom(3u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::RandomConditionalJump {
            chance: 192u8,
            label: IscriptLabel(566u16),
        },
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::Wait(6u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::Wait(6u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
    ],
    &[
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(456u16),
        },
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
        IC::TurnCounterClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::TurnCounterClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(1u8),
        IC::TurnCounterClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::TurnCounterClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(1u8),
        IC::TurnCounterClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(1u8),
        IC::TurnCounterClockwise(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(96u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(25u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::TempRemoveGraphicEnd,
        IC::Wait(4u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(1u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(3u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(3u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(4u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(1u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(3u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(3u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(4u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(3u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(1u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(1u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
    ],
    &[
        IC::Wait(2u8),
        IC::TempRemoveGraphicStart,
        IC::WaitRandom { min: 2u8, max: 5u8 },
        IC::TempRemoveGraphicEnd,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(133u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::EngineFrame(FrameIdByte(0u8)),
        IC::Wait(1u8),
        IC::EngineFrame(FrameIdByte(17u8)),
        IC::Wait(1u8),
    ],
    &[
        IC::TempRemoveGraphicEnd,
        IC::Wait(5u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(5u8),
    ],
    &[
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(107u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(130u16),
        },
    ],
    &[
        IC::SpriteOverlay {
            sprite: SpriteId(368u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(368u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(368u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(368u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(368u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(978u16),
        },
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::TurnCounterClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::TurnCounterClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::TurnCounterClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::TurnCounterClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::TurnCounterClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
        IC::TurnCounterClockwise(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::SpriteOverlay {
            sprite: SpriteId(309u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(19u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(20u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(21u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(22u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(23u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(2u16),
    }],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
        IC::TurnRandom(3u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::Wait(1u8),
        IC::EngineFrame(FrameIdByte(0u8)),
        IC::Wait(1u8),
        IC::EngineFrame(FrameIdByte(17u8)),
    ],
    &[
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(111u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(108u16),
        },
    ],
    &[IC::TurnRandom(3u8)],
    &[
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(123u16),
        },
    ],
    &[
        IC::TempRemoveGraphicEnd,
        IC::Wait(5u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(5u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::Wait(3u8),
        IC::SpriteUnderlay {
            sprite: SpriteId(351u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(103u16),
        },
    ],
    &[
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(141u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(150u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::RandomConditionalJump {
            chance: 192u8,
            label: IscriptLabel(638u16),
        },
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::Wait(6u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::Wait(6u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
    ],
    &[IC::Wait(1u8), IC::FollowMainGraphic],
    &[
        IC::TempRemoveGraphicEnd,
        IC::Wait(5u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(5u8),
    ],
    &[
        IC::Wait(1u8),
        IC::EngineFrame(FrameIdByte(0u8)),
        IC::Wait(1u8),
        IC::EngineFrame(FrameIdByte(17u8)),
    ],
    &[
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(159u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(158u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::EngineSet(FrameSet(0u8)),
        IC::Wait(1u8),
        IC::EngineSet(FrameSet(1u8)),
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::TurnRandom(3u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::SpriteOverlay {
            sprite: SpriteId(309u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(2u8),
    ],
    &[
        IC::TempRemoveGraphicEnd,
        IC::Wait(5u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(5u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::TurnRandom(3u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::RandomConditionalJump {
            chance: 192u8,
            label: IscriptLabel(862u16),
        },
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::Wait(6u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::Wait(6u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
    ],
    &[
        IC::SpriteOverlay {
            sprite: SpriteId(309u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::TurnRandom(3u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::RandomConditionalJump {
            chance: 192u8,
            label: IscriptLabel(696u16),
        },
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::Wait(6u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnClockwise(2u8),
        IC::Wait(3u8),
        IC::Wait(6u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
        IC::TurnCounterClockwise(2u8),
        IC::Wait(3u8),
    ],
    &[
        IC::Wait(2u8),
        IC::SpriteOverlay {
            sprite: SpriteId(373u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::GroundSpriteOverlay {
            sprite: SpriteId(511u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoGroundDamage,
        IC::Wait(2u8),
    ],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::AttackMelee {
            num_sounds: 1u8,
            sounds: &[SoundId(111u16)],
        },
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
    ],
    &[
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::WaitRandom { min: 2u8, max: 3u8 },
    ],
    &[
        IC::PlaySound(SoundId(481u16)),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(69u16)),
        IC::AttackWith(WeaponType::Ground),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(5u8),
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::DoMissileDamage,
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySoundBetween {
            min: SoundId(35u16),
            max: SoundId(39u16),
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::Wait(1u8)],
    &[],
    &[
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::UseWeapon(WeaponId(14u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::WaitRandom {
            min: 8u8,
            max: 10u8,
        },
    ],
    &[
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::Wait(1u8),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PowerupConditionalJump(IscriptLabel(1u16)),
        IC::ImageUnderlay {
            image: ImageId(405u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::ImageOverlayUseLo {
            image: ImageId(421u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SetFlingyDirection(12u8),
        IC::AttackShiftProjectiles(24u8),
        IC::Wait(2u8),
        IC::AttackShiftProjectiles(52u8),
        IC::Wait(1u8),
        IC::AttackShiftProjectiles(80u8),
        IC::Wait(10u8),
        IC::IgnoreRest,
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(70u16)),
        IC::AttackWith(WeaponType::Ground),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::SignalOrder(SignalId(1u8)),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlaySound(SoundId(1018u16)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
    ],
    &[
        IC::Call(IscriptLabel(29u16)),
        IC::Call(IscriptLabel(29u16)),
        IC::WaitRandom { min: 3u8, max: 6u8 },
        IC::Call(IscriptLabel(29u16)),
        IC::WaitRandom { min: 3u8, max: 6u8 },
        IC::Call(IscriptLabel(29u16)),
        IC::Call(IscriptLabel(29u16)),
        IC::Call(IscriptLabel(29u16)),
        IC::WaitRandom { min: 3u8, max: 6u8 },
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::Move(5u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Move(5u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::Move(5u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(272u16),
        },
        IC::Move(5u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(289u16),
        },
        IC::Move(5u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Move(5u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Move(5u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Move(5u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::AttackWith(WeaponType::Ground),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(96u16)),
        IC::ImageOverlayUseLo {
            image: ImageId(537u16),
            x: 2i8,
            y: 0i8,
        },
        IC::Wait(2u8),
        IC::AttackWith(WeaponType::Ground),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::DoMissileDamage,
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(3u8),
    ],
    &[
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::Wait(1u8),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(98u16)),
        IC::Attack,
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::AttackMelee {
            num_sounds: 1u8,
            sounds: &[SoundId(894u16)],
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::PlaySound(SoundId(70u16)),
        IC::AttackWith(WeaponType::Ground),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(1u8),
        IC::GotoRepeatAttack,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
    ],
    &[
        IC::PlaySound(SoundId(111u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
    ],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(112u16)),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::AttackMelee {
            num_sounds: 1u8,
            sounds: &[SoundId(0u16)],
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
    ],
    &[
        IC::Wait(1u8),
        IC::Attack,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(28u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(29u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(30u16),
        },
        IC::Wait(2u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Air),
        IC::PlaySound(SoundId(1060u16)),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(133u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::WaitRandom { min: 2u8, max: 3u8 },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[],
    &[
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::WaitRandom { min: 2u8, max: 3u8 },
    ],
    &[
        IC::Move(0u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(0u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(1u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(1u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(3u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(1u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::SetVerticalPosition(0i8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::AttackWith(WeaponType::Ground),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::GotoRepeatAttack,
    ],
    &[IC::ImageOverlay {
        image: ImageId(446u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(3u8),
    ],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::OrderDone(SignalId(1u8)),
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SetFlipState(FlipState(0u8)),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::SetFlipState(FlipState(1u8)),
        IC::SetHorizontalPosition(-3i8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(272u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(289u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
    ],
    &[IC::Wait(1u8)],
    &[
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(8u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(9u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(5u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Move(7u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::AttackMelee {
            num_sounds: 1u8,
            sounds: &[SoundId(786u16)],
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(19u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(20u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(21u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(22u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(23u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::AttackWith(WeaponType::Air),
        IC::Wait(10u8),
        IC::AttackWith(WeaponType::Air),
        IC::Wait(10u8),
        IC::AttackWith(WeaponType::Air),
        IC::Wait(10u8),
        IC::AttackWith(WeaponType::Air),
        IC::Wait(10u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(3u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(7u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(8u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(7u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(8u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(7u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
    ],
    &[
        IC::SetVerticalPosition(0i8),
        IC::PlaySound(SoundId(64u16)),
        IC::SpriteOverlayUseLo {
            sprite: SpriteId(332u16),
            overlay: OverlayId(0u8),
        },
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::GotoRepeatAttack,
    ],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Wait(3u8),
        IC::SpriteUnderlayUseLo {
            sprite: SpriteId(320u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(103u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::SpriteOverlay {
            sprite: SpriteId(309u16),
            x: 0i8,
            y: 10i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::ImageOverlayUseLo {
            image: ImageId(421u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SetFlingyDirection(20u8),
        IC::AttackShiftProjectiles(24u8),
        IC::Wait(2u8),
        IC::AttackShiftProjectiles(52u8),
        IC::Wait(1u8),
        IC::AttackShiftProjectiles(80u8),
        IC::Wait(10u8),
        IC::IgnoreRest,
    ],
    &[
        IC::PlaySound(SoundId(816u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::DoMissileDamage,
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Air),
        IC::GotoRepeatAttack,
    ],
    &[
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::Wait(1u8),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::WaitRandom { min: 2u8, max: 3u8 },
    ],
    &[IC::ImageOverlay {
        image: ImageId(446u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::SetFlingySpeed(Speed(2048u16)),
        IC::Move(16u8),
        IC::SpriteOverlay {
            sprite: SpriteId(373u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(813u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::DoMissileDamage,
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[IC::NoBreakCodeEnd, IC::Wait(1u8)],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::SetFlipState(FlipState(1u8)),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::CurrentDirectionConditionalJump {
            angle1: 160u16,
            angle2: 10u16,
            label: IscriptLabel(367u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlaySound(SoundId(110u16)),
        IC::ImageOverlayUseLo {
            image: ImageId(518u16),
            x: 0i8,
            y: 0i8,
        },
        IC::AttackWith(WeaponType::Ground),
        IC::GotoRepeatAttack,
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(27u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(36u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(45u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(54u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(63u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(72u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(81u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(90u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(99u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(108u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(117u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(126u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(614u16)),
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlaySound(SoundId(101u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
    ],
    &[
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(3u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(3u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Move(3u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(5u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(5u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(8u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(5u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(69u16)),
        IC::AttackWith(WeaponType::Ground),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(4u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::SetSpawnFrame(20u8),
        IC::Wait(1u8),
        IC::CurrentDirectionConditionalJump {
            angle1: 160u16,
            angle2: 10u16,
            label: IscriptLabel(367u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::GotoRepeatAttack,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(98u16)),
        IC::Attack,
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(95u16)),
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(112u16)),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::AttackMelee {
            num_sounds: 1u8,
            sounds: &[SoundId(0u16)],
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::CurrentDirectionConditionalJump {
            angle1: 160u16,
            angle2: 10u16,
            label: IscriptLabel(258u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
    ],
    &[
        IC::NoBreakCodeStart,
        IC::Wait(1u8),
        IC::PlaySound(SoundId(64u16)),
        IC::Attack,
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
    ],
    &[
        IC::Wait(1u8),
        IC::CurrentDirectionConditionalJump {
            angle1: 160u16,
            angle2: 10u16,
            label: IscriptLabel(258u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Air),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::GotoRepeatAttack,
    ],
    &[
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
    ],
    &[
        IC::TriggerTargetCConditionalJump {
            angle1: 74u16,
            angle2: 42u16,
            label: IscriptLabel(652u16),
        },
        IC::TriggerTargetCConditionalJump {
            angle1: 159u16,
            angle2: 42u16,
            label: IscriptLabel(941u16),
        },
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(78u16)),
        IC::PlayFrame {
            frame: FrameId(24u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(25u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(26u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(27u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(28u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(29u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(30u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(31u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(32u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(33u16),
        },
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::Wait(6u8),
        IC::PlayFrame {
            frame: FrameId(32u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(31u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(30u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(29u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(28u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(27u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(26u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(25u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(24u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(272u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(289u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(306u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(323u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(340u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(357u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(374u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(391u16),
        },
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::PlayFrame {
            frame: FrameId(374u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(357u16),
        },
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(340u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(323u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(306u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(289u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(272u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(98u16)),
        IC::Attack,
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
    ],
    &[
        IC::Wait(3u8),
        IC::SpriteOverlay {
            sprite: SpriteId(309u16),
            x: 0i8,
            y: -10i8,
        },
    ],
    &[
        IC::SetVerticalPosition(0i8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(936u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(374u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(391u16),
        },
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::PlayFrame {
            frame: FrameId(374u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(357u16),
        },
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(340u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(323u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(306u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(289u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(272u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::Wait(1u8),
        IC::Attack,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::SetVerticalPosition(0i8),
        IC::WaitRandom { min: 5u8, max: 8u8 },
        IC::PlaySound(SoundId(847u16)),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[IC::Wait(1u8)],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[],
    &[
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(5u8),
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::Wait(20u8),
        IC::WaitRandom {
            min: 5u8,
            max: 100u8,
        },
    ],
    &[
        IC::PlaySound(SoundId(778u16)),
        IC::SpriteOverlay {
            sprite: SpriteId(132u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Air),
    ],
    &[
        IC::Move(0u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(0u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(0u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Move(0u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(0u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(0u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(0u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(0u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(0u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(0u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
    ],
    &[IC::NoBreakCodeEnd, IC::GotoRepeatAttack, IC::Wait(1u8)],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(68u16)),
        IC::ImageOverlay {
            image: ImageId(536u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(2u8),
        IC::AttackWith(WeaponType::Ground),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
    ],
    &[
        IC::PlaySound(SoundId(942u16)),
        IC::ImageOverlay {
            image: ImageId(41u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(66u16)),
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::Wait(1u8), IC::FollowMainGraphic],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(98u16)),
        IC::Attack,
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::SetFlipState(FlipState(1u8)),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::Wait(1u8),
        IC::PlaySound(SoundId(64u16)),
        IC::Attack,
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::Wait(20u8),
        IC::WaitRandom {
            min: 5u8,
            max: 100u8,
        },
    ],
    &[
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(340u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(323u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(306u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(289u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(272u16),
        },
    ],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::ImageOverlay {
        image: ImageId(196u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::WaitRandom { min: 2u8, max: 3u8 },
    ],
    &[IC::Wait(1u8)],
    &[IC::Wait(1u8), IC::FollowMainGraphic],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(7u16)),
        IC::Wait(1u8),
        IC::DoMissileDamage,
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::WaitRandom { min: 2u8, max: 3u8 },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::SetFlipState(FlipState(1u8)),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::UseWeapon(WeaponId(63u8)),
        IC::Wait(1u8),
        IC::WaitRandom {
            min: 8u8,
            max: 10u8,
        },
    ],
    &[
        IC::PlaySound(SoundId(355u16)),
        IC::Move(16u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Move(16u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Move(16u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Move(16u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Move(16u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Move(16u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Move(16u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Move(16u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
    ],
    &[IC::ImageOverlay {
        image: ImageId(200u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(331u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::SetVerticalPosition(0i8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::AttackWith(WeaponType::Ground),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::PlaySound(SoundId(64u16)),
        IC::SpriteOverlayUseLo {
            sprite: SpriteId(332u16),
            overlay: OverlayId(0u8),
        },
        IC::Attack,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::PlaySound(SoundId(64u16)),
        IC::SpriteOverlayUseLo {
            sprite: SpriteId(332u16),
            overlay: OverlayId(0u8),
        },
        IC::Attack,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::Move(4u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Move(8u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Move(8u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Move(2u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Move(6u8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PowerupConditionalJump(IscriptLabel(1001u16)),
        IC::ImageUnderlay {
            image: ImageId(411u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(267u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(4u16),
    }],
    &[IC::PlayFrame {
        frame: FrameId(272u16),
    }],
    &[
        IC::PlaySound(SoundId(58u16)),
        IC::Attack,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(272u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::SetVerticalPosition(0i8),
        IC::Wait(3u8),
        IC::SpriteUnderlayUseLo {
            sprite: SpriteId(320u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(471u16)),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::SignalOrder(SignalId(16u8)),
        IC::NoBreakCodeEnd,
    ],
    &[IC::ImageOverlay {
        image: ImageId(139u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(77u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(80u16)),
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(2u8),
        IC::SpriteOverlay {
            sprite: SpriteId(373u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(953u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::FollowMainGraphic,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::Wait(1u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::ImageUnderlay {
            image: ImageId(950u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::SetFlipState(FlipState(1u8)),
        IC::Wait(1u8),
        IC::SetFlingyDirection(20u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(108u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::ImageOverlay {
            image: ImageId(58u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::SetFlipState(FlipState(1u8)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
    ],
    &[
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(471u16)),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::SignalOrder(SignalId(16u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::PowerupConditionalJump(IscriptLabel(1u16)),
        IC::ImageUnderlay {
            image: ImageId(407u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
    ],
    &[IC::Wait(1u8), IC::SwitchUnderlay(ImageId(410u16))],
    &[IC::ImageOverlay {
        image: ImageId(176u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::DoMissileDamage,
        IC::ImageOverlay {
            image: ImageId(508u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(272u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlaySound(SoundId(896u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(289u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(290u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(291u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(292u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(293u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(294u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(295u16),
        },
        IC::Wait(2u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(160u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::Wait(1u8),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(471u16)),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::SignalOrder(SignalId(16u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(236u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(279u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlaySound(SoundId(598u16)),
        IC::ImageOverlay {
            image: ImageId(213u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
        IC::TurnClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::TurnClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(1u8),
        IC::TurnClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::TurnClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(1u8),
        IC::TurnClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(1u8),
        IC::TurnClockwise(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(159u16),
            x: 0i8,
            y: 0i8,
        },
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(3u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(3u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::TempRemoveGraphicEnd,
        IC::Wait(5u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(5u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[IC::ImageOverlay {
        image: ImageId(143u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(73u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(427u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(58u16)),
        IC::Attack,
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[
        IC::PlaySound(SoundId(514u16)),
        IC::ImageOverlay {
            image: ImageId(213u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlaySound(SoundId(1093u16)),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(1u8),
        IC::ImageOverlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(272u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(289u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(306u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(323u16),
        },
        IC::Wait(1u8),
        IC::ImageOverlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(272u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(289u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(306u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(323u16),
        },
        IC::Wait(1u8),
        IC::ImageOverlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(340u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(357u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(374u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(391u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(408u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(108u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 2u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[IC::Wait(1u8), IC::SetFlingyDirection(12u8)],
    &[IC::ImageOverlay {
        image: ImageId(115u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlaySound(SoundId(7u16)),
        IC::ImageOverlay {
            image: ImageId(215u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PowerupConditionalJump(IscriptLabel(668u16)),
        IC::ImageUnderlay {
            image: ImageId(417u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(3u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(222u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(133u16),
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(106u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(25u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(25u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(25u8),
    ],
    &[IC::ImageUnderlay {
        image: ImageId(12u16),
        x: 0i8,
        y: 42i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::NoBreakCodeStart,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlaySound(SoundId(1059u16)),
        IC::CastSpell,
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::SignalOrder(SignalId(2u8)),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlaySound(SoundId(1022u16)),
        IC::ImageOverlay {
            image: ImageId(530u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::ImageOverlay {
            image: ImageId(440u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlaySound(SoundId(72u16)),
        IC::ImageOverlay {
            image: ImageId(963u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(10u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(94u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageOverlay {
            image: ImageId(249u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SetVerticalPosition(0i8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(25u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(25u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(25u8),
    ],
    &[
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlaySound(SoundId(813u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::DoMissileDamage,
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlaySoundBetween {
            min: SoundId(276u16),
            max: SoundId(277u16),
        },
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(222u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(223u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(224u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(225u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(226u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(227u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(228u16),
        },
        IC::Wait(2u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(236u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(544u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(948u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[IC::PlayFrame {
        frame: FrameId(374u16),
    }],
    &[
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::ImageUnderlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(273u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(160u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlayNextId { x: 0i8, y: 0i8 },
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(107u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 2u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::NoBreakCodeStart,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::CastSpell,
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::SignalOrder(SignalId(2u8)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PowerupConditionalJump(IscriptLabel(929u16)),
        IC::ImageUnderlay {
            image: ImageId(415u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(138u16),
            x: 0i8,
            y: 7i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlaySound(SoundId(8u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(264u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(19u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(20u16),
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlaySound(SoundId(7u16)),
        IC::ImageOverlay {
            image: ImageId(213u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::Unknown45,
    ],
    &[
        IC::PlaySound(SoundId(100u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(108u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(4u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(259u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::SignalOrder(SignalId(2u8)),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(85u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(96u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::ImageOverlay {
        image: ImageId(121u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(19u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(20u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(21u16),
        },
        IC::Wait(2u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(321u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlaySound(SoundId(741u16)),
        IC::ImageOverlay {
            image: ImageId(153u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(348u16)),
        IC::ImageOverlay {
            image: ImageId(333u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(119u16),
    }],
    &[
        IC::PlaySound(SoundId(7u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::ImageUnderlay {
            image: ImageId(555u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::Wait(1u8),
        IC::PlaySound(SoundId(548u16)),
        IC::CastSpell,
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::SignalOrder(SignalId(2u8)),
    ],
    &[IC::ImageOverlay {
        image: ImageId(146u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::ImageOverlay {
            image: ImageId(109u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(290u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::Wait(1u8),
        IC::PlaySound(SoundId(240u16)),
        IC::CastSpell,
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::SignalOrder(SignalId(2u8)),
    ],
    &[
        IC::SetFlipState(FlipState(1u8)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[IC::Wait(1u8)],
    &[IC::SetVerticalPosition(0i8)],
    &[
        IC::TempRemoveGraphicEnd,
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::ImageOverlay {
            image: ImageId(421u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::AttackShiftProjectiles(24u8),
        IC::Wait(1u8),
        IC::AttackShiftProjectiles(52u8),
        IC::Wait(1u8),
        IC::AttackShiftProjectiles(80u8),
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::IgnoreRest,
    ],
    &[
        IC::PlaySound(SoundId(776u16)),
        IC::ImageOverlay {
            image: ImageId(3u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(120u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SetVerticalPosition(0i8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::SetFlingyDirection(12u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(3u16),
    }],
    &[IC::PlayFrame {
        frame: FrameId(4u16),
    }],
    &[
        IC::NoBreakCodeStart,
        IC::Wait(18u8),
        IC::PlaySound(SoundId(472u16)),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SignalOrder(SignalId(16u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(442u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(276u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(510u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(527u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(544u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(561u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(578u16),
        },
        IC::Wait(50u8),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(471u16)),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::SignalOrder(SignalId(16u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(5u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::SetFlipState(FlipState(1u8)),
    ],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::SignalOrder(SignalId(4u8)),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(319u16),
            x: 0i8,
            y: 0i8,
        },
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(109u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(4u16),
    }],
    &[
        IC::Wait(13u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(55u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[IC::ImageUnderlay {
        image: ImageId(350u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlaySound(SoundId(617u16)),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[IC::Wait(1u8)],
    &[
        IC::SetFlipState(FlipState(1u8)),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(930u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::NoBreakCodeStart,
        IC::ImageUnderlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlaySound(SoundId(354u16)),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SignalOrder(SignalId(4u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::PlaySound(SoundId(493u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(408u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(409u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(410u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(411u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(412u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(413u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(414u16),
        },
        IC::Wait(2u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(192u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PowerupConditionalJump(IscriptLabel(1u16)),
        IC::ImageUnderlay {
            image: ImageId(407u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::SpriteOverlay {
            sprite: SpriteId(310u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(6u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(2u16),
    }],
    &[],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SetFlipState(FlipState(1u8)),
    ],
    &[IC::PlayFrame {
        frame: FrameId(5u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(620u16)),
        IC::TempRemoveGraphicStart,
        IC::Wait(5u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(71u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlaySound(SoundId(830u16)),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(143u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(93u16)),
        IC::SpriteOverlay {
            sprite: SpriteId(365u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(341u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[IC::ImageOverlay {
        image: ImageId(205u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlaySound(SoundId(7u16)),
        IC::ImageOverlay {
            image: ImageId(214u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::TempRemoveGraphicStart,
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::TempRemoveGraphicEnd,
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::WaitRandom { min: 2u8, max: 3u8 },
    ],
    &[
        IC::PlaySound(SoundId(617u16)),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SetVerticalPosition(0i8),
    ],
    &[
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::ImageUnderlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::Wait(20u8),
        IC::PlaySound(SoundId(472u16)),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SignalOrder(SignalId(16u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(289u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(66u16)),
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(102u16),
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(291u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(315u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::SetFlipState(FlipState(1u8)),
        IC::Wait(1u8),
        IC::SetFlingyDirection(20u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(170u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::CastSpell, IC::SignalOrder(SignalId(1u8))],
    &[IC::ImageOverlay {
        image: ImageId(294u16),
        x: 0i8,
        y: 0i8,
    }],
    &[],
    &[IC::ImageOverlay {
        image: ImageId(143u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(309u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::TempRemoveGraphicStart,
        IC::TriggerTargetRangeConditionalJump {
            distance: 70u16,
            label: IscriptLabel(584u16),
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(2u16),
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(9u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::PlayFrameTile {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlaySound(SoundId(1040u16)),
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(4u16),
    }],
    &[
        IC::ImageOverlay {
            image: ImageId(108u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::TempRemoveGraphicEnd,
        IC::Wait(5u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(5u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(100u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(428u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(3u16),
    }],
    &[
        IC::ImageOverlayUseLo {
            image: ImageId(254u16),
            x: 2i8,
            y: 0i8,
        },
        IC::ImageUnderlay {
            image: ImageId(255u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
        IC::SetFlingySpeed(Speed(0u16)),
        IC::PlaySound(SoundId(319u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(38u8),
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::AttackWith(WeaponType::Ground),
        IC::SpriteOverlay {
            sprite: SpriteId(136u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(106u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Wait(13u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(87u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlaySound(SoundId(638u16)),
        IC::ImageOverlay {
            image: ImageId(214u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(50u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(177u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(270u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(1023u16)),
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(505u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(123u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::ImageOverlay {
            image: ImageId(59u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(867u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(205u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(206u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(207u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(208u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(209u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(210u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(211u16),
        },
        IC::Wait(2u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(147u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(274u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::ImageOverlay {
            image: ImageId(58u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(78u16)),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::Wait(6u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(935u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SetFlipState(FlipState(0u8)),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::SetFlipState(FlipState(1u8)),
        IC::SetHorizontalPosition(-3i8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(69u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::ImageOverlayOriginal(ImageId(543u16))],
    &[IC::ImageOverlay {
        image: ImageId(314u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::ImageOverlay {
            image: ImageId(261u16),
            x: 0i8,
            y: 0i8,
        },
        IC::ImageUnderlay {
            image: ImageId(262u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(50u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::ImageUnderlayUseLo {
            image: ImageId(552u16),
            x: 2i8,
            y: 0i8,
        },
        IC::ImageUnderlayUseLo {
            image: ImageId(550u16),
            x: 2i8,
            y: 1i8,
        },
        IC::ImageOverlayUseLo {
            image: ImageId(553u16),
            x: 2i8,
            y: 2i8,
        },
        IC::ImageOverlayUseLo {
            image: ImageId(551u16),
            x: 2i8,
            y: 3i8,
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(225u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SetVerticalPosition(0i8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(156u16),
        x: 0i8,
        y: 0i8,
    }],
    &[IC::Wait(1u8), IC::SwitchUnderlay(ImageId(418u16))],
    &[
        IC::ImageOverlay {
            image: ImageId(107u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::CastSpell,
        IC::SignalOrder(SignalId(2u8)),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(51u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
    ],
    &[
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(65u16)),
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::TempRemoveGraphicStart,
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::TempRemoveGraphicEnd,
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(305u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(106u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(169u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(212u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlaySound(SoundId(177u16)),
        IC::ImageOverlay {
            image: ImageId(333u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::TempRemoveGraphicEnd,
        IC::Wait(5u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(5u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(324u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(22u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(20u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(21u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(22u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(23u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(280u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrameTile {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(346u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[IC::OrderDone(SignalId(64u8))],
    &[IC::ImageOverlay {
        image: ImageId(310u16),
        x: 0i8,
        y: 0i8,
    }],
    &[IC::PlaySoundBetween {
        min: SoundId(314u16),
        max: SoundId(315u16),
    }],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(7u16)),
        IC::ImageOverlay {
            image: ImageId(928u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[IC::Wait(13u8)],
    &[IC::ImageOverlay {
        image: ImageId(197u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(224u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlaySound(SoundId(53u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(154u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(155u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(156u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(157u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(158u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(159u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(160u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::TempRemoveGraphicStart,
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::TempRemoveGraphicEnd,
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::WaitRandom { min: 2u8, max: 3u8 },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(1016u16)),
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(310u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlaySound(SoundId(533u16)),
        IC::ImageOverlay {
            image: ImageId(213u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[IC::SignalOrder(SignalId(4u8))],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SetVerticalPosition(0i8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::NoBreakCodeStart,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::CastSpell,
        IC::SignalOrder(SignalId(2u8)),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
    ],
    &[
        IC::PlaySound(SoundId(924u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(165u16),
        x: 0i8,
        y: 0i8,
    }],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(3u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(50u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(284u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::OrderDone(SignalId(1u8)),
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::CastSpell,
        IC::SignalOrder(SignalId(2u8)),
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(3u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(447u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::ImageUnderlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(168u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlaySound(SoundId(9u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(27u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(36u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(45u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(54u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(63u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(72u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(81u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(90u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(99u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(108u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[],
    &[IC::PlayFrame {
        frame: FrameId(10u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(311u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(75u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(127u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::WaitRandom { min: 1u8, max: 2u8 },
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(98u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(297u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::ImageOverlay {
            image: ImageId(235u16),
            x: 0i8,
            y: 0i8,
        },
        IC::ImageUnderlay {
            image: ImageId(236u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(19u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(20u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(21u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(22u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(23u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(24u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(25u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(26u16),
        },
        IC::Wait(2u8),
        IC::ImageOverlay {
            image: ImageId(926u16),
            x: 0i8,
            y: 0i8,
        },
        IC::ImageOverlay {
            image: ImageId(927u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SignalOrder(SignalId(4u8)),
        IC::NoBreakCodeEnd,
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(975u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(205u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(206u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(207u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(208u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(209u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(210u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(211u16),
        },
        IC::Wait(2u8),
    ],
    &[],
    &[IC::SignalOrder(SignalId(16u8))],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[IC::ImageOverlay {
        image: ImageId(220u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlaySound(SoundId(533u16)),
        IC::ImageOverlay {
            image: ImageId(213u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(172u16),
        x: 0i8,
        y: 0i8,
    }],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(1u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
        IC::TempRemoveGraphicEnd,
        IC::Wait(1u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(942u16)),
        IC::ImageOverlay {
            image: ImageId(41u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
    ],
    &[IC::ImageOverlay {
        image: ImageId(185u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(931u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SetVerticalPosition(0i8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(313u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlaySoundBetween {
            min: SoundId(595u16),
            max: SoundId(596u16),
        },
        IC::ImageOverlay {
            image: ImageId(214u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::TempRemoveGraphicStart,
        IC::WaitRandom { min: 3u8, max: 8u8 },
        IC::TempRemoveGraphicEnd,
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::NoBreakCodeStart,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::CastSpell,
        IC::SignalOrder(SignalId(2u8)),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(3u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::ImageUnderlay {
            image: ImageId(219u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(19u16),
        },
        IC::Wait(2u8),
        IC::SignalOrder(SignalId(4u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(108u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(78u16)),
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(368u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(368u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(368u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(368u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(368u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlaySound(SoundId(967u16)),
        IC::ImageOverlay {
            image: ImageId(231u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::ImageUnderlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(263u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(246u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(229u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(212u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(272u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(49u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(205u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(206u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(207u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(208u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(209u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(210u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(211u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(47u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 4u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(209u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(188u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(179u16)),
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::Wait(3u8),
        IC::SpriteUnderlay {
            sprite: SpriteId(351u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlaySound(SoundId(785u16)),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(134u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(141u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Wait(38u8),
        IC::PlaySound(SoundId(319u16)),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(5u8),
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::PlaySoundBetween {
            min: SoundId(91u16),
            max: SoundId(92u16),
        },
        IC::SpriteOverlay {
            sprite: SpriteId(365u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
    ],
    &[
        IC::PlaySound(SoundId(830u16)),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(143u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(50u8),
    ],
    &[IC::ImageUnderlay {
        image: ImageId(348u16),
        x: 0i8,
        y: 0i8,
    }],
    &[IC::PlayFrame {
        frame: FrameId(102u16),
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(1u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SetVerticalPosition(0i8),
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::SetVerticalPosition(1i8),
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::SetVerticalPosition(2i8),
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::SetVerticalPosition(1i8),
        IC::Wait(3u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(5u8),
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(34u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(96u16),
        },
    ],
    &[
        IC::NoBreakCodeStart,
        IC::Wait(1u8),
        IC::PlaySound(SoundId(240u16)),
        IC::CastSpell,
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::SignalOrder(SignalId(2u8)),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(107u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 2u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(2u16),
    }],
    &[IC::PlayFrame {
        frame: FrameId(4u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::SetFlingyDirection(12u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(26u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 4u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(39u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 4u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(328u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(272u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(152u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
    ],
    &[
        IC::CurrentDirectionConditionalJump {
            angle1: 128u16,
            angle2: 32u16,
            label: IscriptLabel(347u16),
        },
        IC::SpriteOverlay {
            sprite: SpriteId(309u16),
            x: 0i8,
            y: 10i8,
        },
        IC::Wait(3u8),
    ],
    &[],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(206u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(945u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(103u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::SetFlingyDirection(20u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(427u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::TurnOnceClockwise,
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(248u16),
            x: 0i8,
            y: 7i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(212u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(229u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(246u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(263u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(280u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(14u16),
            x: 0i8,
            y: 3i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 4u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(3u8),
    ],
    &[
        IC::SetFlingyDirection(15u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(4u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(222u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(223u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(224u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(225u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(226u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(227u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(228u16),
        },
        IC::Wait(2u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(230u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(194u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(43u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlaySound(SoundId(878u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(256u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(257u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(258u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(259u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(260u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(261u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(262u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(263u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(264u16),
        },
        IC::Wait(2u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(158u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(22u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 2u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(276u16),
        x: 0i8,
        y: 0i8,
    }],
    &[IC::ImageOverlay {
        image: ImageId(320u16),
        x: 0i8,
        y: 0i8,
    }],
    &[IC::PlayFrame {
        frame: FrameId(272u16),
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(255u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
        IC::SetFlingySpeed(Speed(0u16)),
        IC::PlaySound(SoundId(319u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(38u8),
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(108u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(27u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(36u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(45u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(54u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(63u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(72u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(81u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(90u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(99u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(108u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(117u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(126u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::TempRemoveGraphicStart,
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::TempRemoveGraphicEnd,
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::WaitRandom { min: 2u8, max: 3u8 },
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(108u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(429u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(221u16),
    }],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::PlaySound(SoundId(257u16)),
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlaySound(SoundId(7u16)),
        IC::ImageOverlay {
            image: ImageId(215u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlaySound(SoundId(210u16)),
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageOverlay {
            image: ImageId(359u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlaySound(SoundId(9u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySoundBetween {
            min: SoundId(107u16),
            max: SoundId(109u16),
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Wait(15u8),
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(1u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(1u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SetFlipState(FlipState(1u8)),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::NoBreakCodeStart,
        IC::Wait(15u8),
        IC::PlaySound(SoundId(472u16)),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(8u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(8u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(8u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(8u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SignalOrder(SignalId(16u8)),
        IC::NoBreakCodeEnd,
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlaySoundRandom {
            num_sounds: 2u8,
            sounds: &[SoundId(662u16), SoundId(663u16)],
        },
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::AttackMelee {
            num_sounds: 1u8,
            sounds: &[SoundId(664u16)],
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::AttackMelee {
            num_sounds: 1u8,
            sounds: &[SoundId(664u16)],
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PowerupConditionalJump(IscriptLabel(1u16)),
        IC::ImageUnderlay {
            image: ImageId(406u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::Wait(13u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(269u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::Wait(38u8),
        IC::PlaySound(SoundId(318u16)),
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(3u8),
        IC::SetFlingyDirection(28u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::ImageOverlay {
            image: ImageId(58u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(182u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::ImageOverlay {
        image: ImageId(154u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlaySoundRandom {
            num_sounds: 2u8,
            sounds: &[SoundId(662u16), SoundId(663u16)],
        },
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::AttackMelee {
            num_sounds: 1u8,
            sounds: &[SoundId(664u16)],
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::AttackMelee {
            num_sounds: 1u8,
            sounds: &[SoundId(664u16)],
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(916u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 4u8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(300u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::ImageOverlay {
            image: ImageId(421u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::AttackShiftProjectiles(24u8),
        IC::Wait(1u8),
        IC::AttackShiftProjectiles(52u8),
        IC::Wait(1u8),
        IC::AttackShiftProjectiles(80u8),
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::IgnoreRest,
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(922u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(123u16),
        },
    ],
    &[
        IC::PlaySound(SoundId(85u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlaySound(SoundId(13u16)),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlaySound(SoundId(13u16)),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlaySound(SoundId(13u16)),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(2u8),
        IC::PlaySound(SoundId(13u16)),
        IC::PlayFrame {
            frame: FrameId(19u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(20u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(21u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(22u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(23u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(24u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(25u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
        IC::Wait(125u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(63u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(3u8),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::Wait(15u8),
        IC::PlaySound(SoundId(472u16)),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SignalOrder(SignalId(16u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::SpriteOverlay {
            sprite: SpriteId(365u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(471u16)),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::SignalOrder(SignalId(16u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(173u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(408u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SetFlipState(FlipState(1u8)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(193u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlaySound(SoundId(99u16)),
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(132u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SetVerticalPosition(0i8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(108u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlaySound(SoundId(849u16)),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(150u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(941u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SignalOrder(SignalId(64u8)),
        IC::SetVerticalPosition(0i8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(62u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::DoMissileDamage,
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
    ],
    &[
        IC::PlaySound(SoundId(741u16)),
        IC::ImageOverlay {
            image: ImageId(153u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::ImageUnderlay {
            image: ImageId(298u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(4u16),
    }],
    &[IC::Wait(1u8)],
    &[
        IC::PlaySound(SoundId(9u16)),
        IC::ImageOverlay {
            image: ImageId(530u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(238u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(222u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(223u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(224u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(225u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(226u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(227u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(228u16),
        },
        IC::Wait(2u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(230u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::ImageUnderlay {
            image: ImageId(119u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(343u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::DoMissileDamage,
        IC::PlaySound(SoundId(72u16)),
        IC::ImageOverlay {
            image: ImageId(510u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(353u16)),
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(83u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(74u16)),
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(323u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[],
    &[
        IC::SetFlingyDirection(12u8),
        IC::PlaySound(SoundId(318u16)),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(303u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::NoBreakCodeStart,
        IC::Wait(18u8),
        IC::PlaySound(SoundId(472u16)),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SignalOrder(SignalId(16u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::ImageOverlay {
            image: ImageId(737u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::ImageUnderlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::Wait(18u8),
        IC::PlaySound(SoundId(472u16)),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SignalOrder(SignalId(16u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(938u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(80u16)),
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(309u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlaySound(SoundId(8u16)),
        IC::ImageOverlay {
            image: ImageId(213u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(287u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::TempRemoveGraphicStart,
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::TempRemoveGraphicEnd,
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::WaitRandom { min: 2u8, max: 3u8 },
    ],
    &[IC::Wait(1u8), IC::SwitchUnderlay(ImageId(416u16))],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(106u16)),
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(310u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::SetFlipState(FlipState(1u8)),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(19u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(20u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(21u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(22u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(23u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(157u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(326u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(9u16),
    }],
    &[IC::Wait(1u8)],
    &[
        IC::ImageUnderlay {
            image: ImageId(244u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlaySound(SoundId(910u16)),
        IC::ImageOverlay {
            image: ImageId(45u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[IC::Wait(1u8)],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(78u16)),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(19u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(20u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(21u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(22u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(23u16),
        },
        IC::Wait(1u8),
        IC::AttackWith(WeaponType::Ground),
        IC::Wait(6u8),
        IC::PlayFrame {
            frame: FrameId(23u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(22u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(21u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(20u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(19u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::IgnoreRest,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(107u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 2u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::ImageOverlay {
            image: ImageId(58u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::ImageOverlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlaySound(SoundId(354u16)),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(4u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlaySound(SoundId(774u16)),
        IC::ImageOverlay {
            image: ImageId(60u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
        IC::ImageOverlay {
            image: ImageId(89u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(187u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::ImageOverlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(50u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageOverlay {
            image: ImageId(357u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlaySound(SoundId(1097u16)),
        IC::SpriteOverlay {
            sprite: SpriteId(483u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(133u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlaySound(SoundId(7u16)),
        IC::SpriteOverlay {
            sprite: SpriteId(272u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(108u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(227u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(159u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(158u16),
        },
    ],
    &[IC::Wait(1u8)],
    &[IC::ImageOverlay {
        image: ImageId(302u16),
        x: 0i8,
        y: 0i8,
    }],
    &[IC::ImageOverlay {
        image: ImageId(128u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::ImageOverlay {
            image: ImageId(444u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::ImageUnderlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(357u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(340u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(323u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(306u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 2u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(81u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(5u16),
    }],
    &[
        IC::TempRemoveGraphicStart,
        IC::Wait(2u8),
        IC::TempRemoveGraphicEnd,
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(1080u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(425u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(442u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(459u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(476u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(493u16),
        },
        IC::Wait(2u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(484u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(635u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(289u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(290u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(291u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(292u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(293u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(294u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(295u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(12u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::WaitRandom {
            min: 10u8,
            max: 15u8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlaySound(SoundId(678u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(222u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(223u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(224u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(225u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(226u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(227u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::PowerupConditionalJump(IscriptLabel(1u16)),
        IC::ImageUnderlay {
            image: ImageId(406u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::TurnClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::TurnClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::TurnClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::TurnClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::TurnClockwise(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
        IC::TurnClockwise(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(924u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(108u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::SetVerticalPosition(0i8),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::UseWeapon(WeaponId(14u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::WaitRandom {
            min: 8u8,
            max: 10u8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(79u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::ImageUnderlayNextId { x: 0i8, y: 42i8 },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::TempRemoveGraphicStart,
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::TempRemoveGraphicEnd,
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::WaitRandom { min: 2u8, max: 3u8 },
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlaySound(SoundId(979u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(205u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(206u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(207u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(208u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(209u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(210u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(211u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(212u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(213u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(955u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SetFlipState(FlipState(1u8)),
        IC::ImageUnderlayNextId { x: 0i8, y: 0i8 },
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(166u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlaySoundBetween {
            min: SoundId(276u16),
            max: SoundId(277u16),
        },
        IC::ImageOverlay {
            image: ImageId(242u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::SpriteOverlay {
            sprite: SpriteId(310u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlaySoundBetween {
            min: SoundId(925u16),
            max: SoundId(927u16),
        },
        IC::SpriteOverlay {
            sprite: SpriteId(156u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(222u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(223u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(224u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(225u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(226u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(227u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::ImageOverlay {
            image: ImageId(59u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(1110u16)),
        IC::ImageOverlay {
            image: ImageId(530u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoGroundDamage,
        IC::Wait(1u8),
    ],
    &[
        IC::Wait(1u8),
        IC::SwitchUnderlay(ImageId(412u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlaySound(SoundId(57u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(188u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(189u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(190u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(191u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(192u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(193u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(194u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::SetVerticalPosition(0i8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::NoBreakCodeStart,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::CastSpell,
        IC::SignalOrder(SignalId(2u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[IC::ImageOverlay {
        image: ImageId(282u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(295u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::NoBreakCodeStart,
        IC::Wait(25u8),
        IC::PlaySound(SoundId(472u16)),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SignalOrder(SignalId(16u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(30u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(111u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(108u16),
        },
    ],
    &[
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(614u16)),
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySoundBetween {
            min: SoundId(296u16),
            max: SoundId(298u16),
        },
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(50u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(238u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(141u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(150u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::SpriteOverlay {
            sprite: SpriteId(380u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(6u8),
        IC::SpriteOverlay {
            sprite: SpriteId(380u16),
            x: 32i8,
            y: 32i8,
        },
        IC::Wait(2u8),
        IC::SpriteOverlay {
            sprite: SpriteId(380u16),
            x: 48i8,
            y: 5i8,
        },
        IC::Wait(5u8),
        IC::SpriteOverlay {
            sprite: SpriteId(380u16),
            x: 32i8,
            y: -32i8,
        },
        IC::Wait(2u8),
        IC::SpriteOverlay {
            sprite: SpriteId(380u16),
            x: -5i8,
            y: -48i8,
        },
        IC::Wait(2u8),
        IC::SpriteOverlay {
            sprite: SpriteId(380u16),
            x: -32i8,
            y: -32i8,
        },
        IC::Wait(5u8),
        IC::SpriteOverlay {
            sprite: SpriteId(380u16),
            x: -48i8,
            y: -2i8,
        },
        IC::Wait(3u8),
        IC::SpriteOverlay {
            sprite: SpriteId(380u16),
            x: -32i8,
            y: 32i8,
        },
        IC::Wait(5u8),
        IC::SpriteOverlay {
            sprite: SpriteId(380u16),
            x: 3i8,
            y: 48i8,
        },
        IC::Wait(63u8),
        IC::Wait(63u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PowerupConditionalJump(IscriptLabel(1u16)),
        IC::ImageUnderlay {
            image: ImageId(408u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(19u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(184u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::ImageOverlay {
        image: ImageId(190u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlaySound(SoundId(1000u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(222u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(223u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(224u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(225u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(226u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(227u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(228u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(229u16),
        },
        IC::Wait(4u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(490u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(283u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(852u16)),
        IC::ImageOverlay {
            image: ImageId(28u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(2u16),
    }],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlaySound(SoundId(470u16)),
        IC::ImageOverlay {
            image: ImageId(231u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(369u16)),
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(257u16),
            x: 0i8,
            y: 7i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::ImageOverlay {
        image: ImageId(201u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(940u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::PlaySound(SoundId(178u16)),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::NoBreakCodeStart,
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::CastSpell,
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(2u8)),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::TempRemoveGraphicStart,
        IC::WaitRandom { min: 2u8, max: 5u8 },
        IC::TempRemoveGraphicEnd,
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[IC::PlayFrame {
        frame: FrameId(102u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(50u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(18u16),
            x: 0i8,
            y: 7i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(4u16),
    }],
    &[IC::Wait(1u8)],
    &[
        IC::CastSpell,
        IC::SignalOrder(SignalId(1u8)),
        IC::TempRemoveGraphicStart,
        IC::ImageOverlay {
            image: ImageId(428u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SpriteOverlay {
            sprite: SpriteId(267u16),
            x: 0i8,
            y: -42i8,
        },
    ],
    &[
        IC::CastSpell,
        IC::SignalOrder(SignalId(2u8)),
        IC::GotoRepeatAttack,
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(352u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::ImageOverlay {
        image: ImageId(175u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::CastSpell,
        IC::SignalOrder(SignalId(2u8)),
        IC::GotoRepeatAttack,
    ],
    &[
        IC::Wait(13u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(162u16),
        x: 0i8,
        y: 0i8,
    }],
    &[IC::ImageOverlay {
        image: ImageId(181u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlaySound(SoundId(7u16)),
        IC::SpriteOverlay {
            sprite: SpriteId(222u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::PowerupConditionalJump(IscriptLabel(1u16)),
        IC::ImageUnderlay {
            image: ImageId(408u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(119u16),
        },
        IC::Wait(3u8),
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(3u16),
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(117u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::ImageUnderlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(149u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(50u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(198u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::SetVerticalPosition(0i8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageOverlay {
            image: ImageId(355u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::TempRemoveGraphicStart,
        IC::TriggerTargetRangeConditionalJump {
            distance: 40u16,
            label: IscriptLabel(447u16),
        },
        IC::TempRemoveGraphicEnd,
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(245u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SetVerticalPosition(0i8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SetFlipState(FlipState(1u8)),
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(50u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(367u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(539u16),
            x: 0i8,
            y: 0i8,
        },
        IC::NoBreakCodeStart,
        IC::Wait(3u8),
        IC::CastSpell,
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
        IC::SignalOrder(SignalId(2u8)),
    ],
    &[
        IC::PlaySound(SoundId(833u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(272u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(289u16),
        },
        IC::Wait(2u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(141u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySoundBetween {
            min: SoundId(35u16),
            max: SoundId(39u16),
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::TempRemoveGraphicStart,
        IC::TriggerTargetRangeConditionalJump {
            distance: 40u16,
            label: IscriptLabel(914u16),
        },
        IC::TempRemoveGraphicEnd,
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(74u16)),
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(67u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlaySound(SoundId(971u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(137u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(138u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(139u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(140u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(141u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(142u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(143u16),
        },
        IC::Wait(50u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(280u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(145u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(5u16),
    }],
    &[
        IC::PlaySound(SoundId(9u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(1u8),
    ],
    &[],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(131u16),
            x: 0i8,
            y: 42i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(191u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PowerupConditionalJump(IscriptLabel(444u16)),
        IC::ImageUnderlay {
            image: ImageId(409u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(277u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(277u16),
            x: 0i8,
            y: 0i8,
        },
        IC::ImageOverlay {
            image: ImageId(101u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(1095u16)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(142u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::SetVerticalPosition(0i8),
    ],
    &[],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(108u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::AttackWith(WeaponType::Ground),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(79u16)),
        IC::SpriteOverlay {
            sprite: SpriteId(365u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlaySound(SoundId(1018u16)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(50u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(50u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(556u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(255u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(272u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(7u16)),
        IC::Wait(1u8),
        IC::DoMissileDamage,
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySoundBetween {
            min: SoundId(82u16),
            max: SoundId(83u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::SpriteOverlay {
            sprite: SpriteId(309u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(163u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[IC::Wait(1u8)],
    &[
        IC::PlaySound(SoundId(77u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(1108u16)),
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::GroundSpriteOverlay {
            sprite: SpriteId(511u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoGroundDamage,
        IC::Wait(2u8),
    ],
    &[],
    &[IC::Wait(1u8)],
    &[
        IC::SetFlingyDirection(15u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlaySound(SoundId(816u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::DoMissileDamage,
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::TempRemoveGraphicStart,
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::TempRemoveGraphicEnd,
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::WaitRandom { min: 2u8, max: 3u8 },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(308u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(5u8),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(1u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(1u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
    ],
    &[
        IC::Wait(1u8),
        IC::ImageUnderlay {
            image: ImageId(252u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SetFlingyDirection(12u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(471u16)),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(8u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(8u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(8u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::SignalOrder(SignalId(16u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::PlaySound(SoundId(99u16)),
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(9u16)),
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(180u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::Wait(1u8),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::PlaySound(SoundId(471u16)),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(5u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::SignalOrder(SignalId(16u8)),
        IC::NoBreakCodeEnd,
    ],
    &[
        IC::PlaySound(SoundId(815u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(136u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(153u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(170u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(2u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(139u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PowerupConditionalJump(IscriptLabel(1164u16)),
        IC::ImageUnderlay {
            image: ImageId(413u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(2u8),
        IC::Wait(1u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::ImageOverlay {
            image: ImageId(108u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::PlaySound(SoundId(95u16)),
        IC::DoMissileDamage,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(1u16),
    }],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::TempRemoveGraphicEnd,
        IC::Wait(5u8),
        IC::TempRemoveGraphicStart,
        IC::Wait(5u8),
    ],
    &[
        IC::PlayFrameTile {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(346u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(15u8),
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(1u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(0u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(1u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
        IC::CreateGasOverlays(GasOverlay(2u8)),
        IC::WaitRandom {
            min: 5u8,
            max: 50u8,
        },
    ],
    &[
        IC::PlaySound(SoundId(922u16)),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(3u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(3u8),
    ],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(2u8),
        IC::SignalOrder(SignalId(4u8)),
        IC::NoBreakCodeEnd,
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::ImageOverlay {
            image: ImageId(109u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(4u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(4u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(85u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
    ],
    &[IC::ImageOverlay {
        image: ImageId(114u16),
        x: 0i8,
        y: 0i8,
    }],
    &[IC::PlayFrame {
        frame: FrameId(4u16),
    }],
    &[
        IC::PlaySoundBetween {
            min: SoundId(651u16),
            max: SoundId(652u16),
        },
        IC::ImageOverlay {
            image: ImageId(214u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::Wait(1u8),
    ],
    &[IC::OrderDone(SignalId(1u8))],
    &[
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(19u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(20u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(21u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(22u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(23u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(24u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(25u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(26u16),
        },
        IC::Wait(2u8),
        IC::ImageOverlay {
            image: ImageId(135u16),
            x: 0i8,
            y: 0i8,
        },
        IC::ImageOverlay {
            image: ImageId(136u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SignalOrder(SignalId(4u8)),
        IC::NoBreakCodeEnd,
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlaySound(SoundId(79u16)),
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::DoMissileDamage,
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(92u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[
        IC::TempRemoveGraphicStart,
        IC::WaitRandom { min: 1u8, max: 5u8 },
        IC::TempRemoveGraphicEnd,
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(222u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(223u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(224u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(225u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(226u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(227u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(228u16),
        },
        IC::Wait(2u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(236u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlaySound(SoundId(317u16)),
        IC::ImageOverlay {
            image: ImageId(333u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlaySound(SoundId(317u16)),
        IC::ImageOverlay {
            image: ImageId(333u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[
        IC::ImageOverlay {
            image: ImageId(977u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[
        IC::NoBreakCodeStart,
        IC::CastSpell,
        IC::SignalOrder(SignalId(2u8)),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(202u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::Wait(1u8), IC::SwitchUnderlay(ImageId(414u16))],
    &[
        IC::WarpOverlay(FrameId(0u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(1u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(2u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(3u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(4u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(5u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(6u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(7u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(8u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(9u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(10u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(11u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(12u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(13u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(14u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(15u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(16u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(17u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(18u16)),
        IC::Wait(1u8),
        IC::WarpOverlay(FrameId(19u16)),
        IC::SignalOrder(SignalId(1u8)),
        IC::Wait(1u8),
    ],
    &[IC::Wait(1u8)],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(5u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(6u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlaySound(SoundId(9u16)),
        IC::ImageOverlay {
            image: ImageId(530u16),
            x: 0i8,
            y: 0i8,
        },
        IC::DoMissileDamage,
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::SetFlingyDirection(12u8),
        IC::ImageOverlayUseLo {
            image: ImageId(251u16),
            x: 2i8,
            y: 0i8,
        },
        IC::Wait(1u8),
        IC::ImageUnderlay {
            image: ImageId(252u16),
            x: 0i8,
            y: 0i8,
        },
        IC::SetFlingyDirection(12u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::Wait(1u8)],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlayFrame {
            frame: FrameId(15u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(16u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(18u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(19u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(20u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(21u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(22u16),
        },
        IC::Wait(2u8),
        IC::SignalOrder(SignalId(4u8)),
        IC::NoBreakCodeEnd,
    ],
    &[IC::PlayFrame {
        frame: FrameId(2u16),
    }],
    &[IC::ImageOverlay {
        image: ImageId(286u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(240u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(107u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(130u16),
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(6u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::WaitRandom {
            min: 25u8,
            max: 30u8,
        },
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::WaitRandom {
            min: 25u8,
            max: 30u8,
        },
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(229u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::WaitRandom {
            min: 63u8,
            max: 75u8,
        },
        IC::RandomConditionalJump {
            chance: 25u8,
            label: IscriptLabel(155u16),
        },
        IC::RandomConditionalJump {
            chance: 128u8,
            label: IscriptLabel(154u16),
        },
    ],
    &[IC::Wait(1u8)],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[IC::ImageOverlay {
        image: ImageId(150u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::ImageUnderlay {
            image: ImageId(957u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
    ],
    &[IC::SignalOrder(SignalId(4u8))],
    &[
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlaySoundRandom {
            num_sounds: 3u8,
            sounds: &[SoundId(891u16), SoundId(892u16), SoundId(893u16)],
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::AttackMelee {
            num_sounds: 2u8,
            sounds: &[SoundId(894u16), SoundId(895u16)],
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(2u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[
        IC::PlaySound(SoundId(774u16)),
        IC::SpriteOverlay {
            sprite: SpriteId(185u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(65u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(423u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(306u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(323u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(340u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(357u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(374u16),
        },
        IC::Wait(1u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::UseWeapon(WeaponId(82u8)),
        IC::SignalOrder(SignalId(1u8)),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::WaitRandom { min: 1u8, max: 3u8 },
        IC::PlayFrame {
            frame: FrameId(1u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(2u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(3u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(4u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(1u8),
        IC::NoBreakCodeStart,
        IC::PlaySoundRandom {
            num_sounds: 3u8,
            sounds: &[SoundId(891u16), SoundId(892u16), SoundId(893u16)],
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::AttackMelee {
            num_sounds: 2u8,
            sounds: &[SoundId(894u16), SoundId(895u16)],
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(187u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(204u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(221u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(2u8),
        IC::NoBreakCodeEnd,
        IC::GotoRepeatAttack,
    ],
    &[
        IC::ImageUnderlay {
            image: ImageId(339u16),
            x: 0i8,
            y: 0i8,
        },
        IC::PlayFrame {
            frame: FrameId(102u16),
        },
    ],
    &[
        IC::PlaySound(SoundId(774u16)),
        IC::SpriteOverlay {
            sprite: SpriteId(185u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageUnderlay {
            image: ImageId(265u16),
            x: 0i8,
            y: 0i8,
        },
    ],
    &[
        IC::PlaySound(SoundId(8u16)),
        IC::ImageOverlay {
            image: ImageId(332u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(3u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(7u16),
        },
        IC::Wait(1u8),
        IC::WaitRandom {
            min: 10u8,
            max: 15u8,
        },
        IC::PlayFrame {
            frame: FrameId(8u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(1u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(14u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(13u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(12u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(11u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(10u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(9u16),
        },
        IC::Wait(2u8),
        IC::SignalOrder(SignalId(4u8)),
    ],
    &[
        IC::SetVerticalPosition(0i8),
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(17u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(34u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(51u16),
        },
        IC::Wait(2u8),
        IC::PlayFrame {
            frame: FrameId(68u16),
        },
        IC::Wait(2u8),
    ],
    &[
        IC::PlayFrame {
            frame: FrameId(0u16),
        },
        IC::ImageOverlay {
            image: ImageId(586u16),
            x: -126i8,
            y: -77i8,
        },
        IC::ImageOverlay {
            image: ImageId(585u16),
            x: 126i8,
            y: 77i8,
        },
        IC::ImageOverlay {
            image: ImageId(587u16),
            x: -126i8,
            y: 77i8,
        },
        IC::SetPosition { x: 126i8, y: -77i8 },
    ],
    &[IC::PlayFrame {
        frame: FrameId(0u16),
    }],
    &[IC::ImageOverlay {
        image: ImageId(125u16),
        x: 0i8,
        y: 0i8,
    }],
    &[
        IC::PlaySound(SoundId(67u16)),
        IC::SetFlingyDirection(0u8),
        IC::PlayFrame {
            frame: FrameId(238u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(239u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(240u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(241u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(242u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(243u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(244u16),
        },
        IC::Wait(1u8),
        IC::PlayFrame {
            frame: FrameId(245u16),
        },
        IC::Wait(1u8),
        IC::LowSpriteUnderlay {
            sprite: SpriteId(236u16),
            x: 0i8,
            y: 0i8,
        },
        IC::Wait(1u8),
    ],
];
