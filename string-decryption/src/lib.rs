/*
    Prevent the compiler from inlining this function.

    If it is inlined, the decryption stage will
    be optimized away, and the real text will
    appear in the produced executable.
*/
#[inline(never)]
pub fn d((cipher_text, key): (&[u8], &[u8])) -> String {
    let mut text = vec![0; cipher_text.len()];

    for i in 0..text.len() {
        text[i] = cipher_text[i] ^ key[i];
    }

    String::from_utf8(text).unwrap()
}
