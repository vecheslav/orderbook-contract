use crate::commands::batch::{
    deploy_all::DeployAllCommand, deploy_proxy::DeployProxyCommand,
    deploy_teth_tusdc_proxy::DeployTethTusdcProxyCommand,
};
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum BatchCommands {
    /// Batch Deploy a new market contracts and setup them
    #[clap(short_flag = 'A')]
    DeployAll(DeployAllCommand),

    /// Deploy a new market and proxy contracts and setup them
    #[clap(short_flag = 'P')]
    DeployProxy(DeployProxyCommand),

    /// Deploy a new teth/tusdc market and proxy contracts and setup them
    #[clap(short_flag = 'T')]
    DeployTethTusdcProxy(DeployTethTusdcProxyCommand),
}
