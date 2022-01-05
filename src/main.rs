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
enum CreateCommands {
    Function {
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
    StatefulStore {
        /// name of stateful store
        name: String,
    },
}

#[derive(Subcommand)]
enum DeleteCommands {
    Function {
        /// name of function
        name: String,
    },
    StatefulStore {
        /// name of stateful store
        name: String,
    },
}

#[derive(Subcommand)]
enum Commands {
    /// Create Eigr Functions Resources
    Create {
        #[clap(subcommand)]
        create: CreateCommands,
    },

    /// Delete Eigr Function
    Delete {
        #[clap(subcommand)]
        delete: DeleteCommands,
    },

    /// Expose function to outside the cluster
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
        Commands::Create { create } => match create {
            CreateCommands::Function {
                name,
                image,
                replicas,
                memory,
                cpu,
            } => {
                let replicas = replicas.unwrap_or(1);
                let memory = memory.unwrap_or("128Mi".to_string());
                let cpu = cpu.unwrap_or("100m".to_string());
                let _namespace = cli.namespace;
                let _name = name;
                let _image = image;
                let _replicas = replicas;
                let _memory = memory;
                let _cpu = cpu;
                /*create(
                    &namespace,
                    &name,
                    &image,
                    replicas,
                    &memory,
                    &cpu,
                );*/
            }
            CreateCommands::StatefulStore { name } => {
                let _namespace = cli.namespace;
                let _name = name;
                /*create(
                    &namespace,
                    &name,
                    "eigr-stateful-store",
                    1,
                    "128Mi",
                    "100m",
                );*/
            }
        },
        Commands::Delete { delete } => match delete {
            DeleteCommands::Function { name } => {
                let _namespace = cli.namespace;
                let _name = name;
                /*delete(
                    &namespace,
                    &name,
                    "eigr-function",
                );*/
            }
            DeleteCommands::StatefulStore { name } => {
                let _namespace = cli.namespace;
                let _name = name;
                /*delete(
                    &namespace,
                    &name,
                    "eigr-stateful-store",
                );*/
            }
        },
        Commands::Expose {
            name,
            r#type,
            ingress_class_name,
        } => {
            let _namespace = cli.namespace;
            let _name = name;
            let _expose_type = r#type.unwrap_or("ingress".to_string());
            let _ingress_class_name = ingress_class_name.unwrap_or("nginx".to_string());
            //expose(&namespace, &name, &expose_type, &ingress_class_name);
        }
        Commands::Get { name, all, output } => {
            let _namespace = cli.namespace;
            let _name = name.unwrap_or("".to_string());
            let _all = all.unwrap_or(false);
            let _output = output.unwrap_or("".to_string());
            //get(&namespace, &name, all, &output);
        }
        Commands::Install { version } => {
            let _version = version.unwrap_or("".to_string());
            //install(&version);
        }
    }
}
