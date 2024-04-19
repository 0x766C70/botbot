use sqlite::Connection;
use unidecode::unidecode;
use std::process::Child;
use crate::database::{is_admin, is_sadmin, get_model};
use crate::answers::*;
use crate::matrix::*;
use curl::easy::{Handler, WriteError};
extern crate json;

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

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

// _traits de Message
impl Message{
    // _détermine les actions de botbot lorsqu'il est déclenché
    pub fn thinking(&mut self, connection_db: &Connection, user_list: &Vec<String>) -> Result<String, String> {
        let botbot_phrase = String::from(unidecode(&self.m_message).to_string());
        println!("{} from {} room: {}", &self.sender_name, &self.room_origin, botbot_phrase);
        let answer =
            // _traitement si superadmin command par un super admin
            if botbot_phrase.contains("!sadmin") && is_sadmin(connection_db, user_list, &self.sender_name).unwrap() {
                Ok(("Admin Super bitch !!!").to_string())
            // _traitement si admin command par un admin
            }else if botbot_phrase.contains("!admin") && is_admin(connection_db, user_list, &self.sender_name).unwrap() {
                Ok(("Admin bitch !!!").to_string())
            // _traitement par un user
            } else{
                // _définit le model de réponse: sql, openai
                let answer_model = 
                    match get_model(connection_db, user_list, &self.sender_id, &self.room_origin){
                        Ok(model_ctrl) => Ok(format!("{}", model_ctrl)),
                        Err(e) => Err(format!("ERROR: unable to get model {}", e)),
                    };
                // _récupère la réponse en fonction du model
                let user_answer =
                    match get_answer(botbot_phrase, &self.sender_name, answer_model.unwrap()) {
                        Ok(answer_ctrl) => Ok(format!("{}", answer_ctrl)),
                        Err(e) => Err(format!("ERROR: unable to get anwser {}", e)),
                };
                user_answer
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
    pub fn _change_room(&mut self, new_room_id: String, new_room_origin: String) {
        self.room_id = new_room_id;
        self.room_origin = new_room_origin;
    }
}
