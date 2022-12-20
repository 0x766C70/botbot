////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  botbot v2 by vlp

/// INTERNAL CRATES
mod message;
mod database;
use crate::database::init_db;
mod matrix;
use crate::matrix::matrix_commander_daemon_launch;
mod botbot_actions;
use crate::botbot_actions::*;
mod my_system;

/// EXTERNAL CRATES
use std::io::{BufRead, BufReader};
use procfs::process::Process;
use regex::Regex;

// CONSTANTS
const MATRIX_FOLDER: &str = "/srv/bot/my_py3.8_env/my_app_venv/lib/python3.8/site-packages/matrix_commander/matrix_commander.py";
const MATRIX_CREDITENTIALS: &str = "-c/srv/bot/my_py3.8_env/my_app_venv/lib/python3.8/site-packages/matrix_commander/credentials.json";
const MATRIX_DB_FOLDER: &str = "-s/srv/bot/my_py3.8_env/my_app_venv/lib/python3.8/site-packages/matrix_commander/store/";
const MATRIX_DRIVE: &str = "/dev/vdb";
////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  FONCTION principale

fn main() {

    println!("///// botbot v2.1 by lovely fdn team");

    println!("[Database]");

    // _connexion à la db ou création de la db si n'existe pas
    // _initialisation de la liste des mots trigger "trigger_word_list": qui déclenchent une réponse de botbot
    // _la liste est placée dans un tableau remplis depuis la db pour pas à avoir à faire une requête
    // dans la db à chaque fois que botbot doit analyser les phrases.
    let (connection_db_result, mut trigger_word_list, adminsys_list, admincore_list) = init_db ();

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

    // _
    // _si error on quite le programme
    let matrix_commander_raw_buffer =
        match matrix_commander.stdout.as_mut(){
            Some(matrix_commander_raw_buffer) => matrix_commander_raw_buffer,
            None => {
                println!("Error: fail to attach buffer");
                return
            }
        };

    // _crée un buffer allimenter par le stdout du processus matrix-commander
    let mut matrix_commander_ready_buffer = BufReader::new(matrix_commander_raw_buffer);

    // _crée la variable "line_from_buffer" qui va pouvoir réceptionner les data du buffer ligne à ligne
    let mut line_from_buffer = String::new();

    // _pré-construit le regex pour identifier les numéros de tickets
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

    line_from_buffer.clear();

    //////////////// test
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1000));
    //     }
    // });
    //////////////// test

    // _boucle global qui est bloquante à cause de read.line qui attend un '\n' pour avancer
    loop {
        // _vérifie que le 'processus' de matrix-commander existe toujours en mémoire sinon arréte le program
        if matrix_pid.statm().unwrap().size == 0 {
            println!("matrix-commander do not respond, the application will shutdown");
            return;
        }

        //////////////// test
        // _affiche un message chaques minutes
        // for datetime in schedule.upcoming(Local).take(1) {
        //     let raw_date_now: DateTime<Local> = Local::now();
        //     let date_now = raw_date_now.format("%Y-%m-%d %H:%M:%S").to_string();
        //     let next_date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        //     if  date_now ==  next_date {
        //         println!("!ALIVE: {}", date_now);
        //     }
        // }
        //////////////// test

        // _lecture ligne à ligne du buffer
        let _buffer_control =
            match matrix_commander_ready_buffer.read_line(&mut line_from_buffer) {
                Ok(buffer_control_ctrl) => {
                    botbot_read(&line_from_buffer, &connection_db, &mut trigger_word_list, &adminsys_list, &admincore_list, &ticket_regex);
                    line_from_buffer.clear();
                    buffer_control_ctrl
                }
                Err(_e) => {
                    //println!("Unreadable line: {}", e);
                    line_from_buffer.clear();
                    continue;
                }
            };
        }
}
