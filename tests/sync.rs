mod common;

use common::Sandbox;
use predicates::str::contains;

const COMMON_ONLY: &str = "[[module]]\nname = \"common\"\n";

#[test]
fn links_files_from_active_modules() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write("dotfiles/common/.zshrc", "z");
    sb.write("dotfiles/common/.config/nvim/init.lua", "n");

    sb.perch(&["sync"]).assert().success();

    sb.assert_link(".zshrc", "dotfiles/common/.zshrc");
    sb.assert_link(
        ".config/nvim/init.lua",
        "dotfiles/common/.config/nvim/init.lua",
    );
    // parents are real directories
    assert!(!sb.is_link(".config/nvim"));
}

#[test]
fn resync_is_a_noop() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write("dotfiles/common/.zshrc", "z");

    sb.perch(&["sync"]).assert().success();
    sb.perch(&["sync"]).assert().success();
    sb.assert_link(".zshrc", "dotfiles/common/.zshrc");
}

#[test]
fn skips_inactive_modules() {
    let sb = Sandbox::new();
    sb.config("[[module]]\nname = \"common\"\n\n[[module]]\nname = \"off\"\nwhen = \"false\"\n");
    sb.write("dotfiles/common/.a", "a");
    sb.write("dotfiles/off/.b", "b");

    sb.perch(&["sync"]).assert().success();
    sb.assert_link(".a", "dotfiles/common/.a");
    assert!(!sb.path(".b").exists());
}

#[test]
fn conflict_aborts_without_changes() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write("dotfiles/common/.a", "repo");
    sb.write("dotfiles/common/.b", "repo");
    sb.write(".b", "real");

    sb.perch(&["sync"])
        .assert()
        .failure()
        .stderr(contains("is not mine"))
        .stderr(contains("no changes applied"));

    assert!(!sb.path(".a").exists());
    assert_eq!(sb.read(".b"), "real");
}

#[test]
fn honors_perchignore() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write("dotfiles/.perchignore", ".DS_Store\ncommon/.config/a/\n");
    sb.write("dotfiles/common/.DS_Store", "x");
    sb.write("dotfiles/common/.config/a/x", "x");
    sb.write("dotfiles/common/.config/b/.perchignore", "secret\n");
    sb.write("dotfiles/common/.config/b/secret", "x");
    sb.write("dotfiles/common/.config/b/keep", "x");

    sb.perch(&["sync"]).assert().success();

    assert!(!sb.path(".DS_Store").exists());
    assert!(!sb.path(".config/a/x").exists());
    assert!(!sb.path(".config/b/secret").exists());
    sb.assert_link(".config/b/keep", "dotfiles/common/.config/b/keep");
    assert!(!sb.path(".config/b/.perchignore").exists());
}

#[test]
fn skips_active_modules_without_a_directory() {
    let sb = Sandbox::new();
    // no config: defaults are common + linux/macos, but only common exists
    sb.write("dotfiles/common/.a", "a");

    sb.perch(&["sync"])
        .assert()
        .success()
        .stderr(contains("doesn't exist — skipping"))
        .stderr(contains("1 modules synced"));
    sb.assert_link(".a", "dotfiles/common/.a");
}

#[test]
fn dry_run_changes_nothing() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write("dotfiles/common/.a", "a");
    sb.write("dotfiles/common/.b", "b");
    sb.write(".b", "real");

    sb.perch(&["sync", "--dry-run"])
        .assert()
        .failure()
        .stdout(contains("link ~/.a -> ~/dotfiles/common/.a"))
        .stdout(contains("conflict ~/.b"))
        .stderr(contains("nothing changed"));

    assert!(!sb.path(".a").exists());
    assert!(!sb.state_dir().join("perch/links").exists());
}

#[test]
fn dry_run_succeeds_without_conflicts() {
    let sb = Sandbox::new();
    sb.config(COMMON_ONLY);
    sb.write("dotfiles/common/.a", "a");

    sb.perch(&["sync", "-n"])
        .assert()
        .success()
        .stdout(contains("link ~/.a"));
    assert!(!sb.path(".a").exists());
}
