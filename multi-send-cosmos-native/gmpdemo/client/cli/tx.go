package cli

import (
	"fmt"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/spf13/cobra"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/gaia/v7/x/gmpdemo/types"
)

// GetTxCmd returns the transaction commands for this module
func GetTxCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:                        types.ModuleName,
		Short:                      fmt.Sprintf("%s transactions subcommands", types.ModuleName),
		DisableFlagParsing:         true,
		SuggestionsMinimumDistance: 2,
		RunE:                       client.ValidateCmd,
	}

	cmd.AddCommand(CmdMultiSend())

	return cmd
}

func CmdMultiSend() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "multi-send [dest-chain] [dest-addr] [coin] [receiver]...",
		Short: "",
		Args:  cobra.MinimumNArgs(4),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			coin, err := sdk.ParseCoinNormalized(args[2])
			if err != nil {
				return err
			}

			// native assets are optional
			receivers := make([]string, len(args[3:]))
			for i, addr := range args[3:] {
				receivers[i] = addr
			}

			msg := types.NewMsgMultiSend(
				clientCtx.GetFromAddress().String(),
				args[0],
				args[1],
				coin,
				receivers,
			)
			if err := msg.ValidateBasic(); err != nil {
				return err
			}
			return tx.GenerateOrBroadcastTxCLI(clientCtx, cmd.Flags(), msg)
		},
	}

	flags.AddTxFlagsToCmd(cmd)

	return cmd
}
