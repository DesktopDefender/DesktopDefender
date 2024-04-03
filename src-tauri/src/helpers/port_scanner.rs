use std::io::ErrorKind;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::sync::mpsc::channel;
use std::time::Duration;
use threadpool::ThreadPool;

// Code from https://github.com/kristoferfannar/port_scanner,
// which was initially developed specifically for this project

/*
Given an ip address and a vector of ports,
returns a subset of those ports which could be connected to
If the port list is empty, the method will try all ports it wants to.
*/
#[tauri::command]
pub fn find_open_ports(ip: &str, in_ports: Vec<i32>) -> Vec<i32> {
    let ports;
    let mut open_ports: Vec<i32> = Vec::new();

    if in_ports.len() == 0 {
        println!("No ports provided, generating...");
        ports = generate_ports();
    } else {
        ports = in_ports.clone();
    }

    // create a channel for adding ports in a vector on the main thread
    // connector threads will add ports to the channel if they are open
    let (sender, receiver) = channel::<i32>();

    // create a threadpool to limit the
    // upper bound of concurrent threads
    let pool_size = 100;
    let pool = ThreadPool::new(pool_size);

    for p in ports {
        pool.execute({
            // hmm I'm not sure what to do here,
            // whether to use String or &str
            let host = ip.to_string();
            let sender = sender.clone();
            move || {
                if port_is_open(host.as_str(), p.to_string().as_str()) {
                    // send the port on the channel
                    sender.send(p).unwrap();
                }
            }
        });
    }

    // run the threads, *pool_size* at a time
    pool.join();

    // close the channel...
    drop(sender);

    // ...and push the received ports into a vector
    for p in receiver {
        open_ports.push(p);
    }

    return open_ports;
}

fn generate_ports() -> Vec<i32> {
    let mut generated_ports: Vec<i32> = Vec::new();

    for i in 1..10000 {
        generated_ports.push(i);
    }

    return generated_ports;
}

fn port_is_open(host: &str, port: &str) -> bool {
    let mut address = String::new();
    address.push_str(&host.trim());
    address.push(':');
    address.push_str(&port.trim());

    let mut socket_addresses = format!("{}:{}", host, port).to_socket_addrs().unwrap();
    let socket_address = socket_addresses.next().unwrap();

    let result = TcpStream::connect_timeout(&socket_address, Duration::from_secs(1));

    if let Err(e) = result {
        match e.kind() {
            ErrorKind::TimedOut => {}
            ErrorKind::ConnectionRefused => {}
            _ => {
                println!("Error: {}", e);
            }
        }
        return false;
    }

    return result.is_ok();
}
