////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  FONCTIONS pour nettoyer les trames de matrix-commander

// _isole l'information "room id"
pub fn clean_room_id(raw_room_id:String) -> Result<String, String> {
    let debut = match raw_room_id.find("[") {
        Some(debut_index) => debut_index + 1,
        None => return Err("ERROR: clean_room_id start".to_string()),
    };
    let fin = match raw_room_id.find("]") {
        Some(fin_index) => fin_index,
        None => return Err("ERROR: clean_room_id end".to_string()),
    };
    if debut >= fin {
        Err("ERROR: clean_room_id matrix-commander output unreadable".to_string())
    } else {
        let clean_room_id = &raw_room_id[debut..fin];
        Ok(clean_room_id.to_string())
    }
}

// _isole l'information "room"
pub fn clean_room_origin(raw_room_origin:String) -> Result<String, String> {
    let debut = match raw_room_origin.find("room") {
        Some(debut_index) => debut_index + 5,
        None => return Err("ERROR: clean_room_origin start".to_string()),
    };
    let fin = match raw_room_origin.find("[") {
        Some(fin_index) => fin_index - 1,
        None => return Err("ERROR: clean_room_origin end".to_string()),
    };
    if debut >= fin {
        Err("ERROR: clean_room_origin matrix-commander output unreadable".to_string())
    }else {
        let clean_room_origin = &raw_room_origin[debut..fin];
        Ok(clean_room_origin.to_string())
    }
}

// _isole l'information "sender_id"
pub fn clean_sender_id(raw_sender_id:String) -> Result<String, String> {
    let debut = match raw_sender_id.find("[") {
        Some(debut_index) => debut_index + 1,
        None => return Err("ERROR: clean_sender_id start".to_string()),
    };
    let fin = match raw_sender_id.find("]") {
        Some(fin_index) => fin_index,
        None => return Err("ERROR: clean_sender_id end".to_string()),
    };
    if debut > fin {
        Err("ERROR: clean_sender_id matrix-commander output unreadable".to_string())
    } else {
        let clean_sender_id = &raw_sender_id[debut..fin];
        Ok(clean_sender_id.to_string())
    }
}

// _isole l'information "sender"
pub fn clean_sender_name(raw_sender_name:String) -> Result<String, String> {
    let debut = match raw_sender_name.find("sender") {
        Some(debut_index) => debut_index + 7,
        None => return Err("ERROR: clean_sender_name start".to_string()),
    };
    let fin = match raw_sender_name.find("[") {
        Some(fin_index) => fin_index - 1,
        None => return Err("ERROR: clean_sender_name end".to_string()),
    };
    if debut > fin {
        Err("clean_sender_name ERROR: Matrix-Commander output unreadable".to_string())
    } else {
        let raw_sender_name = &raw_sender_name[debut..fin];
        let no_irc_sender_name = raw_sender_name.replace(" (IRC)","");
        Ok(no_irc_sender_name.to_string())
    }
}

pub fn clean_trame(matrix_trame:Vec<&str>) -> Result <(String,String,String,String,String), String> {

    // _construction du message: cf la struct
    let clean_room_id           =
        match clean_room_id(String::from(matrix_trame[0])) {
            Ok(clean_room_id_ok) => clean_room_id_ok,
            Err(e) => return Err(format!("ERROR: clean_trame {}", e)),
        };
    let clean_room           =
        match clean_room_origin(String::from(matrix_trame[0])) {
            Ok(clean_room_ok) => clean_room_ok,
            Err(e) => return Err(format!("ERROR: clean_trame {}", e)),
        };
    let clean_sender_id           =
        match clean_sender_id(String::from(matrix_trame[1])) {
            Ok(clean_sender_id_ok) => clean_sender_id_ok,
            Err(e) => return Err(format!("ERROR: clean_trame {}", e)),
        };
    let clean_sender_name           =
        match clean_sender_name(String::from(matrix_trame[1])) {
            Ok(clean_sender_name_ok) => clean_sender_name_ok,
            Err(e) => return Err(format!("ERROR: clean_trame {}", e)),
        };
    let raw_message = String::from(matrix_trame[3]);
    // _on retire le \n de fin de trame
    let pre_clean_message = &raw_message[1..(raw_message.len()-1)];
    // _on retransforme en string
    let clean_message = pre_clean_message.to_string();

    Ok((clean_room_id, clean_room, clean_sender_id, clean_sender_name, clean_message))
}
