use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpServer};
use serde::{self, serde};
use reqwest;

struct SupabaseConfig {
    api_url: &'static str,
    api_key: &'static str,
}

const SUPABASE_CONFIG: SupabaseConfig = SupabaseConfig {
    api_url: "https://rftiqjpzbdnbbyywmbuz.supabase.co",
    api_key: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InJmdGlxanB6YmRuYmJ5eXdtYnV6Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3MDY1NTU1MDksImV4cCI6MjAyMjEzMTUwOX0.0wuS8vmGyw-n4Gs-hYm5gFy-ruWvF0FWJv534ISw3cM",
};

#[derove(Debug, Serialize, Deserialize)]
struct Ticket {
    id: usize,
    title: String,
    description: String,
    created_at: String, // Need to repalce with Supabase Schema
    updated_at: String, // Need to repalce with Supabase Schema
}