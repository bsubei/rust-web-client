#![feature(lookup_host)]
use std::io::prelude::*;
use std::env;
use std::net;
use std::net::TcpStream;


fn main() {

    println!("Welcome to this simple Rust wget clone...");

    // First, parse the url the user wants to wget.
    let url = parse_args();

    // Grab the domain name portion so we know WHERE to make the request.
    let domain_name = get_domain_name(&url);
    // Grab the path portion of the url so we know WHAT to request.
    let path = get_path_from_url(&url);
    
    // Request the page at the specific domain address.
    let response = match request_page(&domain_name, &path) {
    	Ok(s) => {println!("oh hai"); s},
    	Err(e) => {println!("oh shit"); panic!("Error requesting page: {:?}", e)}
    };

    // Display the response.
    println!("response:\n{:?}", response);
}

// TODO return slice instead of whole string?
// Get the path portion after the domain name from the url.
fn get_path_from_url(url: &str) -> String {
	String::from("/java/host/test.html")
}

// TODO return slice instead of whole string?
fn get_domain_name(url: &str) -> String {
	String::from("www.brainjar.com")
}

fn build_http_message(path: &str) -> String {
    format!("GET {} HTTP/1.0\n\n", path)
}

fn request_page(domain_name: &str, path: &str) -> Result<String, &'static str> {
	let http_message = build_http_message(path);
    println!("[DEBUG] The http message is:\n{}", &http_message);

    // TODO need to go over every DNS result, some of the results don't give valid IPs.
    let mut sock_addr = net::lookup_host(domain_name).expect("Failed to perform dns lookup").nth(0).unwrap();
    // We need to set port 80 manually because this is an HTTP message.
    sock_addr.set_port(80);
    println!("{:?}", sock_addr);

	// Then we can open a TCP socket to that IP address.
    let mut stream = TcpStream::connect(sock_addr).unwrap();
    println!("[DEBUG] connected!");

    // Write out the entire HTTP message byte-by-byte.
    let _ = stream.write_all(http_message.as_bytes()).expect("Failed to write HTTP message to address");
    println!("[DEBUG] done writing!");

    // Read back the response from the stream.
    let mut response = String::new();
    let _ = stream.read_to_string(&mut response).expect("Failed to read response from address");
    println!("[DEBUG] done reading!");

    Ok(response)
}

fn parse_args() -> String {
	// Grab all the args from cmdline.
	let args: Vec<String> = env::args().collect();

	// TODO clean up this ugly mess! Return Option or Result and do error handling outside in main.
	return match args.len() {
		// If no args, panic!
		1 => {help(); panic!();},
		2 => {
			// If one arg, check that it can be parsed as a string.
			match args[1].parse() {
				Ok(arg) => {arg},
				// Otherwise, panic!
				_ => {help(); panic!();},
			}
		},
		// If more than one arg, panic!
		_ => {help(); panic!();},
	}
}

fn help() {
    println!("usage:
rust_web_client <url>
Fetch stuff from HTTP server at given url.");
}