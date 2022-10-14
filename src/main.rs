use std::net::UdpSocket;
mod packets;
use std::thread;
use std::time::Duration;

const SERVER_ADDR: &str = "127.0.0.1:8080";
const MY_ADDR: &str = "127.0.0.1";

fn main() {
    println!("\nClient Started");

    // let ping_packet = packets::BasePacket::new("ping".to_string(), "".to_string());
    // let ping_packet = serde_json::to_string(&ping_packet).unwrap();

    let login_info = packets::LoginPacket {
        username: "TheDude27".to_string(),
        password: "123".to_string(),
    };

    let login_packet = packets::BasePacket::new(
        "login".to_string(),
        String::new(),
        serde_json::to_string(&login_info).unwrap(),
    );
    let login_packet = serde_json::to_string(&login_packet).unwrap();

    let logout_info = packets::LogoutPacket {
        username: "TheDude27".to_string(),
        password: "123".to_string(),
    };

    let logout_packet = packets::BasePacket::new(
        "logout".to_string(),
        String::new(),
        serde_json::to_string(&logout_info).unwrap(),
    );
    let logout_packet = serde_json::to_string(&logout_packet).unwrap();

    let di_packet_one = packets::DirectionalPacket { x: 1.0, y: 0.0 };

    let di_packet_one = packets::BasePacket::new(
        "directional".to_string(),
        String::new(),
        serde_json::to_string(&di_packet_one).unwrap(),
    );
    let di_packet_one = serde_json::to_string(&di_packet_one).unwrap();

    let di_packet_two = packets::DirectionalPacket { x: -1.0, y: 0.0 };

    let di_packet_two = packets::BasePacket::new(
        "directional".to_string(),
        String::new(),
        serde_json::to_string(&di_packet_two).unwrap(),
    );
    let di_packet_two = serde_json::to_string(&di_packet_two).unwrap();

    let di_packet_three = packets::DirectionalPacket { x: 0.0, y: 0.0 };

    let di_packet_three = packets::BasePacket::new(
        "directional".to_string(),
        String::new(),
        serde_json::to_string(&di_packet_three).unwrap(),
    );
    let di_packet_three = serde_json::to_string(&di_packet_three).unwrap();

    let socket = bind_to_next_port(3000);

    println!("Attempting to connect to server at: {}", SERVER_ADDR);

    match socket.connect(SERVER_ADDR) {
        Ok(_) => println!("Connected to server."),
        Err(message) => println!("Failed to connect to server. {}", message),
    };

    // --- test area ---

    let socket_clone = socket.try_clone().expect("Failed to clone socket");

    thread::spawn(move || {
        // incoming packets
        loop {
            let mut buffer = vec![0u8; 4096];
            let (data_len, addr) = socket_clone
                .recv_from(&mut buffer)
                .expect("Failed to receive message on UDP socket.");

            if addr.to_string() == SERVER_ADDR {
                let (buffer, _) = &buffer[..].split_at(data_len);
                let message = String::from_utf8_lossy(&buffer).to_string();

                if message == "awk" {
                    println!("Last packet successful.");
                }
            }
        }
    });

    // login
    socket.send(login_packet.as_bytes()).unwrap();
    thread::sleep(Duration::from_millis(3000));

    // walk forward
    socket.send(di_packet_one.as_bytes()).unwrap();
    thread::sleep(Duration::from_millis(3000));

    // walk backwards
    socket.send(di_packet_two.as_bytes()).unwrap();
    thread::sleep(Duration::from_millis(3000));

    // stop walking
    socket.send(di_packet_three.as_bytes()).unwrap();
    thread::sleep(Duration::from_millis(5000));

    // logout
    socket.send(logout_packet.as_bytes()).unwrap();
    thread::sleep(Duration::from_millis(3000));
}

fn bind_to_next_port(port_number: i32) -> UdpSocket {
    let address = format!("{}:{}", MY_ADDR, port_number);
    return match UdpSocket::bind(&address) {
        Ok(socket) => {
            println!("Socket bound to: {}", &address);
            socket
        }
        Err(message) => bind_to_next_port(port_number + 1),
    };
}
