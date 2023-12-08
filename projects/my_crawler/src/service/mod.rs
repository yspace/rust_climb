


use std::sync::Arc;
pub use crate::config::config::ApplicationConfig;
// pub use cache_service::*;
// pub use mem_service::*;
use once_cell::sync::Lazy;
use rbatis::rbatis::RBatis;



/// CONTEXT is all of the service struct
pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::service::CONTEXT.rb
    };
}

pub struct ServiceContext {
    // FIXME: 这里是拷贝自rbatis 后台管理程序的 ApplicationConfig 等价于Settings
    pub config: ApplicationConfig,
    pub rb: RBatis,
    // pub cache_service: CacheService,
    // pub sys_permission_service: SysPermissionService,
    // pub sys_user_service: SysUserService,
    // pub sys_role_service: SysRoleService,
    // pub sys_role_permission_service: SysRoleResService,
    // pub sys_user_role_service: SysUserRoleService,
    // pub sys_dict_service: SysDictService,
    // pub sys_auth_service: SysAuthService,
    // pub sys_trash_service: SysTrashService,
}

impl ServiceContext {
    /// init database pool
    pub async fn init_database(&self) {
        // log::info!(
        //     "[abs_admin] rbatis pool init ({})...",
        //     self.config.database_url
        // );
        // self.rb
        //     .link(include!("../../target/driver.rs"), &self.config.database_url).await
        //     .expect("[abs_admin] rbatis pool init fail!");
        // self.rb.intercepts.push(Arc::new(SysTrashService::new()));
        // log::info!(
        //     "[abs_admin] rbatis pool init success! pool state = {}",
        //     self.rb.get_pool().expect("pool not init!").state().await
        // );
        // log::info!(
        //     " - Local:   http://{}",
        //     self.config.server_url.replace("0.0.0.0", "127.0.0.1")
        // );
    }
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        ServiceContext {
            rb: {
                let rb = RBatis::new();
                // if rb.is_debug_mode() == false && config.debug.eq(&true) {
                //     panic!(
                //         r#"please edit application.json5   “debug: false” "#
                //     );
                // }
                rb
            },
            // cache_service: CacheService::new(&config).unwrap(),
            // sys_permission_service: SysPermissionService {},
            // sys_user_service: SysUserService {},
            // sys_role_service: SysRoleService {},
            // sys_role_permission_service: SysRoleResService {},
            // sys_user_role_service: SysUserRoleService {},
            // sys_dict_service: SysDictService {},
            // sys_auth_service: SysAuthService {},
            // sys_trash_service: SysTrashService::new(),
            config,
        }
    }
}
