package keeper

import (
	"context"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	transferkeeper "github.com/cosmos/ibc-go/v4/modules/apps/transfer/keeper"
	ibctransfertypes "github.com/cosmos/ibc-go/v4/modules/apps/transfer/types"
	clienttypes "github.com/cosmos/ibc-go/v4/modules/core/02-client/types"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
)

// AxelarGMPAcc is the address that receives the message from a cosmos chain
const AxelarGMPAcc = "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5s"

type MessageType int

const (
	// TypeUnrecognized means coin type is unrecognized
	TypeUnrecognized = iota
	// TypeGeneralMessage is a pure message
	TypeGeneralMessage
	// TypeGeneralMessageWithToken is a general message with token
	TypeGeneralMessageWithToken
	// TypeSendToken is a direct token transfer
	TypeSendToken
)

// Message is attached in ICS20 packet memo field
type Message struct {
	DestinationChain   string `json:"destination_chain"`
	DestinationAddress string `json:"destination_address"`
	Payload            []byte `json:"payload"`
	Type               int64  `json:"type"`
}

type msgServer struct {
	ibcTransferK transferkeeper.Keeper
}

// NewMsgServerImpl returns an implementation of the MsgServer interface
// for the provided Keeper.
func NewMsgServerImpl(ibcTransferK transferkeeper.Keeper) types.MsgServer {
	return &msgServer{
		ibcTransferK: ibcTransferK,
	}
}

func (k msgServer) MultiSend(goCtx context.Context, msg *types.MsgMultiSend) (*types.MsgMultiSendResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	sender, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		return nil, err
	}

	// build payload that can be decoded by solidity
	addressesType, err := abi.NewType("address[]", "address[]", nil)
	if err != nil {
		return nil, err
	}

	var addresses []common.Address
	for _, receiver := range msg.ReceiverAddresses {
		addresses = append(addresses, common.HexToAddress(receiver))
	}

	payload, err := abi.Arguments{{Type: addressesType}}.Pack(addresses)
	if err != nil {
		return nil, err
	}

	message := Message{
		DestinationChain:   msg.DestinationChain,
		DestinationAddress: msg.DestinationAddress,
		Payload:            payload,
		Type:               TypeGeneralMessageWithToken,
	}

	bz, err := message.Marshal()
	if err != nil {
		return nil, err
	}

	msg := ibctransfertypes.NewMsgTransfer(
		ibctransfertypes.PortID,
		"channel-17", // hard-coded channel id for demo
		msg.Amount,
		msg.Sender,
		AxelarGMPAcc,
		clienttypes.ZeroHeight(),
		uint64(ctx.BlockTime().Add(6*time.Hour).UnixNano()),
	)
	msg.Memo = string(payload)

	res, err := k.ibcTransferK.Transfer(goCtx, msg)
	if err != nil {
		return err
	}

	return &types.MsgMultiSendResponse{}, nil
}
