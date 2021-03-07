# parser-benchmarks
Simple benchmarks for log parsers. Performance in events per second with synthetic logs. 

## Results:
(single-thread)

| Log Source        | Events/second     | Date       |
|-------------------|-------------------|------------|
| OpnSense Firewall | 474653            | 7/3/2021   |
| Sonicwall         | 197425            | 7/3/2021   |
| PaloAlto          | 290006            | 7/3/2021   |
| DNS Server        | 702740            | 7/3/2021   |
| Squid Proxy       | 452488            | 7/3/2021   |
| Squid Guard       | 335705            | 7/3/2021   |
| Apache2           | 287207            | 7/3/2021   |
| Apache ModSecurity| 90184             | 7/3/2021   |

DNS Server is the fastest because the logs are structured in a simple and easy to analyze way. On the contrary, Sonicwall has more modules (IPS, VPN, Tunnels, Firewall...) and they require more complex processing. 

The Apache ModSecurity is not performing well, I should try to optimize its code.