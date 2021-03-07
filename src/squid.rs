use usiem_squid::squid;
use usiem_squid::squidguard;
use usiem::events::SiemLog;
use usiem::events::field::SiemIp;
use std::time::Instant;

pub fn benchmark_squid(n_logs : usize) -> u128 {
    let start_time = Instant::now();
    for _i in 0..n_logs{
        let log = "1613260836.628    287 172.17.0.1 TCP_TUNNEL_ABORTED/200 18353 CONNECT www.google.com:443 - HIER_DIRECT/142.250.184.4 -";
        let log = SiemLog::new(log.to_string(), 0, SiemIp::V4(0));
        let _siem_log = squid::parse_log(log);
    }
    start_time.elapsed().as_millis()
}

pub fn benchmark_squidguard(n_logs : usize) -> u128 {
    let start_time = Instant::now();
    for _i in 0..n_logs{
        let log = "2021-02-14 00:02:33 [26] Request(default/porn/-) pornpage.com:443 172.17.0.1/172.17.0.1 - CONNECT REDIRECT";
        let log = SiemLog::new(log.to_string(), 0, SiemIp::V4(0));
        let _siem_log = squidguard::parse_log(log);
    }
    start_time.elapsed().as_millis()
}