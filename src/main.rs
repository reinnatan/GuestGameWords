
extern crate termion;
use std::io::{stdout, Write};
use rand::Rng;
use console::Term;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Word{
  description:String,
  detail:String,
}

#[tokio::main]
async fn main()  { 
let url = "https://raw.githubusercontent.com/reinnatan/RubyWebCrawler/master/output.json";
let response = reqwest::get(url).await.unwrap();

  print!("{}[2J", 27 as char);
  print!("=========================================\n");
  print!("=======     GUEST GAME WORD       =======\n");
  print!("=========================================\n");

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
  print!("Opportunity Guest : {}\n", tolerancy_answer); 
  print!("Your Guest : ");
  let result = stdout().flush();
  match result {
      Ok(_value)=> { 
        let character = term.read_char().unwrap();
        println!("\nYou're opportunity guest word : {}\n",tolerancy_answer-count_wrong_guest);
        play_game_guest_word(character, empty_string, word_selected, count_wrong_guest, tolerancy_answer); 
      },
      Err(err)=>{
        print!("Error occured {}", err)
      }
  }
  
  
}
  

fn play_game_guest_word(mut key_find:char, mut empty_list_word:String, selected_word:&Word, mut count_wrong_guest:i32, tolerancy_answer:i32){
  
  while empty_list_word.contains("_") && (tolerancy_answer-count_wrong_guest)>0{
   
    if selected_word.description.contains(key_find){
      for u in 0..selected_word.description.len(){
        if selected_word.description.chars().nth(u).unwrap() == key_find{
          empty_list_word.replace_range(u..u+1, &String::from(key_find))
        }
      }
    }else{
      count_wrong_guest +=1;
    }

    print!("\nCurrent word guest : {}\n",empty_list_word);
    println!("\nYou're opportunity guest word : {}\n",tolerancy_answer-count_wrong_guest);
    print!("Your Guest : ");
    let result = stdout().flush();
    match result {
      Ok(_value)=> { 
        let term = Term::stdout();
        key_find = term.read_char().unwrap();
      },
      Err(err)=>{
        print!("Error occured {}", err)
      }
    }
  }

  if tolerancy_answer-count_wrong_guest==0{
    print!("\n\n==========================\n");
    print!("Sorry you are Loose...\n");
    print!("Guested Word : {}",selected_word.description);
    print!("\n==========================\n");
  }else {
    print!("\n\n==========================\n");
    print!("Horayyy... you win...\n");
    print!("Guested Word : {}",empty_list_word);
    print!("\n==========================\n");
  }
  
}

