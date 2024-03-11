use neon::prelude::*;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;

fn hash(mut cx: FunctionContext) -> JsResult<JsString> {
    let plaintext = cx.argument::<JsString>(0)?.value(&mut cx);

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2.hash_password(
        &plaintext.to_string().as_bytes(),
        &salt,
    ).unwrap();

    Ok(JsString::new(&mut cx, hash.serialize()))
}

fn verify(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let plaintext = cx.argument::<JsString>(0)?.value(&mut cx);
    let password_hash = cx.argument::<JsString>(1)?.value(&mut cx);

    let parsed_hash = PasswordHash::new(&password_hash).unwrap();

    let argon2 = Argon2::default();

    Ok(JsBoolean::new(&mut cx, if argon2.verify_password(plaintext.as_bytes(), &parsed_hash).is_ok() { true } else { false }))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hash", hash)?;
    cx.export_function("verify", verify)?;
    Ok(())
}
