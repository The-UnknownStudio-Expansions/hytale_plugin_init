use serde::Serialize;

pub trait TemplateContext: Serialize {}

#[derive(Serialize)]
pub struct JavaBuildGradleKtsContext<'a> {
    pub group: &'a str,
}

#[derive(Serialize)]
pub struct KotlinBuildGradleKtsContext<'a> {
    pub group: &'a str,
}

#[derive(Serialize)]
pub struct JavaPomXmlContext<'a> {
    pub group: &'a str,
    pub name: &'a str,
}

#[derive(Serialize)]
pub struct KotlinPomXmlContext<'a> {
    pub group: &'a str,
    pub name: &'a str,
}

#[derive(Serialize)]
pub struct ManifestJsonContext<'a> {
    pub author_email: &'a str,
    pub author_name: &'a str,
    pub author_url: &'a str,
    pub description: &'a str,
    pub group: &'a str,
    pub name: &'a str,
    pub website: &'a str,
}

#[derive(Serialize)]
pub struct MainJavaContext<'a> {
    pub group: &'a str,
    pub name: &'a str,
}

#[derive(Serialize)]
pub struct MainKotlinContext<'a> {
    pub group: &'a str,
    pub name: &'a str,
}

#[derive(Serialize)]
pub struct SettingsGradleKtsContext<'a> {
    pub name: &'a str,
}

impl<'a> TemplateContext for JavaBuildGradleKtsContext<'a> {}
impl<'a> TemplateContext for KotlinBuildGradleKtsContext<'a> {}
impl<'a> TemplateContext for JavaPomXmlContext<'a> {}
impl<'a> TemplateContext for KotlinPomXmlContext<'a> {}
impl<'a> TemplateContext for ManifestJsonContext<'a> {}
impl<'a> TemplateContext for MainJavaContext<'a> {}
impl<'a> TemplateContext for MainKotlinContext<'a> {}
impl<'a> TemplateContext for SettingsGradleKtsContext<'a> {}
