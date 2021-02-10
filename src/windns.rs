use usiem_windns::parsers;
use usiem::events::SiemLog;
use usiem::events::field::SiemIp;
use std::time::Instant;

pub fn benchmark_windns(n_logs : usize) -> u128 {
    let start_time = Instant::now();
    for _i in 0..n_logs{
        let log = "6/5/2013 10:00:32 AM 0E70 PACKET  00000000033397A0 UDP Rcv 10.161.60.71    5b47   Q [0001   D   NOERROR] A      (12)somecomputer(6)domain(3)com(0)";
        let log = SiemLog::new(log.to_string(), 0, SiemIp::V4(0));
        let _siem_log = parsers::parse_log(log);
    }
    start_time.elapsed().as_millis()
    
}
