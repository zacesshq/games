//! The background sound under each game: one of the Zacess Scenes ambient loops, chosen to suit
//! the game, such as a machine room under Heatsink or distant traffic under Slipstream. It plays
//! beside the music, is off until asked for and never carries meaning of its own.
//!
//! Like the music it is kept in a static, so the terminal guard can stop it on every way out.

use std::sync::Mutex;

use zacess_scenes::ambient::{Ambience, Ambient};

use crate::sound::Sound;

static PLAYING: Mutex<Option<Ambient>> = Mutex::new(None);

/// The loop that suits each game. A game not listed gets the quiet night bed.
pub fn for_game(id: &str) -> Ambience {
    match id {
        "stackline" | "merge-gates" | "trace" | "rooms" | "relay" | "heatsink" => Ambience::Hum,
        "termle" | "signal" => Ambience::Pad,
        "drift" => Ambience::Birds,
        "through-ball" => Ambience::Wind,
        "slipstream" => Ambience::City,
        "array-guard" => Ambience::Night,
        "touch-typing" | "learn-to-code" => Ambience::Rain,
        _ => Ambience::Night,
    }
}

/// Stops the background sound.
pub fn stop() {
    let taken = PLAYING.lock().ok().and_then(|mut p| p.take());
    if let Some(mut ambient) = taken {
        ambient.stop();
    }
}

/// Plays the background sound for a game. `None` stops it. Called every frame: the loop starts
/// again as it ends and switches when the game or the volume changes.
pub fn keep(sound: &Sound, game: Option<&str>) {
    let Some(id) = game.filter(|_| sound.background_on()) else {
        stop();
        return;
    };
    let Ok(mut playing) = PLAYING.lock() else {
        return;
    };
    let ambient = playing.get_or_insert_with(|| Ambient::new(true, sound.background_volume));
    ambient.set_volume(sound.background_volume);
    ambient.set_ambience(for_game(id));
    ambient.tick();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::games;

    #[test]
    fn every_game_has_a_background_sound_that_suits_it() {
        for info in games::ALL {
            let ambience = for_game(info.id);
            assert!(Ambience::ALL.contains(&ambience), "{}", info.id);
        }
        assert_eq!(for_game("heatsink"), Ambience::Hum);
        assert_eq!(for_game("slipstream"), Ambience::City);
        assert_eq!(for_game("drift"), Ambience::Birds);
        // every game named in the table is a real game, so a renamed one cannot fall through
        for id in [
            "stackline",
            "merge-gates",
            "trace",
            "rooms",
            "relay",
            "heatsink",
            "termle",
            "signal",
            "drift",
            "through-ball",
            "slipstream",
            "array-guard",
            "touch-typing",
            "learn-to-code",
        ] {
            assert!(games::ALL.iter().any(|i| i.id == id), "{id}");
        }
    }

    #[test]
    fn a_silent_sound_never_starts_the_background() {
        let mut sound = Sound::silent();
        sound.background_enabled = true;
        keep(&sound, Some("heatsink"));
        assert!(PLAYING.lock().unwrap().is_none());
    }
}
