use std::error::Error;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use iceberg::{Catalog, NamespaceIdent};
use iceberg_catalog_rest::{RestCatalog, RestCatalogConfig};
use serde::{Deserialize, Serialize};

// CLI structure using Clap's derive API
#[derive(Parser, Debug)]
#[command(
    name = "iceberg-rust-cli",
    about = "CLI for interacting with an Iceberg REST Catalog",
    version = "0.1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(
        long,
        default_value = "http://localhost:8080/catalog",
        help = "Base URL of the Iceberg REST Catalog"
    )]
    rest_catalog_url: String,
}

// Subcommands (extensible)
#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "List namespaces in the catalog")]
    ListNamespaces {
        #[arg(long, help = "Parent namespace to list child namespaces from")]
        parent: Option<String>,
    },

    #[command(about = "List tables in a namespace")]
    ListTables {
        #[arg(long, help = "Namespace to list tables from")]
        namespace: String,
    },
}

// Iceberg namespace response structure
#[derive(Debug, Serialize, Deserialize)]
struct NamespaceResponse {
    namespaces: Vec<Vec<String>>,
}

// Main async function
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // TODO: use the catalog loader so it can be used with other catalog types
    // https://github.com/apache/iceberg-rust/issues/1228
    let config = RestCatalogConfig::builder()
        .uri(cli.rest_catalog_url)
        .build();
    let catalog: Arc<dyn Catalog> = Arc::new(RestCatalog::new(config));

    match cli.command {
        Commands::ListNamespaces { parent } => {
            iceberg_rust_cli::list_namespaces(catalog, parent.as_deref()).await?;
        }

        Commands::ListTables { namespace } => {
            iceberg_rust_cli::list_tables(catalog, &NamespaceIdent::new(namespace)).await?;
        }
    }

    Ok(())
}
