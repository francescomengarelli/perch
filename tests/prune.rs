mod common;

use common::Sandbox;
use predicates::str::contains;
use std::{fs, os::unix::fs::symlink};

fn synced(config: &str) -> Sandbox {
    let sb = Sandbox::new();
    sb.config(config);
    sb.write("dotfiles/common/.a", "a");
    sb.write("dotfiles/common/.b", "b");
    sb.write("dotfiles/work/.w", "w");
    sb.perch(&["sync"]).assert().success();
    sb
}

const BOTH: &str = "[[module]]\nname = \"common\"\n\n[[module]]\nname = \"work\"\n";

#[test]
fn prunes_links_whose_file_is_gone() {
    let sb = synced(BOTH);
    fs::remove_file(sb.path("dotfiles/common/.b")).unwrap();

    sb.perch(&["prune"])
        .assert()
        .success()
        .stderr(contains("1 stale links pruned"));

    assert!(sb.path(".b").symlink_metadata().is_err());
    sb.assert_link(".a", "dotfiles/common/.a");
}

#[test]
fn prunes_links_of_inactive_modules() {
    let sb = synced(BOTH);
    sb.config("[[module]]\nname = \"common\"\n\n[[module]]\nname = \"work\"\nwhen = \"false\"\n");

    sb.perch(&["prune"]).assert().success();

    assert!(sb.path(".w").symlink_metadata().is_err());
    assert!(sb.path("dotfiles/work/.w").exists());
    sb.assert_link(".a", "dotfiles/common/.a");
}

#[test]
fn dry_run_lists_without_deleting() {
    let sb = synced(BOTH);
    fs::remove_file(sb.path("dotfiles/common/.b")).unwrap();

    sb.perch(&["prune", "--dry-run"])
        .assert()
        .success()
        .stdout(contains("prune ~/.b (its file is gone)"));

    assert!(sb.is_link(".b"));
}

#[test]
fn leaves_foreign_links_alone() {
    let sb = synced(BOTH);
    // replace one of our links with someone else's dangling link
    fs::remove_file(sb.path(".a")).unwrap();
    symlink(sb.path("nowhere"), sb.path(".a")).unwrap();

    sb.perch(&["prune"])
        .assert()
        .success()
        .stderr(contains("0 stale links pruned"));
    assert!(sb.is_link(".a"));
}

#[test]
fn asks_for_a_sync_first() {
    let sb = Sandbox::new();
    sb.perch(&["prune"])
        .assert()
        .success()
        .stderr(contains("run perch sync once"));
}
