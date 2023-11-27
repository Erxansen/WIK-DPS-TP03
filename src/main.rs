use std::{
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use dotenv::dotenv;
use hostname::get;

fn main() {
    dotenv().ok();

    let port = env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &port).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn get_hostname() -> String {
    match get() {
        Ok(name) => name.to_string_lossy().into_owned(),
        Err(_) => "Unknown".to_string(),
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| line.len() > 0)
        .collect();

    let first_line = &http_request[0];
    println!("Request: {}", first_line);

    if first_line.starts_with("GET /ping") && first_line.ends_with("HTTP/1.1") {
        // Obtenir le nom d'hôte du système en utilisant la nouvelle fonction
        let hostname = get_hostname();
        println!("Hostname: {}", hostname);

        let headers = parse_headers(&http_request);
        let response_content = format_headers_as_json(&headers, &hostname);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
            response_content
        );
        stream.write(response.as_bytes()).unwrap();
    } else {
        let response_content = "";
        let content_length = response_content.len();
        let response = format!(
            "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",
            content_length,
            response_content
        );
        stream.write(response.as_bytes()).unwrap();
    }

    stream.flush().unwrap();
}

fn parse_headers(request: &Vec<String>) -> std::collections::HashMap<String, String> {
    let mut headers = std::collections::HashMap::new();
    for line in request.iter().skip(1) {
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            headers.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    headers
}

fn format_headers_as_json(headers: &std::collections::HashMap<String, String>, hostname: &str) -> String {
    let mut json = "{".to_owned();
    let mut first = true;

    // Ajouter le nom d'hôte aux en-têtes JSON
    json.push_str(&format!("\"hostname\": \"{}\"", hostname));

    for (key, value) in headers {
        if !first {
            json.push_str(", ");
        }
        json.push_str(&format!("\"{}\": \"{}\"", key, value));
        first = false;
    }

    json.push_str("}");
    json
}
