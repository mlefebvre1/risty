// pub struct RtcpConfig {
//     // RTCP Config.
//     /// The sender may choose any arbitrary source port M for the RTP flow
//     /// RIST senders may offer the user the ability to manually configure source ports M
//     rtcp_listener_port: u16,
// }

// pub struct RtcpSender {
//     config: Config,
// }

// impl RtcpSender {
//     pub fn new(config: Config) -> Self {}

//     /// RIST senders shall periodically transmit the compound RTCP packets specified in section
//     /// 5.2.1 to the configured IP address of the RIST receiver and UDP port P+1
//     pub fn rtcp_receiver_port(&self) -> u16 {
//         self.rtcp_listener_port + 1
//     }

//     /// this function shall be called when receiving a packet on the rtcp socket
//     pub fn handle_rtcp_input(&mut self, packet: &[u8]) {}

//     /// this function shall be called to send a rtcp packet to the receiver
//     pub fn poll_rtcp_transmit(&mut self) {}
// }
