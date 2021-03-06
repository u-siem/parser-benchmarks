use usiem_paloalto::parsers;
use usiem::events::SiemLog;
use usiem::events::field::SiemIp;
use std::time::Instant;

pub fn benchmark_paloalto(n_logs : usize) -> u128 {
    let start_time = Instant::now();
    for _i in 0..n_logs{
        let log = "Feb  5 15:05:55 pa-test 1,2021/02/05 15:05:55,0123456789,TRAFFIC,drop,2305,2021/02/05 15:05:55,192.168.2.1,192.168.3.2,0.0.0.0,0.0.0.0,intrazone-default,,,not-applicable,vsys1,untrust,untrust,ethernet1,,Log Collector,2021/02/05 15:05:55,0,1,35861,514,0,0,0x0,udp,deny,102,102,0,1,2021/02/08 16:06:59,0,any,0,3996658540,0x0,192.168.0.0-192.168.255.255,192.168.0.0-192.168.255.255,0,1,0,policy-deny,0,0,0,0,,pa-test,from-policy,,,0,,0,,N/A,0,0,0,0,aa1d1908-68cd-467a-bc90-68a57096ef3b,0,0,,,,,,,";
        let log = SiemLog::new(log.to_string(), 0, SiemIp::V4(0));
        let _siem_log = parsers::parse_log(log);
    }
    start_time.elapsed().as_millis()
    
}