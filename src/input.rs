use inquire::{Select, Text};

use crate::build_data_context;

#[derive(Debug)]
pub struct DataContext<'a> {
    pub build_system: &'a str,
    pub language: &'a str,
    // pub include_examples: Vec<&'a str>,
    pub output_dir: String,
    pub group: String,
    pub name: String,
    pub description: String,
    pub website: String,
    pub author_name: String,
    pub author_email: String,
    pub author_url: String,
}

impl<'a> DataContext<'a> {
    pub fn new(
        build_system: &'a str,
        language: &'a str,
        // include_examples: Vec<&'a str>,
        output_dir: String,
        group: String,
        name: String,
        description: String,
        website: String,
        author_name: String,
        author_email: String,
        author_url: String,
    ) -> Self {
        Self {
            build_system,
            language,
            // include_examples,
            output_dir,
            group,
            name,
            description,
            website,
            author_name,
            author_email,
            author_url,
        }
    }
}

pub fn input<'a>() -> DataContext<'a> {
    let opt_build_system = vec!["Gradle", "Maven"];
    let opt_language = vec!["Java", "Kotlin"];
    // let opt_include_examples = vec!["Commands", "Events"];

    build_data_context!(
        Select::new("Select Build System", opt_build_system).with_starting_cursor(0),
        Select::new("Select Language", opt_language).with_starting_cursor(0),
        // MultiSelect::new("Select examples to include", opt_include_examples),
        Text::new("Enter output directory >").with_initial_value("output"),
        Text::new("Enter project group >").with_initial_value("com.example"),
        Text::new("Enter project name >").with_initial_value("ExamplePlugin"),
        Text::new("Enter project description >").with_initial_value(""),
        Text::new("Enter project website >").with_initial_value(""),
        Text::new("Enter author's name >").with_initial_value(""),
        Text::new("Enter author's email >").with_initial_value(""),
        Text::new("Enter author's url >").with_initial_value("")
    )
}
