# parser-benchmarks
Simple benchmarks for log parsers. Performance in events per second with synthetic logs. 

## Results:
(single-thread)

| Log Source        | Events/second     | Date       |
|-------------------|-------------------|------------|
| OpnSense Firewall | 475239            | 7/2/2021   |
| Sonicwall         | 180727            | 7/2/2021   |
| PaloAlto          | 260596            | 9/2/2021   |

OpnSense is the fastest because the logs are structured in a simple and easy to analyze way. On the contrary, Sonicwall has more modules (IPS, VPN, Tunnels, Firewall...) and they require more complex processing. 