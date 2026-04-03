# Simulateur de Loto 🎰

## Installation
```bash
cargo build --release
cargo run --release
```

## Règles

6 numéros parmi 49
1 numéro chance parmi 10

📊 Nouvelles probabilités (avec 6 numéros)
Avec 10 000 joueurs, tu devrais voir environ :

Jackpot (6+1) : 0 (1 chance sur 19 millions !)

6 numéros : 0 (1 chance sur 13.9 millions)

5 numéros+chance : 0-1 joueur

5 numéros : ~1-2 joueurs

4 numéros+chance : ~3-4 joueurs

4 numéros : ~50-60 joueurs

3 numéros : ~800-1000 joueurs

2 numéros : ~8000 joueurs