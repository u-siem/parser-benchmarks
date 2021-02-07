use usiem_sonicwall::parsers;
use std::time::Instant;
use usiem::events::SiemLog;
use usiem::events::field::SiemIp;

pub fn benchmark_sonicwall(n_logs : usize) -> u128 {
    let start_time = Instant::now();
    for _i in 0..n_logs{
        let log = "<12>Feb 5 01:02:03 FWSonicWall 10.1.99.1 id=firewall sn=SERIALNUMBER111 time=\"2021-02-05 01:02:03 UTC\" fw=111.222.111.222 pri=6 c=1024 m=1153 msg=\"SSL VPN Traffic\" sess=\"sslvpnc\" n=1234567890 usr=\"test@usiem.com\" src=10.1.2.3:3080:X6-V80 dst=10.2.3.4:50005:X1 srcMac=9d:88:a1:7c:af:1a dstMac=5c:61:a0:81:cc:f1 proto=tcp/50005 rcvd=392 rule=\"123 (SSLVPN->NET_RRHH1)\"";
        let log = SiemLog::new(log.to_string(), 0, SiemIp::V4(0));
        let _siem_log = parsers::parse_log(log);
    }
    start_time.elapsed().as_millis()
}