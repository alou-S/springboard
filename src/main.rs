use colored::*;
use inquire::{error::InquireResult, required, validator::Validation, Confirm, Select, Text};
use std::time::Instant;

struct ProjectTemplate {
    module: String,
    name: String,
    author: String,
    license: String,
    pkgmgr: String,
    typescript: bool,
    express: Option<bool>,
    stylings: Option<String>,
}

impl ProjectTemplate {
    fn print_template(&self) {
        println!(
            "{}",
            "\n\nProject initialized with the following configuration =>".green()
        );
        println!(
            "{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}",
            "Module".blue(),
            self.module,
            "Name".blue(),
            self.name,
            "Author".blue(),
            self.author,
            "License".blue(),
            self.license,
            "PkgMgr".blue(),
            self.pkgmgr,
            "TypeScript".blue(),
            self.typescript
        );

        if let Some(value) = self.express {
            println!("{}: {}", "Express".blue(), value);
        }

        if let Some(value) = &self.stylings {
            println!("{}: {}", "Styling".blue(), value);
        }
    }

    fn gen_template_string(&self) -> String {
        let mut result = String::new();

        match self.module.as_str() {
            "Server" => {
                result.push_str("server");

                if let Some(true) = self.express {
                    result.push_str("-express");
                }

                match self.typescript {
                    true => result.push_str("-ts"),
                    false => result.push_str("-js"),
                }
            }

            "Client" => {
                result.push_str("client");

                if let Some(styling) = &self.stylings {
                    match styling.as_str() {
                        "CSS" => result.push_str("-css"),
                        "SCSS" => result.push_str("-scss"),
                        "Bootstrap" => result.push_str("-bootstrap"),
                        "Styled Components" => result.push_str("-styled-components"),
                        "Tailwind CSS" => result.push_str("-tailwind"),
                        _ => {}
                    }
                }

                match self.typescript {
                    true => result.push_str("-ts"),
                    false => result.push_str("-js"),
                }
            }
            _ => {
                result.push_str("full-stack");

                if let Some(styling) = &self.stylings {
                    match styling.as_str() {
                        "CSS" => result.push_str("-css"),
                        "SCSS" => result.push_str("-scss"),
                        "Bootstrap" => result.push_str("-bootstrap"),
                        "Styled Components" => result.push_str("-styled-components"),
                        "Tailwind CSS" => result.push_str("-tailwind"),
                        _ => {}
                    }
                }

                if let Some(true) = self.express {
                    result.push_str("-express");
                }

                match self.typescript {
                    true => result.push_str("-ts"),
                    false => result.push_str("-js"),
                }
            }
        }

        result
    }
}

fn get_project_template() -> InquireResult<ProjectTemplate> {
    // Validator for name input
    let validator = |input: &str| {
        if input.chars().all(|c| c.is_ascii_alphanumeric()) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Name should be alphanumeric.".into()))
        }
    };

    let module = Select::new(
        "What modules would you like to add?",
        vec!["Server", "Client", "Full stack"],
    )
    .prompt()?;

    let name = Text::new("What's the name of your project?")
        .with_validator(required!("This field is required"))
        .with_validator(validator)
        .prompt()?;

    let author = Text::new("What's the author's name?")
        .with_validator(required!("this field is required"))
        .with_validator(validator)
        .prompt()?;

    let license = Text::new("What's the license for your project?").prompt()?;

    let pkgmgr = Select::new(
        "What package manager would you like to use?",
        vec!["NPM (Node Package Manager)", "Yarn"],
    )
    .prompt()?;

    let typescript = Confirm::new("Would you like to add TypeScript to your project?").prompt()?;

    match module {
        "Server" => {
            let express = Confirm::new("Would you like to initialize a basic server?").prompt()?;
            Ok(ProjectTemplate {
                module: module.to_owned(),
                name,
                author,
                license,
                pkgmgr: pkgmgr.to_owned(),
                typescript,
                express: Some(express),
                stylings: None,
            })
        }

        "Client" => {
            let stylings = Select::new(
                "What stylings would you like to add?",
                vec![
                    "CSS",
                    "SCSS",
                    "Bootstrap",
                    "Styled Components",
                    "Tailwind CSS",
                ],
            )
            .prompt()?;

            Ok(ProjectTemplate {
                module: module.to_owned(),
                name,
                author,
                license,
                pkgmgr: pkgmgr.to_owned(),
                typescript,
                express: None,
                stylings: Some(stylings.to_owned()),
            })
        }

        _ => {
            let express = Confirm::new("Would you like to initialize a basic server?").prompt()?;

            let stylings = Select::new(
                "What stylings would you like to add?",
                vec![
                    "CSS",
                    "SCSS",
                    "Bootstrap",
                    "Styled Components",
                    "Tailwind CSS",
                ],
            )
            .prompt()?;

            Ok(ProjectTemplate {
                module: module.to_owned(),
                name,
                author,
                license,
                pkgmgr: pkgmgr.to_owned(),
                typescript,
                express: Some(express),
                stylings: Some(stylings.to_owned()),
            })
        }
    }
}

fn main() {
    let start = Instant::now();

    println!("{}", "+-+-+-+-+-+-+-+-+-+-+-+".green());
    println!("{}", "|S|p|r|i|n|g|b|o|a|r|d|".green());
    println!("{}", "+-+-+-+-+-+-+-+-+-+-+-+".green());
    println!(
        "{}",
        "Springboard jumpstarts your projects for you.".green()
    );
    println!(
        "{}",
        "\nCrafted with ❤ in Rust 🦀\n\n".truecolor(255, 128, 0)
    );

    let template = get_project_template().unwrap();
    template.print_template();
    let template_string = template.gen_template_string();

    println!("\n{}: {}", "Template".blue(), template_string.green());

    let duration = start.elapsed().as_secs_f64();
    println!(
        "\nSprung in {} seconds 🚀",
        format!("{:.2}", duration).green()
    );
}
