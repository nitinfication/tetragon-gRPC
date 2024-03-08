use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;
use serde::{Deserialize, Serialize};

// Define the TetraEvent structure
#[derive(Debug, Deserialize)]
struct TetraEvent {
    process_exit: ProcessExitEvent,
    time: String,
}

#[derive(Debug, Deserialize)]
struct ProcessExitEvent {
    process: ProcessData,
    parent: ParentData,
    time: String,
}

#[derive(Debug, Deserialize)]
struct ProcessData {
    exec_id: String,
    pid: u64,
    uid: u64,
    binary: String,
    flags: String,
    start_time: String,
    auid: u64,
    parent_exec_id: String,
    refcnt: u64,
    tid: u64,
}

#[derive(Debug, Deserialize)]
struct ParentData {
    exec_id: String,
    pid: u64,
    uid: u64,
    binary: String,
    flags: String,
    start_time: String,
    auid: u64,
    parent_exec_id: String,
    tid: u64,
}

fn main() {
    let socket_path = "/var/run/tetragon/tetragon.sock";
    if let Ok(socket) = open_tetra_socket(socket_path) {
        let mut mutable_socket = socket.try_clone().unwrap();

        println!("Socket connection established");

        while let Some(event) = grpc_read_tetra_event(&mut mutable_socket) {
            println!("Received event: {:?}", event);
        }
    } else {
        println!("Failed to establish socket connection");
    }
}

fn open_tetra_socket(socket_path: &str) -> io::Result<UnixStream> {
    UnixStream::connect(socket_path)
}

fn grpc_read_tetra_event(socket: &mut UnixStream) -> Option<TetraEvent> {
    // reading event
    let query = "getevents";
    submit_query(socket, query)
}

fn submit_query(socket: &mut UnixStream, query: &str) -> Option<TetraEvent> {
    // Send query
    if let Err(_) = socket.write_all(query.as_bytes()) {
        return None;
    }

    // Read the response
    let mut response = String::new();
    if let Err(_) = socket.read_to_string(&mut response) {
        return None;
    }

    // Parse the response
    parse_event(response)
}

fn parse_event(event_data: String) -> Option<TetraEvent> {
    // Parse the even
    serde_json::from_str(&event_data).ok()
}
