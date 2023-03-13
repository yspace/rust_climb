
## rust re-exporting module
[Re-exporting with pub use (documentation)](https://users.rust-lang.org/t/re-exporting-with-pub-use-documentation/39286/3)

use self::module or use module are relative paths, similar to doing ./module on a filesystem.
use crate::module is an absolute path, similar to /module.
use super::module looks in the parent module, similar to ../module .
use name_of_extern_crate::module looks in an external crate for the module. The filesystem analogy kinda falls apart here :sweat_smile: