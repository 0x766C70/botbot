////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  botbot v2 by vlp

// INTERNAL CRATES
mod message;
mod database;
use crate::database::init_db;
mod matrix;
use crate::matrix::{ matrix_commander_daemon_launch};
mod actions;
use crate::actions::*;
mod answers;

// EXTERNAL CRATES
use std::io::{BufRead, BufReader};
use std::env;
use procfs::process::Process;
use regex::Regex;
// CONSTANTS
const MATRIX_FOLDER: &str = "/srv/botbot_python3.8_venv/bin/matrix-commander";
const MATRIX_CREDITENTIALS: &str = "/srv/botbot_python3.8_venv/credentials.json";
const MATRIX_DB_FOLDER: &str = "/srv/botbot_python3.8_venv/store";
const MATRIX_MEDIA_FOLDER: &str = "/srv/botbot_python3.8_venv/media";
// ENV

////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  FONCTION principale

fn main() {
    // _env pour aichat afin d'avoir le fichier de role dans le repo
    let aichat_roles: &str = "AICHAT_ROLES_FILE";
    env::set_var(aichat_roles, "/srv/botbot_python3.8_venv/botbot_v2/roles.yaml");

    println!("///// botbot v3.0 by lovely fdn team");

    println!("[Database]");

    // _connexion à la db ou création de la db si n'existe pas
    // _initialisation de la liste des mots trigger "trigger_word_list": qui déclenchent une réponse de botbot
    // _la liste est placée dans un tableau remplis depuis la db pour pas à avoir à faire une requête
    // dans la db à chaque fois que botbot doit analyser les phrases.

    let (connection_db_result, mut trigger_word_list, user_list) = init_db ();
    // _controle de la connexion à la db
    // _si error on quite le programme
    let connection_db =
        match connection_db_result {
            Ok(connection_db_ctrl) => {
                println!(" > Database initialized with {} words", trigger_word_list.len());
                connection_db_ctrl
            }
            Err(e) => {
                println!("Error: Database initialization failed: {}", e);
                return
            }
        };

    println!("[Matrix Connection]");


    // _créer un processus fils au programme qui lance matrix-commander et qui pipe son flux stdout
    // _si error on quite le programme

    let mut matrix_commander =
        match matrix_commander_daemon_launch() {
            Ok(matrix_commander_ctrl) => matrix_commander_ctrl,
            Err(e) => {
                println!("Error: Fail to launch matrix-commander: {}", e);
                return
            }
        };

    // _crée une object 'processus" que l'on va pouvoir interroger pour vérifier que matrix-commander est toujours en vie
    // _si error on quite le programme
    let matrix_pid =
        match Process::new(matrix_commander.id() as i32) {
            Ok(matrix_pid_ctrl) => {
                println!(" > matrix-commander launched: pid {}", matrix_pid_ctrl.pid);
                matrix_pid_ctrl
            }
            Err(e) => {
                println!("Error: fail to get matrix-commander pid: {}", e);
                return
            }
        };

    // _pipe le stdout de matrix_commander dans un buffer
    // _si error on quite le programme
    let matrix_commander_raw_buffer =
        match matrix_commander.stdout.as_mut(){
            Some(matrix_commander_raw_buffer) => matrix_commander_raw_buffer,
            None => {
                println!("Error: fail to attach buffer");
                return
            }
        };

    // _crée un buffer allimenté par le stdout du processus matrix-commander
    let mut matrix_commander_ready_buffer = BufReader::new(matrix_commander_raw_buffer);

    //println!("from main: {:?}", matrix_commander_ready_buffer);
    
    // _crée la variable "line_from_buffer" qui va pouvoir réceptionner les data du buffer ligne à ligne
    let mut line_from_buffer = String::new();
    line_from_buffer.clear();

    // _pré-construit le regex ticket
    let ticket_to_search_re = "#[0-9]{4,6}".to_string();
    let ticket_regex =
        match Regex::new(&ticket_to_search_re){
            Ok(ticket_re_ctrl) => ticket_re_ctrl,
            Err(_e) => {
                println!("Error: fail to build ticket regex");
                return
            }
        };

    println!("[botbot is running]");

    // _boucle global qui est bloquante à cause de read.line qui attend un '\n' pour avancer
    loop {
        // _vérifie que le 'processus' de matrix-commander existe toujours en mémoire sinon arréte le program
        if matrix_pid.statm().unwrap().size == 0 {
            println!("matrix-commander do not respond, the application will shutdown");
            return;
        }

        // _lecture ligne à ligne du buffer
        let _buffer_control =
            match matrix_commander_ready_buffer.read_line(&mut line_from_buffer) {
                Ok(buffer_control_ctrl) => {
                    botbot_read(&line_from_buffer, &connection_db, &user_list, &mut trigger_word_list, &ticket_regex);
                    line_from_buffer.clear();
                    buffer_control_ctrl
                }
                Err(_e) => {
                    line_from_buffer.clear();
                    continue;
                }
            };
        }
}
