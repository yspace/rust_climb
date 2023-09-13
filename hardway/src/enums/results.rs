
fn _doc(){

/*
 * error processing: if a function fails, how should that failure be reported? 
 * Historically, special sentinel values (e.g. -errno return values from Linux system calls)
 * or global variables (errno for POSIX systems) were used. 
 * More recently, languages that support multiple or tuple return values (such as Go) from functions may have a convention of returning a (result, error) pair,
 *  assuming the existence of some suitable "zero" value for the result when the error is non-"zero".

In Rust, always encode the result of an operation that might fail as a Result<T, E>.
 The T type holds the successful result (in the Ok variant), 
 and the E type holds error details (in the Err variant) on failure. 
 Using the standard type makes the intent of the design clear, 
 and allows the use of standard transformations 
  and error processing ; 
  it also makes it possible to streamline error processing with the ? operator.
 */

}