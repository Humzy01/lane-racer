#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror,
    Env, Address, Vec, BytesN
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    GameSession(u32),
    Leaderboard,
    Admin,
    GameHub,
}

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    SessionExists = 2,
    SessionNotFound = 3,
    NotAuthorized = 4,
    InvalidProof = 5,
}

#[contracttype]
#[derive(Clone)]
pub struct GameSession {
    pub session_id: u32,
    pub player: Address,
    pub score: u32,
    pub active: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct ScoreEntry {
    pub player: Address,
    pub score: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct ZKProof {
    pub seal: BytesN<64>,
    pub journal: BytesN<32>,
}

#[contract]
pub struct LaneRacerContract;

#[contractimpl]
impl LaneRacerContract {
    pub fn init(env: Env, admin: Address, game_hub: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::GameHub, &game_hub);
        let empty: Vec<ScoreEntry> = Vec::new(&env);
        env.storage().instance().set(&DataKey::Leaderboard, &empty);
    }

    pub fn start_game(
        env: Env,
        session_id: u32,
        player: Address,
    ) -> Result<(), Error> {
        player.require_auth();

        let game_hub: Address = env
            .storage()
            .instance()
            .get(&DataKey::GameHub)
            .ok_or(Error::NotInitialized)?;

        let session_key = DataKey::GameSession(session_id);
        if env.storage().instance().has(&session_key) {
            return Err(Error::SessionExists);
        }

        // Call game hub start_game
        env.invoke_contract::<()>(
            &game_hub,
            &soroban_sdk::Symbol::new(&env, "start_game"),
            soroban_sdk::vec![
                &env,
                soroban_sdk::IntoVal::into_val(&env.current_contract_address(), &env),
                soroban_sdk::IntoVal::into_val(&session_id, &env),
                soroban_sdk::IntoVal::into_val(&player, &env),
                soroban_sdk::IntoVal::into_val(&player, &env),
                soroban_sdk::IntoVal::into_val(&1000i128, &env),
                soroban_sdk::IntoVal::into_val(&1000i128, &env),
            ],
        );

        let session = GameSession {
            session_id,
            player,
            score: 0,
            active: true,
        };
        env.storage().instance().set(&session_key, &session);
        Ok(())
    }

    pub fn submit_score(
        env: Env,
        session_id: u32,
        player: Address,
        score: u32,
        _proof: ZKProof,
    ) -> Result<(), Error> {
        player.require_auth();

        let session_key = DataKey::GameSession(session_id);
        let mut session: GameSession = env
            .storage()
            .instance()
            .get(&session_key)
            .ok_or(Error::SessionNotFound)?;

        if session.player != player {
            return Err(Error::NotAuthorized);
        }

        let game_hub: Address = env
            .storage()
            .instance()
            .get(&DataKey::GameHub)
            .ok_or(Error::NotInitialized)?;

        // Call game hub end_game
        env.invoke_contract::<()>(
            &game_hub,
            &soroban_sdk::symbol_short!("end_game"),
            soroban_sdk::vec![
                &env,
                soroban_sdk::IntoVal::into_val(&session_id, &env),
                soroban_sdk::IntoVal::into_val(&true, &env),
            ],
        );

        // Update session
        session.score = score;
        session.active = false;
        env.storage().instance().set(&session_key, &session);

        // Update leaderboard
        let mut leaderboard: Vec<ScoreEntry> = env
            .storage()
            .instance()
            .get(&DataKey::Leaderboard)
            .unwrap_or(Vec::new(&env));

        leaderboard.push_back(ScoreEntry { player, score });
        env.storage().instance().set(&DataKey::Leaderboard, &leaderboard);

        Ok(())
    }

    pub fn get_leaderboard(env: Env) -> Vec<ScoreEntry> {
        env.storage()
            .instance()
            .get(&DataKey::Leaderboard)
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_session(env: Env, session_id: u32) -> Option<GameSession> {
        env.storage().instance().get(&DataKey::GameSession(session_id))
    }
}