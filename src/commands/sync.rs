use std::path::PathBuf;

use anyhow::Result;

use crate::{context, utils};

pub fn run(context: &context::Context) -> Result<()> {
    let home = utils::get_home_dir()?;
    for module in &context.filtered_modules {
        eprintln!("settling '{}' into place...", module);
        let module_path = context.dotfiles_dir.join(module);
        crate::stow::stow(&module_path, &PathBuf::from(&home))?;
    }

    Ok(())
}
