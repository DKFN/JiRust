extern crate reqwest;
extern crate hyper;
extern crate base64;
extern crate clap;

use base64::encode;

fn main() {
    let cli_conf = clap::App::new("JiRust")
        .version("0.1.0")
        .author("Valere Tetelin - Deadly :3 -")
        .about("Formats your Jira issues to the TODO file for EPITA")
        .arg(clap::Arg::with_name("JIRA_HOST")
                 .short("h")
                 .long("host")
                 .takes_value(true)
                 .required(true)
                 .help("Your JIRA HOST protocol://adress:port/"))
        .arg(clap::Arg::with_name("JIRA_MAIL")
                 .short("l")
                 .long("login")
                 .takes_value(true)
                 .required(true)
                 .help("Your JIRA mail"))
        .arg(clap::Arg::with_name("JIRA_PASS")
                 .short("p")
                 .long("pass")
                 .takes_value(true)
                 .required(true)
                 .help("Your JIRA pass"))
        .arg(clap::Arg::with_name("JIRA_PROJECT_ID")
                 .short("i")
                 .long("project_id")
                 .takes_value(true)
                 .required(true)
                 .help("Your JIRA project"))
        .get_matches();

    let jira_host = cli_conf.value_of("JIRA_HOST").unwrap();
    let jira_mail = cli_conf.value_of("JIRA_MAIL").unwrap();
    let jira_pass = cli_conf.value_of("JIRA_PASS").unwrap();
    let jira_project = cli_conf.value_of("JIRA_PROJECT_ID").unwrap();

    println!("Welcome to JiRust !");

    println!("We are now trying to talk with your Jira's API .......");

    jira_get_issues(jira_host, jira_mail, jira_pass, jira_project);
}

fn jira_get_issues(jira_host: &str,
                   jira_mail: &str,
                   jira_pass: &str,
                   jira_project: &str)
{
    let fin = format!("{}{}{}", jira_host, "rest/api/2/search?jql=project=", jira_project);

    println!("Targetting URL : {}", fin);
    let client = reqwest::Client::new();

    let mut credentials = jira_mail.to_owned();
    credentials.push_str(":");
    credentials.push_str(jira_pass);

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
