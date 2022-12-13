package keeper

import (
	"context"
	"github.com/status-im/keycard-go/hexutils"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	transferkeeper "github.com/cosmos/ibc-go/v3/modules/apps/transfer/keeper"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"

	"github.com/cosmos/gaia/v7/x/gmpdemo/types"
)

// hard code the channel
const sourceChannel = "channel-17"

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

	ibcTransfer := ibctransfertypes.NewMsgTransfer(
		ibctransfertypes.PortID,
		sourceChannel,
		msg.Amount,
		sender.String(),
		types.AxelarModuleAcc,
		clienttypes.ZeroHeight(),
		uint64(ctx.BlockTime().Add(6*time.Hour).UnixNano()),
	)

	// build payload
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

	metadata := types.Metadata{
		Sender:      msg.Sender,
		DestChain:   msg.DestinationChain,
		DestAddress: msg.DestinationAddress,
		Payload:     payload,
		Type:        types.GeneralMsgWithToken,
	}
	bz, err := metadata.Marshal()
	if err != nil {
		return nil, err
	}

	ibcTransfer.Memo = hexutils.BytesToHex(bz)

	_, err = k.ibcTransferK.Transfer(goCtx, ibcTransfer)
	if err != nil {
		return nil, err
	}

	return &types.MsgMultiSendResponse{}, nil
}
