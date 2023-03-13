package gmpdemo

import sdk "github.com/cosmos/cosmos-sdk/types"

type GeneralMessageHandler interface {
	HandleGeneralMessage(ctx sdk.Context, srcChain, srcAddress string, payload []byte, receiver string) error
	HandleGeneralMessageWithToken(ctx sdk.Context, srcChain, srcAddress string, payload []byte, receiver string, coin sdk.Coin) error
}
