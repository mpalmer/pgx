use super::{SqlGraphEntity, SqlGraphIdentifier, ToSql};
use std::cmp::Ordering;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct InventorySchema {
    pub module_path: &'static str,
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
}

impl Ord for InventorySchema {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file
            .cmp(other.file)
            .then_with(|| self.file.cmp(other.file))
    }
}

impl PartialOrd for InventorySchema {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<SqlGraphEntity> for InventorySchema {
    fn into(self) -> SqlGraphEntity {
        SqlGraphEntity::Schema(self)
    }
}

impl SqlGraphIdentifier for InventorySchema {
    fn dot_identifier(&self) -> String {
        format!("schema {}", self.module_path)
    }
    fn rust_identifier(&self) -> String {
        self.module_path.to_string()
    }

    fn file(&self) -> Option<&'static str> {
        Some(self.file)
    }

    fn line(&self) -> Option<u32> {
        Some(self.line)
    }
}

impl ToSql for InventorySchema {
    #[tracing::instrument(level = "debug", err, skip(self, _context), fields(identifier = %self.rust_identifier()))]
    fn to_sql(&self, _context: &super::PgxSql) -> eyre::Result<String> {
        let sql = format!(
            "\n\
                    -- {file}:{line}\n\
                    CREATE SCHEMA IF NOT EXISTS {name}; /* {module_path} */\
                ",
            name = self.name,
            file = self.file,
            line = self.line,
            module_path = self.module_path,
        );
        tracing::debug!(%sql);
        Ok(sql)
    }
}