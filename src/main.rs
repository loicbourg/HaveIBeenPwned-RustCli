use std::fmt;
use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use serde_json::Error;

#[derive(Debug, Serialize, Deserialize)]
// TODO: find why rename_all is ignored
#[serde(rename_all = "snake_case")]
struct Pwned {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "DataClasses")]
    data_classes: Vec<String>
}

#[derive(Debug, StructOpt)]
#[structopt(name = "pwned ?", about = "have i been pwned cli.")]
struct Cli {
    mail: String
}

fn get_json(mail: String) -> Result<String, reqwest::Error>  {

    let resp = reqwest::get(&format!("https://haveibeenpwned.com/api/v2/breachedaccount/{}", mail))?
        .text()?;

    Ok(resp)
}

fn parse_json (json: &String) -> Result<Vec<Pwned>, Error> {
    let pwned_struct: Vec<Pwned> = serde_json::from_str(json).unwrap();

    Ok(pwned_struct)
}

fn main() {
    let args = Cli::from_args();

    let res = get_json(args.mail);

    let unwrapped_res = res.unwrap();

    let pwned_structs = parse_json(&unwrapped_res).unwrap();

    println!("Votre compte a été pwned  {} fois", pwned_structs.len());

    for pwned_struct in pwned_structs {
        println!("Nom de la société: {}", pwned_struct.name);

        println!("type de données ayant fuitées");
        
        for data_class in pwned_struct.data_classes {
            println!("- {}", data_class);  
        }

        println!("------------------------");
    }

    // println!("{}", pwned_structs);   
}
