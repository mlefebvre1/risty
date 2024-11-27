use anyhow::Result;
use clap::Parser;
use risty_runtime::RtpSender;
use std::{
    net::UdpSocket,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "0.0.0.0:0")]
    local_addr: String,

    #[arg(short, long)]
    remote_addr: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let udpsock = UdpSocket::bind("0.0.0.0:8080")?;
    let mut send_buf = vec![0u8; 1500];

    let media_payload: Vec<u8> = (0..1300u16).map(|i| i as u8).collect();

    let mut rtp_sender = RtpSender::new()
        .set_payload_type(96)
        .set_clock_frequency(90_000)
        .set_buffer_size(2048)
        .build()?;

    loop {
        rtp_sender.push_to_queue(
            media_payload.clone(),
            SystemTime::now().duration_since(UNIX_EPOCH)?,
            false,
        );

        let n = rtp_sender
            .poll_transmit(&mut send_buf, SystemTime::now().duration_since(UNIX_EPOCH)?)?;
        let _n = udpsock.send_to(&send_buf[0..n], &cli.remote_addr)?;
        std::thread::sleep(Duration::from_millis(500));
    }
}
