use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpServer};
use serde::{Serializer, Deserialize};
use reqwest;
use select::document::Document;
use select::node::Node;
use supabase::SupabaseClient;

#[derive(Debug, Serialize, Deserialize)]
struct Ticket {
    id: usize,
    title: String,
    description: String,
    created_at: String, // Replace with Schema
    updated_at: String, // Replace with Schema
}

/*fn main() {
    let supabase_url = "https://rftiqjpzbdnbbyywmbuz.supabase.co";
    let supabase_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InJmdGlxanB6YmRuYmJ5eXdtYnV6Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3MDY1NTU1MDksImV4cCI6MjAyMjEzMTUwOX0.0wuS8vmGyw-n4Gs-hYm5gFy-ruWvF0FWJv534ISw3cM";
    let client = SupabaseClient::new(supabase_url, supabase_key);
}*/

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

fn parse_html(html: &str) {
    let document = Document::from_read(html.as_bytes()).unwrap();

    for node in document.find("selector here") {
        // Exact information from the nodes 
        let text = node.text();
        println!("{}", text);

        // Extract the href attribute from the node
        let ticket = Ticket {
            id: 0,
            title: text.clone(),
            description: "Description Here".to_string(), 
            created_at: "2021-11-25T00:00:00.000Z".to_string(),
            updated_at: "2021-11-25T00:00:00.000Z".to_string(),
        };

        update_supabase(ticket);
    }
}

fn update_supabase(ticket: Ticket) {
    // Update the Supabase client 
    let supabase_url = "https://rftiqjpzbdnbbyywmbuz.supabase.co";
    let supabase_key = 
    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InJmdGlxanB6YmRuYmJ5eXdtYnV6Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3MDY1NTU1MDksImV4cCI6MjAyMjEzMTUwOX0.0wuS8vmGyw-n4Gs-hYm5gFy-ruWvF0FWJv534ISw3cM";
    let client = SupabaseClient::new(supabase_url, supabase_key);


}
#[tokio::main]
async fn main() {
    // List of URLS for scrape
    let urls = vec![
        "https://ticketera.com/",
        "https://www.coliseodepuertorico.com/",
        "https://boletos.prticket.com/events/en/listaEventos",
        "https://tcpr.com/",
        "https://www.eventbrite.com/d/united-states--puerto-rico/events/",
        "https://tcpr.com/categories/concerts/2",
        "https://www.ticketcity.com/puerto-rico-tickets/coliseo-de-puerto-rico-tickets.html",
        "https://www.cocacolamusichall.com/events/events-listing"
    ];

    for url in urls {
        match fetch_html(url).await {
            Ok(html) => parse_html(&html),
            Err(e) => println!("Error fetching {}: {}", url, e),
        }
    }
}
/* fn get_scraps () {
let response = reqwest::blocking::get("https://ticketera.com/");
let response = request::blocking::get("https://boletos.prticket.com/events/en/listaEventos");
let html_content = response.unwrap().text().unwrap();
}*/

#[derove(Debug, Serialize, Deserialize)]
struct Ticket {
    id: usize,
    title: String,
    description: String,
    created_at: String, // Need to repalce with Supabase Schema
    updated_at: String, // Need to repalce with Supabase Schema
}

#[derive(Debug, serde::Serialize, Deserialize, serde::Deserialize)]
struct User {
    id: i32,
    username: String,
    password: String,
    socical_media_links: serde_json::Value,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Evvent {
    event_id: i32,
    event_name: String,
    event_date: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct UserTicket {
    user_id: i32,
    event_id: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct EventMemory {
    user_id: i32,
    event_id: i32,
    photo_links: serde_json::Value,
    video_links: serde_json::Value,
    audio_links: serde_json::Value,
}