// ─────────────────────────────────────────────────────────────────────────────
// Lane Racer – RISC Zero ZK Guest Program
// 
// This program runs inside the zkVM. It:
//   1. Reads the game seed + ordered list of player inputs from the host
//   2. Re-simulates the entire game deterministically
//   3. Computes the canonical score
//   4. Commits the (player_address, score, game_id) to the public journal
//
// The verifier (Soroban contract) only sees what is committed to the journal.
// The input sequence stays private – proving "I played honestly" without
// revealing every keystroke.
// ─────────────────────────────────────────────────────────────────────────────

#![no_main]

use risc0_zkvm::guest::env;
use shared::{GameInput, GameResult};  // ← import shared types

risc0_zkvm::guest::entry!(main);


// ─────────────────────────────────────────────────────────────────────────────
// Deterministic game simulation (mirrors frontend logic exactly)
// ─────────────────────────────────────────────────────────────────────────────

const LANES: usize = 3;
const BASE_SPEED_SCALE: u32 = 100; // 1.00x = 100
const SPEED_INCREMENT: u32 = 25;   // 0.25x per 15 obstacles
const OBSTACLES_PER_SPEED_UP: u32 = 15;

/// Simple LCG for deterministic obstacle/gem generation from seed
struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Self { state: seed ^ 0x9e3779b97f4a7c15 }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 32) as f32 / u32::MAX as f32
    }

    fn next_usize(&mut self, max: usize) -> usize {
        (self.next_u64() % max as u64) as usize
    }
}

#[derive(Debug, Clone)]
struct Obstacle {
    lane: usize,
    /// y position in game units (starts negative, moves down)
    y: i32,
    passed: bool,
}

#[derive(Debug, Clone)]
struct Gem {
    lane: usize,
    y: i32,
    collected: bool,
}

fn simulate_game(input: &GameInput) -> GameResult {
    let mut rng = Rng::new(input.seed);

    let mut player_lane: usize = 1;
    let mut score: u32 = 0;
    let mut obstacles_dodged: u32 = 0;
    let mut gems_collected: u32 = 0;
    let mut speed: u32 = BASE_SPEED_SCALE; // 100 = 1.00x
    let base_speed_px: i32 = 6;

    let mut obstacles: Vec<Obstacle> = Vec::new();
    let mut gems: Vec<Gem> = Vec::new();
    let mut collision = false;

    // Canvas constants (match frontend)
    let canvas_height: i32 = 600;
    let player_y: i32 = canvas_height - 200;
    let player_height: i32 = 100;

    // let _last_obstacle_y: i32 = -999;
    // let _tick: u64 = 0;

    for action in &input.actions {
        // tick += 1;

        // ── Player movement ─────────────────────────────────────────────────
        match action {
            1 if player_lane > 0 => player_lane -= 1,
            2 if player_lane < LANES - 1 => player_lane += 1,
            _ => {}
        }

        let effective_speed = (base_speed_px * speed as i32) / BASE_SPEED_SCALE as i32;

        // ── Move obstacles ──────────────────────────────────────────────────
        for obs in obstacles.iter_mut() {
            obs.y += effective_speed;

            // Collision check
            if !obs.passed
                && obs.y + 20 > player_y
                && obs.y - 20 < player_y + player_height
                && obs.lane == player_lane
            {
                collision = true;
            }

            // Passed check
            if !obs.passed && obs.y > player_y + player_height {
                obs.passed = true;
                obstacles_dodged += 1;
                score += 2;

                if obstacles_dodged % OBSTACLES_PER_SPEED_UP == 0 {
                    speed += SPEED_INCREMENT;
                }
            }
        }

        if collision {
            break;
        }

        // ── Move gems ───────────────────────────────────────────────────────
        for gem in gems.iter_mut() {
            gem.y += effective_speed;

            if !gem.collected
                && gem.y + 20 > player_y
                && gem.y - 20 < player_y + player_height
                && gem.lane == player_lane
            {
                gem.collected = true;
                gems_collected += 1;
                score += 10;
            }
        }

        // ── Remove off-screen objects ───────────────────────────────────────
        obstacles.retain(|o| o.y <= canvas_height + 50);
        gems.retain(|g| !g.collected && g.y <= canvas_height + 50);

        // ── Spawn obstacles (probabilistic, seeded) ─────────────────────────
        // ~1.5% chance per tick * speed multiplier
        let spawn_prob = (0.015 * (speed as f32 / BASE_SPEED_SCALE as f32) * 1000.0) as u64;
        if rng.next_u64() % 1000 < spawn_prob {
            // Ensure available lanes
            let mut available: Vec<usize> = (0..LANES)
                .filter(|&l| !obstacles.iter().any(|o| o.lane == l && o.y > -350))
                .collect();

            if available.len() >= 2 {
                // Shuffle
                for i in (1..available.len()).rev() {
                    let j = rng.next_usize(i + 1);
                    available.swap(i, j);
                }

                let num_spawn = if rng.next_f32() > 0.6 { 2 } else { 1 };
                let num_spawn = num_spawn.min(available.len() - 1); // always leave one lane clear

                for k in 0..num_spawn {
                    obstacles.push(Obstacle {
                        lane: available[k],
                        y: -50,
                        passed: false,
                    });
                }
                // last_obstacle_y = -50;
            }
        }

        // ── Spawn gems (0.8% chance per tick) ───────────────────────────────
        if rng.next_u64() % 1000 < 8 {
            let lane = rng.next_usize(LANES);
            let has_nearby = obstacles.iter().any(|o| o.lane == lane && o.y > -200 && o.y < 100);
            if !has_nearby {
                gems.push(Gem { lane, y: -50, collected: false });
            }
        }
    }

    GameResult {
        player_address: input.player_address.clone(),
        game_id: input.game_id,
        score,
        obstacles_dodged,
        gems_collected,
        speed_reached: speed,
        collision_occurred: collision,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Main entry point
// ─────────────────────────────────────────────────────────────────────────────

fn main() {
    // Read private inputs from host
    let input: GameInput = env::read();

    // Simulate game deterministically
    let result = simulate_game(&input);

    // Commit public outputs to the journal (visible to verifier / smart contract)
    env::commit(&result);
}
