use crate::app::entity;
use crate::config;
use crate::config::Config;
use crate::infra::db;
use std::sync::Arc;
use wd_run::{Context, Task, TaskBuild, TaskInfo};
use wd_tools::PFArc;

#[derive(Debug, Default, Clone)]
pub struct CleanTask {}

#[wd_run::async_trait]
impl TaskBuild for CleanTask {
    fn args(&self) -> TaskInfo {
        TaskInfo::new("clean", "clean all task").register_arg(
            "-c",
            "./src/config/dev_config.toml",
            "config file path",
        )
    }

    async fn build(&mut self, mut ctx: Context) -> Arc<dyn Task> {
        let config_file = ctx.load::<String>("c").unwrap();
        wd_log::log_debug_ln!("config path:{}", config_file);

        let cfg = wd_log::res_panic!(config::load_config_by_file(config_file);"load config failed");
        wd_log::log_debug_ln!("config load success --->{}", cfg.to_string());

        CleanEntity { cfg }.arc()
    }
}

pub struct CleanEntity {
    cfg: Config,
}

#[wd_run::async_trait]
impl Task for CleanEntity {
    async fn run(&self) -> anyhow::Result<()> {
        let client = db::EtcdClient::init(self.cfg.etcd.clone()).await?;
        entity::clean_tasks(client).await?;
        Ok(())
    }
}
