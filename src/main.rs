mod input;
pub mod macros;
mod template;
mod template_context;

use core::panic;
use std::{
    fs::{copy, create_dir_all},
    path::Path,
};

use tera::{Context as TeraContext, Tera};

use crate::{
    input::{DataContext, input},
    template::apply_context,
    template_context::{
        JavaBuildGradleKtsContext, JavaPomXmlContext, KotlinBuildGradleKtsContext,
        KotlinPomXmlContext, MainJavaContext, MainKotlinContext, ManifestJsonContext,
        SettingsGradleKtsContext,
    },
};

fn main() {
    let tera = Tera::new("resources/templates/**/*").unwrap();

    let data_context = input();

    create_dirs(&data_context);

    copy_common_files(&data_context, &tera);

    match data_context.build_system {
        "Gradle" => match data_context.language {
            "Java" => init_comb_gradle_java(&data_context, &tera),
            "Kotlin" => init_comb_gradle_kotlin(&data_context, &tera),
            _ => panic!("Unrecognized language: {}", &data_context.language),
        },
        "Maven" => match data_context.language {
            "Java" => init_comb_maven_java(&data_context, &tera),
            "Kotlin" => init_comb_maven_kotlin(&data_context, &tera),
            _ => panic!("Unrecognized language: {}", &data_context.language),
        },
        _ => panic!("Unrecognized build system: {}", &data_context.build_system),
    };
}

fn copy_common_files(data_context: &DataContext, tera: &Tera) {
    let store: Vec<(&str, &str)> = vec![
        (".gitignore", ".gitignore"),
        ("gradle-wrapper/gradle-wrapper.jar", "gradle/wrapper/gradle-wrapper.jar"),
        (
            "gradle-wrapper/gradle-wrapper.properties",
            "gradle/wrapper/gradle-wrapper.properties",
        ),
        ("gradlew", "gradlew"),
        ("gradlew.bat", "gradlew.bat"),
    ];

    for item in store {
        let dir_in = format!("resources/{}", item.0);
        let dir_out = format!("{}/{}/{}", &data_context.output_dir, &data_context.name, item.1);

        copy(&dir_in, &dir_out)
            .unwrap_or_else(|_| panic!("Failed to copy {} to {}", &dir_in, &dir_out));
    }

    tera_render(
        &tera,
        &apply_context(ManifestJsonContext {
            author_email: &data_context.author_email,
            author_name: &data_context.author_name,
            author_url: &data_context.author_url,
            description: &data_context.description,
            group: &data_context.group,
            name: &data_context.name,
            website: &data_context.website,
        }),
        "common/manifest.json",
        &format!("{}/{}/src/main/resources/{}", &data_context.output_dir, &data_context.name, "manifest.json")
    );

    tera_render(
        &tera,
        &apply_context(SettingsGradleKtsContext {
            name: &data_context.name,
        }),
        "gradle/base/settings.gradle.kts",
        &format!("{}/{}/{}", &data_context.output_dir, &data_context.name, "settings.gradle.kts")
    );
}

fn init_comb_gradle_java(data_context: &DataContext, tera: &Tera) {
    tera_render(
        &tera,
        &apply_context(JavaBuildGradleKtsContext {
            group: &data_context.group,
        }),
        "gradle/java/java.build.gradle.kts",
        &format!("{}/{}/{}", &data_context.output_dir, &data_context.name, "build.gradle.kts")
    );

    tera_render(
        &tera,
        &apply_context(MainJavaContext {
            group: &data_context.group,
            name: &data_context.name,
        }),
        "src/java/Main.java",
        &format!(
            "{}/{}/src/main/java/{}/{}.java",
            &data_context.output_dir, &data_context.name, &data_context.group.replace(".", "/"), &data_context.name
        ),
    );
}
fn init_comb_gradle_kotlin(data_context: &DataContext, tera: &Tera) {
    tera_render(
        &tera,
        &apply_context(KotlinBuildGradleKtsContext {
            group: &data_context.group,
        }),
        "gradle/kotlin/kotlin.build.gradle.kts",
        &format!("{}/{}/{}", &data_context.output_dir, &data_context.name, "build.gradle.kts")
    );
    tera_render(
        &tera,
        &apply_context(MainKotlinContext {
            group: &data_context.group,
            name: &data_context.name,
        }),
        "src/kotlin/Main.kt",
        &format!(
            "{}/{}/src/main/kotlin/{}/{}.kt",
            &data_context.output_dir, &data_context.name, &data_context.group.replace(".", "/"), &data_context.name
        ),
    );
}
fn init_comb_maven_java(data_context: &DataContext, tera: &Tera) {
    tera_render(
        &tera,
        &apply_context(JavaPomXmlContext {
            group: &data_context.group,
            name: &data_context.name,
        }),
        "maven/java/java.pom.xml",
        &format!("{}/{}/{}", &data_context.output_dir, &data_context.name, "pom.xml"),
    );
    tera_render(
        &tera,
        &apply_context(MainJavaContext {
            group: &data_context.group,
            name: &data_context.name,
        }),
        "src/java/Main.java",
        &format!(
            "{}/{}/src/main/java/{}/{}.java",
            &data_context.output_dir, &data_context.name, &data_context.group.replace(".", "/"), &data_context.name
        ),
    );
}
fn init_comb_maven_kotlin(data_context: &DataContext, tera: &Tera) {
    tera_render(
        &tera,
        &apply_context(KotlinPomXmlContext {
            group: &data_context.group,
            name: &data_context.name,
        }),
        "maven/kotlin/kotlin.pom.xml",
        &format!("{}/{}/{}", &data_context.output_dir, &data_context.name, "pom.xml"),
    );
    tera_render(
        &tera,
        &apply_context(MainKotlinContext {
            group: &data_context.group,
            name: &data_context.name,
        }),
        "src/kotlin/Main.kt",
        &format!(
            "{}/{}/src/main/kotlin/{}/{}.kt",
            &data_context.output_dir, &data_context.name, &data_context.group.replace(".", "/"), &data_context.name
        ),
    );
}

fn tera_render(tera: &Tera, tera_context: &TeraContext, template_name: &str, output_dir: &str) {
    let rendered = tera
        .render(template_name, tera_context)
        .unwrap();
        // .unwrap_or_else(|_| panic!("Failed to render [{}]", template_name));

    std::fs::write(Path::new(output_dir), rendered)
        .unwrap_or_else(|_| panic!("Failed to write {}/{}", output_dir, template_name));
}

fn create_dirs(data_context: &DataContext) {
    let root = Path::new(&data_context.output_dir).join(&data_context.name);

    let path = root.join(if data_context.language == "Java" {
        "src/main/java"
    } else {
        "src/main/kotlin"
    });

    let path_code = path.join(data_context.group.replace(".", "/"));
    let path_res = path.parent().unwrap().join("resources");
    let path_gradle_wrapper = root.join("gradle/wrapper");

    create_dir_all(&path_code)
        .unwrap_or_else(|_| panic!("Error creating output path: {}", &path_code.display()));

    create_dir_all(&path_res)
        .unwrap_or_else(|_| panic!("Error creating output path: {}", &path_res.display()));

    create_dir_all(&path_gradle_wrapper)
        .unwrap_or_else(|_| panic!("Error creating output path: {}", &path_gradle_wrapper.display()));
}
