//! Sounds in the story format: built-in names, audio files, effects that make
//! their own sound, background loops by scene, mood and story, and validation.

mod common;

use common::{stories, temp_dir, template};
use theatre_engine::StoryPack;
use theatre_engine::scene::{LineFx, Scene};
use theatre_engine::sound::{AmbienceRef, Ambient, Audio, Sfx, SoundRef, sound_for};

#[test]
fn every_built_in_name_loads() {
    for s in Sfx::ALL {
        let scene: Scene = toml::from_str(&format!(
            "text = \"x\"\nsound = \"{0}\"\ndialogue = [{{ who = \"a\", line = \"b\", sound = \"{0}\" }}]",
            s.name()
        ))
        .unwrap();
        assert_eq!(scene.sound, Some(Audio::Builtin(s)));
        assert_eq!(scene.dialogue[0].sound, Some(Audio::Builtin(s)));
    }
    for a in Ambient::ALL {
        let scene: Scene =
            toml::from_str(&format!("text = \"x\"\nambience = \"{}\"", a.name())).unwrap();
        assert_eq!(scene.ambience, Some(Audio::Builtin(a)));
    }
}

#[test]
fn files_and_unknown_names() {
    let scene: Scene =
        toml::from_str("text = \"x\"\nsound = \"sounds/door.ogg\"\nambience = \"club.wav\"")
            .unwrap();
    assert_eq!(scene.sound, Some(Audio::File("sounds/door.ogg".into())));
    assert_eq!(scene.ambience, Some(Audio::File("club.wav".into())));
    let err = toml::from_str::<Scene>("text = \"x\"\nsound = \"gunshoot\"")
        .expect_err("should fail")
        .to_string();
    assert!(err.contains("gunshoot") && err.contains("gunshot"), "{err}");
    let err = toml::from_str::<Scene>("text = \"x\"\nambience = \"jungle\"")
        .expect_err("should fail")
        .to_string();
    assert!(err.contains("jungle") && err.contains("rain"), "{err}");
}

#[test]
fn effects_make_their_own_sound_unless_told() {
    let gunshot: SoundRef = Audio::Builtin(Sfx::Gunshot);
    let quiet: SoundRef = Audio::Builtin(Sfx::Silence);
    assert_eq!(
        sound_for(None, Some(LineFx::Lightning)),
        Some(Audio::Builtin(Sfx::Thunder))
    );
    assert_eq!(
        sound_for(Some(&gunshot), Some(LineFx::Shake)),
        Some(gunshot.clone())
    );
    assert_eq!(sound_for(Some(&quiet), Some(LineFx::Shake)), None);
    assert_eq!(sound_for(None, None), None);
    for fx in LineFx::ALL {
        assert_ne!(fx.sound(), Sfx::Silence, "{fx:?} should make a sound");
    }
}

#[test]
fn background_by_scene_then_mood_then_story() {
    let pack = template();
    let rain: AmbienceRef = Audio::Builtin(Ambient::Rain);
    assert_eq!(pack.ambience_of("opening"), Some(&rain), "the story's");
    assert_eq!(
        pack.ambience_of("silence"),
        Some(&Audio::Builtin(Ambient::Wind)),
        "the scene's own"
    );
    assert_eq!(
        pack.meta.ambience_moods.values().next(),
        Some(&Audio::Builtin(Ambient::Drone))
    );
}

/// The template copied to a temporary folder, with `edit` applied to its first scene file.
fn template_with(name: &str, edit: impl Fn(String) -> String) -> std::path::PathBuf {
    let dir = temp_dir(name);
    let src = stories().join("_template");
    for sub in ["scenes", "characters", "images"] {
        std::fs::create_dir_all(dir.join(sub)).unwrap();
        for f in std::fs::read_dir(src.join(sub)).unwrap() {
            let f = f.unwrap().path();
            std::fs::copy(&f, dir.join(sub).join(f.file_name().unwrap())).unwrap();
        }
    }
    std::fs::copy(src.join("story.toml"), dir.join("story.toml")).unwrap();
    let scenes = dir.join("scenes/01_beginning.toml");
    let text = std::fs::read_to_string(&scenes)
        .unwrap()
        // Windows checkouts may have CRLF line endings
        .replace("\r\n", "\n");
    std::fs::write(&scenes, edit(text)).unwrap();
    dir
}

#[test]
fn audio_files_must_exist_and_be_playable() {
    let dir = template_with("sound-missing", |t| {
        t.replace("sound = \"knock\"", "sound = \"sounds/knock.ogg\"")
    });
    let err = format!("{:#}", StoryPack::load(&dir).err().expect("should fail"));
    assert!(
        err.contains("scene 'opening'") && err.contains("not found"),
        "{err}"
    );

    std::fs::create_dir_all(dir.join("sounds")).unwrap();
    std::fs::write(dir.join("sounds/knock.ogg"), b"not really ogg").unwrap();
    StoryPack::load(&dir).expect("an existing .ogg is accepted (decoded when played)");

    std::fs::write(dir.join("sounds/knock.txt"), b"x").unwrap();
    let scenes = dir.join("scenes/01_beginning.toml");
    let text = std::fs::read_to_string(&scenes)
        .unwrap()
        // Windows checkouts may have CRLF line endings
        .replace("\r\n", "\n");
    std::fs::write(
        &scenes,
        text.replace("sounds/knock.ogg", "sounds/knock.txt"),
    )
    .unwrap();
    let err = format!("{:#}", StoryPack::load(&dir).err().expect("should fail"));
    assert!(err.contains(".ogg"), "{err}");
    std::fs::remove_dir_all(dir).ok();
}
