use std::path::PathBuf;

use dialoguer::{Input, Select, theme::ColorfulTheme};

use crate::error::{AppError, AppResult};

pub fn choose_destination(default_dir: PathBuf) -> AppResult<PathBuf> {
    let options = vec![
        format!("Current directory ({})", default_dir.display()),
        "Choose another directory".to_string(),
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Where should the templates be saved?")
        .items(&options)
        .default(0)
        .interact()
        .map_err(|err| AppError::Prompt(err.to_string()))?;

    if selection == 0 {
        Ok(default_dir)
    } else {
        let value: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter a destination directory")
            .interact_text()
            .map_err(|err| AppError::Prompt(err.to_string()))?;

        Ok(PathBuf::from(value))
    }
}
