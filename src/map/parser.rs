use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct Map {
    pub mapName: String,
    pub nodes: Vec<Node>,
    pub lanes: Vec<Lane>,
}

#[derive(Debug, Deserialize)]
pub struct Node {
    pub id: i32,
    #[serde(rename = "type")]
    pub node_type: String,
    pub x: f32,
    pub y: f32,
    pub theta: f32,
}

#[derive(Debug, Deserialize)]
pub struct Lane {
    pub laneNum: i32,
    pub bidirectional: bool,
    pub nodes: [i32; 2],
}

pub fn load_map_from_file(path: &str) -> Result<Map, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let map = serde_json::from_reader(reader)?;
    Ok(map)
}