#![allow(unused)]
use std::fs;
use std::env::Args;
use std::process::Command;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::process::Output;
use xlsxwriter::*;

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

macro_rules! typestd {
    ($x:expr) => (String::from_utf8_lossy($x).replace('\"', "").replace("[", "").replace("]", "").trim());
}

macro_rules! handmade_try_catch {
    ($x:expr, $write_val:expr) => (if $x.len()>=1 {}else{return $write_val;});
}



pub fn az_anormal(json: serde_json::Value, num_str: String, stddout: Vec<String>)-> String{

    helper::printer(&json, &num_str);
    let mut returned = String::from("");
    let mut write_val = String::from("");

   
    handmade_try_catch!(&stddout, write_val.clone());

    for output_arg in stddout.iter(){
        let second = Command::new(typeone!("Command", &json, &num_str))
        .args(typetwo!("Second_Args", &json, &num_str))
        .args(output_arg.as_str().split_whitespace())
        .args(typetwo!("Second_To_Second_Args", &json, &num_str))
        .output()
        .expect("failed to execute process");

        returned = helper::finisher(json.clone(), num_str.clone(), second.clone(), (&output_arg).to_string());

        if returned != ""{
            write_val = write_val+ &returned + "\n";
        }
    }
    return write_val.replace("\n\n", "\n");
}



pub fn az_anormal3(json: serde_json::Value, num_str: String, stddout: Vec<String>)-> String{
    helper::printer(&json, &num_str);
    let mut returned = String::from("");
    let mut write_val = String::from("");
    let mut stddout2: Vec<String>= vec![]; 


    handmade_try_catch!(&stddout, write_val.clone());

    for output_arg in stddout.iter(){

        let second = Command::new(typeone!("Command", &json, &num_str))
        .args(typetwo!("Second_Args", &json, &num_str))
        .args(output_arg.as_str().split_whitespace())
        .args(typetwo!("Second_To_Second_Args", &json, &num_str))
        .output()
        .expect("failed to execute process");



            
            stddout2 = match serde_json::from_slice(&second.stdout){
                Ok(e)=> e,
                Err(_e) => vec![]
            };   
                

            handmade_try_catch!(&stddout2, write_val.clone());

            for third_output in stddout2.iter(){
                    let third = Command::new(typeone!("Command", &json, &num_str))
                .args(typetwo!("Third_Args", &json, &num_str))
                .args(third_output.as_str().split_whitespace())
                .args(typetwo!("Third_To_Third_Args", &json, &num_str))
                .output()
                .expect("failed to execute process");
                

                returned = helper::finisher(json.clone(), num_str.clone(), third.clone(), (&third_output).to_string());
                        
                if returned != ""{
                    write_val = write_val+ &returned + "\n";
                }
            }
        
    }
    return write_val.replace("\n\n", "\n");
}

pub  fn az_anormal_resource_name_3(json: serde_json::Value, num_str: String, stddout: Vec<String>)->String{
    helper::printer(&json, &num_str);
    let mut returned = String::from("");
    let mut write_val = String::from("");
    let mut stddout2: Vec<String>= vec![];
    let mut uq: serde_json::Value= Default::default();

    handmade_try_catch!(&stddout, write_val.clone());

    for argg in stddout.iter(){
        let len = Command::new(typeone!("Command", &json, &num_str))
            .args(typetwo!("Second_Args", &json, &num_str))
            .args(argg.as_str().split_whitespace())
            .args(typetwo!("Len", &json, &num_str))
            .output()
            .expect("failed to execute process");
            
        stddout2 = match serde_json::from_slice(&len.stdout.clone()){
            Ok(e)=> e,
            Err(_e) => vec![]
        };
        


        let lenght2 = stddout2.iter().len();

        handmade_try_catch!(&stddout2, write_val.clone());

        let mut resource_req = Command::new( typeone!("Command", &json, &num_str))
        .args(typetwo!("Second_Args", &json, &num_str))
        .args(argg.as_str().split_whitespace())
        .args(typetwo!("Second_To_Second_Args", &json, &num_str))
        .output()
        .expect("failed to execute process");



    let qqqq = resource_req.stdout.clone().to_owned();
    let cccc = String::from_utf8_lossy(&qqqq);
    
    uq = match serde_json::from_str(&cccc){
        Ok(e)=> e,
        Err(_e) => serde_json::Value::Null
    };
        

    handmade_try_catch!(&cccc, write_val.clone());

    for qq in 0..=lenght2-1 {
    let mut output2 = Command::new( typeone!("Command", &json, &num_str))
        .args(typetwo!("Third_Args", &json, &num_str))
        .args(uq[qq]["name"].as_str().unwrap().split_whitespace())
        .args(typetwo!("Third_To_Third_Args", &json, &num_str))
        .args(uq[qq]["resourceGroup"].as_str().unwrap().split_whitespace())
        .output()
        .expect("failed to execute process");




        let bbb = String::from("Name:")+ uq[qq]["name"].as_str().unwrap()+ " / Resource Group:" + uq[qq]["resourceGroup"].as_str().unwrap();
        returned = helper::finisher(json.clone(), num_str.clone(), output2.clone(), bbb);
        
        if returned != ""{
            write_val = write_val+ &returned + "\n";
        }
    }
    }

    return write_val.replace("\n\n", "\n");
}



pub  fn az_anormal_double_equal_2(json: serde_json::Value, num_str: String, stddout: Vec<String>)->String{  
            helper::printer(&json, &num_str);
            let mut returned = String::from("");
            let mut write_val = String::from("");
            let mut uq: serde_json::Value;
            

            handmade_try_catch!(&stddout, write_val.clone());

            
            for output_arg in stddout.iter(){
            let second2 = Command::new(typeone!("Command", &json, &num_str))
            .args(typetwo!("Second_Args", &json, &num_str))
            .args(output_arg.as_str().split_whitespace())
            .args(typetwo!("Second_To_Second_Args", &json, &num_str))
            .output()
            .expect("failed to execute process");



            let qqqq = second2.stdout.clone().to_owned();
            let cccc = String::from_utf8_lossy(&qqqq);
            
            handmade_try_catch!(&cccc, write_val.clone());



            let mut uq: serde_json::Value = serde_json::from_str(&cccc).expect("Wrong Format");
            
            uq = match serde_json::from_str(&cccc){
                Ok(e)=> e,
                Err(_e) => serde_json::Value::Null
            };
                
            
           

            if eq!(typeone!("Part", &json, &num_str),"KeyVault") || (eq!(typeone!("Part", &json, &num_str),"Virtual Machines") && eq!(typeone!("Title", &json, &num_str),"Use Managed Disk Volumes for Virtual Machines\n")) {
                let first = typeone!("first", &json, &num_str);
                let second = typeone!("second", &json, &num_str);
                if eq!(typeone!("Problem", &json, &num_str),"equal"){
                    if json[num_str.clone()]["Valid1"].as_str().unwrap() == uq[&first].to_string() && json[num_str.clone()]["Valid2"].as_str().unwrap() == uq[&second].to_string(){
                        println!("{}", &output_arg);
                        returned= output_arg.clone();
                    }
                }
                else if eq!(typeone!("Problem", &json, &num_str),"equal_is_not_equal"){
                    if json[num_str.clone()]["Valid1"].as_str().unwrap() != uq[&first].to_string() && json[num_str.clone()]["Valid2"].as_str().unwrap() == uq[&second].to_string(){
                        println!("{}", &output_arg);
                        returned= output_arg.clone();
                    }
                }
            }
            else{
                let main = typeone!("main", &json, &num_str);
                let first = typeone!("first", &json, &num_str);
                let second = typeone!("second", &json, &num_str);
                let number:usize = typeone!("number", &json, &num_str).parse().unwrap();
                if eq!(typeone!("Problem", &json, &num_str),"equal"){
                    if json[num_str.clone()]["Valid1"].as_str().unwrap() == uq[&main][&number][&first].as_str().unwrap().to_string() && json[num_str.clone()]["Valid2"].as_str().unwrap() == uq[&main][&number][&second].as_str().unwrap().to_string(){
                        println!("{}", &output_arg);
                        returned= output_arg.clone();
                    }
                }
                else if eq!(typeone!("Problem", &json, &num_str),"equal_is_not_equal"){
                    if json[num_str.clone()]["Valid1"].as_str().unwrap() != uq[&main][&number][&first].as_str().unwrap().to_string() && json[num_str.clone()]["Valid2"].as_str().unwrap() == uq[&main][&number][&second].as_str().unwrap().to_string(){
                        println!("{}", &output_arg);
                        returned= output_arg.clone();
                    }
                }
            }
            
                
                if returned != ""{
                    write_val = write_val+ &returned + "\n";
                }
            
            
        }
        return write_val.replace("\n\n", "\n");
}



pub fn az_anormal_1_to_3(json: serde_json::Value, num_str: String, stddout: Vec<String>)->String{

    
            helper::printer(&json, &num_str);
            let mut returned = String::from("");
            let mut write_val = String::from("");
            let mut stddout2: Vec<String> = vec![];

            handmade_try_catch!(&stddout, write_val.clone());

        for output_arg in stddout.iter(){
          

            let second2 = Command::new(typeone!("Command", &json, &num_str))
            .args(typetwo!("Second_Args", &json, &num_str))
            .args(output_arg.as_str().split_whitespace())
            .args(typetwo!("Second_To_Second_Args", &json, &num_str))
            .output()
            .expect("failed to execute process");

            
            stddout2 = match serde_json::from_slice(&second2.stdout.clone()){
                Ok(e)=> e,
                Err(_e) => vec![]
            };
                


            handmade_try_catch!(&stddout2, write_val.clone());

            for third_output in stddout2.iter(){
                let third = Command::new(typeone!("Command", &json, &num_str))
                .args(typetwo!("Third_Args", &json, &num_str))
                .args(output_arg.as_str().split_whitespace())
                .args(typetwo!("Third_To_Third_Args", &json, &num_str))
                .args(third_output.as_str().split_whitespace())
                .output()
                .expect("failed to execute process");



                returned = helper::finisher(json.clone(), num_str.clone(), third.clone(), (&third_output).to_string());
                
                if returned != ""{
                    write_val = write_val+ &returned + "\n";
                }
            }
            
        }
        return write_val.replace("\n\n", "\n");
}

