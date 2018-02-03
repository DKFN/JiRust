extern crate reqwest;
extern crate hyper;
extern crate base64;
extern crate rpassword;

use std::io;
use base64::encode;

fn main() {
    // Script variables
    let mut jira_host = String::new();
    let mut jira_mail = String::new();
    let mut jira_project = String::new();

    println!("Welcome to JiRust (unstable) made by Bataillion APPING");
    println!("We will ask you your atlassian email and password for processing requests");
    println!("Please note we do not save them in any way");
    println!("If you have questions about usage for this script please email valere.tetelin@gmail.com");

    println!("Please enter the adress of your Jira (with full protocol and port)");
    println!("Format is :: protocol://adresss:port/");

    io::stdin().read_line(&mut jira_host).expect("Unable to read line");

    println!("Please enter your Jira email :");

    io::stdin().read_line(&mut jira_mail).expect("Unable to read line");

    println!("Please enter your Jira password");

    let jira_pass = rpassword::prompt_password_stdout("->")
        .expect("Unable to catch the password");

    // io::stdin().read_line(&mut jira_pass).expect("Unable to read line");

    println!("Please enter the identifier of the project you wish to import (UPPERCASE)");

    io::stdin().read_line(&mut jira_project).expect("Unable to read line");

    println!("We are now trying to talk with your Jira's API .......");

    jira_get_issues(jira_host, jira_mail, jira_pass, jira_project);
}

fn jira_get_issues(jira_host: String,
                   jira_mail: String,
                   jira_pass: String,
                   jira_project: String)
{
    let mut host = jira_host.to_owned();
    let mut pass = jira_pass.to_owned();

    // pass.pop();
    host.pop();

    let fin = format!("{}{}{}", host, "rest/api/2/search?jql=project=", jira_project);

    println!("Targetting URL : {}", fin);
    let client = reqwest::Client::new();

    let mut credentials = jira_mail.to_owned();
    credentials.pop();
    credentials.push_str(":");
    credentials.push_str(&*pass);

    let cred_hash = encode(&credentials);
    println!("Using auth hash : {}", cred_hash);

    let mut response = client.get(reqwest::Url::parse(&*fin).unwrap())
        .header(reqwest::header::Authorization("Basic ".to_owned() + &*cred_hash))
        .send()
        .unwrap();

    if !response.status().is_success()
    {
        panic!("JIRA Host [{}]- It did not respond accordingly, please check your parameter",
               response.status());
    }

    println!("Body gotten : {:?}", response.text());
}
