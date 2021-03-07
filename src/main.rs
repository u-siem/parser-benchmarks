mod opnsense;
mod sonicwall;
mod paloalto;
mod windns;
mod squid;

fn main() {
    println!("Starting Benchmark:");
    println!("---------------------------------------------------------------");
    let n_events = 5_000_000;
    let elapsed_opnsense = opnsense::benchmark_opnsense(n_events);
    println!("OPNSense:\t{} EPS",(n_events * 1000) as u128 / elapsed_opnsense);
    println!("---------------------------------------------------------------");
    let elapsed_sonicwall = sonicwall::benchmark_sonicwall(n_events);
    println!("SonicWall:\t{} EPS",(n_events * 1000) as u128 / elapsed_sonicwall);
    println!("---------------------------------------------------------------");
    let elapsed_paloalto = paloalto::benchmark_paloalto(n_events);
    println!("PaloAlto:\t{} EPS",(n_events * 1000) as u128 / elapsed_paloalto);
    println!("---------------------------------------------------------------");
    let elapsed_windns = windns::benchmark_windns(n_events);
    println!("Windows DNS Server:\t{} EPS",(n_events * 1000) as u128 / elapsed_windns);
    println!("---------------------------------------------------------------");
    let elapsed_squid = squid::benchmark_squid(n_events);
    println!("Squid Proxy:\t{} EPS",(n_events * 1000) as u128 / elapsed_squid);
    println!("---------------------------------------------------------------");
    let elapsed_squidguard = squid::benchmark_squidguard(n_events);
    println!("SquidGuard:\t{} EPS",(n_events * 1000) as u128 / elapsed_squidguard);
    println!("---------------------------------------------------------------");
}


