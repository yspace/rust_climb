/*

不同的占位符 是对应不同的trait实现的

Trait 	Placeholder 	Description
Display 	{} 	display representation
Debug 	{:?} 	debug representation
Octal 	{:o} 	octal representation
LowerHex 	{:x} 	lowercase hex representation
UpperHex 	{:X} 	uppercase hex representation
Pointer 	{:p} 	memory address
Binary 	{:b} 	binary representation
LowerExp 	{:e} 	lowercase exponential representation
UpperExp 	{:E} 	uppercase exponential representation

 */pub mod display_to_string;
pub mod debug;
