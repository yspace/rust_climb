// oop 中经常更偏爱继承。在rust中更偏爱组合

// Shared behavior
pub trait Module {
    fn name(&self) -> String;
    fn description(&self) -> String;
}

pub trait ModuleInstaller : Module {
    fn install(&self) ;
}

// 还有异步traits #[async_trait] 可以通过crate：async_trait 实现