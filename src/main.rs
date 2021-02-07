mod opnsense;
mod sonicwall;

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
}


