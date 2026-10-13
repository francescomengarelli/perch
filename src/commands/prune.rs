use anyhow::{Context, Result};
use std::fs;

use crate::{
    context,
    manifest::Manifest,
    utils::{self, normalize},
};

/// Remove links perch made that no longer lead anywhere useful: their file
/// is gone from the repo, or their module isn't active anymore.
pub fn run(context: &context::Context, dry_run: bool) -> Result<()> {
    let mut manifest = Manifest::load();
    if manifest.is_empty() {
        eprintln!("no link record yet — run perch sync once");
        return Ok(());
    }

    let dotfiles_dir = normalize(&context.dotfiles_dir);
    let mut pruned = 0;

    for link in manifest.links() {
        // Anything that is no longer one of our links isn't ours to judge.
        let Ok(target) = fs::read_link(&link) else {
            manifest.remove(&link);
            continue;
        };
        let target = normalize(&target);
        let Ok(relative) = target.strip_prefix(&dotfiles_dir) else {
            manifest.remove(&link);
            continue;
        };
        let module = relative
            .components()
            .next()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .unwrap_or_default();

        let reason = if fs::symlink_metadata(&target).is_err() {
            "its file is gone".to_string()
        } else if !context.filtered_modules.contains(&module) {
            format!("module '{}' isn't active", module)
        } else {
            continue;
        };

        let shown = utils::unexpand_tilde(&link)?;
        if dry_run {
            println!("prune {} ({})", shown.display(), reason);
        } else {
            fs::remove_file(&link)
                .with_context(|| format!("i couldn't remove {}", link.display()))?;
            manifest.remove(&link);
            context.log(1, &format!("pruned {} ({})", shown.display(), reason));
        }
        pruned += 1;
    }

    if dry_run {
        eprintln!("dry run: {} links to prune. nothing changed", pruned);
    } else {
        manifest.save();
        eprintln!("{} stale links pruned", pruned);
    }
    Ok(())
}
