use std::env;
use std::path::PathBuf;

use dialoguer::{Select, theme::ColorfulTheme};

use crate::config::ResolvedConfig;
use crate::error::{AppError, AppResult};
use crate::export::writer::{ExportOutcome, export_exists, export_template};
use crate::flow::choose_brand::choose_brand;
use crate::flow::choose_destination::choose_destination;
use crate::flow::choose_template::{TemplateSelection, choose_template};
use crate::okta::client::OktaClient;
use crate::okta::models::{Brand, EmailCustomization, EmailTemplate};

pub async fn run(config: ResolvedConfig, client: OktaClient) -> AppResult<()> {
    let brand = resolve_brand(&config, &client).await?;
    let templates = client.list_templates(&brand.id).await?;

    if templates.is_empty() {
        return Err(AppError::NoTemplates);
    }

    let selected_templates = resolve_templates(&config, templates)?;
    let destination = resolve_destination(&config)?;

    let mut outcomes = Vec::with_capacity(selected_templates.len());
    for template in selected_templates {
        if config.verbose {
            eprintln!(
                "fetching template '{}' from brand '{}'",
                template.name, brand.id
            );
        }

        let should_overwrite = if config.overwrite {
            true
        } else if export_exists(&destination, &brand.display_name(), &template.name) {
            if config.non_interactive {
                return Err(AppError::Config {
                    message: format!(
                        "files for template '{}' already exist. Re-run with --overwrite to replace them",
                        template.name
                    ),
                });
            }

            match choose_overwrite_action(&template.name)? {
                OverwriteAction::Overwrite => true,
                OverwriteAction::Skip => {
                    if config.verbose {
                        eprintln!("skipping existing template '{}'", template.name);
                    }
                    continue;
                }
                OverwriteAction::Stop => {
                    return Err(AppError::Config {
                        message: "download cancelled by user".to_string(),
                    });
                }
            }
        } else {
            false
        };

        let customizations = client
            .list_customizations(&brand.id, &template.name)
            .await?;
        let outcome = if let Some(customization) = select_template_content(&customizations) {
            if config.verbose {
                eprintln!(
                    "using brand customization '{}' for template '{}'",
                    customization.id, template.name
                );
            }

            export_template(
                &destination,
                &brand.display_name(),
                &template.name,
                &customization.subject,
                &customization.body,
                should_overwrite,
            )?
        } else {
            if config.verbose {
                eprintln!(
                    "no customization found for '{}'; using default-content",
                    template.name
                );
            }

            let default_content = client.default_content(&brand.id, &template.name).await?;
            export_template(
                &destination,
                &brand.display_name(),
                &template.name,
                &default_content.subject,
                &default_content.body,
                should_overwrite,
            )?
        };
        outcomes.push(outcome);
    }

    print_summary(&brand, &outcomes, destination, config.config_path.as_ref());
    Ok(())
}

async fn resolve_brand(config: &ResolvedConfig, client: &OktaClient) -> AppResult<Brand> {
    let brands = client.list_brands().await?;
    if brands.is_empty() {
        return Err(AppError::NoBrands);
    }

    if let Some(requested_brand) = config.brand.as_ref() {
        return brands
            .into_iter()
            .find(|brand| {
                brand.id == *requested_brand || brand.name.as_deref() == Some(requested_brand)
            })
            .ok_or_else(|| AppError::Config {
                message: format!("brand '{requested_brand}' was not found"),
            });
    }

    if config.non_interactive {
        if brands.len() == 1 {
            return Ok(brands[0].clone());
        }

        return Err(AppError::NonInteractiveMissing(
            "--brand when multiple brands are available".to_string(),
        ));
    }

    if brands.len() == 1 {
        return Ok(brands[0].clone());
    }

    choose_brand(&brands)
}

fn resolve_templates(
    config: &ResolvedConfig,
    templates: Vec<EmailTemplate>,
) -> AppResult<Vec<EmailTemplate>> {
    if let Some(requested_template) = config.template.as_ref() {
        let template = templates
            .into_iter()
            .find(|template| template.name == *requested_template)
            .ok_or_else(|| AppError::Config {
                message: format!("template '{requested_template}' was not found"),
            })?;
        return Ok(vec![template]);
    }

    if config.all {
        return Ok(templates);
    }

    if config.non_interactive {
        return Err(AppError::NonInteractiveMissing(
            "--template or --all".to_string(),
        ));
    }

    match choose_template(&templates)? {
        TemplateSelection::All => Ok(templates),
        TemplateSelection::One(template) => Ok(vec![template]),
    }
}

fn select_template_content(customizations: &[EmailCustomization]) -> Option<&EmailCustomization> {
    customizations
        .iter()
        .find(|customization| customization.is_default)
        .or_else(|| customizations.first())
}

enum OverwriteAction {
    Overwrite,
    Skip,
    Stop,
}

fn choose_overwrite_action(template_name: &str) -> AppResult<OverwriteAction> {
    let options = vec![
        format!("Overwrite files for {template_name}"),
        format!("Skip {template_name}"),
        "Stop download".to_string(),
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Files for '{template_name}' already exist. What do you want to do?"
        ))
        .items(&options)
        .default(0)
        .interact()
        .map_err(|err| AppError::Prompt(err.to_string()))?;

    match selection {
        0 => Ok(OverwriteAction::Overwrite),
        1 => Ok(OverwriteAction::Skip),
        _ => Ok(OverwriteAction::Stop),
    }
}

fn resolve_destination(config: &ResolvedConfig) -> AppResult<PathBuf> {
    let default_dir = config
        .output
        .clone()
        .or_else(|| config.output_dir.clone())
        .unwrap_or(env::current_dir()?);

    if config.output.is_some() || config.output_dir.is_some() || config.non_interactive {
        return Ok(default_dir);
    }

    choose_destination(default_dir)
}

fn print_summary(
    brand: &Brand,
    outcomes: &[ExportOutcome],
    destination: PathBuf,
    config_path: Option<&PathBuf>,
) {
    println!("Brand: {}", brand.display_name());
    println!("Destination: {}", destination.display());

    if let Some(path) = config_path {
        println!("Config: {}", path.display());
    }

    for outcome in outcomes {
        println!(
            "Saved {} -> {}, {}",
            outcome.template_name,
            outcome.html_path.display(),
            outcome.subject_path.display()
        );
    }
}
