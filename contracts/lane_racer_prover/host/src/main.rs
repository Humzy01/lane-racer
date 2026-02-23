use anyhow::Result;
use methods::{LANE_RACER_PROVER_ELF, LANE_RACER_PROVER_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use shared::{GameInput, GameResult};
use sha2::{Digest, Sha256};
use std::time::Instant;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[derive(serde::Serialize)]
struct ProofResponse {
    seal: String,
    journal: String,
    score: u32,
    obstacles_dodged: u32,
    gems_collected: u32,
    image_id: String,
    prove_time_secs: f64,
}

#[derive(serde::Deserialize)]
struct ProveRequest {
    score: Option<u32>,
    player: Option<String>,
    seed: Option<u64>,
    actions: Option<Vec<u32>>,
    game_id: Option<u32>,
}

fn prove_game(input: GameInput) -> Result<ProofResponse> {
    println!("[ZK] Building executor environment...");
    let env = ExecutorEnv::builder().write(&input)?.build()?;
    println!("[ZK] Generating proof...");
    let start = Instant::now();
    let prover = default_prover();
    let info = prover.prove(env, LANE_RACER_PROVER_ELF)?;
    let receipt = info.receipt;
    let elapsed = start.elapsed().as_secs_f64();
    println!("[ZK] Proof generated in {:.1}s", elapsed);
    receipt.verify(LANE_RACER_PROVER_ID)?;
    println!("[ZK] Verification passed ✓");
    let result: GameResult = receipt.journal.decode()?;
    println!("Score: {} | Obstacles: {} | Gems: {}", result.score, result.obstacles_dodged, result.gems_collected);
    let journal_hash = hex::encode(Sha256::digest(&receipt.journal.bytes));
    let receipt_bytes = bincode::serialize(&receipt)?;
    let seal = hex::encode(Sha256::digest(&receipt_bytes));
    let image_id_bytes: Vec<u8> = LANE_RACER_PROVER_ID.iter().flat_map(|x| x.to_be_bytes()).collect();
    let image_id = hex::encode(&image_id_bytes);
    Ok(ProofResponse { seal, journal: journal_hash, score: result.score, obstacles_dodged: result.obstacles_dodged, gems_collected: result.gems_collected, image_id, prove_time_secs: elapsed })
}

fn read_request(stream: &mut TcpStream) -> Option<(String, String)> {
    let mut buf = [0u8; 8192];
    let n = stream.read(&mut buf).ok()?;
    let raw = String::from_utf8_lossy(&buf[..n]).to_string();
    let first_line = raw.lines().next()?;
    let mut parts = first_line.split_whitespace();
    let method = parts.next()?.to_string();
    let path = parts.next()?.to_string();
    let body = if let Some(idx) = raw.find("\r\n\r\n") { raw[idx + 4..].to_string() } else { String::new() };
    Some((format!("{} {}", method, path), body))
}

fn send_response(stream: &mut TcpStream, status: u16, body: &str) {
    let status_text = if status == 200 { "OK" } else { "Bad Request" };
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: {}\r\n\r\n{}",
        status, status_text, body.len(), body
    );
    let _ = stream.write_all(response.as_bytes());
}

fn handle_connection(mut stream: TcpStream) {
    let (route, body) = match read_request(&mut stream) {
        Some(r) => r,
        None => return,
    };
    if route.starts_with("OPTIONS") { send_response(&mut stream, 200, "{}"); return; }
    if route == "GET /health" { send_response(&mut stream, 200, r#"{"status":"ok"}"#); return; }
    if route == "POST /prove" {
        println!("[SERVER] Received prove request");
        let req: ProveRequest = match serde_json::from_str(&body) {
            Ok(r) => r,
            Err(e) => { send_response(&mut stream, 400, &format!(r#"{{"error":"{}"}}"#, e)); return; }
        };
        let seed = req.seed.unwrap_or(42);
        let game_id = req.game_id.unwrap_or(1);
        let player = req.player.unwrap_or_else(|| "UNKNOWN".to_string());
        let actions = req.actions.unwrap_or_else(|| {
            let ticks = (req.score.unwrap_or(0) as usize * 10).max(50);
            vec![0u32; ticks]
        });
        let input = GameInput { seed, actions, player_address: player, game_id };
        match prove_game(input) {
            Ok(proof) => { let json = serde_json::to_string(&proof).unwrap(); send_response(&mut stream, 200, &json); }
            Err(e) => { send_response(&mut stream, 400, &format!(r#"{{"error":"{}"}}"#, e)); }
        }
        return;
    }
    send_response(&mut stream, 400, r#"{"error":"Unknown route"}"#);
}

fn main() -> Result<()> {
    let addr = "127.0.0.1:3002";
    let listener = TcpListener::bind(addr)?;
    println!("╔══════════════════════════════════════╗");
    println!("║   Lane Racer ZK Prover — Port 3002   ║");
    println!("║   POST /prove  — generate ZK proof   ║");
    println!("║   GET  /health — health check        ║");
    println!("╚══════════════════════════════════════╝");
    for stream in listener.incoming() {
        if let Ok(s) = stream {
            std::thread::spawn(move || handle_connection(s));
        }
    }
    Ok(())
}
