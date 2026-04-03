// Ligne 1: Imports nécessaires
use serde::{Serialize, Deserialize};

// Ligne 3: Imports pour les noms français
use fake::faker::name::raw::*;
use fake::locales::FR;
use fake::Fake;

// Ligne 6: Notre module loto
use crate::loto::LotoGrid;

// Ligne 8: Structure du joueur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub grid: LotoGrid,
}

// Ligne 15: Implémentation
impl Player {
    // Ligne 17: Générer un joueur aléatoire
    pub fn generate(id: u32) -> Self {
        // Ligne 19: Je génère un prénom français
        let first_name: String = FirstName(FR).fake();
        
        // Ligne 20: Je génère un nom français
        let last_name: String = LastName(FR).fake();
        
        // Ligne 22: Je crée le joueur avec sa grille aléatoire
        Self {
            id,
            first_name,
            last_name,
            grid: LotoGrid::generate(),
        }
    }
}

// Ligne 29: Structure des données du jeu
#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    pub draw_date: String,
    pub players: Vec<Player>,
    pub winning_grid: LotoGrid,
    pub results: Vec<GameResult>,
}

// Ligne 36: Résultat d'un joueur
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameResult {
    pub player_id: u32,
    pub player_name: String,
    pub numbers_matched: u8,
    pub chance_matched: bool,
    pub category: WinCategory,
}

// Ligne 44: Catégories de gains (comme au vrai Loto)
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Hash, Eq)]
pub enum WinCategory {
    Jackpot,     // 5 numéros + chance
    Rank2,       // 5 numéros
    Rank3,       // 4 numéros + chance
    Rank4,       // 4 numéros
    Rank5,       // 3 numéros + chance
    Rank6,       // 3 numéros
    Rank7,       // 2 numéros + chance
    Rank8,       // 2 numéros
    Rank9,       // 1 numéro + chance
    Rank10,      // 1 numéro
    NoWin,
}

// Ligne 60: Implémentation des catégories
impl WinCategory {
    // Ligne 62: Déterminer la catégorie selon les matches
    pub fn from_matches(numbers: u8, chance: bool) -> Self {
        match (numbers, chance) {
            (5, true) => WinCategory::Jackpot,
            (5, false) => WinCategory::Rank2,
            (4, true) => WinCategory::Rank3,
            (4, false) => WinCategory::Rank4,
            (3, true) => WinCategory::Rank5,
            (3, false) => WinCategory::Rank6,
            (2, true) => WinCategory::Rank7,
            (2, false) => WinCategory::Rank8,
            (1, true) => WinCategory::Rank9,
            (1, false) => WinCategory::Rank10,
            _ => WinCategory::NoWin,
        }
    }
    
    // Ligne 78: Description textuelle pour l'affichage
    pub fn description(&self) -> String {
        match self {
            WinCategory::Jackpot => "5 numéros + numéro chance - JACKPOT!".to_string(),
            WinCategory::Rank2 => "5 numéros".to_string(),
            WinCategory::Rank3 => "4 numéros + numéro chance".to_string(),
            WinCategory::Rank4 => "4 numéros".to_string(),
            WinCategory::Rank5 => "3 numéros + numéro chance".to_string(),
            WinCategory::Rank6 => "3 numéros".to_string(),
            WinCategory::Rank7 => "2 numéros + numéro chance".to_string(),
            WinCategory::Rank8 => "2 numéros".to_string(),
            WinCategory::Rank9 => "1 numéro + numéro chance".to_string(),
            WinCategory::Rank10 => "1 numéro".to_string(),
            WinCategory::NoWin => "Pas de gain".to_string(),
        }
    }
}