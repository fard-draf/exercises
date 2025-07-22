use std::{borrow::Cow, cell::Cell, collections::HashMap, vec};

use crate::{api_key::Config, doubled::{double_in_place, doubled_as_new}, logger::{service_a, service_b, Logger}};
mod sensor;
mod api_key;
mod logger;
mod doubled;
mod eval;

fn main() {
    // let api_conf = Config::default();
    // api_conf.get_api_key();
    // api_conf.get_api_key();
    // api_conf.get_api_key();
    // api_conf.get_api_key();
    // api_conf.get_api_key();
    // println!("Counter => {:?}", api_conf.reading);


    // let logger = Logger::default();

    // service_a(&logger, "Please take the cheese".to_string());
    // service_b(&logger, "No cheese left behind".to_string());

    // println!("{:?}", logger.messages);

    let mut vec1 = vec![1, 2, 3];
    println!("Original vec1: {:?}", vec1);
    double_in_place(&mut vec1);
    println!("Vec1 après modification: {:?}", vec1);

        let vec2 = vec![5, 6, 7];
    println!("\nOriginal vec2: {:?}", vec2);
    let new_vec = doubled_as_new(&vec2);
    println!("Vec2 (inchangé): {:?}", vec2);
    println!("Nouveau vecteur créé: {:?}", new_vec);
}

fn copy_or_not_copy() {

    let copy_1 = 1;
    let copy_2 = [1u8; 20];
    let copy_3 = &copy_2;
    let copy_4 = "This is a static string, hard coded.";
    let copy_5 = &copy_4;

    let mut copy_6 = [0u8; 20];
    copy_6.copy_from_slice(copy_3);

    let a = copy_1;
    println!("copy i32:{:?}", copy_1);
    let b = copy_2;
    println!("copy arr [u8; usize] :{:?}", copy_2);
    let c = copy_3;
    println!("copy ref arr :{:?}", copy_3);
    let d = copy_4;
    println!("copy &str :{:?}", copy_4);
    let e = copy_5;
    println!("copy &&str :{:?}", copy_5);
    let f = copy_6;
    println!("copy slice arr :{:?}", copy_6);

    // NON COPY TYPE

    let non_copy_1 = String::from("Non copy string");
    let non_copy_2 = vec![1,2,3,4,5,6,7,8,9];
    let non_copy_3: Box<Vec<i32>> = Box::from(vec![12,12]);
    let non_copy_4 = Cow::from(String::from("Non cop"));
    let non_copy_5: HashMap<String, usize> = HashMap::new();


    // let a = non_copy_1;
    // println!("Non copy -> {:?}", non_copy_1); // Ne compile pas car une String est sur le heap. Sur la stack, une String est un fat pointer. 
    // let b = non_copy_2;
    // println!("Non copy -> {:?}", non_copy_2);  // Dynamique -> heap
    // let c = non_copy_3;
    // println!("Non copy -> {:?}", non_copy_3); // Dynamique -> heap   
    // let d = non_copy_4;
    // println!("Non copy -> {:?}", non_copy_4); // idem   
    // let e = non_copy_5;
    // println!("Non copy -> {:?}", non_copy_5); //idem   
}



