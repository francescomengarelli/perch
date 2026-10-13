use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Result, bail};

use crate::{context, manifest::Manifest, utils};

/// What a sync would do, computed without touching anything.
pub struct SyncPlan {
    /// (source in the repo, link location in $HOME)
    pub to_link: Vec<(PathBuf, PathBuf)>,
    pub already_linked: Vec<PathBuf>,
    pub conflicts: Vec<PathBuf>,
    pub synced_modules: usize,
}

pub fn plan(context: &context::Context) -> Result<SyncPlan> {
    let home = utils::get_home_dir()?;

    let mut plan = SyncPlan {
        to_link: vec![],
        already_linked: vec![],
        conflicts: vec![],
        synced_modules: 0,
    };
    for module in &context.filtered_modules {
        let module_path = context.dotfiles_dir.join(module);
        if !module_path.is_dir() {
            eprintln!(
                "module '{}' is active but {} doesn't exist — skipping",
                module,
                module_path.display()
            );
            continue;
        }
        plan.synced_modules += 1;
        for entry in utils::walk_dotfiles(&context.dotfiles_dir, &module_path) {
            let path = entry?;
            let relative = path.strip_prefix(&module_path)?;
            let target_path = home.join(relative);

            if target_path.is_symlink() || target_path.exists() {
                if is_our_symlink(&target_path, &path) {
                    plan.already_linked.push(target_path);
                } else {
                    plan.conflicts.push(target_path);
                }
                continue;
            }

            plan.to_link.push((path, target_path));
        }
    }

    Ok(plan)
}

pub fn run(context: &context::Context, dry_run: bool) -> Result<()> {
    let plan = plan(context)?;

    if dry_run {
        for (from, to) in &plan.to_link {
            println!("link {} -> {}", display(to), display(from));
        }
        for target in &plan.conflicts {
            println!("conflict {} (not mine)", display(target));
        }
        eprintln!(
            "dry run: {} to link, {} already in place, {} conflicts. nothing changed",
            plan.to_link.len(),
            plan.already_linked.len(),
            plan.conflicts.len()
        );
        if !plan.conflicts.is_empty() {
            bail!(
                "{} conflicts found. resolve before syncing",
                plan.conflicts.len()
            );
        }
        return Ok(());
    }

    for target in &plan.conflicts {
        eprintln!("{} is not mine. i am not touching it", target.display());
    }
    if !plan.conflicts.is_empty() {
        bail!(
            "{} conflicts found. resolve before syncing. no changes applied",
            plan.conflicts.len()
        );
    }

    let mut manifest = Manifest::load();
    for target in &plan.already_linked {
        manifest.insert(target);
    }
    let result = (|| -> Result<()> {
        for (from, to) in &plan.to_link {
            utils::create_parent_dirs(to)?;
            utils::symlink(from, to)?;
            manifest.insert(to);
            context.log(2, &format!("{} is in place", to.display()));
        }
        Ok(())
    })();
    manifest.save();
    result?;

    eprintln!("{} modules synced", plan.synced_modules);

    Ok(())
}

fn display(path: &Path) -> String {
    utils::unexpand_tilde(path)
        .unwrap_or_else(|_| path.to_path_buf())
        .display()
        .to_string()
}

fn is_our_symlink(target: &Path, source: &Path) -> bool {
    match fs::read_link(target) {
        Ok(actual) => utils::paths_equal(&actual, source),
        Err(_) => false,
    }
}
