### Example instantiation message
``` json
{"channel": "channel-1946"}
```

### Multi Send
distributes equal amount to recipients
``` json
{"multi_send": {"recipients": ["osmo139a8plum50nhyqvu42papdf6xa9s3nfqdn5lx3","osmo1kux208ex604jh4l6js4sap4nuygqw6eakzu9ye"]}}
```

### Call multi send contract on avalanche fuji testnet
sends a cross chain message, transfers Axelar suppoted token, and distributes equal amount to recipients
``` json
{"multi_send_to_evm": {"destination_chain": "avalanche", "destination_address": "0xE2cd00e8BBf48AdFb7DF0B00F55260f17127D445", "recipients": ["0x68B93045fe7D8794a7cAF327e7f855CD6Cd03BB8","0xB8Cd93C83A974649D76B1c19f311f639e62272BC"]}, "fee": {"amount": "1", "recipient": "axelar1jqqar3h9q62h2ed09wc8sxfp88ke8mulsvh8r3"}}
```
