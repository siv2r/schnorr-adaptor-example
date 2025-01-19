use bitcoin_hashes::sha256;
use secp256k1_zkp::{Secp256k1, Keypair, Message, SchnorrAdaptorPreSignature};

fn main() {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();

    let msg = b"This is some message";
    let msg = sha256::Hash::hash(msg);
    let msg = Message::from_digest_slice(msg.as_ref()).unwrap();

    // Alice setup
    let alice_keypair = Keypair::new(&secp, &mut rng);
    let (alice_pubkey, _parity) = alice_keypair.x_only_public_key();

    // t := secret_adaptor
    // There exists an public adaptor point T = t*G,
    // where t is unknown to Bob
    let (secret_adaptor, adaptor) = secp.generate_keypair(&mut rng);


    // Alice creates a pre-signature using the adaptor point T,
    // and sends it to Bob.
    let pre_sig = SchnorrAdaptorPreSignature::presign(&secp, &msg, &alice_keypair, &adaptor);

    // Bob extracts the adaptor point from the pre-signature,
    // and verifies if it is equal to T
    let adaptor_extracted = pre_sig.extract_adaptor(&msg, &alice_pubkey).unwrap();
    assert_eq!(adaptor, adaptor_extracted);

    // Bob learns t (the discrete logarithm of T). For example, Bob can
    // pay a Lightning invoice that reveals t, assuming Lightning uses
    // PTLC (Point Time Locked Contracts).

    // Bob adapts Alice's pre-signature with the discrete logarithm of T to
    // create a valid BIP 340 Schnorr signature.
    let bip340_sig = pre_sig.adapt(&secret_adaptor).unwrap();
    let is_valid = secp.verify_schnorr(&bip340_sig, &msg, &alice_pubkey);
    assert_eq!(is_valid, Ok(()));

    // Alice learns the BIP340 signature after Bob publishes it on the blockchain.

    // Alice extracts the discrete logarithm of T from the pre-signature and the
    // BIP 340 signature.
    let secret_adaptor_extracted = pre_sig.extract_secadaptor(&bip340_sig).unwrap();
    assert_eq!(secret_adaptor, secret_adaptor_extracted);

    println!("Execution successful!");
}
