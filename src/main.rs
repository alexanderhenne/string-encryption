/*
    Loading string_encryption_plugin as a
    plugin gives us access to the "e" macro,
    which encrypts string literals passed
    to it, preventing them from being seen
    by software that finds strings in files.
*/
#![feature(plugin)]
#![plugin(string_encryption_plugin)]

/*
    We also need to import the string_decryption,
    library which provides us with the "d" function,
    the function that gets the original string
    from the ciphertext and its key.
*/
extern crate string_decryption;

/*
    The injected macro calls the decryption function
    "d", so lets use it, so that the macro can find it.
*/
use string_decryption::d;

fn main() {
    /*
        The "Hello, world!" string will not
        be visible in the compiled binary.
    */
    let string = e!("Hello, world!");

    println!("{}", string); // Hello world!
}
