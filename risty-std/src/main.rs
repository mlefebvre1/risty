use anyhow::Result;
use clap::Parser;
use risty_core::RtpClock;
use risty_runtime::{RtpConfig, RtpSender};
use std::{net::UdpSocket, time::Duration};

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

    let rtp_clock = RtpClock::new(90000);

    let mut rtp_sender = RtpSender::new(
        RtpConfig {
            rtp_pt: 96,
            peer_sockaddr: cli.remote_addr,
        },
        rtp_clock.now()?,
    );

    loop {
        let transmit =
            rtp_sender.poll_rtp_transmit(&media_payload, rtp_clock.now()?, false, &mut send_buf);
        let _n = udpsock.send_to(&send_buf[0..transmit.buf_size], &transmit.remote_sockaddr)?;
        std::thread::sleep(Duration::from_millis(500));
    }
}
