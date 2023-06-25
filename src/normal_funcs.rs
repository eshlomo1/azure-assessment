#![allow(unused)]

use std::fmt::format;
use std::fs;
use std::env::Args;
use std::os::unix::process::CommandExt;
use std::process::{Command,Stdio};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::process::Output;
use serde_json::Value;
use xlsxwriter::*;
use std::str;

use crate::anormal_funcs;
#[path = "helper.rs"] mod helper;

macro_rules! eq {
    ($x:expr, $y:expr) => ($x == $y);
}
macro_rules! n_eq {
    ($x:expr, $y:expr) => ($x != $y);
}
macro_rules! typeone {
    ($x:expr, $json:expr, $num_str:expr) => (String::from($json[$num_str][$x].as_str().unwrap()));
}

macro_rules! typetwo {
    ($x:expr, $json:expr, $num_str:expr) => ($json[$num_str][$x].as_str().unwrap().split_whitespace());
}

macro_rules! handmade_try_catch {
    ($x:expr, $return_value:expr, $write_val:expr) => (if $x.len()>=1 {}else{return ($return_value,$write_val);});
}



pub  fn oneshot(json: serde_json::Value, num_str: String) -> (String){
    
    helper::printer(&json, &num_str); 
    let mut returned = String::from("");
    let mut write_val = String::from("");
    let mut output_arg = String::from("");

    let ps_child = Command::new("echo")
        .args(typetwo!("Args", &json, &num_str))           
        .stdout(Stdio::piped())      
        .spawn()                      
        .unwrap();                   
    let grep_child_one = Command::new("base64")
        .arg("--decode")
        .stdin(Stdio::from(ps_child.stdout.unwrap())) 
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let second = Command::new("bash")
        .stdin(Stdio::from(grep_child_one.stdout.unwrap()))
       .output().expect("msg");




        returned = helper::finisher(json.clone(), num_str.clone(), second.clone(), (&output_arg).to_string());

        if returned != ""{
            write_val = write_val+ &returned + "\n";
        }
    
   
    return (write_val.replace("\n\n", "\n"));
}

pub  fn az_normal(json: serde_json::Value, num_str: String) -> (Vec<String>, String){

    let stddout = helper::first_args_command(json.clone(), &num_str);
    let mut write_val = String::new();
    handmade_try_catch!(&stddout, stddout.clone(), write_val.clone());
        
    let mut write_val = anormal_funcs::az_anormal(json, num_str, stddout.clone());


        
       
    return (stddout,write_val.replace("\n\n", "\n"));
}







pub fn az_normal3(json: serde_json::Value, num_str: String) -> (Vec<String>, String){

    let stddout = helper::first_args_command(json.clone(), &num_str);
    let mut write_val = String::new();
    handmade_try_catch!(&stddout, stddout.clone(), write_val.clone());

    let mut write_val = anormal_funcs::az_anormal3(json, num_str, stddout.clone()); 

           
        return (stddout,write_val.replace("\n\n", "\n"));
}





pub  fn az_resource_name_2(json: serde_json::Value, num_str: String)->(Vec<String>, String){
    helper::printer(&json, &num_str);
    let mut returned = String::from("");
    let mut write_val = String::from("");
    let mut uq: serde_json::Value = Default::default();
    let mut lenght: Vec<String> = vec![];
    
    let output = Command::new(typeone!("Command", &json, &num_str))
                    .args(typetwo!("Args", &json, &num_str))
                    .args(typetwo!("Len", &json, &num_str))
                    .output()
                    .expect("failed to execute process");


        
                lenght = match serde_json::from_slice(&output.stdout){
                    Ok(e)=> e,
                    Err(_e) => vec![]
                };
            

        handmade_try_catch!(&lenght, lenght.clone(), write_val.clone());

    let lenght2 = lenght.iter().len();
    

        let resource_req = Command::new( typeone!("Command", &json, &num_str))
        .args(typetwo!("Args", &json, &num_str))
        .output()
        .expect("failed to execute process");



        let qqqq = resource_req.stdout.clone().to_owned();
        let cccc = String::from_utf8_lossy(&qqqq);


       handmade_try_catch!(&cccc, lenght.clone(), write_val.clone());

        
        uq = match serde_json::from_str(&cccc){
            Ok(e)=> e,
            Err(_e) => serde_json::Value::Null
        };
            
        
    
    for qq in 0..=lenght2-1 {

        let mut output2 = Command::new( typeone!("Command", &json, &num_str))
        .args(typetwo!("Second_Args", &json, &num_str))
        .args(uq[qq]["name"].as_str().unwrap().split_whitespace())
        .args(typetwo!("Second_To_Second_Args", &json, &num_str))
        .args(uq[qq]["resourceGroup"].as_str().unwrap().split_whitespace())
        .output()
        .expect("failed to execute process");

        let bbb = String::from("Name:")+ uq[qq]["name"].as_str().unwrap()+ " / Resource Group:" + uq[qq]["resourceGroup"].as_str().unwrap();
        


        returned = helper::finisher(json.clone(), num_str.clone(), output2.clone(), bbb);
        
        if returned != ""{
            write_val = write_val+ &returned + "\n";
        }
    }
    return (lenght,write_val.replace("\n\n", "\n"));
}


pub  fn az_resource_name_3(json: serde_json::Value, num_str: String)->(Vec<String>, String){
    let stddout = helper::first_args_command(json.clone(), &num_str);
    let mut write_val = String::new();
    handmade_try_catch!(&stddout, stddout.clone(), write_val.clone());

    let mut write_val = anormal_funcs::az_anormal_resource_name_3(json, num_str, stddout.clone());



    

    return (stddout,write_val);
}

pub  fn az_normal_double_equal_2(json: serde_json::Value, num_str: String)->(Vec<String>, String){
    let stddout = helper::first_args_command(json.clone(), &num_str);
    let mut write_val = String::new();
    handmade_try_catch!(&stddout, stddout.clone(), write_val.clone());

    let mut write_val = anormal_funcs::az_anormal_double_equal_2(json, num_str, stddout.clone());



    

    return (stddout,write_val.replace("\n\n", "\n"));
}

pub fn az_normal_1_to_3(json: serde_json::Value, num_str: String) -> (Vec<String>, String){

    let stddout = helper::first_args_command(json.clone(), &num_str);
    let mut write_val = String::new();
    
    handmade_try_catch!(&stddout, stddout.clone(), write_val.clone());

    let mut write_val = anormal_funcs::az_anormal_1_to_3(json, num_str, stddout.clone());



    

    return (stddout,write_val.replace("\n\n", "\n"));
}

pub  fn az_resource_name_3_second_resource_group(json: serde_json::Value, num_str: String)->(Vec<String>, String){
    helper::printer(&json, &num_str);
    let mut returned = String::from("");
    let mut write_val = String::from("");
    let mut uq: serde_json::Value = Default::default();
    let mut lenght: Vec<String> = vec![];
    let mut prep_third:Vec<String> = vec![];

    let output = Command::new(typeone!("Command", &json, &num_str))
                    .args(typetwo!("Args", &json, &num_str))
                    .args(typetwo!("Len", &json, &num_str))
                    .output()
                    .expect("failed to execute process");



        
    lenght = match serde_json::from_slice(&output.stdout){
        Ok(e)=> e,
        Err(_e) => vec![]
    };
        
        

        
        handmade_try_catch!(&lenght, lenght.clone(), write_val.clone());
    let lenght2 = lenght.iter().len();



        let resource_req = Command::new( typeone!("Command", &json, &num_str))
        .args(typetwo!("Args", &json, &num_str))
        .output()
        .expect("failed to execute process");



        let qqqq = resource_req.stdout.clone().to_owned();
        let cccc = String::from_utf8_lossy(&qqqq);


       handmade_try_catch!(&cccc, lenght.clone(), write_val.clone());

    
        

        
        
        uq = match serde_json::from_str(&cccc){
            Ok(e)=> e,
            Err(_e) => serde_json::Value::Null
        };
            
        
    
    for qq in 0..=lenght2-1 {

        let mut output2 = Command::new( typeone!("Command", &json, &num_str))
        .args(typetwo!("Second_Args", &json, &num_str))
        .args(uq[qq]["name"].as_str().unwrap().split_whitespace())
        .args(typetwo!("Second_To_Second_Args", &json, &num_str))
        .args(uq[qq]["resourceGroup"].as_str().unwrap().split_whitespace())
        .output()
        .expect("failed to execute process");



            
            prep_third = match serde_json::from_slice(&output2.stdout){
                Ok(e)=> e,
                Err(_e) => vec![]
            };
                

            for second_output in prep_third.iter(){

                let third = Command::new(typeone!("Command", &json, &num_str))
                .args(typetwo!("Third_Args", &json, &num_str))
                .args(second_output.as_str().split_whitespace())
                .args(typetwo!("Third_To_Third_Args", &json, &num_str))

                .output()
                .expect("failed to execute process");




                returned = helper::finisher(json.clone(), num_str.clone(), third.clone(), (&second_output).to_string());
                
                if returned != ""{
                    write_val = write_val+ &returned + "\n";
                }
            }
            
        }
        return (lenght,write_val.replace("\n\n", "\n"));
}