use anyhow::Result;
use clap::Parser;
use rand::Rng;
use risty_runtime::{RtpConfig, RtpSender};
use std::{
    net::UdpSocket,
    time::{Duration, UNIX_EPOCH},
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

    let mut media_payload = vec![0u8; 1300];

    let mut rtp_sender = RtpSender::new(
        RtpConfig {
            rtp_pt: 96,
            peer_sockaddr: cli.remote_addr,
        },
        get_rtp_time(),
    );

    let mut rng = rand::thread_rng();

    loop {
        media_payload.iter_mut().for_each(|v| *v = rng.gen());
        let transmit =
            rtp_sender.poll_rtp_transmit(&media_payload, get_rtp_time(), false, &mut send_buf);
        let _n = udpsock.send_to(&send_buf[0..transmit.buf_size], &transmit.remote_sockaddr)?;
        std::thread::sleep(Duration::from_micros(500));
    }
}

fn get_rtp_time() -> u32 {
    let ntp_time = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    ntp_to_rtp_time(ntp_time)
}

fn ntp_to_rtp_time(ntp_time: Duration) -> u32 {
    let seconds = ntp_time.as_secs().wrapping_mul(9000u64) as u32;
    let fract = 9000_u32.wrapping_mul(1_000_000_000_u32 / ntp_time.as_nanos() as u32);

    seconds + fract
}

// 1 / 90000 -> 0.0000011
//
