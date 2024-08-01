use std::env;
use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;
use rusoto_core::Region;
use rusoto_cognito_idp::{CognitoIdentityProvider, CognitoIdentityProviderClient, InitiateAuthRequest};
use tokio;
use rpassword::read_password;
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
struct ProjectConfig {
    region: String,
    pool_id: String,
    client_id: String,
    default_accounts: Option<Vec<DefaultAccount>>,
}

#[derive(Debug, Deserialize)]
struct DefaultAccount {
    username: String,
    password: String,
}

#[derive(Debug)]
struct AuthConfig {
    pool_id: String,
    client_id: String,
    region: Region,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if .env file exists
    if !fs::metadata(".env").is_ok() {
        return Err("Error: .env file is missing. Please create a .env file with the required configuration.".into());
    }

    loop {
        let environment = select_environment()?;
        load_env(&environment);

        let projects = load_projects_from_env();

        if projects.is_empty() {
            return Err("No projects found in the environment configuration.".into());
        }

        println!("Select a project to login:");
        for (i, (project_name, _)) in projects.iter().enumerate() {
            println!("{}. {}", i + 1, project_name);
        }

        let selected_project = get_user_input("Enter the number of your choice: ")?;
        let project_index = selected_project.parse::<usize>()? - 1;

        if project_index >= projects.len() {
            return Err("Invalid project selection".into());
        }

        let (project_name, project_config) = projects.iter().nth(project_index).unwrap();
        println!("Selected project: {}", project_name);

        let auth_config = AuthConfig {
            pool_id: project_config.pool_id.clone(),
            client_id: project_config.client_id.clone(),
            region: project_config.region.parse()?,
        };

        let (username, password) = if let Some(default_accounts) = &project_config.default_accounts {
            select_or_enter_credentials(default_accounts)?
        } else {
            println!("No default accounts found for this project.");
            (
                get_user_input("Enter your username: ")?,
                read_password_with_prompt("Enter your password: ")?,
            )
        };

        let client = CognitoIdentityProviderClient::new(auth_config.region);

        let auth_result = client
            .initiate_auth(InitiateAuthRequest {
                auth_flow: "USER_PASSWORD_AUTH".to_string(),
                client_id: auth_config.client_id.clone(),
                auth_parameters: Some({
                    let mut map = std::collections::HashMap::new();
                    map.insert("USERNAME".to_string(), username);
                    map.insert("PASSWORD".to_string(), password);
                    map
                }),
                ..Default::default()
            })
            .await;

        match auth_result {
            Ok(result) => {
                if let Some(auth_result) = result.authentication_result {
                    println!("Authentication successful!");
                    println!("Payload:");
                    println!("  ID Token: {}", auth_result.id_token.unwrap_or_default());
                    println!("  Access Token: {}", auth_result.access_token.unwrap_or_default());
                    println!("  Refresh Token: {}", auth_result.refresh_token.unwrap_or_default());
                    println!("  Expires In: {} seconds", auth_result.expires_in.unwrap_or_default());
                } else {
                    println!("Authentication failed. No authentication result returned.");
                }
            },
            Err(e) => {
                println!("Authentication error: {:?}", e);
            }
        }

        let continue_choice = get_user_input("Do you want to continue? Type 'yes' to start over or 'no' to quit: ")?;
        if continue_choice.trim().eq_ignore_ascii_case("no") {
            break;
        }
    }

    Ok(())
}

fn get_user_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn read_password_with_prompt(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    read_password()
}

fn select_environment() -> io::Result<String> {
    let mut available_environments = vec!["development"];
    
    if fs::metadata(".env.staging").is_ok() {
        available_environments.push("staging");
    }
    
    if fs::metadata(".env.production").is_ok() {
        available_environments.push("production");
    }

    println!("Select an environment:");
    for (i, env) in available_environments.iter().enumerate() {
        println!("{}. {}", i + 1, env);
    }

    loop {
        let choice = get_user_input("Enter the number of your choice: ")?;
        let index = choice.parse::<usize>().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"))?;
        if index > 0 && index <= available_environments.len() {
            return Ok(available_environments[index - 1].to_string());
        }
        println!("Invalid choice. Please try again.");
    }
}

fn load_env(environment: &str) {
    let env_file = match environment {
        "development" => ".env",
        "staging" => ".env.staging",
        "production" => ".env.production",
        _ => {
            println!("Warning: Invalid environment specified. Using default .env");
            ".env"
        },
    };

    if let Err(e) = dotenv::from_filename(env_file) {
        println!("Warning: Failed to load {} file: {}", env_file, e);
    }
}

fn parse_custom_format(input: &str) -> Result<Vec<DefaultAccount>, Box<dyn std::error::Error>> {
    let trimmed = input.trim_start_matches('[').trim_end_matches(']');
    let accounts: Result<Vec<DefaultAccount>, Box<dyn std::error::Error>> = trimmed
        .split("},{")
        .map(|account_str| {
            let parts: HashMap<&str, &str> = account_str
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .filter_map(|part| {
                    let mut iter = part.splitn(2, ':');
                    Some((iter.next()?, iter.next()?))
                })
                .collect();

            Ok(DefaultAccount {
                username: parts.get("username").ok_or("Missing username")?.trim_matches('"').to_string(),
                password: parts.get("password").ok_or("Missing password")?.trim_matches('"').to_string(),
            })
        })
        .collect();

    accounts
}

fn load_projects_from_env() -> HashMap<String, ProjectConfig> {
    let mut projects = HashMap::new();

    for (key, value) in env::vars() {
        if key.ends_with("_REGION") {
            let project_name = key.trim_end_matches("_REGION");
            let pool_id_key = format!("{}_POOL_ID", project_name);
            let client_id_key = format!("{}_CLIENT_ID", project_name);
            let default_accounts_key = format!("{}_DEFAULT_ACCOUNTS", project_name);

            if let (Some(pool_id), Some(client_id)) = (env::var(&pool_id_key).ok(), env::var(&client_id_key).ok()) {
                println!("Processing project: {}", project_name);
                let default_accounts = env::var(&default_accounts_key)
                    .ok()
                    .and_then(|accounts_str| {
                        println!("Default accounts string: {}", accounts_str);
                        // Try to parse as JSON first
                        let parsed: Result<Vec<DefaultAccount>, _> = serde_json::from_str(&accounts_str);
                        match parsed {
                            Ok(accounts) => {
                                println!("Successfully parsed {} default accounts", accounts.len());
                                Some(accounts)
                            },
                            Err(e) => {
                                println!("Failed to parse default accounts as JSON: {}", e);
                                // If JSON parsing fails, try to parse as a custom format
                                match parse_custom_format(&accounts_str) {
                                    Ok(accounts) => {
                                        println!("Successfully parsed {} default accounts using custom format", accounts.len());
                                        Some(accounts)
                                    },
                                    Err(e) => {
                                        println!("Failed to parse default accounts using custom format: {}", e);
                                        None
                                    }
                                }
                            }
                        }
                    });

                projects.insert(project_name.to_string(), ProjectConfig {
                    region: value,
                    pool_id,
                    client_id,
                    default_accounts,
                });
            }
        }
    }

    projects
}

fn select_or_enter_credentials(default_accounts: &[DefaultAccount]) -> io::Result<(String, String)> {
    println!("Default accounts available:");
    for (i, account) in default_accounts.iter().enumerate() {
        println!("{}. {}", i + 1, account.username);
    }
    println!("{}. Enter credentials manually", default_accounts.len() + 1);

    loop {
        let choice = get_user_input("Enter the number of your choice: ")?;
        let index = choice.parse::<usize>().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"))?;

        if index > 0 && index <= default_accounts.len() {
            let selected_account = &default_accounts[index - 1];
            return Ok((selected_account.username.clone(), selected_account.password.clone()));
        } else if index == default_accounts.len() + 1 {
            let username = get_user_input("Enter your username: ")?;
            let password = read_password_with_prompt("Enter your password: ")?;
            return Ok((username, password));
        }
        println!("Invalid choice. Please try again.");
    }
}
