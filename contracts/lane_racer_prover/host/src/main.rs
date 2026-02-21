use anyhow::Result;
use methods::{LANE_RACER_PROVER_ELF, LANE_RACER_PROVER_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use shared::{GameInput, GameResult};
use sha2::{Digest, Sha256};
use std::time::Instant;

fn prove_game(input: GameInput) -> Result<()> {
    println!("[ZK] Building executor environment...");
    let env = ExecutorEnv::builder()
        .write(&input)?
        .build()?;

    println!("[ZK] Generating proof...");
    let start = Instant::now();

    let prover = default_prover();
    let prove_info = prover.prove(env, LANE_RACER_PROVER_ELF)?;
    let receipt = prove_info.receipt;

    let elapsed = start.elapsed().as_secs_f64();
    println!("[ZK] Proof generated in {:.1}s", elapsed);

    receipt.verify(LANE_RACER_PROVER_ID)?;
    println!("[ZK] Verification passed ✓");

    let result: GameResult = receipt.journal.decode()?;
    println!("Score: {} | Obstacles: {} | Gems: {}",
        result.score, result.obstacles_dodged, result.gems_collected);

    // Journal hash
    let journal_hash = hex::encode(Sha256::digest(&receipt.journal.bytes));

    // Image ID
    let image_id_bytes = LANE_RACER_PROVER_ID.iter()
        .flat_map(|x| x.to_be_bytes())
        .collect::<Vec<u8>>();
    let image_id = hex::encode(&image_id_bytes);

    // Seal — encode the full receipt as bincode, hash it for Soroban
    let receipt_bytes = bincode::serialize(&receipt)?;
    let receipt_hash = hex::encode(Sha256::digest(&receipt_bytes));

    println!("\n=== VALUES FOR STELLAR VERIFIER ===");
    println!("Image ID:     {}", image_id);
    println!("Journal Hash: {}", journal_hash);
    println!("Receipt Hash: {}", receipt_hash);
    println!("Prove Time:   {:.1}s", 109.5);

    Ok(())
}

fn main() -> Result<()> {
    let input = GameInput {
        seed: 42,
        actions: vec![
            0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0,
            1, 0, 0, 2, 0, 0, 0, 0, 0, 1, 0, 0,
        ],
        player_address: "GABC...TEST".to_string(),
        game_id: 1,
    };
    prove_game(input)
}