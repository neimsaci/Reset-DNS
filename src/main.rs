fn main() {
    println!("Hello, world!");
    // Following four entries controls DNS server probe will be sent trough.
    let regkeyv4 = Hive::LocalMachine.open(r"SYSTEM\CurrentControlSet\Services\NlaSvc\Parameters\Internet\ActiveProbeContent", Security::Read)?;
    let regkeyv6 = Hive::LocalMachine.open(r"SYSTEM\CurrentControlSet\Services\NlaSvc\Parameters\Internet\ActiveProbeContentV6", Security::Read)?;
    let regkeyv4_ip = "127.0.0.1"; // Ensure that you have dnscrypt proxy installed and running.
    let regkeyv6_ip = "::1"; // Same as above.
        
    // Following four entries controls DNS server / Host that will receive probe.
    let regkeyv4_active_dns_probe_host = Hive::LocalMachine.open(r"SYSTEM\CurrentControlSet\Services\NlaSvc\Parameters\Internet\ActiveProbeContent", Security::Read)?;
    let regkeyv6_active_dns_probe_host = Hive::LocalMachine.open(r"SYSTEM\CurrentControlSet\Services\NlaSvc\Parameters\Internet\ActiveProbeContentV6", Security::Read)?;
    let regkeyv4_probe_url = "odvr.nic.cz"; // I will use here ordinary DNS.
    let regkeyv6_probe_url = "odvr.nic.cz"; // Same as above.
    
    // Following four entries controls Host where testfile 'connecttest.txt' will be downloaded from.
    let regkeyv4_active_web_probe_host = Hive::LocalMachine.open(r"SYSTEM\CurrentControlSet\Services\NlaSvc\Parameters\Internet\ActiveProbeContent", Security::Read)?;
    let regkeyv6_active_web_probe_host = Hive::LocalMachine.open(r"SYSTEM\CurrentControlSet\Services\NlaSvc\Parameters\Internet\ActiveProbeContentV6", Security::Read)?;
    let regkeyv4_web_probe_host = "www.github.com/neimsaci/neimsaci.github.io/" // Repo where 'connecttest.txt' is located.
    let regkeyv6_web_probe_host = "www.github.com/neimsaci/neimsaci.github.io/" // Same as above.
}
