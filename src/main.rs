mod app;
mod template;

use std::path::Path;

use tera::Tera;

use template::{AuthorContext, ManifestJsonContext, PomXmlContext, render_template};

use app::App;
use clap::Parser;

use crate::template::JavaContext;

fn main() {
    let app: App = App::parse();

    let tera =
        Tera::new("resources/templates/*.template").expect("No templates found under templates/");

    let pom_ctx = PomXmlContext {
        group: app.group.clone(),
        name: app.name.clone(),
    };

    let manifest_ctx = ManifestJsonContext {
        group: app.group.clone(),
        name: app.name.clone(),
        description: app.description.clone(),
        authors: AuthorContext {
            author_name: app.authors.author_name.clone(),
            author_email: app.authors.author_email.clone(),
            author_url: app.authors.author_url.clone(),
        },
        website: app.website.clone(),
    };

    let java_ctx = JavaContext {
        group: app.group.clone(),
        name: app.name.clone(),
    };

    let project_root = Path::new(&app.output_dir).join(&app.name);

    create_dirs(&project_root, &app.group);

    // ! rendering templates in their paths

    render_template(&tera, "pom.xml.template", &project_root, &pom_ctx, None)
        .expect("Error in rendering template [pom.xml]");

    render_template(
        &tera,
        "manifest.json.template",
        &project_root.join("src/main/resources"),
        &manifest_ctx,
        None,
    )
    .expect("Error in rendering template [manifest.json]");

    render_template(
        &tera,
        "Main.java.template",
        &project_root.join(format!("src/main/java/{}", app.group.replace(".", "/"))),
        &java_ctx,
        Some(&format!("{}.java", app.name.clone())),
    )
    .expect("Error in rendering template [Main.java]");

    // ! Copy .gitignore file to new directory

    std::fs::copy(
        Path::new("resources/templates/.gitignore"),
        Path::new(&format!("{}/.gitignore", &project_root.display())),
    )
    .unwrap_or_else(|_| {
        panic!("Failed to copy .gitignore, please do so manually or open an issue")
    });
}

fn create_dirs(root: &Path, group: &str) {
    let path_java = root.join("src/main/java").join(group.replace(".", "/"));

    let path_res = root.join("src/main/resources");

    std::fs::create_dir_all(&path_java)
        .unwrap_or_else(|_| panic!("Error creating output path: {}", &path_java.display()));

    std::fs::create_dir_all(&path_res)
        .unwrap_or_else(|_| panic!("Error creating output path: {}", &path_res.display()));
}
