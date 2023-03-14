package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/ethereum/go-ethereum/accounts/abi"
)

type BankK interface {
	SendCoins(ctx sdk.Context, fromAddr sdk.AccAddress, toAddr sdk.AccAddress, amt sdk.Coins) error
}

type MultiSendHandler struct {
	bank BankK
}

func NewMultiSendHandler(k BankK) *MultiSendHandler {
	return &MultiSendHandler{
		bank: k,
	}

}

func (h MultiSendHandler) HandleGeneralMessage(ctx sdk.Context, srcChain, srcAddress string, payload []byte) error {
	return nil
}

func (h MultiSendHandler) HandleGeneralMessageWithToken(ctx sdk.Context, srcChain, srcAddress string, payload []byte, receiver string, coin sdk.Coin) error {
	coinHolder, err := sdk.AccAddressFromBech32(receiver)
	if err != nil {
		return err
	}

	// decode payload
	addressesType, err := abi.NewType("string[]", "string[]", nil)
	if err != nil {
		return err
	}

	args, err := abi.Arguments{{Type: addressesType}}.Unpack(payload)
	if err != nil {
		return err
	}
	addresses := args[0].([]string)

	amt := coin.Amount.Quo(sdk.NewInt(int64(len(addresses))))
	c := sdk.NewCoin(coin.GetDenom(), amt)

	for _, addr := range addresses {
		accAddr, err := sdk.AccAddressFromBech32(addr)
		if err != nil {
			return err
		}

		if err = h.bank.SendCoins(ctx, coinHolder, accAddr, sdk.NewCoins(c)); err != nil {
			return err
		}
	}

	return nil
}
