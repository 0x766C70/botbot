use std::process::{Command, Stdio, Child};
use std::io::{Error};
pub use matrix_mgmt::*;
pub mod matrix_mgmt;

////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  FONCTION de lancement du processus matrix_commander

// _initialise le daemon matrix-commander
pub fn matrix_commander_daemon_launch() -> Result<Child, Error> {

    let daemon = Command::new(crate::MATRIX_FOLDER)
        .arg("--credentials")
        .arg(crate::MATRIX_CREDITENTIALS)
        .arg("--store")
        .arg(crate::MATRIX_DB_FOLDER)
        .arg("--download-media")
        .arg(crate::MATRIX_MEDIA_FOLDER)
        .arg("--download-media-name")
        .arg("clean")
        .arg("-lforever")
        .stdout(Stdio::piped())
        .spawn()
        .expect("ERROR: process launch"); 

    Ok(daemon)

}

// _envoie un message
pub fn matrix_commander_message_send(room: String, blabla: String) -> Result<Child, String> {
    let message_to_send =
        match Command::new(crate::MATRIX_FOLDER)
        .arg("--credentials")
        .arg(crate::MATRIX_CREDITENTIALS)
        .arg("--store")
        .arg(crate::MATRIX_DB_FOLDER)
        .arg(room)
        .arg(blabla)
        .arg("--log-level")
        .arg("CRITICAL")
        .spawn() {
            Ok(talking_status_ctrl) => Ok(talking_status_ctrl),
            Err(e) => Err(format!("ERROR: sending message - {}", e)),
        };
    message_to_send
}
