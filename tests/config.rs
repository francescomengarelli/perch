mod common;

use common::Sandbox;

const COMMENTED: &str = "# my perch config\ndotfiles_dir = \"~/dotfiles\" # where it lives\n\n[[module]]\nname    = \"common\" # always\n";

#[test]
fn config_is_left_untouched_by_sync_and_status() {
    let sb = Sandbox::new();
    sb.config(COMMENTED);
    sb.write("dotfiles/common/.a", "a");
    sb.git(&sb.dotfiles(), &["init", "-q"]);

    sb.perch(&["sync"]).assert().success();
    sb.perch(&["status"]).assert().success();

    assert_eq!(sb.read(".config/perch/config.toml"), COMMENTED);
}

#[test]
fn config_without_dotfiles_dir_is_left_untouched() {
    let sb = Sandbox::new();
    let config = "# defaults please\n[[module]]\nname = \"common\"\n";
    sb.config(config);
    sb.mkdir("dotfiles/common");

    sb.perch(&["sync"]).assert().success();

    assert_eq!(sb.read(".config/perch/config.toml"), config);
}

#[test]
fn relative_dotfiles_dir_is_relative_to_home() {
    let sb = Sandbox::new();
    let config = "dotfiles_dir = \"dots\"\n[[module]]\nname = \"common\"\n";
    sb.config(config);
    sb.write("dots/common/.a", "a");
    sb.mkdir("somewhere/else");

    sb.perch(&["sync"])
        .current_dir(sb.path("somewhere/else"))
        .assert()
        .success();

    sb.assert_link(".a", "dots/common/.a");
    assert_eq!(sb.read(".config/perch/config.toml"), config);
}
