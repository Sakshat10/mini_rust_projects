use std::io;
use rand::prelude::*;
use colored::*;

pub fn game(){
    let ques = ["flat ka chor","biggest harami","sbka beta","sbki beti","sbse bdi raand"];
    let guess_name = ["shivansh","saurabh","darshit","prateek","piyush","umar"];
    let mut rng = thread_rng();

    let index1 = rng.gen_range(0..ques.len());
    let index = rng.gen_range(0..guess_name.len());


    let random_name = guess_name[index];
    let random_ques = ques[index1];


    println!("ques: {}", random_ques);

loop {
    
    let mut input:String = String::new();
    println!("write a name:");
    match io::stdin().read_line(&mut input) {
        Ok(_)=>{
            let selected_name = input.trim().to_lowercase();
            println!("Name selected:{}",selected_name);

            if !guess_name.contains(&selected_name.as_str()){
                println!("only flatmates names are valid except sakshat becz he is a good boy");
                continue;
            }

            if guess_checker(&selected_name,random_name){
                println!("{}","you guessed it right !!!!!".green());
                break;
            }else{
                println!("{}","naah!!! wrong guess Dont be sad Retry again".red());
            }
        }
        Err(error)=>{
            println!("Error:{}",error);
        }
    }
}
}

fn guess_checker(guessed_name:&str,random_selected:&str)->bool{
    return guessed_name==random_selected;
}