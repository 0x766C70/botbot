# [botbot]
***Disclaimer: ceci est un projet perso dans le but de découvrir le langage Rust, tout conseil sera donc très apprécié !***
* bot dédié à accompagner les personnes connecté sur les chan IRC/matrix d'fdn
* Il permet aussi des actions basiques d'interraction pour les admins

## Installation
- ***Pré-requis: le bot se base sur l'api python https://github.com/8go/matrix-commander. Il est nécessaire de l'installer en amont et de la configurer avec le compte dédié au bot***
- ***Pré-requis: le bot utilise l'API openai via https://github.com/sigoden/aichat***

1. git clone
2. dans le fichier main.rs modifier les chemins des répertoires de l'API pour les variables: MATRIX_FOLDER, MATRIX_CREDITENTIALS et MATRIX_DB_FOLDER
3. `cargo build`
4. lancer le program avec: `cargo run`

## How-to
botbot est un bot conversationnel basé sur openai avec un paramétrage du prompt pour décrire son environnement, sa fonction et son comportement (cf: https://platform.openai.com/docs/api-reference/making-requests)

## Admin
  - la config de bot se fait via le "role" `bar` utilisé par aichat.
  - la description de ce role se trouve dans un fichier yaml à la racine du projet.
