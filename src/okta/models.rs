use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Brand {
    pub id: String,
    pub name: Option<String>,
}

impl Brand {
    pub fn display_name(&self) -> String {
        self.name
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| self.id.clone())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailTemplate {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailDefaultContent {
    pub subject: String,
    #[serde(alias = "body", alias = "htmlBody", alias = "emailBody")]
    pub body: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailCustomization {
    pub id: String,
    #[serde(rename = "isDefault", default)]
    pub is_default: bool,
    pub subject: String,
    pub body: String,
}
