An example showcasing the (not yet merged) Schnorr adaptor module in the rust-libsecp256k1-zkp library.

API Details
---
- `SchnorrAdaptorPreSignature::presign`
   - creates a pre-signature for a given message and adaptor point
   - The pre-signature can be converted into a valid BIP-340 Schnorr signature by combining it with the discrete logarithm of the adaptor point
```rust
pub fn presign<C: Signing>(
    secp: &Secp256k1<C>,
    msg: &Message,
    keypair: &Keypair,
    adaptor: &PublicKey,
) -> SchnorrAdaptorPreSignature
```
- `SchnorrAdaptorPreSignature::extract_adaptor`
   - Extracts the adaptor point from a pre-signature
```rust
pub fn extract_adaptor(&self, msg: &Message, pubkey: &XOnlyPublicKey) -> Result<PublicKey, Error>
```
- `SchnorrAdaptorPreSignature::adapt`
   - Adapts the pre-signature to produce a BIP-340 Schnorr signature
```rust
pub fn adapt(&self, sec_adaptor: &SecretKey) -> Result<SchnorrSignature, Error>
```
- `SchnorrAdaptorPreSignature::extract_secadaptor`
   - Extracts the secret adaptor (discrete logarithm of the adaptor point) from a pre-signature and the corresponding BIP-340 signature
```rust
pub fn extract_secadaptor(&self, sig: &SchnorrSignature) -> Result<SecretKey, Error>
```