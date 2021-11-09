use isahc::{
    HttpClient, 
    config::{
        RedirectPolicy, 
        VersionNegotiation,
        SslOption}, 
    prelude::*
};

use std::{
    str,
    env,
    thread::sleep,
    fs::create_dir,
    path::Path,
    time::Duration
};

use serde_json::Value;
use clap::{
    Arg, App
    };


struct Http {
    timeout : u64
}

impl Http {

    fn build(&self) -> HttpClient {
        let client = HttpClient::builder()
                .version_negotiation(VersionNegotiation::http11())
                .redirect_policy(RedirectPolicy::None)
                .timeout(Duration::from_secs(self.timeout))                
                .ssl_options(SslOption::DANGER_ACCEPT_INVALID_CERTS | SslOption::DANGER_ACCEPT_REVOKED_CERTS);

        return client.build().unwrap()
    }



}

fn main(){

    let matches = App::new("")
        .arg(Arg::new("url")
            .short('u')
            .long("url")
            .value_name("url")
            .takes_value(true)
            .default_value("https://meme-api.herokuapp.com/gimme"))
        .arg(Arg::new("memes")
            .short('m')
            .long("memes")
            .value_name("memes")
            .takes_value(true)
            .default_value("5"))

        .arg(Arg::new("delay")
            .short('d')
            .long("delay")
            .takes_value(true)
            .default_value("2")
            .value_name("delay"))
        
        .arg(Arg::new("timeout")
            .short('t')
            .long("timeout")
            .takes_value(true)
            .default_value("20")
            .value_name("timeout"))

        .get_matches();
        
    
    let test = Http{timeout:matches.value_of("timeout").unwrap().parse().unwrap()};
    let http = test.build();
    if Path::new("output").exists() == false {
        create_dir("output").unwrap();
    
    }
    env::set_current_dir(Path::new("output")).unwrap();    
    for i in 0..matches.value_of("memes").unwrap().parse().unwrap() {
        let req= http.get(matches.value_of("url").unwrap());
        let mut data = req.unwrap().text().unwrap();
        let Json : Value = serde_json::from_str(&mut data).unwrap();
        let url = Json["url"].as_str().unwrap();
        let name = Json["title"].as_str().unwrap();
        let res = http.get(url);
        let mut filename = str::replace("test.png","test",name);
        res.unwrap().copy_to_file(&mut filename).unwrap();
        sleep(Duration::from_secs(matches.value_of("delay").unwrap().parse().unwrap()));
        println!("Downloaded {}",&mut filename);

    }
}