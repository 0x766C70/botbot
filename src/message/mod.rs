use unidecode::unidecode;
use sqlite::Connection;
use std::process::Child;
use crate::database::{get_answer, add_chat, del_chat};
mod message_mgmt;
pub use message_mgmt::*;
use crate::my_system::*;
use crate::matrix::*;
use curl::easy::Easy;
use std::io::{stdout, Write};

////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  Structure et traits des messages reçus

// _structure d'un Message
pub struct Message{
    pub room_origin: String,
    pub room_id: String,
    pub sender_id: String,
    pub sender_name: String,
    pub m_message: String,
}

// _traits de Message
impl Message{
    // _détermine les actions de botbot lorsqu'il est déclenché
    pub fn thinking(&mut self, adminsys_list: &Vec<String>, admincore_list: &Vec<String>, trigger_word_list: &mut Vec<String>, connection_db: &Connection) -> Result<String, String> {
        let mut botbot_phrase = String::from(unidecode(&self.m_message).to_string());
        // _uppercases
        botbot_phrase.make_ascii_lowercase();
        let answer =
            if botbot_phrase.contains("botbot admin") && adminsys_list.contains(&self.sender_id){
                let admin_answer =
                    // _mode admin pour ajout de trigger
                    if botbot_phrase.contains("admin add") {
                        let chat_to_add =
                            match add_chat(botbot_phrase, connection_db, trigger_word_list) {
                                Ok(chat_to_add_ctrl) => Ok(format!("[admin mode by: {}] {} ajouté !", &self.sender_name, chat_to_add_ctrl)),
                                Err(e) => Err(format!("ERROR: chat_to_add process to add {}", e)),
                            };
                        chat_to_add
                    // _mode admin pour suppression de trigger
                    } else if botbot_phrase.contains("admin del") {
                        let chat_to_del =
                            match del_chat(botbot_phrase, connection_db, trigger_word_list) {
                                Ok(del_chat_ctrl) => Ok(format!("[admin mode by: {}] {} supprimé !", &self.sender_name, del_chat_ctrl)),
                                Err(e) => Err(format!("ERROR: chat_to_del proceed to del {}", e)),
                            };
                        chat_to_del
                    // _mode admin pour afficher l'espace restant dans /var
                    } else if botbot_phrase.contains("admin space") {
                        let chat_to_space_left=
                            match monit_disk_space(crate::MATRIX_DRIVE.to_string()) {
                                Ok(chat_to_space_left_ctrl) => Ok(format!("Disk usage: {}%", chat_to_space_left_ctrl)),
                                Err(e) => Err(format!("ERROR: unable to get disk usage {}", e)),
                            };
                        chat_to_space_left
                    } else if botbot_phrase.contains("admin annonce") {
                        &self.change_room("!JHyLuasLCpiDxIlcks:matrix.fdn.fr".to_string(), "fdn".to_string());
                        let chat_to_all= Ok(format!("ANNONCE: {}", &self.m_message));
                        chat_to_all
                    } else {
                        Err("ERROR: no admin command".to_string())
                    };
                admin_answer
            // _ping équipe admincore + adminsys
            } else if botbot_phrase.contains("ping adminsys") {
                let chat_to_ping_adminsys=
                    match get_admin_list(&self.sender_name, adminsys_list) {
                        Ok(chat_to_ping_adminsys_ctrl) => Ok(format!("Hello les adminsys: {} vous contacte ! {}", &self.sender_name, chat_to_ping_adminsys_ctrl)),
                        Err(e) => return Err(format!("ERROR: unable to get adminsys list {}", e)),
                    };
                chat_to_ping_adminsys
            // _ping equipe admincore
            } else if botbot_phrase.contains("ping admincore") {
                let chat_to_ping_admincore=
                    match get_admin_list(&self.sender_name, admincore_list) {
                        Ok(chat_to_ping_admincore_ctrl) => Ok(format!("Hello les admincore: {} vous contacte ! {}", &self.sender_name, chat_to_ping_admincore_ctrl)),
                        Err(e) => return Err(format!("ERROR: unable to get admincore list {}", e)),
                    };
                chat_to_ping_admincore
            // _envoie une alerte sur #adminsys
            } else if botbot_phrase.contains("!alert") || botbot_phrase.contains("!alerte") {
                // _on change le message pour que la réponse parte sur le chan adminsys
                &self.change_room("!sjkTrbbOksVnLWuzlc:matrix.fdn.fr".to_string(), "fdn-adminsys".to_string());
                let chat_to_alert_admincore=
                    match get_admin_list(&self.sender_name, admincore_list) {
                        Ok(chat_to_ping_admincore_ctrl) => Ok(format!("ALERTE remontée par {} ! {}", &self.sender_name, chat_to_ping_admincore_ctrl)),
                        Err(e) => return Err(format!("ERROR: unable to get admincore list {}", e)),
                    };
                chat_to_alert_admincore
            } else if botbot_phrase.contains("blague") {
                let mut easy = Easy::new();
                easy.url("https://v2.jokeapi.dev/joke/Any?lang=fr&format=txt").unwrap();
                easy.write_function(|data| {
                    stdout().write_all(data).unwrap();
                    Ok(data.len())
                }).unwrap();
                easy.perform().unwrap();
                let chat_to_jockes = Ok(format!("{}",easy.response_code().unwrap()));
                chat_to_jockes
            } else {
                // _réponse de botbot
                let chat_answer =
                    match get_answer(botbot_phrase, connection_db, trigger_word_list){
                        Ok(answer_ctrl) => {
                            // _remplace les %s par le nom du sender et les %n par un retour à la ligne
                            let answer_with_name= &answer_ctrl[..].replace("%s", &self.sender_name);
                            let answer_with_new_line = &answer_with_name[..].replace("%n", "\n");
                            Ok(answer_with_new_line.to_string())
                        }
                        Err(e) => Err(format!("ERROR: chat answer {}",  e)),
                    };
                chat_answer
            };
        answer
    }

    // _détermine les actions de botbot lorsqu'il voit un numéro de ticket
    pub fn ticket(&self, ticket_number: String) -> Result<String, String> {
       let ticket_url = format!("Ticket: https://tickets.fdn.fr/rt/Ticket/Display.html?id={}", &ticket_number[1..]);
       Ok(ticket_url)
    }

    // _fait parler botbot
    pub fn talking(&self, phrase_to_say: String) -> Result<Child, String> {
        let mut blabla = "-m".to_string();
        blabla.push_str(&phrase_to_say[..]);
        let mut room = "-r".to_string();
        room.push_str(&self.room_id);
        let talking_status =
            match matrix_commander_message_send(room, blabla){
                    Ok(talking_status_ctrl) => Ok(talking_status_ctrl),
                    Err(e) => Err(format!("ERROR: sending message - {}", e)),
            };
        talking_status
    }

    // _change la room de la réponse
    pub fn change_room(&mut self, new_room_id: String, new_room_origin: String) {
        self.room_id = new_room_id;
        self.room_origin = new_room_origin;
    }
}
