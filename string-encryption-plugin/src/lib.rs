#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

extern crate rand;

use rand::Rng;

use std::ops::Deref;
use std::rc::Rc;

use syntax::ptr::P;
use syntax::parse::token;
use syntax::tokenstream::TokenTree;
use syntax::ast::{Expr, LitKind, Ident};
use syntax::ext::quote::rt::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;
use rustc_plugin::Registry;

fn encrypt_str(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    let text = match args[0] {
        TokenTree::Token(_, token::Literal(token::Lit::Str_(s), _)) => s,
        _ => {
            cx.span_err(sp, "argument must be a string literal");
            return DummyResult::any(sp);
        }
    };
    let text = text.as_str();
    let text = text.deref();

    let (cipher_text, key) = encrypt_str_with_rand_key(text);

    let cipher_text_expr = get_expr_from_bytes(cx, sp, cipher_text);
    let key_expr = get_expr_from_bytes(cx, sp, key);

    // Generate a call to the decryption function in string-decryption
    MacEager::expr(cx.expr_call(
        sp,
        cx.expr_path(cx.path_global(sp, vec![Ident::from_str("d")])),
        vec![cx.expr_tuple(sp, vec![cipher_text_expr, key_expr])]
    ))
}

fn encrypt_str_with_rand_key(text: &str) -> (Vec<u8>, Vec<u8>) {
    let text = String::from(text);
    let mut text = text.into_bytes();

    // Generate a random key using the rand crate
    let mut random = rand::thread_rng();
    let mut key = vec![0; text.len()];
    random.fill_bytes(&mut key);

    // Encrypt the text using XOR
    for i in 0..text.len() {
        text[i] = text[i] ^ key[i];
    }

    (text, key)
}

fn get_expr_from_bytes(cx: &mut ExtCtxt, sp: Span, bytes: Vec<u8>) -> P<Expr> {
    cx.expr_lit(sp, LitKind::ByteStr(Rc::new(bytes)))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("e", encrypt_str);
}
