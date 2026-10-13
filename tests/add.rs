mod common;

use common::Sandbox;
use predicates::str::contains;

const COMMON_ONLY: &str = "[[module]]\nname = \"common\"\n";

#[test]
fn moves_file_into_module_and_links_back() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write(".zshrc", "z");

    sb.perch(&["add", "common", ".zshrc"]).assert().success();

    sb.assert_link(".zshrc", "dotfiles/common/.zshrc");
    assert_eq!(sb.read("dotfiles/common/.zshrc"), "z");
}

#[test]
fn adopts_directories_recursively() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write(".config/zathura/zathurarc", "a");
    sb.write(".config/zathura/.hidden", "b");

    sb.perch(&["add", "common", ".config/zathura"])
        .assert()
        .success();

    sb.assert_link(
        ".config/zathura/zathurarc",
        "dotfiles/common/.config/zathura/zathurarc",
    );
    sb.assert_link(
        ".config/zathura/.hidden",
        "dotfiles/common/.config/zathura/.hidden",
    );
}

#[test]
fn refuses_when_destination_exists() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write(".a", "new");
    sb.write("dotfiles/common/.a", "old");

    sb.perch(&["add", "common", ".a"])
        .assert()
        .failure()
        .stderr(contains("not overwriting"));

    assert_eq!(sb.read(".a"), "new");
    assert!(!sb.is_link(".a"));
}

#[test]
fn skips_paths_outside_home() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    let outside = tempfile::NamedTempFile::new().unwrap();

    sb.perch(&["add", "common", outside.path().to_str().unwrap()])
        .assert()
        .success()
        .stderr(contains("outside your home directory"));

    assert!(outside.path().exists());
}

#[test]
fn honors_root_perchignore() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write(
        "dotfiles/.perchignore",
        ".DS_Store\ncommon/.config/app/cache/\n",
    );
    sb.write(".config/app/config", "c");
    sb.write(".config/app/.DS_Store", "x");
    sb.write(".config/app/cache/blob", "x");

    sb.perch(&["add", "common", ".config/app"])
        .assert()
        .success();

    sb.assert_link(".config/app/config", "dotfiles/common/.config/app/config");
    assert!(!sb.is_link(".config/app/.DS_Store"));
    assert!(!sb.is_link(".config/app/cache/blob"));
    assert!(!sb.path("dotfiles/common/.config/app/.DS_Store").exists());
}

#[test]
fn warns_about_unknown_module() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write(".a", "a");

    sb.perch(&["add", "typo", ".a"])
        .assert()
        .success()
        .stderr(contains("isn't in your config"));
}

#[test]
fn warns_about_inactive_module() {
    let sb = Sandbox::new();
    sb.config("[[module]]\nname = \"off\"\nwhen = \"false\"\n");
    sb.write(".a", "a");

    sb.perch(&["add", "off", ".a"])
        .assert()
        .success()
        .stderr(contains("isn't active on this machine"));
}
