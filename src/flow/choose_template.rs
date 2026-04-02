use dialoguer::{Select, theme::ColorfulTheme};

use crate::error::{AppError, AppResult};
use crate::okta::models::EmailTemplate;

pub enum TemplateSelection {
    All,
    One(EmailTemplate),
}

pub fn choose_template(templates: &[EmailTemplate]) -> AppResult<TemplateSelection> {
    let mut items = Vec::with_capacity(templates.len() + 1);
    items.push("All templates".to_string());
    items.extend(templates.iter().map(|template| template.name.clone()));

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a template")
        .items(&items)
        .default(0)
        .max_length(20)
        .interact()
        .map_err(|err| AppError::Prompt(err.to_string()))?;

    if selection == 0 {
        Ok(TemplateSelection::All)
    } else {
        Ok(TemplateSelection::One(templates[selection - 1].clone()))
    }
}
