
use std::rc::Rc;
use deno_core::error::AnyError;


async fn run_js(file_path: &str) -> Result<(), AnyError> {
   // let main_module = deno_core::resolve_path(file_path)?;
   // let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
   //     module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
   //       ..Default::default()
   //  });
   //
   //  // todo: js_runtime.execute_script()
   // let mod_id = js_runtime.load_main_module(&main_module, None).await?;
   // let result = js_runtime.mod_evaluate(mod_id);
   // js_runtime.run_event_loop(false).await?;
   // result.await?;

   Ok(())
}

// main.rs
fn main() {
  let runtime = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap();
  if let Err(error) = runtime.block_on(run_js("./example.js")) {
    eprintln!("error: {}", error);
  }
}

