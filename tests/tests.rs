extern crate pkce;

#[test]
fn base64_url_is_valid() {
    let verifier = pkce::code_verifier(128);
    let challenge = pkce::code_challenge(&verifier);
    assert_eq!(
        challenge.chars().any(|c| c == '=' && c == '+' && c == '/'),
        false
    );
}

#[test]
fn verify_pregenerated_challenges() {
    let mut code_challenge = pkce::code_challenge(b"Hello-World~");
    assert_eq!(
        code_challenge,
        "qYov35Fip11uQLoKWCysGAKt8jP_7UYZY_-EuqNQOBg"
    );

    code_challenge =
        pkce::code_challenge(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
    assert_eq!(
        code_challenge,
        "20v8vU2gzYWmDDw30_vYgFx38V_Gsf3-YU7gp8j9tMA"
    );

    code_challenge = pkce::code_challenge(b"Ma la volpe col suo balzo ha raggiunto il quieto Fido");
    assert_eq!(
        code_challenge,
        "Wqg2eog5_VX3Opr14PYaKUImbpHzkK1aFLZvkYNbQkM"
    );
}

#[test]
fn code_verifier_valid_length() {
    // See https://tools.ietf.org/html/rfc7636#section-4.1
    assert_eq!(43, pkce::code_verifier(43).len());
    assert_eq!(128, pkce::code_verifier(128).len());
}

#[test]
#[should_panic]
fn code_verifier_invalid_length() {
    // See https://tools.ietf.org/html/rfc7636#section-4.1
    pkce::code_verifier(42);
    pkce::code_verifier(128);
}
