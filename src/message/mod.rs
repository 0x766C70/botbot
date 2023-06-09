use unidecode::unidecode;
use std::process::Child;
use crate::answers::*;
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
        println!("{} from {}: {}", &self.sender_id, &self.room_origin, botbot_phrase);
        let answer =
            if botbot_phrase.contains("!alert") || botbot_phrase.contains("!alerte") {
                // _on change le Message pour que la réponse parte sur le chan adminsys
                let _ = &self.change_room("!sjkTrbbOksVnLWuzlc:matrix.fdn.fr".to_string(), "fdn-adminsys".to_string());
                Ok(format!("Alerte signalée par {}!", &self.sender_name))
            } else if (&self.sender_id == "@vlp:matrix.fdn.fr" || &self.sender_id == "tom28:matrix.fdn.fr" || &self.sender_id == "pandaroux:matrix.fdn.fr" || &self.sender_id == "asmadeus:codewreck.org" || &self.sender_id == "afriqs:matrix.fdn.fr" ) && botbot_phrase.contains("!admin")  {
                let chat_admin=
                    match admin_answer(botbot_phrase) {
                        Ok(admin_ctrl) => {
                            let admin_ctrl_with_name = &admin_ctrl[..].replace("dummyname", &self.sender_name);
                            Ok(format!("{}", admin_ctrl_with_name))
                        }
                        Err(e) => Err(format!("ERROR: unable to get anwser {}", e)),
                };
                chat_admin
            } else if &self.room_origin != "fdn-adminsys"{
                let chat_user=
                    match user_answer(botbot_phrase) {
                        Ok(user_ctrl) => {
                            let user_ctrl_with_name = &user_ctrl[..].replace("dummyname", &self.sender_name);
                            Ok(format!("{}", user_ctrl_with_name))
                        }
                        Err(e) => Err(format!("ERROR: unable to get anwser {}", e)),
                };
                chat_user
            } else {
                Err(format!("ERROR: no match"))
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
