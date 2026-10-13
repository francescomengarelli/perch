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
