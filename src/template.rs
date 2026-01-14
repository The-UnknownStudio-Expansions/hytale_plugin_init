use std::path::{Path, PathBuf};

use serde::Serialize;
use tera::{Context, Tera};

pub trait TemplateContext: Serialize {}

#[derive(Serialize)]
pub struct PomXmlContext {
    pub group: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct ManifestJsonContext {
    pub group: String,
    pub name: String,
    pub description: String,
    pub authors: AuthorContext,
    pub website: String,
}

#[derive(Serialize)]
pub struct AuthorContext {
    pub author_name: String,
    pub author_email: String,
    pub author_url: String,
}

#[derive(Serialize)]
pub struct JavaContext {
    pub group: String,
    pub name: String,
}

impl TemplateContext for PomXmlContext {}
impl TemplateContext for ManifestJsonContext {}
impl TemplateContext for JavaContext {}

pub fn render_template<T>(
    tera: &Tera,
    template_name: &str,
    output_dir: &Path,
    context: &T,
    output_file_name: Option<&str>,
) -> tera::Result<PathBuf>
where
    T: TemplateContext,
{
    let mut ctx = Context::new();
    ctx.try_insert("self", context)?;

    let rendered = tera.render(template_name, &ctx)?;

    let output_path = output_dir.join(if let Some(out_file_name) = output_file_name {
        out_file_name
    } else {
        template_name
            .strip_suffix(".template")
            .unwrap_or(template_name)
    });

    std::fs::write(&output_path, rendered).map_err(|e| tera::Error::msg(e.to_string()))?;

    Ok(output_path)
}
