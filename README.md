# [botbot]
***Disclaimer: ceci est un projet perso dans le but de découvrir le langage Rust, tout conseil sera donc très apprécié !***
* bot dédié à accompagner les personnes connectées sur les chan IRC/matrix d'fdn
* Il permet aussi des actions basiques d'interraction pour les admins

## Installation
- ***Pré-requis: le bot se base sur l'api python https://github.com/8go/matrix-commander. Il est nécessaire de l'installer en amont et de la configurer avec le compte dédié au bot***
- ***Pré-requis: le bot utilise l'API openai via https://github.com/sigoden/aichat. Il est nécessaire d'inialiser l'api openai avec son token et son fichiers de roles dans ~/.config/aichat***

1. git clone https://git.fdn.fr/adminsys/botbot_v2.git
2. dans le fichier main.rs modifier les chemins des répertoires de l'API pour les variables: MATRIX_FOLDER, MATRIX_CREDITENTIALS et MATRIX_DB_FOLDER
3. `cargo build`
4. lancer le program avec: `cargo run`

## How-to
botbot est un bon conversationnel basé sur:
- un system de réponses via mot-clés dans une base sqlite
- openai et différents prompts qui décrivent son environnement, sa fonction et son comportement (cf: https://platform.openai.com/docs/api-reference/making-requests)

Le choix entre sql/openai se fait en fonction de la demande des utilisateurs. Par default, tout le monde fonctionne via sql. Sur demande, on peut mettre en place openai pour un utilisateur sur une room/chan spécifique avec un prompt spécifique. Sur la même room peut cohabiter des réponses sql et des réponses openai.
