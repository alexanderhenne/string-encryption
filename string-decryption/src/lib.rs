#[inline(never)]
pub fn d((cipher_text, key): (&[u8], &[u8])) -> String {
    let mut text = vec![0; cipher_text.len()];

    for i in 0..text.len() {
        text[i] = cipher_text[i] ^ key[i];
    }

    String::from_utf8(text).unwrap()
}
