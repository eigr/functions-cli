use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    /// Set the namespace for the command
    #[clap(short, long, default_value = "eigr-functions")]
    namespace: String,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create Eigr Function
    Create {
        /// name of function
        name: String,

        /// container image of function
        image: String,

        /// set number of replicas of function
        replicas: Option<u32>,

        /// set memory limit of function
        memory: Option<String>,

        /// set cpu limit of function
        cpu: Option<String>,
    },

    /// Delete Eigr Function
    Delete { name: String },

    Expose {
        /// name of function
        name: String,

        /// set method to expose function [ingress, nodePort, loadBalancer]
        r#type: Option<String>,

        /// set ingressClassName if type is ingress [nginx]
        ingress_class_name: Option<String>,
    },

    /// List Eigr Functions
    Get {
        /// name of function
        name: Option<String>,

        /// show all resources of function
        all: Option<bool>,

        /// choose the output format [json, yaml]
        output: Option<String>,
    },

    /// Install Eigr k8s Operator
    Install { version: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create {
            name,
            image,
            replicas,
            memory,
            cpu,
        } => {
            let replicas = replicas.unwrap_or(1);
            let memory = memory.unwrap_or("128Mi".to_string());
            let cpu = cpu.unwrap_or("100m".to_string());
            let namespace = cli.namespace;
            let name = name;
            let image = image;
            let replicas = replicas;
            let memory = memory;
            let cpu = cpu;
            /*create(
                &namespace,
                &name,
                &image,
                replicas,
                &memory,
                &cpu,
            );*/
        }
        Commands::Delete { name } => {
            let namespace = cli.namespace;
            let name = name;
            //delete(&namespace, &name);
        }
        Commands::Expose {
            name,
            r#type,
            ingress_class_name,
        } => {
            let namespace = cli.namespace;
            let name = name;
            let expose_type = r#type.unwrap_or("ingress".to_string());
            let ingress_class_name = ingress_class_name.unwrap_or("nginx".to_string());
            //expose(&namespace, &name, &expose_type, &ingress_class_name);
        }
        Commands::Get { name, all, output } => {
            let namespace = cli.namespace;
            let name = name.unwrap_or("".to_string());
            let all = all.unwrap_or(false);
            let output = output.unwrap_or("".to_string());
            //get(&namespace, &name, all, &output);
        }
        Commands::Install { version } => {
            let version = version.unwrap_or("".to_string());
            //install(&version);
        }
    }
}
