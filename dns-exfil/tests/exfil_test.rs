mod common;

use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

use dns_exfil::{start_server, stop_server};
use trust_dns_resolver::{
    self,
    config::{NameServerConfig, NameServerConfigGroup},
};

// use dns_exfil::start_server;

#[test]
fn test() {
    // create own runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    let serv = rt.block_on(start_server("0.0.0.0", "8888", "disna-m.top"));
    // use a resolver that will only use 127.0.0.1 as a dns server, at port 8888
    let target_dns_server = NameServerConfigGroup::from_ips_clear(
        &[IpAddr::from_str("127.0.0.1").unwrap()],
        8888,
        true,
    );
    let resolver = trust_dns_resolver::Resolver::new(
        trust_dns_resolver::config::ResolverConfig::from_parts(None, vec![], target_dns_server),
        trust_dns_resolver::config::ResolverOpts::default(),
    )
    .unwrap();
    let response = resolver
        .lookup_ip("disna-m.top")
        .unwrap();
    println!("response: {:?}", response);
    stop_server(serv);
    rt.shutdown_background();
}
