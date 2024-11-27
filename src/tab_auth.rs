use anyhow::{anyhow, Result};
use sqlx::sqlite::Sqlite;
use sqlx::{query, query_scalar, Pool};

#[derive(Debug, sqlx::FromRow, Clone, Default)]
pub struct AuthOption {
    pub user_id: Option<i32>,
    pub auth_name: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuthQB {
    pool: Pool<Sqlite>,
    parm: AuthOption,
}

impl AuthQB {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        AuthQB {
            pool,
            parm: AuthOption::default(),
        }
    }

    pub fn set_parm(mut self, parm: AuthOption) -> Self {
        self.parm = parm;
        self
    }

    pub async fn init_table(&self) -> Result<()> {
        query(
            r#"CREATE TABLE IF NOT EXISTS auths (
                        user_id INTEGER NULL,
                        auth_name TEXT NOT NULL,
                        content TEXT NOT NULL,
                        PRIMARY KEY (user_id, auth_name))"#,
        )
        .execute(&self.pool.clone())
        .await?;
        Ok(())
    }

    pub async fn read(&self) -> Result<Option<String>> {
        let parm = self.parm.clone();
        parm.auth_name.as_ref().ok_or(anyhow!("auth_name is None"))?;

        let query_str = match parm.user_id {
            Some(_) => "SELECT content FROM auths WHERE user_id = $1 AND auth_name = $2",
            None => "SELECT content FROM auths WHERE user_id IS NULL AND auth_name = $2",
        };

        let content = query_scalar(query_str)
            .bind(parm.user_id)
            .bind(parm.auth_name)
            .fetch_optional(&self.pool.clone())
            .await?;
        return Ok(content);
    }

    pub async fn save(self) -> Result<Self> {
        let parm = self.parm.clone();
        parm.auth_name.as_ref().ok_or(anyhow!("auth_name is None"))?;
        parm.content.as_ref().ok_or(anyhow!("content is None"))?;

        let query_str = match self.read().await? {
            Some(_) => "UPDATE auths SET content = $3 WHERE user_id = $1 AND auth_name = $2",
            None => "INSERT INTO auths (user_id, auth_name, content) VALUES ($1, $2, $3)",
        };
        query(query_str)
            .bind(parm.user_id)
            .bind(parm.auth_name)
            .bind(parm.content)
            .execute(&self.pool.clone())
            .await?;
        return Ok(self);
    }
}
