package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

var _ sdk.Msg = &MsgMultiSend{}

func NewMsgMultiSend(sender, destChain, destAddr string, amount sdk.Coin, receivers []string) *MsgMultiSend {
	return &MsgMultiSend{
		Sender:             sender,
		DestinationChain:   destChain,
		DestinationAddress: destAddr,
		Amount:             amount,
		ReceiverAddresses:  receivers,
	}
}

func (msg *MsgMultiSend) Route() string {
	return RouterKey
}

func (msg *MsgMultiSend) Type() string {
	return "MultiSend"
}

func (msg *MsgMultiSend) GetSigners() []sdk.AccAddress {
	sender, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{sender}
}

func (msg *MsgMultiSend) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgMultiSend) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid sender address (%s)", err)
	}
	return nil
}
