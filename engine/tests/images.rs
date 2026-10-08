//! Scene pictures: paths resolve inside the story folder and bad ones are caught.

mod common;

use common::{stories, temp_dir, template};
use theatre_engine::StoryPack;

#[test]
fn image_path_resolves_inside_the_story() {
    let pack = template();
    let path = pack
        .image_path("opening")
        .expect("the template's opening has a picture");
    assert!(path.ends_with("_template/images/rain.png"));
    assert!(path.is_file());
    assert_eq!(pack.image_path("silence"), None);
    assert!(pack.scenes["opening"].has_art());
}

/// A copy of the template with the opening's picture changed.
fn template_with_image(name: &str, image_line: &str) -> std::path::PathBuf {
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
        .replace("image = \"images/rain.png\"", image_line);
    std::fs::write(scenes, text).unwrap();
    dir
}

#[test]
fn missing_picture_is_an_error() {
    let dir = template_with_image("img-missing", "image = \"images/nope.png\"");
    let err = format!("{:#}", StoryPack::load(&dir).err().expect("should fail"));
    assert!(
        err.contains("images/nope.png") && err.contains("not found"),
        "{err}"
    );
    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn only_png_and_jpeg_are_accepted() {
    let dir = template_with_image("img-ext", "image = \"story.toml\"");
    let full = format!("{:#}", StoryPack::load(&dir).err().expect("should fail"));
    assert!(full.contains(".png, .jpg or .svg"), "{full}");
    std::fs::remove_dir_all(dir).ok();
}
