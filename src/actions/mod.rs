use sqlite::Connection;
use regex::Regex;
use crate::message::*;
use crate::matrix::*;

// Différentes ACTIONS de botbot
// 1- on lui parle et le trigger est "botbot"
// 2- il détecte un numéro de ticket


pub fn botbot_read(line_from_buffer: &String, connection_db: &Connection, user_list: &Vec<String>, trigger_word_list: &mut Vec<String>, ticket_regex: &Regex) -> () {
    // _split la ligne de buffer selon le char "|" cf: https://github.com/8go/matrix-commander
    let raw_data: Vec<&str> = line_from_buffer.split('|').collect();
    // _check que la trame a bien 5 partie cf: https://github.com/8go/matrix-commander
    if raw_data.len() == 4 {
        // _on crée la variable raw_message qui est la dernière partie de la trame
        // _on mets tout en lowercase + on retire les accents afin de maximiser les match dans la db
        let mut raw_message = String::from(raw_data[3]);
        raw_message.make_ascii_lowercase();
        // _on ignore les trames qui commencent par '>' qui sont dans matrix la reprise d'un message auquel on répond
        let raw_message_fist_char = raw_message.chars().nth(1).unwrap_or(' ');
        // _on clean les différentes parties de la trame
        if raw_message_fist_char !=  '>' {
            let (clean_room_id, clean_room, clean_sender_id, clean_sender_name, clean_message) =
            match clean_trame(raw_data){
                Ok(trame_ctrl) => {
                    trame_ctrl
                }
                Err(_e) => {
                    return
                },
            };
            // _le buffer est pret à être converti en Message qui est une structure comportant: la
            // room d'origine et son id le sender d'origine et son id et enfin le texte
            let mut incoming_message = Message{room_origin: clean_room, room_id: clean_room_id, sender_id: clean_sender_id, sender_name: clean_sender_name, m_message: clean_message};
            // _check si dans le Message il y a un trigger (botbot || #ticket)
            let trigger_answer_result =
                if raw_message.contains("botbot") {
                // _si le message reçu contient "botbot"
                    let thinking_check =
                        match incoming_message.thinking(&connection_db, &user_list, trigger_word_list){
                            Ok(answer_ctrl) => Ok(answer_ctrl),
                            Err(e) => Err(format!("Message from {}: {}", incoming_message.sender_name, e)),
                        };
                    thinking_check
                } else if ticket_regex.is_match(&raw_message)  && incoming_message.room_origin == "fdn-tickets-internal" {
                    //_isole le numéro du ticket avec le regex
                    let regex_capture = ticket_regex.captures(&incoming_message.m_message).unwrap();
                    let raw_ticket_number = match regex_capture.get(0) {
                        Some(raw_ticket_number_ctrl) => raw_ticket_number_ctrl.as_str(),
                        None => return,
                    };
                    let ticket_number = raw_ticket_number.to_string();
                    let ticket_check=
                    match incoming_message.ticket(ticket_number){
                        Ok(answer_ctrl) => Ok(answer_ctrl),
                        Err(e) => Err(format!("ticket: {}", e)),
                    };
                    ticket_check
                } else {
                    Err(format!("Message from {}: No global trigger found", incoming_message.sender_id))
                };
            // _controle du résultat de .thinking si ok affichage de la réponse en console
            let trigger_answer =
                match trigger_answer_result {
                    Ok(trigger_answer_result_ctrl) => {
                        println!("Botbot answer to {}: {}", incoming_message.sender_name, trigger_answer_result_ctrl);
                        trigger_answer_result_ctrl
                    }
                    Err(_e) =>  {
                        //println!("Error: {}", e);
                        return
                    }
                };
            // _affichage de la réponse dans la room
            let _talking_status =
                match incoming_message.talking(trigger_answer){
                    Ok(talking_status_ctrl) => talking_status_ctrl.id(),
                    Err(_e) =>  {
                      //println!("Error: {}", e);
                      return
                  }
              };
        }
  }
}
