////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  FONCTION diverses de gestions de la database

// _récupère l'argument de gauche contenu dans entre crochets
pub fn _get_left_arg(admin_msg: &String) -> Result<String, String> {
    // _trouve l'index de [ de l'argument de gauche
    let debut_mark =
        match admin_msg.find("[") {
            Some(debut_mark_index) => debut_mark_index + 1,
            None => return Err("ERROR: unable to find left arg start".to_string()),
        };
    // _trouve l'index de ] de l'argument de gauche
    let fin_mark =
        match admin_msg.find("]") {
            Some(fin_mark_index) => fin_mark_index,
            None => return Err("ERROR: unable to find left arg end".to_string()),
        };
    // _si arg est vide ERROR sinon retour de la value
    if debut_mark == fin_mark {
        Err("ERROR: no value in left arg".to_string())
    }
    else {
        Ok(admin_msg[debut_mark..fin_mark].to_string())
    }
}

// _récupère l'argument de droite contenu dans entre crochets
pub fn _get_right_arg(admin_msg: &String) -> Result<String, String> {
    // trouve l'index de [ de l'argument de droite
    let debut_mark =
        match admin_msg.rfind("[") {
            Some(debut_mark_index) => debut_mark_index + 1,
            None => return Err("ERROR: unable to find right arg start".to_string()),
        };
    // _trouve l'index de ] de l'argument de droite
    let fin_mark =
        match admin_msg.rfind("]") {
            Some(fin_mark_index) => fin_mark_index,
            None => return Err("ERROR: unable to find right arg end".to_string()),
        };
    // _si arg est vide ERROR sinon retour de la value
    if debut_mark == fin_mark {
        Err("ERROR: no value in right arg".to_string())
    }
    else {
        Ok(admin_msg[debut_mark..fin_mark].to_string())
    }
}


