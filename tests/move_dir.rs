mod common;

use common::Sandbox;
use predicates::str::contains;
use std::{fs, os::unix::fs::symlink};

const CONFIG: &str =
    "# keep me\ndotfiles_dir = \"~/dotfiles\" # here\n\n[[module]]\nname = \"common\"\n";

fn synced() -> Sandbox {
    let sb = Sandbox::new();
    sb.config(CONFIG);
    sb.write("dotfiles/common/.a", "a");
    sb.write("dotfiles/common/.config/x/y", "y");
    sb.perch(&["sync"]).assert().success();
    sb
}

#[test]
fn moves_repo_and_relinks() {
    let sb = synced();

    sb.perch(&["move-dir", "~/new/dots"]).assert().success();

    assert!(!sb.dotfiles().exists());
    sb.assert_link(".a", "new/dots/common/.a");
    sb.assert_link(".config/x/y", "new/dots/common/.config/x/y");
    assert_eq!(
        sb.read(".config/perch/config.toml"),
        CONFIG.replace("\"~/dotfiles\"", "\"~/new/dots\"")
    );

    // a second sync sees everything in place
    sb.perch(&["sync"]).assert().success();
}

#[test]
fn refuses_existing_destination() {
    let sb = synced();
    sb.write("taken/file", "precious");

    sb.perch(&["move-dir", "~/taken"])
        .assert()
        .failure()
        .stderr(contains("already exists"));

    assert_eq!(sb.read("taken/file"), "precious");
    sb.assert_link(".a", "dotfiles/common/.a");
    assert_eq!(sb.read(".config/perch/config.toml"), CONFIG);
}

#[test]
fn accepts_empty_destination() {
    let sb = synced();
    sb.mkdir("empty");

    sb.perch(&["move-dir", "~/empty"]).assert().success();
    sb.assert_link(".a", "empty/common/.a");
}

#[test]
fn refuses_moving_into_itself() {
    let sb = synced();

    sb.perch(&["move-dir", "~/dotfiles/inner"])
        .assert()
        .failure()
        .stderr(contains("inside the other"));
    sb.assert_link(".a", "dotfiles/common/.a");
}

#[test]
fn preserves_symlinks_and_empty_dirs_in_repo() {
    let sb = synced();
    symlink(".a", sb.path("dotfiles/common/.alias")).unwrap();
    sb.mkdir("dotfiles/empty");

    sb.perch(&["move-dir", "~/moved"]).assert().success();

    assert_eq!(
        fs::read_link(sb.path("moved/common/.alias")).unwrap(),
        std::path::Path::new(".a")
    );
    assert!(sb.path("moved/empty").is_dir());
}

#[test]
fn leaves_foreign_links_alone() {
    let sb = synced();
    sb.write("elsewhere/.b", "b");
    sb.write("dotfiles/common/.b", "repo b");
    symlink(sb.path("elsewhere/.b"), sb.path(".b")).unwrap();

    sb.perch(&["move-dir", "~/moved"]).assert().success();

    assert_eq!(
        fs::read_link(sb.path(".b")).unwrap(),
        sb.path("elsewhere/.b")
    );
}

#[test]
fn updates_config_that_lives_in_the_repo() {
    let sb = Sandbox::new();
    sb.write("dotfiles/common/.config/perch/config.toml", CONFIG);
    sb.write("dotfiles/common/.a", "a");
    sb.perch(&[
        "sync",
        "--config",
        "~/dotfiles/common/.config/perch/config.toml",
    ])
    .assert()
    .success();
    sb.assert_link(
        ".config/perch/config.toml",
        "dotfiles/common/.config/perch/config.toml",
    );

    sb.perch(&["move-dir", "~/moved"]).assert().success();

    sb.assert_link(
        ".config/perch/config.toml",
        "moved/common/.config/perch/config.toml",
    );
    assert!(
        sb.read("moved/common/.config/perch/config.toml")
            .contains("dotfiles_dir = \"~/moved\" # here")
    );
}

#[test]
fn updates_config_passed_from_inside_the_repo() {
    let sb = Sandbox::new();
    sb.write("dotfiles/common/perch.toml", CONFIG);
    sb.write("dotfiles/common/.a", "a");
    sb.perch(&["--config", "dotfiles/common/perch.toml", "sync"])
        .assert()
        .success();

    sb.perch(&[
        "--config",
        "dotfiles/common/perch.toml",
        "move-dir",
        "~/moved",
    ])
    .assert()
    .success();

    sb.assert_link(".a", "moved/common/.a");
    assert!(
        sb.read("moved/common/perch.toml")
            .contains("dotfiles_dir = \"~/moved\" # here")
    );
}
