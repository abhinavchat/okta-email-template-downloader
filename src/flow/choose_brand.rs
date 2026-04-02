use dialoguer::{Select, theme::ColorfulTheme};

use crate::error::{AppError, AppResult};
use crate::okta::models::Brand;

pub fn choose_brand(brands: &[Brand]) -> AppResult<Brand> {
    let items: Vec<String> = brands
        .iter()
        .map(|brand| format!("{} ({})", display_brand_name(brand), brand.id))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a brand")
        .items(&items)
        .default(0)
        .max_length(20)
        .interact()
        .map_err(|err| AppError::Prompt(err.to_string()))?;

    Ok(brands[selection].clone())
}

fn display_brand_name(brand: &Brand) -> &str {
    brand
        .name
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("Unnamed brand")
}
