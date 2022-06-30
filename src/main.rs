
extern crate termion;
use rand::Rng;
use serde::{Deserialize, Serialize};
use console::Term;

#[derive(Debug, Deserialize, Serialize)]
struct Word{
  description:String,
  detail:String,
}

#[tokio::main]
async fn main()  { 
let url = "https://raw.githubusercontent.com/reinnatan/RubyWebCrawler/master/output.json";
let response = reqwest::get(url).await.unwrap();
  print!("====================================\n");
  print!("=====     GUEST GAME WORD        ===\n");
  print!("====================================\n");

  match response.status() {      
      reqwest::StatusCode::OK =>{
        match response.json::<Vec<Word>>().await {
            Ok(parsed) => get_random_word(&parsed), //print!("{:?}",parsed.len()),
            Err(err) => print!("{:?}",err),
        }
      }
      
      other =>{
        panic!("Something worng unexpected happened: {:?}", other)
      }
  }

}   

fn get_random_word(list_word:&Vec<Word>){
  let random_number =  rand::thread_rng().gen_range(0..list_word.len()-1);
  let word_selected = list_word.get(random_number).unwrap();
  let term = Term::stdout();
  let count_wrong_guest = 0;
  let tolerancy_answer =  (0.5 * (word_selected.description.len() as f32)).floor() as i32;

  let mut empty_string:String = String::from("");
  for _j in 0..word_selected.description.len(){
    empty_string.push_str("_")
  }

  print!("Word Detail : {}\n", word_selected.detail);
  print!("Total Guest : {}\n", tolerancy_answer); 
  print!("Your Guest : ");

  let character = term.read_char().unwrap();
  let is_win = is_guest_word(character, empty_string, word_selected, count_wrong_guest, tolerancy_answer);
  if is_win{
    print!("Horayyy... you win...\n");
  }else {
    print!("You are Loose...\n");
  }
}
  

fn is_guest_word(key_find:char, mut empty_list_word:String, selected_word:&Word, mut count_wrong_guest:i32, tolerancy_answer:i32)->bool{
    if selected_word.description.contains(key_find){
      for u in 0..selected_word.description.len(){
        if selected_word.description.chars().nth(u).unwrap() == key_find{
          empty_list_word.replace_range(u..u+1, &String::from(key_find))
        }
      }
    }else{
      count_wrong_guest +=1;
    }

    print!("Current word guest : {}",empty_list_word);
  
    if empty_list_word.contains("_") && count_wrong_guest<tolerancy_answer{
      print!("Your Guest : ");
      let term = Term::stdout();
      let character = term.read_char().unwrap();

      return is_guest_word(character, empty_list_word, selected_word, count_wrong_guest, tolerancy_answer)
    }else if count_wrong_guest >= tolerancy_answer{
      return false;
    }else{
      return true;
    }
  }

