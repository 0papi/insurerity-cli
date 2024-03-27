use std::process::Command;
use dialoguer::{ theme::ColorfulTheme, Error, Input, MultiSelect, Select };
use crate::schema::{ AUTH_OPTIONS, NEXT_JS_ADDONS, PACKAGE_MANAGERS };

pub enum FrontendStack {
    Nextjs,
    Angular,
}

pub fn parse_to_exepected_enum(s: &str) -> Option<FrontendStack> {
    match s {
        "Nextjs" => Some(FrontendStack::Nextjs),
        "Angular" => Some(FrontendStack::Angular),
        _ => None,
    }
}

pub fn get_template_based_on_stack(arg: FrontendStack) {
    match arg {
        FrontendStack::Nextjs => {
            let mut selected_config: Vec<&&str> = vec![];
            let selections = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Additional project configurations")
                .items(&NEXT_JS_ADDONS)
                .interact();

            match selections {
                Ok(values) => {
                    for value in values {
                        let chosen_addon = &NEXT_JS_ADDONS[value];
                        selected_config.push(chosen_addon);
                    }
                }
                Err(_) => {
                    eprintln!("Sorry, could not parse chosen arguments");
                }
            }

            let auth_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select authentication package")
                .items(&AUTH_OPTIONS)
                .interact();

            if let Ok(auth_option) = auth_selection {
                let selected_auth_option = &AUTH_OPTIONS[auth_option];

                selected_config.push(selected_auth_option);
            } else {
                eprintln!("Error choosing authentication package");
            }
            let package_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select authentication package")
                .items(&PACKAGE_MANAGERS)
                .interact();

            match package_selection {
                Ok(auth_option) => {
                    let selected_package_manager = &PACKAGE_MANAGERS[auth_option];
                    selected_config.push(selected_package_manager);
                }
                Err(_) => {
                    eprintln!("Error choosing package manager");
                }
            }

            let chosen_project_name: Result<String, Error> = Input::new()
                .with_prompt("Enter project name")
                .interact_text();

            if let Ok(project_name) = chosen_project_name {
                println!("Creating project...");
                let output = Command::new("npx").arg("create-next-app").arg(project_name).output();
            } else {
                eprintln!("Error creating project");
            }
        }
        FrontendStack::Angular => println!("Creating an angular project..."),
    }
}
