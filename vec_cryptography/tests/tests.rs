use vec_cryptography::cryptography::*;

#[test]
fn test_generate_seed_thread() {
    let seed = generate_seed_thread();
    assert_eq!(seed.len(), 32);
}

#[test]
fn test_generate_seed_os() {
    let seed = generate_seed_os();
    assert_eq!(seed.len(), 32);
}

#[test]
fn test_inherit_seed() {
    let seed = inherit_seed();
    assert_eq!(seed.len(), 32);
}

#[test]
fn test_generate_keypair() {
    let keypair = NodeKeypair::generate_keypair();
    assert_eq!(keypair.sk.to_bytes().len(), 32);
    assert_eq!(keypair.pk.to_bytes().len(), 32);
}

#[test]
fn test_sign_and_verify() {
    let message = b"Hello, world!";
    let keypair = NodeKeypair::generate_keypair();
    let signature = keypair.sign(message);
    assert!(keypair.verify(message, &signature));
}

#[test]
fn test_sign_and_verify_different_way() {
    let keypair = NodeKeypair::generate_keypair();
    let message = "Hello, world!".as_bytes();

    let signature = keypair.sign(&message);

    println!("message: {:?}", message);
    println!("pk key: {:?}", keypair.pk.as_bytes());
    println!("signature: {:?}", signature.signature.to_bytes());

    assert!(keypair.verify(&message, &signature));
}
