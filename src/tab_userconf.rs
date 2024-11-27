use anyhow::{anyhow, Result};
use sqlx::sqlite::Sqlite;
use sqlx::{query, query_as, query_scalar, Pool};

#[derive(Debug, Clone, Default, sqlx::FromRow)]
pub struct UconfOption {
    pub user_id: Option<i32>,
    pub conf_id: Option<i32>,
    pub version: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UconfQB {
    pool: Pool<Sqlite>,
    parm: UconfOption,
}

impl UconfQB {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        UconfQB {
            pool,
            parm: UconfOption::default(),
        }
    }

    pub fn set_parm(mut self, parm: UconfOption) -> Self {
        self.parm = parm;
        self
    }

    pub async fn init_table(self) -> Result<Self> {
        query(
            r#"CREATE TABLE IF NOT EXISTS user_confs (
                    user_id INTEGER NOT NULL, 
                    conf_id INTEGER NOT NULL, 
                    version TEXT NOT NULL, 
                    content TEXT NOT NULL, 
                    PRIMARY KEY (user_id, conf_id, version))
                "#,
        )
        .execute(&self.pool.clone())
        .await?;
        Ok(self)
    }

    pub async fn get_all(&self) -> Result<Vec<UconfOption>> {
        let parm = self.parm.clone();
        parm.user_id.as_ref().ok_or(anyhow!("user_id is None"))?;
        parm.conf_id.as_ref().ok_or(anyhow!("conf_id is None"))?;

        let confs: Vec<UconfOption> =
            query_as("SELECT * FROM user_confs WHERE user_id = ? AND conf_id = ?")
                .bind(parm.user_id)
                .bind(parm.conf_id)
                .fetch_all(&self.pool.clone())
                .await?;
        return Ok(confs);
    }

    pub async fn read(&self) -> Result<Option<String>> {
        let parm = self.parm.clone();
        parm.user_id.as_ref().ok_or(anyhow!("user_id is None"))?;
        parm.conf_id.as_ref().ok_or(anyhow!("conf_id is None"))?;
        parm.version.as_ref().ok_or(anyhow!("version is None"))?;

        let content = query_scalar(
            "SELECT content FROM user_confs WHERE user_id = ? AND conf_id = ? AND version = ?",
        )
        .bind(parm.user_id)
        .bind(parm.conf_id)
        .bind(parm.version)
        .fetch_optional(&self.pool.clone())
        .await?;
        return Ok(content);
    }

    pub async fn save(self) -> Result<Self> {
        let parm = self.parm.clone();
        parm.user_id.as_ref().ok_or(anyhow!("user_id is None"))?;
        parm.conf_id.as_ref().ok_or(anyhow!("conf_id is None"))?;
        parm.version.as_ref().ok_or(anyhow!("version is None"))?;
        parm.content.as_ref().ok_or(anyhow!("content is None"))?;

        let query_str = match self.read().await? {
            Some(_) => "UPDATE user_confs SET content = $4 WHERE user_id = $1 AND conf_id = $2 AND version = $3",
            None => "INSERT INTO user_confs (user_id, conf_id, version, content) VALUES ($1, $2, $3, $4)"
        };
        query(query_str)
            .bind(parm.user_id)
            .bind(parm.conf_id)
            .bind(parm.version)
            .bind(parm.content)
            .execute(&self.pool.clone())
            .await?;
        return Ok(self);
    }

    pub async fn remove(self) -> Result<()> {
        let parm = self.parm.clone();
        parm.user_id.as_ref().ok_or(anyhow!("user_id is None"))?;
        parm.conf_id.as_ref().ok_or(anyhow!("conf_id is None"))?;
        parm.version.as_ref().ok_or(anyhow!("version is None"))?;
        query("DELETE FROM user_confs WHERE user_id = ? AND conf_id = ? AND version = ?")
            .bind(parm.user_id)
            .bind(parm.conf_id)
            .bind(parm.version)
            .execute(&self.pool.clone())
            .await?;
        Ok(())
    }
}
