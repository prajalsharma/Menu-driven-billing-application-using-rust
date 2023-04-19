

use std::{io, collections::HashMap};
#[derive(Debug,Clone)]
struct Bill{
    name:String,
    amount:f64
}

struct Bills{
    inner:HashMap<String,Bill>,
}

impl Bills {
    fn new()->Self{
        Self { inner:HashMap::new() }
    }
    fn add(&mut self, bill:Bill){
        self.inner.insert(bill.name.clone(),bill);
    }
    fn get_all(&self)-> Vec<Bill>{
        let mut bills=vec![];
        for bill in self.inner.values(){
            bills.push(bill.clone());
        }
        bills


    }
    fn remove(&mut self,name:&str)-> bool{
        self.inner.remove(name).is_some()

    }
    fn update(&mut self,name:&str,amount:f64)->bool{
        match self.inner.get_mut(name){
            Some(bill)=>{
                bill.amount=amount;
                true
            }
            None=>false,
        }
    }
    
}
fn get_bill_amount()->f64{
    println!("Amount");
    loop{
        let input:String=get_input();
        let parsed_input:Result<f64,_> =input.parse();
        match parsed_input{
            Ok(amount)=>return amount,
            Err(_)=>println!("Please enter a number"), 
        }
    }

}

fn add_bill_menu(bills:&mut Bills){
    //get the bill name
    println!("Bill Name");
    let name=get_input();
    let amount=get_bill_amount();
    let bill=Bill{name,amount}; 
    bills.add(bill);
    println!("Bill added successfully");
    //get bill amount
}

fn remove_bills(bills:&mut Bills){
    println!("enter the bill name to remove");
    for bill in bills.get_all(){
        println!("{:?}",bill);
    }
    let input =get_input();
    if bills.remove(&input){
        println!("removed");

    }else {
        println!("Bill not found");
    }



}

fn update_bills(bills:&mut Bills){
    println!("enter the bill name to update");
    for bill in bills.get_all(){
        println!("{:?}",bill);
    }
    let name =get_input();
    let amount=get_bill_amount();
    if bills.update(&name, amount){
        println!("updated");


    }else{
        println!("bill not found")
    }




}

fn view_bill_menu(bills:&mut Bills){
    for bill in bills.get_all(){
        println!("{:?}",bill);
    }

}

fn get_input()->String{
    let mut buffer=String::new();
    while io::stdin().read_line(&mut buffer).is_err(){    //because it returns Results
        println!("please enter your data again");
    }
    buffer.trim().to_owned()
}

fn main_menu(){
    fn show(){
        println!("");
        println!("===Manage Bills===");
        println!("1.Add bills");
        println!("2.View Bills");
        println!("3.Remove bills");
        println!("4.Edit any bill");
        println!("");
        println!("Enter selection:")
    }
    let mut bills=Bills::new();

    loop{
        show();
        let input=get_input();
        match input.as_str(){
            "1"=>add_bill_menu(&mut bills),
            "2"=>view_bill_menu(&mut bills),
            "3"=>remove_bills(&mut bills),
            "4"=>update_bills(&mut bills),
            _=> break,
        };
    }
}

fn main() {
    main_menu();

}
