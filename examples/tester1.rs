// examples/tester1.rs
use crystals_dilithium::dilithium2::Keypair;
use hex;

fn main() {
    // Generate keypair
    let keypair = Keypair::generate(None);
    
    // Your message
    let msg = b"Test message for sign!";
    
    // Sign the message
    let signature = keypair.sign(msg);
    
    // Verify the signature
    let is_verified = keypair.public.verify(msg, &signature);
    
    // Convert to hex
    let signature_hex = hex::encode(signature.as_ref());
    let public_key_hex = hex::encode(keypair.public.to_bytes());
    
    println!("=== Dilithium Signature Demo ===");
    println!("Message: '{}'", String::from_utf8_lossy(msg));
    println!("Message length: {} bytes", msg.len());
    println!("Public key (hex): {}...", &public_key_hex[..32]); // First 16 bytes
    println!("Signature (hex): {}...", &signature_hex[..32]); // First 16 bytes
    println!("Full signature length: {} bytes", signature.as_ref().len());
    // println!("Full signature (hex): {}", signature_hex);
    println!("Verification result: {}", is_verified);
    
    if is_verified {
        println!("✅ Signature is VALID");
    } else {
        println!("❌ Signature is INVALID");
    }
}
 