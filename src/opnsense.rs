use std::time::Instant;
use usiem::events::SiemLog;
use usiem::events::field::SiemIp;
use usiem_opnsense::parsers::firewall;

pub fn benchmark_opnsense(n_logs : usize) -> u128 {
    let log = "<134>Aug 23 20:30:25 OPNsense.localdomain filterlog[21853]: 82,,,0,igb0,match,pass,out,4,0x0,,62,25678,0,DF,17,udp,60,192.168.1.8,8.8.8.8,5074,53,40";
    let start_time = Instant::now();
    for _i in 0..n_logs {
        let log_event = SiemLog::new(log.to_owned(), 1000000 as i64, SiemIp::V4(0));
        let _logline = firewall::parse_log(log_event).unwrap();
    }
    start_time.elapsed().as_millis()

}