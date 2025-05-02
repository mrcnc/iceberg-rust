use std::error::Error;
use std::sync::Arc;

use iceberg::{Catalog, NamespaceIdent};

pub async fn list_namespaces(
    catalog: Arc<dyn Catalog>,
    parent: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let parent_ns = parent
        .map(|p| NamespaceIdent::from_strs(p.split('.')))
        .transpose()?;
    let namespaces = catalog.list_namespaces(parent_ns.as_ref()).await?;
    for ns in namespaces {
        println!("{}", ns);
    }

    Ok(())
}

pub async fn list_tables(
    catalog: Arc<dyn Catalog>,
    namespace: &NamespaceIdent,
) -> Result<(), Box<dyn Error>> {
    let tables = catalog.list_tables(namespace).await?;
    for table in tables {
        println!("{}", table);
    }

    Ok(())
}
