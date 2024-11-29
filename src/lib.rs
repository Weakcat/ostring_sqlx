pub mod tab_auth;
pub mod tab_conf;
pub mod tab_user;
pub mod tab_userconf;

use anyhow::Result;
use sqlx::sqlite::Sqlite;
use sqlx::Pool;
use tab_auth::{AuthOption, AuthQB};
use tab_conf::{ConfOption, ConfQB};
use tab_user::{UserOption, UserQB, Verify};
use tab_userconf::{UconfOption, UconfQB};

#[derive(Debug, Clone)]
pub struct OSqliteMan {
    pool: Pool<Sqlite>,
}

impl ConfOption {}

impl OSqliteMan {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        OSqliteMan { pool }
    }
    pub async fn init_table(&self) -> Result<()> {
        AuthQB::new(self.pool.clone()).init_table().await?;
        UserQB::new(self.pool.clone()).init_table().await?;
        ConfQB::new(self.pool.clone()).init_table().await?;
        UconfQB::new(self.pool.clone()).init_table().await?;
        Ok(())
    }

    pub async fn add_user(&self, user: UserOption) -> Result<()> {
        let query_build = UserQB::new(self.pool.clone());
        let query_build = query_build.set_parm(user);
        match query_build.clone().confirm().await? {
            Verify::Success => println!("the user confirm pass"),
            Verify::PwdError => println!("PwdError"),
            Verify::NotFond => {
                query_build.clone().regist().await?;
            }
        }
        Ok(())
    }

    pub async fn init_conf(&self, conf: ConfOption) -> Result<()> {
        let query_build = ConfQB::new(self.pool.clone());
        query_build.set_parm(conf).init_conf().await?;
        Ok(())
    }

    pub async fn get_user_id(self, _user: UserOption) -> Result<Option<i32>> {
        Ok(Some(1))
    }

    pub async fn get_conf_id(self, conf_name: String) -> Result<Option<i32>> {
        let query_build = ConfQB::new(self.pool.clone());
        let conf_id = query_build.get_conf_id(conf_name).await?;
        Ok(conf_id)
    }

    pub async fn read_auth(self, auth: AuthOption) -> Result<Option<String>> {
        let query_build = AuthQB::new(self.pool.clone());
        query_build.set_parm(auth).read().await
    }

    pub async fn save_auth(self, auth: AuthOption) -> Result<()> {
        let query_build = AuthQB::new(self.pool.clone());
        query_build.set_parm(auth).save().await?;
        Ok(())
    }

    pub async fn get_all_user_conf(self, uconf: UconfOption) -> Result<Vec<UconfOption>> {
        let query_build = UconfQB::new(self.pool.clone());
        query_build.set_parm(uconf).get_all().await
    }

    pub async fn read_user_conf(self, uconf: UconfOption) -> Result<Option<String>> {
        let query_build = UconfQB::new(self.pool.clone());
        query_build.set_parm(uconf).read().await
    }

    pub async fn save_user_conf(self, uconf: UconfOption) -> Result<()> {
        let uconf_query_build = UconfQB::new(self.pool.clone());
        uconf_query_build.set_parm(uconf).save().await?;
        Ok(())
    }

    pub async fn remove_user_conf(self, uconf: UconfOption) -> Result<()> {
        let uconf_query_build = UconfQB::new(self.pool.clone());
        uconf_query_build.set_parm(uconf).remove().await?;
        Ok(())
    }
}
