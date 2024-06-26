use crate::schema::{ AUTH_OPTIONS, NEXT_JS_ADDONS, PACKAGE_MANAGERS, create_addons_map };
use dialoguer::{ theme::ColorfulTheme, Input, MultiSelect, Select };
use serde_json::{ Map, Value };
use std::process::Command;
use indicatif::ProgressBar;

pub enum FrontendStack {
    Nextjs,
    Angular,
}

pub fn parse_to_expected_enum(s: &str) -> Option<FrontendStack> {
    match s {
        "Nextjs" => Some(FrontendStack::Nextjs),
        "Angular" => Some(FrontendStack::Angular),
        _ => None,
    }
}

pub fn get_template_based_on_stack(arg: FrontendStack) {
    match arg {
        FrontendStack::Nextjs => {
            let mut selected_config: Map<String, Value> = Map::new();

            let project_name = Input::new()
                .with_prompt("What is the name of this project?")
                .interact_text();

            match project_name {
                Ok(name) => {
                    selected_config.insert("project_name".to_string(), Value::String(name));
                }
                Err(_) => {
                    eprintln!("Sorry, could not parse arguments");
                    return;
                }
            }

            let selections = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Additional project configurations")
                .items(&NEXT_JS_ADDONS)
                .interact();

            match selections {
                Ok(values) => {
                    let addons: Vec<String> = values
                        .into_iter()
                        .map(|i| NEXT_JS_ADDONS[i].to_string())
                        .collect();
                    selected_config.insert(
                        "addons".to_string(),
                        Value::Array(addons.into_iter().map(Value::String).collect())
                    );
                }
                Err(_) => {
                    eprintln!("Sorry, could not parse chosen arguments");
                    return;
                }
            }

            let auth_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select authentication package")
                .items(&AUTH_OPTIONS)
                .interact();

            if let Ok(auth_option) = auth_selection {
                selected_config.insert(
                    "auth_option".to_string(),
                    Value::String(AUTH_OPTIONS[auth_option].to_string())
                );
            } else {
                eprintln!("Error choosing authentication package");
                return;
            }

            let package_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select package manager")
                .items(&PACKAGE_MANAGERS)
                .interact();

            match package_selection {
                Ok(package_option) => {
                    selected_config.insert(
                        "package_manager".to_string(),
                        Value::String(PACKAGE_MANAGERS[package_option].to_string())
                    );
                }
                Err(_) => {
                    eprintln!("Error parsing arguments");
                    return;
                }
            }

            create_nextjs_project(selected_config);
        }
        FrontendStack::Angular => println!("Creating an angular project..."),
    }
}

pub fn create_nextjs_project(config: Map<String, Value>) {
    let mut command = Command::new("npx");
    command.arg("create-next-app");

    if let Some(Value::String(project_name)) = config.get("project_name") {
        command.arg(project_name);
    }

    // Add addons if any
    if let Some(Value::Array(addons)) = config.get("addons") {
        let addons_map = create_addons_map();
        for addon in addons {
            if let Value::String(addon_str) = addon {
                if let Some(addon_value) = addons_map.get(addon_str.as_str()) {
                    match *addon_value {
                        "tailwindcss" => {
                            command.arg("--tailwindcss");
                        }
                        "typescript" => {
                            command.arg("--typescript");
                        }
                        "zustand" => {}
                        "react-query" => {}
                        _ => {}
                    }
                }
            }
        }
    }
    command.args(["--eslint", "--src-dir"]);

    // Add package manager option
    if let Some(Value::String(package_manager)) = config.get("package_manager") {
        let pkm = format!("--use-{}", package_manager);

        command.arg(&pkm);
    }


    let bar = ProgressBar::new(1000);
    bar.set_message("Installing Nextjs project");
    for _ in 0..1000 {
        bar.inc(1);
        let result = command.spawn();
        match result {
            Err(err) => eprintln!("An error occurred:{}", err),
            Ok(_res) => {
                //TODO: When initial scaffold is done install any additional packages and configs

                // // Add authentication option
                // if let Some(Value::String(auth_option)) = config.get("auth_option") {
                //     command.arg(auth_option);
                // }
            }
        }
    }

    bar.finish();
}
