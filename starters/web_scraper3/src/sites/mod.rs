// 本模块是所有网站目标trait的实现 也可以起名为 target_site_impls

// 关于插件系统 可以参考下 https://crates.io/crates/inventory

mod ravenel;
mod lnmuseum;

pub use ravenel::*;
pub use lnmuseum::*;

// mod chanliu;
// pub use chanliu::*;
