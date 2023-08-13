mod licenses;
use colored::*;
use fs_extra::dir::{self, CopyOptions};
use inquire::{error::InquireResult, required, validator::Validation, Confirm, Select, Text};
use licenses::{COMMON_LICENSES, LICENSES};
use project_root::get_project_root;
use serde_json::Value;
use std::process::{exit, Command, Stdio};
use std::time::Instant;
use std::{fs, path::Path};

struct ProjectTemplate {
    module: String,
    name: String,
    author: String,
    license_name: String,
    license_keyword: String,
    pkgmgr: String,
    typescript: bool,
    express: Option<bool>,
    stylings: Option<String>,
}

impl ProjectTemplate {
    fn print_template(&self) -> bool {
        println!(
            "{}",
            "\n\nProject will be initialized with the following configuration =>".green()
        );
        println!(
            "{}: {}\n{}: {}\n{}: {}\n{}: {} ({})\n{}: {}\n{}: {}",
            "Module".blue(),
            self.module,
            "Name".blue(),
            self.name,
            "Author".blue(),
            self.author,
            "License".blue(),
            self.license_name,
            self.license_keyword,
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

        return Confirm::new("Do you want to continue with this template?")
            .prompt()
            .unwrap()
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
        if input
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.')
        {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Name should be alphanumeric.".into()))
        }
    };

    let module = Select::new(
        "What modules would you like to add?",
        vec!["Server", "Client", "Full stack"],
    )
    .prompt()
    .unwrap()
    .to_string();

    let name = Text::new("What's the name of your project?")
        .with_validator(required!("This field is required"))
        .with_validator(validator)
        .prompt()?;

    let author = Text::new("What's the author's name?")
        .with_validator(required!("this field is required"))
        .with_validator(validator)
        .prompt()?;

    let common_licenses = COMMON_LICENSES
        .iter()
        .map(|(name, keyword)| format!("{} ({})", name, keyword))
        .collect::<Vec<String>>();

    let all_licenses = LICENSES
        .iter()
        .map(|(name, keyword)| format!("{} ({})", name, keyword))
        .collect::<Vec<String>>();

    let (license_name, license_keyword) = match Select::new(
        &"Which license would you select for your project",
        vec!["Common Licenses", "All Licenses", "Custom License"],
    )
    .prompt()?
    {
        "Common Licenses" => {
            let selection = Select::new("Choose a license", common_licenses)
                .prompt()
                .unwrap();
            let split_index = selection.find('(').unwrap();
            (
                selection[..split_index].trim().to_string(),
                selection[split_index + 1..selection.len() - 1]
                    .trim()
                    .to_string(),
            )
        }

        "All Licenses" => {
            let selection = Select::new("Choose a license", all_licenses).prompt()?;
            let split_index = selection.find('(').unwrap();
            (
                selection[..split_index].trim().to_string(),
                selection[split_index + 1..selection.len() - 1]
                    .trim()
                    .to_string(),
            )
        }

        _ => (
            Text::new("License Name:")
                .with_validator(required!("This field is required"))
                .prompt()
                .unwrap()
                .to_string(),
            Text::new("License Keyword:")
                .with_validator(required!("This field is required"))
                .with_validator(validator)
                .prompt()
                .unwrap()
                .to_string(),
        ),
    };

    let pkgmgr = Select::new(
        "What package manager would you like to use?",
        vec![
            "NPM (Node Package Manager)",
            "Yarn",
            "PNPM (Performant NPM)",
        ],
    )
    .prompt()
    .unwrap()
    .to_string();

    let typescript = Confirm::new("Would you like to add TypeScript to your project?").prompt()?;

    match module.as_str() {
        "Server" => {
            let express = Confirm::new("Would you like to initialize a basic server?").prompt()?;
            Ok(ProjectTemplate {
                module,
                name,
                author,
                license_name,
                license_keyword,
                pkgmgr,
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
            .prompt()
            .unwrap()
            .to_string();

            Ok(ProjectTemplate {
                module,
                name,
                author,
                license_name,
                license_keyword,
                pkgmgr,
                typescript,
                express: None,
                stylings: Some(stylings),
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
            .prompt()
            .unwrap()
            .to_string();

            Ok(ProjectTemplate {
                module,
                name,
                author,
                license_name,
                license_keyword,
                pkgmgr,
                typescript,
                express: Some(express),
                stylings: Some(stylings),
            })
        }
    }
}

fn update_package_json(project_path: &Path, template: &ProjectTemplate) {
    let json_path = project_path.join(Path::new("package.json"));
    let json_data = fs::read_to_string(&json_path).expect("Err: Failed to read the JSON file");
    let mut package: Value = serde_json::from_str(&json_data).expect("Failed to parse JSON");
    let package_obj = package.as_object_mut().expect("Expected a JSON object");

    package_obj.insert("name".to_string(), Value::String(template.name.clone()));
    package_obj.insert("author".to_string(), Value::String(template.author.clone()));
    package_obj.insert(
        "license".to_string(),
        Value::String(template.license_keyword.clone()),
    );

    let json_data_new = serde_json::to_string_pretty(&package).expect("Failed to serialize JSON");
    fs::write(json_path, json_data_new).expect("Failed to write JSON to the file");
}

fn init_git(path: &Path) {
    println!("\n\n{}", "Intitializing git repository =>".blue());
    let commit_message = "🎉 Initialized project using Springboard";
    let command_set = format!(
        "git init -q && git checkout -b main && git add -A && git commit -m \"{}\"",
        commit_message
    );

    Command::new("sh")
        .arg("-c")
        .arg(command_set)
        .current_dir(path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Err: Failed to init a git repository");
}

fn install_deps(path: &Path, pkgmgr: &str) {
    println!("\n\n{}", "Installing dependencies =>".blue());
    match pkgmgr {
        "NPM (Node Package Manager)" => {
            Command::new("npm")
                .arg("i")
                .current_dir(path)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("Err: Failed to run npm i");
        }

        "Yarn" => {
            Command::new("yarn")
                .current_dir(path)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("Err: Failed to run yarn");
        }

        "PNPM (Performant NPM)" => {
            Command::new("pnpm")
                .arg("i")
                .current_dir(path)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("Err: Failed to run pnpm i");
        }
        _ => {}
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

    let mut template;

    loop {
        template = get_project_template().unwrap();
        if template.print_template() {
            break;
        }
    }

    let template_string = template.gen_template_string();

    let springboard_root = match get_project_root() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{} Can't obtain project root {:?}", "Err:".red(), e);
            exit(1);
        }
    };

    let template_path = springboard_root.join(Path::new("src/templates/").join(template_string));
    let project_path = springboard_root.join(&template.name);

    if project_path.exists() {
        eprintln!(
            "{} Destination folder with project name already exist",
            "Err:".red()
        );
        exit(1);
    }

    if let Err(e) = dir::copy(
        template_path,
        &project_path,
        &CopyOptions {
            copy_inside: true,
            ..Default::default()
        },
    ) {
        eprintln!("{} {}", "Err:".red(), e);
        exit(1);
    }

    update_package_json(&project_path, &template);
    init_git(&project_path);
    install_deps(&project_path, template.pkgmgr.as_str());

    if LICENSES
        .iter()
        .any(|license| license.1 == template.license_keyword)
    {
        fs::copy(
            springboard_root.join(format!(
                "src/license-templates/{}",
                template.license_keyword
            )),
            project_path.join("LICENSE"),
        )
        .unwrap();
    }

    println!(
        "{} {}",
        "\n\nProject sucessfully bootstrapped at:".blue(),
        project_path.display().to_string().green()
    );

    let duration = start.elapsed().as_secs_f64();
    println!(
        "Sprung in {} seconds 🚀",
        format!("{:.2}", duration).green()
    );
}
