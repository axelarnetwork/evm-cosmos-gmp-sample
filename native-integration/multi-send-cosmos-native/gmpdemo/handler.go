package gmpdemo

import (
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/cosmos/gaia/v7/x/gmpdemo/keeper"
	"github.com/cosmos/gaia/v7/x/gmpdemo/types"
	transferkeeper "github.com/cosmos/ibc-go/v3/modules/apps/transfer/keeper"
)

// NewHandler ...
func NewHandler(ibctransferK transferkeeper.Keeper) sdk.Handler {
	// this line is used by starport scaffolding # handler/msgServer
	msgServer := keeper.NewMsgServerImpl(ibctransferK)

	return func(ctx sdk.Context, msg sdk.Msg) (*sdk.Result, error) {
		ctx = ctx.WithEventManager(sdk.NewEventManager())

		switch msg := msg.(type) {
		case *types.MsgMultiSend:
			res, err := msgServer.MultiSend(sdk.WrapSDKContext(ctx), msg)
			return sdk.WrapServiceResult(ctx, res, err)
		default:
			errMsg := fmt.Sprintf("unrecognized %s message type: %T", types.ModuleName, msg)
			return nil, sdkerrors.Wrap(sdkerrors.ErrUnknownRequest, errMsg)
		}
	}
}
