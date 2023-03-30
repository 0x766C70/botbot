use std::process::{Command, Stdio, Child};
use std::io::{Error, BufReader};
pub use matrix_mgmt::*;
pub mod matrix_mgmt;

////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  FONCTION de lancement du processus matrix_commander

pub fn matrix_launch() -> i32 {
    // _crée un processus fils au programme qui lance matrix-commander et qui pipe son flux stdout
    let mc = Command::new(crate::MATRIX_FOLDER)
        .arg(crate::MATRIX_CREDITENTIALS)
        .arg(crate::MATRIX_DB_FOLDER)
        .arg("-lforever")
        .arg("--log-level")
        .arg("ERROR")
        .stdout(Stdio::piped())
        .spawn()
        .expect("ERROR: process launch"); 

    let mc_pid = mc.id() as i32; 
    println!(" > matrix-commander launched: pid {}", mc_pid);

    // _crée un objet pour interragir avec un process à partir de son pid
    //let mut mc_process_handler = &Process::new(mc_pid).unwrap();
    //let mc_buffer = BufReader::new(mc.stdout.as_mut().unwrap());
    
    mc_pid
}

// _initialise le daemon matrix-commander
pub fn matrix_commander_daemon_launch() -> Result<Child, Error> {
    let daemon = Command::new(crate::MATRIX_FOLDER)
        .arg(crate::MATRIX_CREDITENTIALS)
        .arg(crate::MATRIX_DB_FOLDER)
        .arg("-lforever")
        .arg("--log-level")
        .arg("ERROR")
        .stdout(Stdio::piped())
        .spawn()
        .expect("ERROR: process launch"); 
    Ok(daemon)
}

// _envoie un message
pub fn matrix_commander_message_send(room: String, blabla: String) -> Result<Child, String> {
    let message_to_send =
        match Command::new(crate::MATRIX_FOLDER)
        .arg(crate::MATRIX_CREDITENTIALS)
        .arg(crate::MATRIX_DB_FOLDER)
        .arg(room)
        .arg(blabla)
        .arg("--log-level")
        .arg("ERROR")
        .spawn() {
            Ok(talking_status_ctrl) => Ok(talking_status_ctrl),
            Err(e) => Err(format!("ERROR: sending message - {}", e)),
        };
    message_to_send
}
