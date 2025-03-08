use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    ScanDirectory {
        #[arg(default_value = "tests/resources")]
        dir_path: String,

        #[arg(
            long,
            help = "Output results in JSON format for easier parsing and integration with other tools"
        )]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::ScanDirectory { dir_path, json }) => {
            if *json {
                let json_output = path_finder::scan_directory_json(dir_path)?;
                println!("{}", json_output);
            } else {
                let endpoints = path_finder::scan_directory(dir_path)?;
                print_endpoints_summary(&endpoints);
            }
        }
        None => {
            println!(
                "サブコマンドが指定されていません。`extract-request-mapping`または`scan-directory`サブコマンドを試してください。"
            );
        }
    }

    Ok(())
}

fn print_endpoints_summary(endpoints: &[path_finder::Endpoint]) {
    for endpoint in endpoints {
        let http_method = match endpoint.http_method.as_str() {
            "GET" => "GET".green(),
            "POST" => "POST".yellow(),
            "PUT" => "PUT".blue(),
            "DELETE" => "DELETE".red(),
            "PATCH" => "PATCH".cyan(),
            "ANY" => "ANY".magenta(),
            _ => endpoint.http_method.normal(),
        };

        println!(
            "{} {} ({}#{}) [{}:{}]",
            http_method,
            endpoint.path.magenta(),
            endpoint.class_name,
            endpoint.method_name,
            endpoint.file_path.blue(),
            endpoint.line_range.0,
        );

        // パラメータがあれば表示
        if !endpoint.parameters.is_empty() {
            print!("  parameters: ");
            for (i, param) in endpoint.parameters.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}:{} ({})", param.name, param.param_type, param.annotation);
            }
            println!();
        }
    }
}
