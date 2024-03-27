mod actions;
mod schema;

use clap::{ command, builder::styling, Command };
use dialoguer::{ Select, theme::ColorfulTheme };
use crate::actions::{ get_template_based_on_stack, parse_to_exepected_enum };

fn main() {
    let ascii_art =
        r#"
.___ _______    _____________ __________________________________.___________________.___.
|   |\      \  /   _____/    |   \______   \_   _____/\______   \   \__    ___/\__  |   |
|   |/   |   \ \_____  \|    |   /|       _/|    __)_  |       _/   | |    |    /   |   |
|   /    |    \/        \    |  / |    |   \|        \ |    |   \   | |    |    \____   |
|___\____|__  /_______  /______/  |____|_  /_______  / |____|_  /___| |____|    / ______|
            \/        \/                 \/        \/         \/                \/
    "#;

    let description_text =
        r#"Welcome to Insurerity CLI Application!

      This tool is designed to streamline the creation of new Insurerity frontend applications. This CLI makes it easy to scaffold a new project using Insurerity's preferred stack. Get started quickly and efficiently with our CLI.

      For more information and options, run '--help'."#;

    let combined_description = format!("{}\n{}", ascii_art, description_text);
    let frontend_stacks = vec![String::from("Nextjs"), String::from("Angular")];
    let styles = styling::Styles
        ::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default());

    let matched_results = command!()
        .author("Evans Kwofie")
        .about(combined_description)
        .version("1.0.0")
        .styles(styles)
        .subcommand(
            Command::new("rest")
                .short_flag('r')
                .about("Create a frontend project to be used with a REST Backend")
        )
        .subcommand(
            Command::new("graphql")
                .short_flag('g')
                .about("Create a frontend project to be used with a REST Backend")
        )
        .get_matches();

    match matched_results.subcommand_name() {
        Some("rest") => {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Which of the following frontend applications are you creating")
                .items(&frontend_stacks)
                .interact();

            match selection {
                Ok(selection) => {
                    let selected_stack = &frontend_stacks[selection];

                    if let Some(item) = parse_to_exepected_enum(selected_stack) {
                        get_template_based_on_stack(item);
                    }
                }
                Err(_) => { eprintln!("Selection does not exist") }
            }
        }

        Some("graphql") => {
            println!("Generating project with REST API stack...");
        }
        _ => {
            eprintln!("Invalid command. Use --help for usage information.");
        }
    }
}
