use std::process::Command;
use chrono::prelude::*;

////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  Gestion des réponses en fonction des actions

pub fn get_answer(botbot_phrase: String, name: &String, role: String) -> Result<String, String>{

    let local: DateTime<Local> = Local::now();
    let f_time=format!("{}",local.format("%Hh%M").to_string());
    let f_date=format!("{}",local.format("%Y-%m-%d").to_string());
    let f_dow=format!("{}",local.weekday().to_string());
    let arg_role=format!("{}:{}:{}:{}:{}",role,f_time,f_date,f_dow,name);

    //println!("role:{} - message {}", arg_role, botbot_phrase);

    let aichat_command = Command::new("aichat")
       .arg("-r")
       .arg(arg_role)   
       .arg(botbot_phrase)
       .output()
       .expect("failed to execute process");

    let aichat_answer = String::from_utf8_lossy(&aichat_command.stdout);
    Ok(aichat_answer.to_string())
    //Ok("ai blabla free of cost".to_string())
}
