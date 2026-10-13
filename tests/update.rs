mod common;

use common::Sandbox;
use std::fs;

/// A dotfiles repo cloned from a local bare remote, plus a second clone
/// ("another machine") that can push changes.
fn with_remote() -> Sandbox {
    let sb = Sandbox::new();
    fs::remove_dir(sb.dotfiles()).unwrap();
    sb.git(
        &sb.home,
        &["init", "-q", "--bare", "-b", "main", "remote.git"],
    );
    sb.git(&sb.home, &["clone", "-q", "remote.git", "dotfiles"]);

    sb.write(
        "dotfiles/common/.config/perch/config.toml",
        "[[module]]\nname = \"common\"\n",
    );
    sb.write("dotfiles/common/.a", "a");
    commit_and_push(&sb, "dotfiles");

    sb.perch(&[
        "sync",
        "--config",
        "~/dotfiles/common/.config/perch/config.toml",
    ])
    .assert()
    .success();
    sb.git(&sb.home, &["clone", "-q", "remote.git", "other"]);
    sb
}

fn commit_and_push(sb: &Sandbox, repo: &str) {
    let dir = sb.path(repo);
    sb.git(&dir, &["add", "-A"]);
    sb.git(&dir, &["commit", "-q", "-m", "change"]);
    sb.git(&dir, &["push", "-q", "-u", "origin", "HEAD"]);
}

#[test]
fn links_new_files_after_pull() {
    let sb = with_remote();
    sb.write("other/common/.b", "b");
    commit_and_push(&sb, "other");

    sb.perch(&["update"]).assert().success();
    sb.assert_link(".b", "dotfiles/common/.b");
}

#[test]
fn picks_up_modules_added_by_the_pull() {
    let sb = with_remote();
    sb.write(
        "other/common/.config/perch/config.toml",
        "[[module]]\nname = \"common\"\n\n[[module]]\nname = \"extra\"\n",
    );
    sb.write("other/extra/.c", "c");
    commit_and_push(&sb, "other");

    sb.perch(&["update"]).assert().success();
    sb.assert_link(".c", "dotfiles/extra/.c");
}

#[test]
fn moves_repo_when_the_pull_changes_dotfiles_dir() {
    let sb = with_remote();
    sb.write(
        "other/common/.config/perch/config.toml",
        "dotfiles_dir = \"~/elsewhere\"\n[[module]]\nname = \"common\"\n",
    );
    commit_and_push(&sb, "other");

    sb.perch(&["update"]).assert().success();
    assert!(!sb.dotfiles().exists());
    sb.assert_link(".a", "elsewhere/common/.a");
    sb.assert_link(
        ".config/perch/config.toml",
        "elsewhere/common/.config/perch/config.toml",
    );
}
