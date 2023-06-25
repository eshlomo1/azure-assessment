#![allow(unused)]
use std::fs;
use std::env::Args;
use std::process::Command;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::process::Output;
use xlsxwriter::*;

macro_rules! typeone {
    ($x:expr, $json:expr, $num_str:expr) => (String::from($json[$num_str][$x].as_str().unwrap()));
}

macro_rules! typetwo {
    ($x:expr, $json:expr, $num_str:expr) => ($json[$num_str][$x].as_str().unwrap().split_whitespace());
}

macro_rules! typestd {
    ($x:expr) => (String::from_utf8_lossy($x).replace('\"', "").replace("[", "").replace("]", "").replace("{", "").replace("}", "").trim());
}

macro_rules! eq {
    ($x:expr, $y:expr) => ($x == $y);
}

macro_rules! space_error_handler {            
    ($value:expr, $final23:expr) => (if $value == ""{return $final23;})

}
macro_rules! match_type {
    ($x:expr, $json:expr, $num_str:expr) => ($json[$num_str][$x].as_str().unwrap());
}

macro_rules! n_eq {
    ($x:expr, $y:expr) => ($x != $y);
}
macro_rules! equal_less {
    ($x:expr, $y:expr) => ($x <= $y);
}
macro_rules! equal_more {
    ($x:expr, $y:expr) => ($x >= $y);
}


pub fn printer(json: &serde_json::Value, num_str: &String){
    println!("---------------------------------------------------------");
    println!("{}", typeone!("Title", &json, &num_str));
    println!("{}", typeone!("Descrip", &json, &num_str));

}



pub fn first_args_command(json:serde_json::Value, num_str: &String)-> Vec<String>{
    let mut stddout: Vec<String>= vec![]; 
    let output = Command::new(typeone!("Command", &json, &num_str))
                    .args(typetwo!("Args", &json, &num_str))
                    .output()
                    .expect("failed to execute process");
    
    stddout = match serde_json::from_slice(&output.stdout){
        Ok(e)=> e,
        Err(_e) => vec![]
    };
        
       

        return stddout;
}


pub fn finisher(json: serde_json::Value, num_str: String, second: Output, output_arg: String)-> String{
        let valid = typeone!("Valid", &json, &num_str);
        let mut final23 = String::from("");

        if eq!(typeone!("Problem", &json, &num_str),"equal") {
            
            if eq!(typeone!("Type", &json, &num_str), "stdout") {
                if eq!(typestd!(&second.stdout.clone()),  &valid) {
                    println!("{}", &output_arg);
                    final23= output_arg;
                }
            }
            else if eq!(&typeone!("Type", &json, &num_str), "error"){
                if eq!(typestd!(&second.stderr.clone()), &valid) {
                    println!("{}", &output_arg);
                    final23= output_arg;
                }
            }
            else if eq!(&typeone!("Type", &json, &num_str), "oneshot"){
                if eq!(typestd!(&second.stdout.clone()), &valid) || eq!(typestd!(&second.stdout.clone()), "null".to_string()){
                    final23= "In Account".to_string()
                }
                
            }
            else if eq!(&typeone!("Type", &json, &num_str), "one_iter_equal"){
                space_error_handler!(typestd!(&second.stdout.clone()), final23.clone());

                let mut checked = "";
                for variable in second.stdout.clone().iter(){
                    if eq!(&variable.to_string(), &valid) {
                        checked = "here";
                    }
                }
                if eq!(checked, ""){
                    println!("{}", &output_arg);
                    final23= output_arg;
                }
                
            }
            else if eq!(&typeone!("Type", &json, &num_str), "two_or_iter_equal"){
                space_error_handler!(typestd!(&second.stdout.clone()), final23.clone());

                let mut checked = "";
                let valid2 = typeone!("Valid2", &json, &num_str);

                for variable in second.stdout.clone().iter(){
                    if eq!(&variable.to_string(), &valid) || eq!(&variable.to_string(), &valid2){
                        checked = "here";
                    }
                }
                if eq!(checked, ""){
                    println!("{}", &output_arg);
                    final23= output_arg;
                }
                
            }    
        }

        else if eq!(typeone!("Problem", &json, &num_str), "three_equal") {

            if eq!(&typeone!("Type", &json, &num_str), "stdout") {
                let valid2 = typeone!("Valid2", &json, &num_str);
                let valid3 = typeone!("Valid3", &json, &num_str);
                if eq!(typestd!(&second.stdout.clone()),  &valid) || eq!(typestd!(&second.stdout.clone()),  &valid2) || eq!(typestd!(&second.stdout.clone()),  &valid3) {
                    println!("{}", &output_arg);
                    final23= output_arg;
                }
            }
        }

        else if eq!(typeone!("Problem", &json, &num_str), "two_equal") {

            if eq!(&typeone!("Type", &json, &num_str), "auto_catch") {
                let first_arg = typeone!("first", &json, &num_str);
                let second_arg = typeone!("second", &json, &num_str);
                let qqqq = second.stdout.clone().to_owned();
                let cccc = String::from_utf8_lossy(&qqqq);
                let mut uq: serde_json::Value = Default::default();
                
                uq = match serde_json::from_str(&cccc){
                    Ok(e)=> e,
                    Err(_e) => serde_json::Value::Null
                };
                    
                let valid = typeone!("Valid", &json, &num_str);
                let valid2 = typeone!("Valid2", &json, &num_str);
                if eq!(uq[&first_arg].to_string(),  valid) || eq!(uq[&second_arg].to_string(),  valid2) {
                    println!("{}", &output_arg);
                    final23= output_arg;
                }
            }
        }

        
        
        else if eq!(typeone!("Problem", &json, &num_str), "not_equal") {
            
            if eq!(&typeone!("Type", &json, &num_str), "stdout") {
                if n_eq!(typestd!(&second.stdout.clone()),  &valid) {
                    println!("{}", &output_arg);
                    final23= output_arg;
                }
            }
            else if eq!(&typeone!("Type", &json, &num_str), "error"){
                if n_eq!(typestd!(&second.stderr.clone()), &valid) {
                    println!("{}", &output_arg);
                    final23= output_arg;
                }
            }
            else if eq!(&typeone!("Type", &json, &num_str), "oneshot"){
                if n_eq!(typestd!(&second.stdout.clone()), &valid) || eq!(typestd!(&second.stdout.clone()), "null".to_string()){
                    final23= "In Account".to_string()
                }
            }
            else if eq!(&typeone!("Type", &json, &num_str), "print_iter"){
                space_error_handler!(typestd!(&second.stdout.clone()), final23.clone());

                let mut stddout2: Vec<String>= vec![]; 
                
                stddout2 = match serde_json::from_slice(&second.stdout){
                    Ok(e)=> e,
                    Err(_e) => vec![]
                };   
                
                
    
                if n_eq!(typestd!(&second.stdout.clone()), &valid){
                    for ii in stddout2.iter(){
                        println!("{}", &ii);
                        let valuee = ii.clone().to_string().replace("\n", "");
                        final23 = final23 + &ii.clone().to_string().replace("\n", "") + "\n";
                    }
                }
            }
            else if eq!(&typeone!("Type", &json, &num_str), "one_iter_equal"){
                space_error_handler!(typestd!(&second.stdout.clone()), final23.clone());
                let mut checked = "";
                for variable in second.stdout.clone().iter(){
                    if n_eq!(&variable.to_string(), &valid) {
                        checked = "here";
                    }
                }
                if eq!(checked, ""){
                    println!("{}", &output_arg);
                    final23= output_arg;
                }
                
            }
            else if eq!(&typeone!("Type", &json, &num_str), "two_or_iter_equal"){
                space_error_handler!(typestd!(&second.stdout.clone()), final23.clone());
                let mut checked = "";
                let valid2 = typeone!("Valid2", &json, &num_str);

                for variable in second.stdout.clone().iter(){
                    if n_eq!(&variable.to_string(), &valid) || n_eq!(&variable.to_string(), &valid2){
                        checked = "here";
                    }
                }
                if eq!(checked, ""){
                    println!("{}", &output_arg);
                    final23= output_arg;
                }
                
            }    
        }
        else if eq!(typeone!("Problem", &json, &num_str), "equal-less") {
            space_error_handler!(typestd!(&second.stdout.clone()), final23.clone());

            let myint:i32 = match match_type!("Type", &json, &num_str){
                "stdout" => typestd!(&second.stdout.clone()).parse().unwrap(),
                "error"=>typestd!(&second.stderr.clone()).parse().unwrap(),
                _=>return final23,
            };
            if equal_less!(myint,  valid.clone().parse().unwrap()) {
                println!("{}", &output_arg);
                final23= output_arg;
            }  
        }
        else if eq!(typeone!("Problem", &json, &num_str), "equal-more") {
            space_error_handler!(typestd!(&second.stdout.clone()), final23.clone());

            let myint:i32 = match match_type!("Type", &json, &num_str){
                "stdout" => typestd!(&second.stdout.clone()).parse().unwrap(),
                "error"=>typestd!(&second.stderr.clone()).parse().unwrap(),
                _=>return final23,
            };
            if equal_more!(myint,  valid.clone().parse().unwrap()) {
                println!("{}", &output_arg);
                final23= output_arg;
            }
        }

        else if eq!(typeone!("Problem", &json, &num_str),"equal_basic_username") {
            
            if eq!(typeone!("Type", &json, &num_str), "stdout") {
                if eq!(typestd!(&second.stdout.clone()),  &valid) || eq!(typestd!(&second.stdout.clone()),  "root") || eq!(typestd!(&second.stdout.clone()),  "user") || eq!(typestd!(&second.stdout.clone()),  "admin") || eq!(typestd!(&second.stdout.clone()),  "admin") || eq!(typestd!(&second.stdout.clone()),  "username") || eq!(typestd!(&second.stdout.clone()),  "password") || eq!(typestd!(&second.stdout.clone()),  "user1") || eq!(typestd!(&second.stdout.clone()),  "toor") || eq!(typestd!(&second.stdout.clone()),  "azure") || eq!(typestd!(&second.stdout.clone()),  "user2"){
                    println!("{}", &output_arg);
                    final23= output_arg;
                }
            }   
        }
          
        let a = final23.to_owned();
     return final23;   
}