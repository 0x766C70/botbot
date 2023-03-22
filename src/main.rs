////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  botbot v2 by vlp

/// INTERNAL CRATES
mod message;
//mod database;
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
const MATRIX_FOLDER: &str = "/srv/botbot_python3.8_venv/lib/python3.8/site-packages/matrix_commander/matrix_commander.py";
const MATRIX_CREDITENTIALS: &str = "-c/srv/botbot_python3.8_venv/lib/python3.8/site-packages/matrix_commander/credentials.json";
const MATRIX_DB_FOLDER: &str = "-s/srv/botbot_python3.8_venv/lib/python3.8/site-packages/matrix_commander/store/";
////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  FONCTION principale

fn main() {

    println!("///// botbot v2.2 by lovely fdn team");

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

    // _pré-construit les regex
    let ticket_to_search_re = "#[0-9]{4,6}".to_string();
    let picture_to_search_re = ".+\\[emxc:.+]".to_string();
    let ticket_regex =
        match Regex::new(&ticket_to_search_re){
            Ok(ticket_re_ctrl) => ticket_re_ctrl,
            Err(_e) => {
                println!("Error: fail to build ticket regex");
                return
            }
        };

    let picture_regex =
        match Regex::new(&picture_to_search_re){
            Ok(picture_re_ctrl) => picture_re_ctrl,
            Err(_e) => {
                println!("Error: fail to build picture regex");
                return
            }
        };
    
    println!("[botbot is running]");

    line_from_buffer.clear();

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
                    botbot_read(&line_from_buffer, &ticket_regex, &picture_regex);
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
