use colored::*;
use inquire::{error::InquireResult, required, validator::Validation, Confirm, Select, Text};
use std::time::Instant;

fn main() -> InquireResult<()> {
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

    let tscript = Confirm::new("Would you like to add TypeScript to your project?").prompt()?;

    match module {
        "Server" => {
            let initserver =
                Confirm::new("Would you like to initialize a basic server?").prompt()?;

            println!(
                "{}",
                "\n\nProject initialized with the following configuration =>".green()
            );
            println!(
                "{}: {} \n{}: {} \n{}: {} \n{}: {} \n{}: {} \n{}: {} \n{}: {}",
                "Module".blue(), module,
                "Name".blue(), name,
                "Author".blue(), author,
                "License".blue(), license,
                "PkgMgr".blue(),pkgmgr,
                "TypeScript".blue(), tscript,
                "Server".blue(), initserver
            );
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

            println!(
                "{}",
                "\n\nProject initialized with the following configuration =>".green()
            );
            println!(
                "{}: {} \n{}: {} \n{}: {} \n{}: {} \n{}: {} \n{}: {} \n{}: {}",
                "Module".blue(), module,
                "Name".blue(), name,
                "Author".blue(), author,
                "License".blue(), license,
                "PkgMgr".blue(), pkgmgr,
                "TypeScript".blue(), tscript,
                "Stylings".blue(), stylings
            );
        }
        _ => {
            let initserver =
                Confirm::new("Would you like to initialize a basic server?").prompt()?;

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

            println!(
                "{}",
                "\n\nProject initialized with the following configuration =>".green()
            );
            println!(
                "{}: {} \n{}: {} \n{}: {} \n{}: {} \n{}: {} \n{}: {} \n{}: {} \n{}: {}",
                "Module".blue(), module,
                "Name".blue(), name,
                "Author".blue(), author,
                "License".blue(), license,
                "PkgMgr".blue(), pkgmgr,
                "TypeScript".blue(), tscript,
                "Server".blue(), initserver,
                "Stylings".blue(), stylings
            );
        }
    }

    let duration = start.elapsed().as_secs_f64();
    println!(
        "\nSprung in {} seconds 🚀",
        format!("{:.2}", duration).green()
    );

    Ok(())
}
