// Ligne 1: Je sais que je vais avoir besoin de Serialize/Deserialize
use serde::{Serialize, Deserialize};

// Ligne 2: Pour mélanger les nombres aléatoirement
use rand::seq::SliceRandom;

// Ligne 3: Pour générer des nombres aléatoires simples
use rand::Rng;

// Ligne 4: Je commence par définir la structure d'une grille
// Je réfléchis: 5 numéros sur 49, 1 chance sur 10
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LotoGrid {
    pub numbers: Vec<u8>,  // u8 car max 49, ça suffit
    pub chance: u8,         // u8 car max 10
}

// Ligne 9: J'implémente les méthodes pour cette structure
impl LotoGrid {
    // Ligne 11: Méthode pour générer une grille aléatoire
    pub fn generate() -> Self {
        // Ligne 13: Je crée un générateur aléatoire
        let mut rng = rand::thread_rng();
        
        // Ligne 15: Je crée une liste de 1 à 49
        let mut numbers: Vec<u8> = (1..=49).collect();
        
        // Ligne 17: Je mélange cette liste
        numbers.shuffle(&mut rng);
        
        // Ligne 19: Je prends les 5 premiers
        let mut selected: Vec<u8> = numbers.into_iter().take(5).collect();
        
        // Ligne 20: Je les trie pour que ce soit plus joli
        selected.sort();
        
        // Ligne 22: Je génère le numéro chance (1-10)
        let chance = rng.gen_range(1..=10);
        
        // Ligne 24: Je retourne la grille
        Self {
            numbers: selected,
            chance,
        }
    }
    
    // Ligne 29: Méthode pour comparer deux grilles
    pub fn matches(&self, other: &LotoGrid) -> (u8, bool) {
        // Ligne 31: Je compte combien de numéros correspondent
        let number_matches = self.numbers.iter()
            .filter(|&n| other.numbers.contains(n))
            .count() as u8;
        
        // Ligne 34: Je vérifie si le numéro chance correspond
        let chance_match = self.chance == other.chance;
        
        // Ligne 36: Je retourne un tuple (nombre_correct, chance_correct)
        (number_matches, chance_match)
    }
}

// Ligne 41: Petit test rapide pour vérifier
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generation() {
        let grid = LotoGrid::generate();
        assert_eq!(grid.numbers.len(), 5);
        assert!(grid.chance >= 1 && grid.chance <= 10);
    }
}