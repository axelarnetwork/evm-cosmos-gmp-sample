# Sample GMP Middleware
The sample middleware provides a template for creating a middleware that can be integrated with GMP (General Message Protocol).

## The middleware
This simple middleware implementation is initialized with a handler that conforms to the gmp_middleware.Handler interface. In the OnRecvPacket method, it checks the original sender, attempts to unmarshal the message, and handles the message based on its type. The handler logs the source chain, source address, and payload.

## Init and add to the app
The GMP middleware is positioned between the router and the transfer module. It is initialized in the app.
```
var ibcStack porttypes.IBCModule
ibcStack = transfer.NewIBCModule(appKeepers.TransferKeeper)

# Add gmp middleware to the stack
ibcStack = gmp_middleware.NewIBCMiddleware(
    ibcStack,
    gmp_middleware.NewDummyHandler(appKeepers.BankKeeper),
)

ibcStack = router.NewIBCMiddleware(
    ibcStack,
    appKeepers.RouterKeeper,
    0,
    routerkeeper.DefaultForwardTransferPacketTimeoutTimestamp,
    routerkeeper.DefaultRefundTransferPacketTimeoutTimestamp,
)
```

This configuration ensures that the GMP middleware is integrated into the application stack, allowing it to process and handle messages as needed.