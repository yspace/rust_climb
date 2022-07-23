use combu::command::presets::func::help_tablize_with_alias_dedup;
use combu::{action_result, check_error, check_help, done, preset_root, Command};
use combu::{Context, Flag};
use std::env;

fn main() {
	let _r = preset_root!(act)
		.usage(env!("CARGO_PKG_NAME").to_string() + " [args]")
		.common_flag(
			Flag::new_bool("help")
				.short_alias('h')
				.description("show help"),
		)
		.local_flag(
			Flag::new_bool("local")
				.short_alias('l')
				.description("local flag"),
		)
		.run_from_args(env::args().collect());
}

fn act(cmd: Command, c: Context) -> action_result!()
{
	check_error!(cmd, c);
	check_help!(cmd, c, help_tablize_with_alias_dedup);
	println!("Hello, combu - {:?}", c.args);

	done!()
}