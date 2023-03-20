use unidecode::unidecode;
use std::process::Child;
use crate::my_system::*;
use crate::matrix::*;
use curl::easy::{Handler, WriteError};

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
    pub fn thinking(&mut self) -> Result<String, String> {
        let botbot_phrase = String::from(unidecode(&self.m_message).to_string());
        // _uppercases
        //botbot_phrase.make_ascii_lowercase();
        println!("{}: {}", &self.sender_id, botbot_phrase);
        let answer =
            if botbot_phrase.contains("!alert") || botbot_phrase.contains("!alerte") {
                // _on change le message pour que la réponse parte sur le chan adminsys
                let _ = &self.change_room("!sjkTrbbOksVnLWuzlc:matrix.fdn.fr".to_string(), "fdn-adminsys".to_string());
                Ok(format!("Alerte !"))
            } else {
                let chat_gpt=
                    match chat_gpt_answer(botbot_phrase, ) {
                        Ok(chat_gpt_ctrl) => {
                            let chat_gpt_ctrl_with_name = &chat_gpt_ctrl[..].replace("dummyname", &self.sender_name);
                            Ok(format!("{}", chat_gpt_ctrl_with_name))
                        }
                        Err(e) => Err(format!("ERROR: unable to get anwser {}", e)),
                };
                chat_gpt
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
