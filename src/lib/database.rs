use std::sync::Arc;
use tokio::sync::Mutex;

use evmap::{ReadHandle, WriteHandle};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};

pub static GET_ALL_TABLES_QUERY: &str = " SELECT n.nspname as \"Schema\", c.relname as \"Name\" FROM pg_catalog.pg_class c LEFT JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace WHERE c.relkind IN ('r','') AND n.nspname <> 'pg_catalog' AND n.nspname <> 'information_schema' AND n.nspname !~ '^pg_toast' AND pg_catalog.pg_table_is_visible(c.oid) ORDER BY 1,2;";

#[derive(Debug, FromRow)]
pub struct Table {
    pub schema: String,
    pub name: String,
}

#[derive(Debug)]
pub struct DatabaseSystem {
    pub pool: PgPool,
    table_read: ReadHandle<String, ()>,
    table_write: Arc<Mutex<WriteHandle<String, ()>>>,
}

impl DatabaseSystem {
    pub async fn connect(uri: &str) -> eyre::Result<Self> {
        let pool = PgPoolOptions::new().max_connections(5).connect(uri).await?;
        let tables = sqlx::query_as::<_, Table>(GET_ALL_TABLES_QUERY)
            .fetch_all(&pool)
            .await?;

        let (name_reader, mut name_writer) = evmap::new();

        for table in &tables {
            println!("Found Table {}", table.name);

            name_writer.insert(table.name.clone(), ());
        }

        name_writer.flush();

        Ok(Self {
            table_read: name_reader,
            table_write: Arc::new(Mutex::new(name_writer)),
            pool,
        })
    }

    pub fn exists_table<T>(&self, name: T) -> bool
    where
        T: ToString,
    {
        let table_name = name.to_string();

        self.table_read.contains_key(&table_name)
    }

    pub async fn refresh_table_list(&mut self) -> eyre::Result<()> {
        let get_tables = || sqlx::query_as::<_, Table>(GET_ALL_TABLES_QUERY).fetch_all(&self.pool);
        let tables = get_tables().await?;
        let mut table_writer = self.table_write.lock().await;

        table_writer.purge();

        println!("Refreshing Table list");

        for table in tables {
            table_writer.insert(table.name.clone(), ());

            println!("Found Table {}", table.name);
        }

        table_writer.flush();

        println!("Refreshed all Tabletable_read: table_names");

        Ok(())
    }
}
