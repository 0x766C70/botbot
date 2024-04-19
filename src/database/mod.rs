use std::vec::*;
use sqlite::{Connection, State};
use rand::Rng;
use regex::Regex;
pub use database_mgmt::*;
pub mod database_mgmt;

////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  FONCTION initialisation de la db

// _initialise la db
pub fn init_db () -> (Result<Connection, String>, Vec<String>, Vec<String>) {
    let mut trigger_word_list: Vec<String> = Vec::new();
    let mut user_list: Vec<String> = Vec::new();

    let connection_db =
       match Connection::open("worterkasten.db") {
           Ok(connection_db_ctrl) => connection_db_ctrl,
           Err(_e) => return (Err("Talking table fail to initialized".to_string()),trigger_word_list, user_list),
        };
    {
        // _crée la table talking si elle n'existe pas
        let mut create_table_talking_statement =
            match connection_db.prepare("CREATE TABLE if not exists talking (chat_id INTEGER PRIMARY KEY, trigger TEXT not null, answer TEXT not null, weight INTEGER not null);") {
                Ok(create_table_talking_statement_ctrl) => create_table_talking_statement_ctrl,
                Err(_e) => return (Err("Talking table fail to initialized".to_string()), trigger_word_list, user_list),
              };

        while let State::Row = create_table_talking_statement.next().unwrap() {};
    }
    {
        // _crée la table users si elle n'existe pas
        // power: user=1 admin=2 superadmin=3
        let mut create_table_user_statement =
            match connection_db.prepare("CREATE TABLE if not exists users (user_id INTEGER PRIMARY KEY, user TEXT not null, power INTEGER not null);") {
                Ok(create_table_user_statement_ctrl) => create_table_user_statement_ctrl,
                Err(_e) => return (Err("Users table fail to initialized".to_string()), trigger_word_list, user_list),
              };

        while let State::Row = create_table_user_statement.next().unwrap() {};
    }
    {
        // _crée la table brain si elle n'existe pas
        let mut create_table_brain_statement =
            match connection_db.prepare("CREATE TABLE if not exists brain (brain_id INTEGER PRIMARY KEY, user TEXT not null, room TEXT not null, ai TEXT not null);") {
                Ok(create_table_brain_statement_ctrl) => create_table_brain_statement_ctrl,
                Err(_e) => return (Err("Users table fail to initialized".to_string()), trigger_word_list, user_list),
              };

        while let State::Row = create_table_brain_statement.next().unwrap() {};
    }
    {
        // _charge dans trigger_word_list tous les triggers de la table talking
        let mut add_words_statement =
            match connection_db.prepare("SELECT trigger FROM talking") {
                Ok(add_words_statement_ctrl) => add_words_statement_ctrl,
                Err(_e) => return (Err("Fail to load trigger from db".to_string()), trigger_word_list, user_list),
              };

        while let State::Row = add_words_statement.next().unwrap() {
                let word_to_add = add_words_statement.read::<String>(0).unwrap();
                if !trigger_word_list.contains(&word_to_add){
                    trigger_word_list.push(word_to_add);
                }
            }
    }
    {
        // _charge dans user_list les users
        let mut add_user_statement =
            match connection_db.prepare("SELECT DISTINCT user FROM brain") {
                Ok(add_user_statement_ctrl) => add_user_statement_ctrl,
                Err(_e) => return (Err("Fail to load user list from db".to_string()), trigger_word_list, user_list),
              };

        while let State::Row = add_user_statement.next().unwrap() {
                let user_to_add = add_user_statement.read::<String>(0).unwrap();
                user_list.push(user_to_add);
            }
    }
   (Ok(connection_db), trigger_word_list, user_list)
}

////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  FONCTION d'échange avec la db

// _renvoie true si admin
pub fn is_admin(_connection_db: &Connection, _user_list: &Vec<String>, _sender_name: &String) -> Result<bool, bool> {
    Ok(true)
}

// _renvoie true si sadmin
pub fn is_sadmin(_connection_db: &Connection, _user_list: &Vec<String>, _sender_name: &String) -> Result<bool, bool> {
    Ok(true)
}

// _renvoie le model
pub fn get_model(_connection_db: &Connection, user_list: &Vec<String>, sender_name: &String, room_name: &String) -> Result<String, String> {
    println!("Model de {} dans {}", sender_name, room_name);
    match user_list.iter().position(|x| x == sender_name) {
        Some(_x) => Ok("mifa".to_string()),
        _ => Ok("sql".to_string()),
    }
}

// _ajoute un trigger/answer dans la base
pub fn _add_chat(botbot_phrase: String, connection_db: &Connection, trigger_word_list: &mut Vec<String>) -> Result<String, String> {

    let trigger =
        match _get_left_arg(&botbot_phrase) {
             Ok(trigger_ctrl) => trigger_ctrl,
             Err(e) => return Err(format!("ERROR: chat_to_add match trigger {}", e)),
        };

    let answer =
        match _get_right_arg(&botbot_phrase) {
             Ok(answer_ctrl) => answer_ctrl,
             Err(e) => return Err(format!("ERROR: chat_to_add match answer {}", e)),
        };

    let mut insert_statement =
        match connection_db.prepare("INSERT INTO talking (trigger, answer) VALUES (?, ?);"){
            Ok(insert_statement_ctrl) => insert_statement_ctrl,
            Err(e) => return Err(format!("ERROR: add prepare db {}", e)),
          };
        let _bind1_statement =
            match insert_statement.bind(1, &trigger[..]){
                Ok(_bind1_statement_ctrl) => _bind1_statement_ctrl,
                Err(e) => return Err(format!("ERROR: add binding trigger {}", e)),
            };
        let _bind2_statement =
            match insert_statement.bind(2, &answer[..]){
                Ok(_bind2_statement_ctrl) => _bind2_statement_ctrl,
                Err(e) => return Err(format!("ERROR: add binding answer {}", e)),
            };
        let _run_statement =
            match insert_statement.next() {
                Ok(_run_statement_ctrl) => _run_statement_ctrl,
                Err(e) => return Err(format!("ERROR: process add trigger {}", e)),
            };
        if !trigger_word_list.contains(&trigger.to_string()){
            trigger_word_list.push(trigger.to_string());
        }
        Ok(trigger)
}

// _supprime un trigger/answer dans la base
pub fn _del_chat(botbot_phrase: String, connection_db: &Connection, trigger_word_list: &mut Vec<String>) -> Result<String, String> {

    let trigger =
        match _get_left_arg(&botbot_phrase) {
             Ok(trigger_ctrl) => trigger_ctrl,
             Err(e) => return Err(format!("ERROR: chat_to_del match trigger {}", e)),
        };

    if !trigger_word_list.contains(&trigger) {
        return Err(format!("ERROR: trigger not in db"))
    }

    let answer =
        match _get_right_arg(&botbot_phrase) {
             Ok(answer_ctrl) => answer_ctrl,
             Err(e) => return Err(format!("ERROR: chat_to_add match answer {}", e)),
        };

    if trigger == answer {
        let mut del_statement =
            match connection_db.prepare("DELETE FROM talking WHERE trigger=?"){
                Ok(del_statement_ctrl) => del_statement_ctrl,
                Err(e) => return Err(format!("ERROR: del prepare db {}", e)),
              };
        let _bind_statement =
          match del_statement.bind(1, &trigger[..]){
              Ok(_bind_statement_ctrl) => _bind_statement_ctrl,
              Err(e) => return Err(format!("ERROR: del binding trigger {}", e)),
          };
        let _run_statement =
            match del_statement.next() {
                Ok(_run_statement_ctrl) => _run_statement_ctrl,
                Err(e) => return Err(format!("ERROR: process del trigger {}", e)),
            };
        trigger_word_list.retain(|x| *x != trigger);
        Ok(trigger)
    }
    else {
        Ok("plop".to_string())
    }
}

// _récupère une answer dans la base à partir de son trigger
pub fn _get_answer(botbot_phrase: String, connection_db: &Connection, trigger_word_list: &mut Vec<String>) -> Result<String, String> {
    let mut tmp_answers: Vec<String> = Vec::new();
    for x in trigger_word_list {
        let re_to_search = format!("(\\s{}|^{}|'{})[\\s\\?!,]*", x, x, x);
        let re =
            match Regex::new(&re_to_search){
                Ok(re_ctrl) => re_ctrl,
                Err(e) => return Err(format!("ERROR: setup regex {}", e)),
            };
        if  re.is_match(&botbot_phrase) {
            let mut select_statement =
                match connection_db.prepare("SELECT answer FROM talking where trigger=?"){
                    Ok(select_statement_ctrl) => select_statement_ctrl,
                    Err(e) =>  return Err(format!("ERROR: select prepare db {}", e)),
                  };
            let _bind_statement =
                match select_statement.bind(1, &x[..]){
                    Ok(_bind_statement_ctrl) => _bind_statement_ctrl,
                    Err(e) => return Err(format!("ERROR: select binding trigger {}", e)),
                };
            while let Ok(State::Row) = select_statement.next() {
                let blabla =
                    match select_statement.read::<String>(0){
                        Ok(blabla_ctrl) => blabla_ctrl,
                        Err(_e) => continue,
                    };
                tmp_answers.push(blabla);
            }
            continue;
        }
    }
    if tmp_answers.len() != 0 {
        let mut rng = rand::thread_rng();
        Ok(tmp_answers[rng.gen_range(0..tmp_answers.len())].to_string())
    }else{
        Err(format!("ERROR: no word found for: {}", botbot_phrase))
    }
}
 
