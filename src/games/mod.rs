//! The fourteen games and the list the menu shows them in.

pub mod array_guard;
pub mod drift;
pub mod heatsink;
pub mod learn_to_code;
pub mod merge_gates;
pub mod relay;
pub mod rooms;
pub mod signal;
pub mod slipstream;
pub mod stackline;
pub mod termle;
pub mod through_ball;
pub mod touch_typing;
pub mod trace;

use crate::game::{Game, Info, Setup};

/// Every game in menu order.
pub const ALL: [&Info; 14] = [
    &stackline::INFO,
    &merge_gates::INFO,
    &termle::INFO,
    &signal::INFO,
    &trace::INFO,
    &drift::INFO,
    &rooms::INFO,
    &relay::INFO,
    &through_ball::INFO,
    &slipstream::INFO,
    &array_guard::INFO,
    &heatsink::INFO,
    &touch_typing::INFO,
    &learn_to_code::INFO,
];

/// Finds a game by its id or by a short form of its name, ignoring case and spaces.
pub fn find(name: &str) -> Option<&'static Info> {
    let wanted: String = name
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    ALL.iter().copied().find(|i| {
        let id: String = i.id.chars().filter(|c| c.is_alphanumeric()).collect();
        id == wanted || (wanted.len() >= 3 && id.starts_with(&wanted))
    })
}

pub fn make(id: &str, setup: Setup) -> Option<Box<dyn Game>> {
    Some(match id {
        "stackline" => Box::new(stackline::Stackline::new(setup)),
        "merge-gates" => Box::new(merge_gates::MergeGates::new(setup)),
        "termle" => Box::new(termle::Termle::new(setup)),
        "signal" => Box::new(signal::Signal::new(setup)),
        "trace" => Box::new(trace::Trace::new(setup)),
        "drift" => Box::new(drift::Drift::new(setup)),
        "rooms" => Box::new(rooms::Rooms::new(setup)),
        "relay" => Box::new(relay::Relay::new(setup)),
        "through-ball" => Box::new(through_ball::ThroughBall::new(setup)),
        "slipstream" => Box::new(slipstream::Slipstream::new(setup)),
        "array-guard" => Box::new(array_guard::ArrayGuard::new(setup)),
        "heatsink" => Box::new(heatsink::Heatsink::new(setup)),
        "touch-typing" => Box::new(touch_typing::TouchTyping::new(setup)),
        "learn-to-code" => Box::new(learn_to_code::LearnToCode::new(setup)),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_find_their_games() {
        assert_eq!(find("Merge Gates").unwrap().id, "merge-gates");
        assert_eq!(find("mergegates").unwrap().id, "merge-gates");
        assert_eq!(find("STACK").unwrap().id, "stackline");
        assert!(find("xx").is_none());
        assert!(find("tetris").is_none());
    }

    #[test]
    fn every_game_has_its_own_id_icon_and_colour() {
        let mut ids: Vec<_> = ALL.iter().map(|i| i.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), ALL.len());
        let mut hues: Vec<_> = ALL.iter().map(|i| format!("{:?}", i.hue)).collect();
        hues.sort();
        hues.dedup();
        assert_eq!(hues.len(), ALL.len());
        for info in ALL {
            assert!(
                make(
                    info.id,
                    Setup {
                        seed: 1,
                        reduced_motion: false,
                        day: 1,
                        practice: false
                    }
                )
                .is_some()
            );
        }
    }
}
