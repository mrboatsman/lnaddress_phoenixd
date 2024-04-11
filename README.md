# Lightning Address for Phoenixd
By running this you are aware that you can lose all your funds and this does not come with any warranty.

This enable Lightning Address with `Phoenix for Server` from ACINQ. This application is
intended to run behind a proxy which can handle TLS/SSL for example Caddy or nginx.

This application does not have any association with ACINQ as a organisation. 

- [phoenixd from Acinq](https://phoenix.acinq.co/server)
- [The lightning Address](https://lightningaddress.com/)

## Command Line Arguments
Every argument can be configure from enviroment variables
 ```                                                                                                                                                                                                                        │
   -l, --listen <IP>
          IPv4 or IPv6 Hostname for the lnurl server shall listen on [env: HOST=] [default: 127.0.0.1]
  -p, --port <PORT>
          Port number for the lnurl server shall listen on [env: PORT=] [default: 3000]
      --domain <Domain>
          Domain name which the server responds to, by default this is auto resolved [env: DOMAIN=]
  -a, --accepted-username [<USERNAMES>...]
          Usernames seperated with space which shall accept payments [env: USERNAMES=] [default: *]
      --phoenixd-config <FILE>
          Sets a custom path for phoenixd config file, default is $HOME/.phoenix/phoenix.conf [env: PHOENIXD_CONFIG=]
      --phoenixd-url <URL>
          Sets a hostname to phoenixd [env: PHOENIXD_URL=] [default: http://127.0.0.1]
      --phoenixd-port <PORT>
          Sets a port to phoenixd [env: PHOENIXD_PORT=] [default: 9740]
      --phoenixd-username <username>
          Configure an username to access phoenixd API [env: PHOENIXD_USERNAME=]
      --phoenixd-password <password>
          Configure a password to access phoenixd API, default is 9740 [env: PHOENIXD_PASSWORD=]
      --lnurl-payment-identify <STRING>
          Sets an identifying name  which is displayed when paying [env: LNURL_PAYMENT_IDENTIFY=] [default: Satoshi]
      --lnurl-payment-description <STRING>
          Sets a message which is displayed when paying [env: LNURL_PAYMENT_DESCRIPTION=] [default: "Hello World"]
      --lnurl-allow-note <lenght>
          Allow payee to add a comment together with the payment [env: LNURL_ALLOW_NOTE=] [default: 0]
      --lnurl-greeting <STRING>
          leave a greating when the payment is done [env: LNURL_GREETING=] [default: ]
      --lnurl-minimum-sendable-milisats <lenght>
          minimum ammount in milisatoshi to send [env: LNURL_MINIMUM_SENDABLE_MILISATS=] [default: 1000]
      --lnurl-maximum-sendable-milisats <lenght>
          maximym ammount in milisatoshi to send [env: LNURL_MAXIMUM_SENDABLE_MILISATS=] [default: 2100000000]
      --debug
          Turn debugging information on
  -h, --help
          Print help
  -V, --version
          Print version
```

Most of the cases there is no need for setting --domain-name but in some cases
there might be needed depending on the deployment environment

The Hostname is resolved through the following, in order:
1. Forwarded header
2. X-Forwarded-Host header
3. Host header
4. request target / URI


## Example

following command will do this
- Listen on all IP numbers on the host 
- listen on port 3000
- load $HOME/.phoenix/phoenix.conf for api username and password
- Receive payments on all usernames
- Connect to phoenixd api on 127.0.0.1 (Localhost) and port 9740
- leave a greeting too payee on successful payment
- Identify as "Satoshi" in payee wallets and with description "Hello World"
- Minimum amount 1 sats and maximum amount 2100000 sats
````bash
./lnaddress_phoenixd -l 0.0.0.0 --lnurl-greeting "Thank you"
````

