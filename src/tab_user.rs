use anyhow::{anyhow, Result};
use sqlx::sqlite::Sqlite;
use sqlx::{query, query_as, Pool};

pub enum Verify {
    Success,
    NotFond,
    PwdError,
}

#[derive(Debug, Clone, Default, sqlx::FromRow)]
pub struct UserOption {
    pub user_id: Option<i32>,
    pub user_name: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserQB {
    pool: Pool<Sqlite>,
    parm: UserOption,
}

impl UserQB {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        UserQB {
            pool,
            parm: UserOption::default(),
        }
    }

    pub fn set_parm(mut self, parm: UserOption) -> Self {
        self.parm = parm;
        self
    }

    pub async fn init_table(self) -> Result<Self> {
        query(
            r#"CREATE TABLE IF NOT EXISTS users (user_id INTEGER PRIMARY KEY, user_name TEXT NOT NULL UNIQUE, password TEXT NOT NULL)"#
        ).execute(&self.pool.clone()).await?;
        Ok(self)
    }

    pub async fn confirm(self) -> Result<Verify> {
        let parm = self.parm.clone();
        parm.user_name.as_ref().ok_or(anyhow!("user_name is None"))?;
        parm.password.as_ref().ok_or(anyhow!("password is None"))?;

        let select_user: Option<UserOption> =
            query_as::<_, UserOption>("SELECT * FROM users WHERE user_name = ?")
                .bind(parm.user_name)
                .fetch_optional(&self.pool.clone())
                .await?;
        if let Some(user) = select_user {
            if let (Some(pwd_select), Some(pwd_parm)) = (user.password, parm.password) {
                if pwd_select == pwd_parm {
                    return Ok(Verify::Success);
                } else {
                    return Ok(Verify::PwdError);
                }
            }
        }
        return Ok(Verify::NotFond);
    }

    pub async fn regist(mut self) -> Result<Self> {
        println!("regist admin......");
        let parm = self.parm.clone();
        parm.user_name.as_ref().ok_or(anyhow!("user_name is None"))?;
        parm.password.as_ref().ok_or(anyhow!("password is None"))?;

        let select_user: Option<UserOption> =
            query_as::<_, UserOption>("SELECT * FROM users WHERE user_name = ?")
                .bind(parm.user_name.clone())
                .fetch_optional(&self.pool.clone())
                .await?;
        if select_user.is_some() {
            return Err(anyhow::anyhow!("User already exists"));
        }
        let select_user: UserOption = query_as::<_, UserOption>(
            "INSERT INTO users (user_name, password) VALUES (?, ?) RETURNING *",
        )
        .bind(parm.user_name.clone())
        .bind(parm.password.clone())
        .fetch_one(&self.pool.clone())
        .await?;

        println!("user:{:?} insert sucessful", select_user.user_name);
        self.parm.user_id = select_user.user_id;
        return Ok(self);
    }

    pub async fn change(mut self, new_pwd: String) -> Result<Self> {
        let parm = self.parm.clone();
        parm.user_id.as_ref().ok_or(anyhow!("user_id is None"))?;
        parm.user_name.as_ref().ok_or(anyhow!("user_name is None"))?;

        query("UPDATE users SET user_name = ?, password = ? WHERE user_id = ?")
            .bind(parm.user_name)
            .bind(new_pwd.clone())
            .bind(parm.user_id)
            .execute(&self.pool.clone())
            .await?;
        // check
        self.parm.password = Some(new_pwd);
        Ok(self)
    }
}
