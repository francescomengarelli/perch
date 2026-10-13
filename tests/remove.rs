mod common;

use common::Sandbox;
use predicates::str::contains;

const COMMON_ONLY: &str = "[[module]]\nname = \"common\"\n";

#[test]
fn restores_a_single_file() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write(".zshrc", "z");
    sb.perch(&["add", "common", ".zshrc"]).assert().success();

    sb.perch(&["remove", ".zshrc"]).assert().success();

    assert!(!sb.is_link(".zshrc"));
    assert_eq!(sb.read(".zshrc"), "z");
    assert!(!sb.path("dotfiles/common/.zshrc").exists());
    assert!(sb.path("dotfiles/common").is_dir());
}

#[test]
fn restores_a_directory_and_tidies_the_repo() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write(".config/app/a", "a");
    sb.write(".config/app/sub/b", "b");
    sb.perch(&["add", "common", ".config/app"])
        .assert()
        .success();
    sb.write(".config/app/local", "not managed");

    sb.perch(&["remove", "~/.config/app"]).assert().success();

    assert_eq!(sb.read(".config/app/a"), "a");
    assert_eq!(sb.read(".config/app/sub/b"), "b");
    assert!(!sb.is_link(".config/app/sub/b"));
    assert_eq!(sb.read(".config/app/local"), "not managed");
    assert!(!sb.path("dotfiles/common/.config").exists());
}

#[test]
fn skips_paths_perch_does_not_manage() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write(".plain", "p");

    sb.perch(&["remove", ".plain"])
        .assert()
        .success()
        .stderr(contains("isn't managed by me"));
    assert_eq!(sb.read(".plain"), "p");
}

#[test]
fn removed_files_are_not_pruned() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write(".a", "a");
    sb.perch(&["add", "common", ".a"]).assert().success();
    sb.perch(&["remove", ".a"]).assert().success();

    sb.perch(&["prune"]).assert().success();
    assert_eq!(sb.read(".a"), "a");
}
