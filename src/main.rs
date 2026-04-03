// Ligne 1: Déclarer les modules
mod loto;
mod player;

// Ligne 4: Imports
use eframe::egui;
// use egui::Ui;  // Ajout important
use player::*;
use loto::*;
use chrono::Local;
use std::collections::HashMap;

// Ligne 9: États de l'application
#[derive(PartialEq)]
enum AppState {
    Config,    // Écran de configuration
    Results,   // Écran des résultats
}

// Ligne 15: Structure principale de l'app
struct LotoApp {
    state: AppState,
    num_players: u32,
    game_data: Option<GameData>,
    statistics: HashMap<WinCategory, u32>,
    jackpot_winners: Vec<Player>,
}

// Ligne 23: Implémentation par défaut
impl Default for LotoApp {
    fn default() -> Self {
        Self {
            state: AppState::Config,
            num_players: 100,  // Valeur par défaut
            game_data: None,
            statistics: HashMap::new(),
            jackpot_winners: Vec::new(),
        }
    }
}

// Ligne 34: Implémentation des méthodes
impl LotoApp {
    fn run_simulation(&mut self) {
        println!("🎲 Simulation du Loto pour {} joueurs...", self.num_players);
        
        // Ligne 39: Générer les joueurs
        let mut players = Vec::new();
        for i in 1..=self.num_players {
            players.push(Player::generate(i));
        }
        
        // Ligne 45: Tirage gagnant
        let winning_grid = LotoGrid::generate();
        println!("🏆 Tirage gagnant: {:?} - Chance: {}", 
                 winning_grid.numbers, winning_grid.chance);
        
        // Ligne 50: Calculer les résultats
        let mut results = Vec::new();
        let mut stats = HashMap::new();
        let mut winners = Vec::new();
        
        for player in &players {
            let (numbers_matched, chance_matched) = player.grid.matches(&winning_grid);
            let category = WinCategory::from_matches(numbers_matched, chance_matched);
            
            if category == WinCategory::Jackpot {
                winners.push(player.clone());
            }
            
            *stats.entry(category.clone()).or_insert(0) += 1;
            
            results.push(GameResult {
                player_id: player.id,
                player_name: format!("{} {}", player.first_name, player.last_name),
                numbers_matched,
                chance_matched,
                category,
            });
        }
        
        // Ligne 72: Sauvegarder en JSON
        let game_data = GameData {
            draw_date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            players,
            winning_grid,
            results,
        };
        
        // Ligne 79: Créer le dossier si nécessaire
        std::fs::create_dir_all("src/data").unwrap();
        
        // Ligne 81: Sauvegarder
        let json = serde_json::to_string_pretty(&game_data).unwrap();
        std::fs::write("src/data/players.json", json).unwrap();
        println!("💾 Données sauvegardées dans src/data/players.json");
        
        // Ligne 85: Mettre à jour l'état
        self.game_data = Some(game_data);
        self.statistics = stats;
        self.jackpot_winners = winners;
        self.state = AppState::Results;
    }
}

// Ligne 92: Implementation du trait eframe::App
impl eframe::App for LotoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Ligne 94: Panel central
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎰 Simulateur de Loto Français");
            ui.separator();
            
            match self.state {
                AppState::Config => {
                    // Ligne 101: Interface de configuration
                    ui.horizontal(|ui| {
                        ui.label("Nombre de joueurs:");
                        let response = ui.add(egui::DragValue::new(&mut self.num_players)
                            .clamp_range(1..=100000)
                            .speed(1));
                        if response.changed() {
                            self.num_players = self.num_players.clamp(1, 100000);
                        }
                    });
                    
                    ui.add_space(20.0);
                    
                    if ui.button("🎲 Lancer le tirage").clicked() {
                        self.run_simulation();
                    }
                }
                
                AppState::Results => {
                    // Ligne 118: Interface des résultats
                    if let Some(data) = &self.game_data {
                        ui.horizontal(|ui| {
                            ui.label(format!("📅 Date du tirage: {}", data.draw_date));
                        });
                        
                        ui.separator();
                        
                        ui.heading("🏆 Résultats du tirage");
                        ui.label(format!(
                            "Numéros gagnants: {:?} | Numéro chance: {}", 
                            data.winning_grid.numbers, 
                            data.winning_grid.chance
                        ));
                        
                        ui.separator();
                        
                        ui.heading("📊 Statistiques des gains");
                        ui.separator();
                        
                        // Ligne 135: Afficher toutes les catégories
                        let categories = vec![
                            WinCategory::Jackpot,
                            WinCategory::Rank2,
                            WinCategory::Rank3,
                            WinCategory::Rank4,
                            WinCategory::Rank5,
                            WinCategory::Rank6,
                            WinCategory::Rank7,
                            WinCategory::Rank8,
                            WinCategory::Rank9,
                            WinCategory::Rank10,
                            WinCategory::NoWin,
                        ];
                        
                        for category in categories {
                            let count = self.statistics.get(&category).unwrap_or(&0);
                            if *count > 0 || category == WinCategory::Jackpot {
                                ui.label(format!(
                                    "{}: {} joueur(s)", 
                                    category.description(),
                                    count
                                ));
                            }
                        }
                        
                        ui.separator();
                        
                        // Ligne 158: Afficher les gagnants
                        if !self.jackpot_winners.is_empty() {
                            ui.colored_label(egui::Color32::GOLD, "🎉 JACKPOT GAGNÉ !!! 🎉");
                            ui.colored_label(egui::Color32::GREEN, "Félicitations aux gagnants :");
                            for winner in &self.jackpot_winners {
                                ui.label(format!(
                                    "🏅 {} {} (ID: {})", 
                                    winner.first_name, 
                                    winner.last_name, 
                                    winner.id
                                ));
                                ui.label(format!(
                                    "   Grille gagnante: {:?} | Chance: {}", 
                                    winner.grid.numbers,
                                    winner.grid.chance
                                ));
                            }
                        } else {
                            ui.colored_label(egui::Color32::RED, "😢 Pas de gagnant du jackpot cette fois-ci...");
                        }
                        
                        ui.separator();
                        
                        if ui.button("🔄 Nouvelle simulation").clicked() {
                            self.state = AppState::Config;
                            self.game_data = None;
                            self.statistics.clear();
                            self.jackpot_winners.clear();
                        }
                    }
                }
            }
        });
    }
}

// Ligne 197: Point d'entrée principal
fn main() -> Result<(), eframe::Error> {
    // Ligne 199: Configuration de la fenêtre
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Simulateur de Loto"),
        ..Default::default()
    };
    
    // Ligne 206: Lancer l'application
    /*     eframe::run_native(
        "Simulateur de Loto",
        options,
        Box::new(|_cc| Ok(Box::new(LotoApp::default()))), 
    ) */

    // Version simplifiée qui devrait fonctionner
    eframe::run_native(
        "Simulateur de Loto",
        options,
        Box::new(|_cc| Box::new(LotoApp::default())),
    )

}
