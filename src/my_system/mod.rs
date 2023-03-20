use std::process::Command;
use regex::Regex;
use chrono::prelude::*;

////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////  FONCTION de monitoring
pub fn chat_gpt_answer(botbot_phrase: String) -> Result<String, String>{

    let local: DateTime<Local> = Local::now();
    let f_time=format!("{}",local.format("%Hh%M").to_string());
    let f_date=format!("{}",local.format("%Y-%m-%d").to_string());
    let role=format!("bar:{}:{}",f_time,f_date);
    let aichat_command = Command::new("aichat")
       .arg("-r")
       .arg(role)   
       .arg(botbot_phrase)
       .output()
       .expect("failed to execute process");

    let aichat_answer = String::from_utf8_lossy(&aichat_command.stdout);
    Ok(aichat_answer.to_string())
}

// _retourn l'espace utilisé du disk passé en argument
pub fn monit_disk_space(disk: String) -> Result<i32, String> {

    let mut disk_status: Vec<&str> = Vec::new();

    // _lance la commande df
    let space_left_command = Command::new("df")
        .output()
        .expect("failed to execute process");

    let space_left = String::from_utf8_lossy(&space_left_command.stdout);

    let disk_list: Vec<&str> = space_left.split('\n').collect();

    // _construit le regex pour prendre la ligne de df contenant l'info passé en arg, ici /var
    let disk_re =
        match Regex::new(&disk){
            Ok(disk_re_ctrl) => disk_re_ctrl,
            Err(_e) => {
                return Err("ERROR: fail to build system regex".to_string());
            }
        };

    // _ remplis un vecteur des infos de la lignes /var affiché par df
    for line in disk_list {
        if disk_re.is_match(&line){
            let data_list: Vec<&str> = line.split(' ').collect();
            for data in data_list{
                if data != ""{
                    disk_status.push(data);
                }
            }
        }
    }

    let disk_usage =
        if disk_status.len() == 6 {
            // _retourne le pourcentage au format i32
            let raw_usage = disk_status[4];
            let clean_usage = &raw_usage[..raw_usage.len()-1];
            let usage =
                match clean_usage.parse::<i32>(){
                    Ok(usage_ctrl) => Ok(usage_ctrl),
                    Err(_e) => Err("ERROR: convert disk usage in Integer".to_string()),
                };
            usage
        } else {
            Err("ERROR: fail to read disk usage".to_string())
        };

        disk_usage
}
