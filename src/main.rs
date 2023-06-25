#![allow(unused)]
use core::num;
use std::{fs,borrow::Borrow, fmt::format, time::Duration, env, process::Command};
use xlsxwriter::{*, prelude::*};
use serde;
use std::{fs::File, io::BufReader};
mod normal_funcs;
mod anormal_funcs;


macro_rules! typeone_main {
    ($x:expr, $json:expr, $num_int:expr) => (String::from($json[($num_int).to_string()][$x].as_str().unwrap()));
}
macro_rules! eq {
    ($x:expr, $y:expr) => ($x == $y);
}
macro_rules! match_type {
    ($x:expr, $json:expr, $num_str:expr) => ($json[$num_str][$x].as_str().unwrap());
}

macro_rules! excel_type {
    ($status2:expr, $sonuc:expr, $json:expr, $num_str:expr, $num_int:expr, $sheet1:expr, $rules2:expr, $description2:expr, $status2_no:expr, $status2_yes:expr, $ids2:expr, $number_formats:expr, $for_ids:expr, $urls:expr) => (
                let hey = ($json[$num_str]["Part"].as_str().unwrap().chars().take(1).last().unwrap().to_string() + &$json[$num_str]["Part"].as_str().unwrap().chars().take(2).last().unwrap().to_string()).to_uppercase() + $for_ids;
                
                $sheet1.write_string($num_int, 1, &hey, Some($number_formats)).expect("A");
                $sheet1.write_string($num_int, 2, $json[$num_str]["Title"].as_str().unwrap(), Some($rules2)).expect("A");
                $sheet1.write_string($num_int, 3, $json[$num_str]["Descrip"].as_str().unwrap(), Some($description2)).expect("A");
                if $sonuc == &String::from("") {
                    $sheet1.write_string($num_int, 4, $status2, Some($status2_yes)).expect("A");
                }
                else{
                    $sheet1.write_string($num_int, 4, $status2, Some($status2_no)).expect("A");
                }
                match $sheet1.write_string($num_int.clone(), 5, $sonuc, Some(&$ids2)){
                    Ok(_e) => (),
                    std::result::Result::Err(_) => {println!("Error");match $sheet1.write_string($num_int.clone(), 5, "Lenght Problem, You can see result on website", Some(&$ids2)){
                        Ok(_e) => (),
                        Err(_e) => (),
                    }},
                };
                $sheet1.write_string($num_int, 6, $json[$num_str]["Risk_Level"].as_str().unwrap(), Some($rules2)).expect("A");
                $sheet1.write_string($num_int, 7, $json[$num_str]["Reference"].as_str().unwrap(), Some($urls)).expect("A");

    );
}


fn karar(sonuc:String)-> String{
    let mut status2 = String::from(""); 
    if  sonuc == status2{
        status2 = String::from("Yes");
    }
    else {
        status2 = String::from("No");
    }
    return status2;
}



fn scaning(json: serde_json::Value, direct_lenght: u32, mut stddout: Vec<String>, mut sonuc:String, mut sheet1:Worksheet, rules2:&Format<>, description2:&Format<>, status2_no:&Format<>, status2_yes:&Format<>, ids2:&Format<>, services_formats:&Format<>, number_formats:&Format<>, urls:&Format<>)-> Result<(), String> {
    let mut row_len = 1;
    let mut for_ids = 1;
    for num_int in 2..=direct_lenght { 
 
        for_ids+=1;

        let num_str = num_int.to_string();
        

        if typeone_main!("Part", json.clone(), num_int.clone()) != typeone_main!("Part", json.clone(), num_int-1.clone()){
            sheet1.merge_range(row_len, 0, num_int-1, 0, typeone_main!("Part", json.clone(), num_int-1.clone()).as_str(), Some(&services_formats)).expect("A");
            row_len = num_int;
            for_ids = 1;

        }
        else if num_int == direct_lenght{
            sheet1.merge_range(row_len, 0, num_int, 0, typeone_main!("Part", json.clone(), num_int.clone()).as_str(), Some(&services_formats)).expect("A");
        }


        if typeone_main!("Args", json.clone(), num_int.clone()) == typeone_main!("Args", json.clone(), num_int-1.clone()) && typeone_main!("num_req", json.clone(), num_int-1.clone()) != "5".to_string() && typeone_main!("num_req", json.clone(), num_int-1.clone()) != "8".to_string(){

            if typeone_main!("num_req", json.clone(), num_int.clone()) == String::from("5") {(stddout, sonuc)  = normal_funcs::az_resource_name_2(json.clone(), num_str.clone());}

            if typeone_main!("num_req", json.clone(), num_int.clone()) == String::from("8") {(stddout, sonuc)  = normal_funcs::az_resource_name_3_second_resource_group(json.clone(), num_str.clone());}

            else{
                sonuc =  match match_type!("num_req", &json, &num_str){
                    "1"=>normal_funcs::oneshot(json.clone(), num_str.clone()),
                    "2"=> anormal_funcs::az_anormal(json.clone(), num_str.clone(), stddout.clone()),
                    "3" => anormal_funcs::az_anormal3(json.clone(), num_str.clone(), stddout.clone()),
                    "4"=> anormal_funcs::az_anormal_resource_name_3(json.clone(), num_str.clone(), stddout.clone()),
                    "6"=> anormal_funcs::az_anormal_double_equal_2(json.clone(), num_str.clone(), stddout.clone()),
                    "7"=> anormal_funcs::az_anormal_1_to_3(json.clone(), num_str.clone(), stddout.clone()),
                    &_ => return Err("Wrong Input!".to_string())
                };
            }
        }


        else {
            
            if typeone_main!("num_req", json.clone(), num_int.clone()) == String::from("1") {sonuc = normal_funcs::oneshot(json.clone(), num_str.clone());}
            else{
                (stddout, sonuc) =  match match_type!("num_req", &json, &num_str){
                    "2"=> normal_funcs::az_normal(json.clone(), num_str.clone()),
                    "3" => normal_funcs::az_normal3(json.clone(), num_str.clone()),
                    "4"=> normal_funcs::az_resource_name_3(json.clone(), num_str.clone()),
                    "5"=>normal_funcs::az_resource_name_2(json.clone(), num_str.clone()),
                    "6"=> normal_funcs::az_normal_double_equal_2(json.clone(), num_str.clone()),
                    "7"=> normal_funcs::az_normal_1_to_3(json.clone(), num_str.clone()),
                    "8"=> normal_funcs::az_resource_name_3_second_resource_group(json.clone(), num_str.clone()),
                    &_ => return Err("Wrong Input!".to_string())
                };
            }   
        }

        let status2 = karar(sonuc.clone()).to_string();
        let sonuc2 = sonuc.replace("\n", ", ").replace(", ,", ",");
        let sonuc2 = format!("{sonuc2}{}", "_".to_string()).replace(", _", "");

        excel_type!(&status2,&sonuc, json.clone(), &num_str, num_int.clone(), sheet1, &rules2, &description2, &status2_no, &status2_yes, &ids2, &number_formats, &for_ids.to_string(), &urls);

        

    }
    Ok(())
}





fn main()-> Result<(), String> {

    let services = vec![obfstr::obfstr!("Compute").to_string(), obfstr::obfstr!("Databases").to_string(), obfstr::obfstr!("Storage").to_string(), obfstr::obfstr!("Logging").to_string(), obfstr::obfstr!("Security").to_string(), obfstr::obfstr!("Network").to_string(), obfstr::obfstr!("Management").to_string()];

    

    let workbook = Workbook::new("scan.xlsx").expect("A");
    let mut binding = Format::new();
    let names2 = binding.set_font_color(FormatColor::Black).set_bold().set_bg_color(FormatColor::Gray);
    let mut binding = Format::new();
    let rules2 = binding.set_font_color(FormatColor::Black).set_bold();
    let mut binding = Format::new();
    let description2 = binding.set_font_color(FormatColor::Black);
    let mut binding = Format::new();
    let status2_yes = binding.set_font_color(FormatColor::Green).set_bold().set_bg_color(FormatColor::Lime);
    let mut binding = Format::new();
    let status2_no = binding.set_font_color(FormatColor::Silver).set_bold().set_bg_color(FormatColor::Red);
    let mut binding = Format::new();
    let ids2 = binding.set_font_color(FormatColor::Black);
    let mut binding = Format::new();
    let urls = binding.set_font_color(FormatColor::Blue).set_underline(FormatUnderline::Single);
    let mut binding = Format::new();
    let services_formats = binding.set_font_color(FormatColor::Black).set_align(FormatAlignment::CenterAcross).set_vertical_align(FormatVerticalAlignment::VerticalCenter).set_bg_color(FormatColor::Orange).set_bold().set_border_color(FormatColor::Black).set_border(FormatBorder::Thick);
    let mut binding = Format::new();
    let number_formats = binding.set_font_color(FormatColor::Black).set_align(FormatAlignment::CenterAcross).set_vertical_align(FormatVerticalAlignment::VerticalCenter).set_bg_color(FormatColor::Orange).set_bold().set_border_color(FormatColor::Black).set_border(FormatBorder::Thin);

    for service in services.iter(){
        
        let azset = service.to_string().replace("_", " ");
        let variable = Some(azset.as_str()).to_owned();
        let mut sheet1 = workbook.add_worksheet(variable).expect("A");
        sheet1.write_string(0, 0, "", Some(&names2)).expect("A");
        sheet1.write_string(0, 1, "IDs", Some(&names2)).expect("A");
        sheet1.write_string(0, 2, "Rules", Some(&names2)).expect("A");
        sheet1.write_string(0, 3, "Descriptions", Some(&names2)).expect("A");
        sheet1.write_string(0, 4, "Status", Some(&names2)).expect("A");
        sheet1.write_string(0, 5, "Comments", Some(&names2)).expect("A");
        sheet1.write_string(0, 6, "Risk Level", Some(&names2)).expect("A");
        sheet1.write_string(0, 7, "References", Some(&names2)).expect("A");
        let mut sonuc = "".to_string();


        let mut filename = format!("files/{}.json", service.to_owned());
        println!("{}", filename);
        let file = File::open(filename).expect("Not Found!");
        let reader = BufReader::new(file);
        let json: serde_json::Value = serde_json::from_reader(reader).expect("Not Json");
       



        filename = format!("files/{service}{}.json", "_len".to_string());
        let file = File::open(filename).expect("Not Found!");
        let reader = BufReader::new(file);
        let lenght_loop: serde_json::Value = serde_json::from_reader(reader).expect("Not Json");

        





       let mut for_ids = 1;


            let num_int = 1;
            let num_str = num_int.to_string();
            let mut stddout:Vec<String> = vec![];
            
            
            if json.to_string().len() > 2{}else{println!("Wrong input!");break;}
            if lenght_loop.to_string().len() > 2{}else{println!("Wrong input!");break;}


            let direct_lenght_str = lenght_loop["lenght"].as_str().unwrap();
            let direct_lenght_u32 = direct_lenght_str.parse::<u32>().unwrap();

            
            println!("{:?}", lenght_loop["lenght"].as_str().unwrap());
            


            if typeone_main!("num_req", json.clone(), num_int.clone()) == String::from("1") {sonuc = normal_funcs::oneshot(json.clone(), num_str.clone());}

            else{
                (stddout, sonuc) =  match match_type!("num_req", &json, &num_str){
                    "2"=> normal_funcs::az_normal(json.clone(), num_str.clone()),
                    "3" => normal_funcs::az_normal3(json.clone(), num_str.clone()),
                    "4"=> normal_funcs::az_resource_name_3(json.clone(), num_str.clone()),
                    "5"=>normal_funcs::az_resource_name_2(json.clone(), num_str.clone()),
                    "6"=> normal_funcs::az_normal_double_equal_2(json.clone(), num_str.clone()),
                    "7"=> normal_funcs::az_normal_1_to_3(json.clone(), num_str.clone()),
                    "8"=> normal_funcs::az_resource_name_3_second_resource_group(json.clone(), num_str.clone()),
                    &_ => return Err("Wrong Input!".to_string())
                };
            }  

            let status2 = karar(sonuc.clone()).to_string();
            let sonuc2 = sonuc.replace("\n", ", ");
            let sonuc2 = format!("{sonuc2}{}", "_".to_string()).replace(", _", "");
            excel_type!(&status2,&sonuc, json.clone(), &num_str, num_int.clone(), sheet1, &rules2, &description2, &status2_no, &status2_yes, &ids2, &number_formats, &for_ids.to_string(), &urls);


            match scaning(json.clone(),direct_lenght_u32.clone() ,stddout.clone(), sonuc, sheet1, &rules2, &description2, &status2_no, &status2_yes, &ids2, &services_formats, &number_formats, &urls){
                Ok(_) => (),
                Err(e) => return Err(e),
        };
    }
    
    workbook.close().expect("Can't save.");

    Ok(())
}