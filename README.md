# pkce-rs
A very simple library for generating code verifiers and challenges to use for OAuth [Proof Key for Code Exchange](https://tools.ietf.org/html/rfc7636). It is not rocket science.

```rs
extern crate pkce;
 
fn main() {
    // Generate a random 128-byte code verifier (must be between 43 and 128 bytes)
    let code_verify = pkce::code_verifier(128);
    // Generate an encrypted code challenge accordingly
    let code_challenge = pkce::code_challenge(&code_verify);

    println!("Code challenge generated: {}", code_challenge);
}
```

Wow, it really is that simple.