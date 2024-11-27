use anyhow::{anyhow, Result};
use sqlx::sqlite::Sqlite;
use sqlx::{query, query_as, query_scalar, Pool};

#[derive(Debug, sqlx::FromRow, Clone, Default)]
pub struct ConfOption {
    pub conf_id: Option<i32>,
    pub conf_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConfQB {
    pool: Pool<Sqlite>,
    parm: ConfOption,
}

impl ConfQB {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        ConfQB {
            pool,
            parm: ConfOption::default(),
        }
    }

    pub fn set_parm(mut self, parm: ConfOption) -> Self {
        self.parm = parm;
        self
    }

    pub async fn init_table(self) -> Result<Self> {
        query("CREATE TABLE IF NOT EXISTS confs (conf_id INTEGER PRIMARY KEY, conf_name TEXT NOT NULL UNIQUE, description TEXT)").execute(&self.pool.clone()).await?;
        Ok(self)
    }

    pub async fn init_conf(mut self) -> Result<Self> {
        let parm = self.parm.clone();
        parm.conf_name.as_ref().ok_or(anyhow!("conf_name is None"))?;

        let select_conf: Option<ConfOption> =
            query_as::<_, ConfOption>("SELECT * FROM confs WHERE conf_name = ?")
                .bind(parm.conf_name.clone())
                .fetch_optional(&self.pool.clone())
                .await?;
        if let Some(conf) = select_conf {
            self.parm = conf;
            return Ok(self);
        }
        let query_str = "INSERT INTO confs (conf_name, description) VALUES (?, ?) RETURNING *";
        let init_dec = self.parm.description.unwrap_or("".to_string());
        let select_conf: ConfOption = query_as::<_, ConfOption>(query_str)
            .bind(parm.conf_name.clone())
            .bind(init_dec)
            .fetch_one(&self.pool.clone())
            .await?;
        self.parm = select_conf;
        return Ok(self);
    }

    pub async fn search(mut self) -> Result<Self> {
        let parm = self.parm.clone();
        parm.conf_name.as_ref().ok_or(anyhow!("conf_name is None"))?;

        let select_conf: ConfOption = query_as("SELECT * FROM confs WHERE conf_name = ?")
            .bind(parm.conf_name.clone())
            .fetch_one(&self.pool.clone())
            .await?;
        self.parm = select_conf;
        return Ok(self);
    }

    pub async fn change(mut self, new_des: String) -> Result<Self> {
        let parm = self.parm.clone();
        parm.conf_id.as_ref().ok_or(anyhow!("conf_id is None"))?;
        parm.conf_name.as_ref().ok_or(anyhow!("conf_name is None"))?;

        let select_conf: ConfOption =
            query_as("UPDATE confs SET conf_name = ? WHERE conf_id = ? RETURNING *")
                .bind(parm.conf_name)
                .bind(new_des.clone())
                .bind(parm.conf_id)
                .fetch_one(&self.pool.clone())
                .await?;
        self.parm = select_conf;
        return Ok(self);
    }

    pub async fn get_conf_id(self, name: String) -> Result<Option<i32>> {
        let select_conf: Option<ConfOption> = query_as("SELECT * FROM confs WHERE conf_name = ?")
            .bind(name.clone())
            .fetch_optional(&self.pool.clone())
            .await?;
        if let Some(conf) = select_conf {
            return Ok(conf.conf_id);
        }

        let desc = self.parm.description.unwrap_or("".to_string());
        let cmd = "INSERT INTO confs (conf_name, description) VALUES (?, ?) RETURNING conf_id";
        let conf_id: i32 = query_scalar(cmd)
            .bind(name.clone())
            .bind(desc)
            .fetch_one(&self.pool.clone())
            .await?;
        println!("INSERT INTO confs successful");
        return Ok(Some(conf_id));
    }
}
