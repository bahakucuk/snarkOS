// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use snarkvm::dpc::Network;

use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum NodeType {
    /// A client node is a full node, capable of sending and receiving blocks.
    Client = 0,
    /// A mining node is a full node, capable of producing new blocks.
    Miner,
    /// A peer node is a discovery node, capable of sharing peers of the network.
    Peer,
    /// A sync node is a discovery node, capable of syncing nodes for the network.
    Sync,
}

#[rustfmt::skip]
pub trait Environment: 'static + Clone + Debug + Default + Send + Sync {
    type Network: Network;
    /// The specified type of node.
    const NODE_TYPE: NodeType;
    /// The version of the network protocol; it can be incremented in order to force users to update.
    const MESSAGE_VERSION: u32 = 5;

    /// If `true`, a mining node will craft public coinbase transactions.
    const COINBASE_IS_PUBLIC: bool = false;
    /// If `true`, a node will remote fetch blocks from genesis.
    const FAST_SYNC: bool = true;

    /// The port for communicating with the node server.
    const DEFAULT_NODE_PORT: u16 = 4130 + Self::Network::NETWORK_ID;
    /// The port for communicating with the RPC server.
    const DEFAULT_RPC_PORT: u16 = 3030 + Self::Network::NETWORK_ID;

    /// The list of peer nodes to bootstrap the node server with.
    const PEER_NODES: [&'static str; 0] = [];
    /// The list of sync nodes to bootstrap the node server with.
    const SYNC_NODES: [&'static str; 2] = ["127.0.0.1:4132", "127.0.0.1:4135"];

    /// The duration in seconds to sleep in between heartbeat executions.
    const HEARTBEAT_IN_SECS: u64 = 10;
    /// The maximum duration in seconds permitted for establishing a connection with a node,
    /// before dropping the connection; it should be no greater than the `HEARTBEAT_IN_SECS`.
    const CONNECTION_TIMEOUT_IN_SECS: u64 = 3;
    /// The duration in seconds to sleep in between ping requests with a connected peer.
    const PING_SLEEP_IN_SECS: u64 = 12;
    /// The duration in seconds after which a connected peer is considered inactive or
    /// disconnected if no message has been received in the meantime.
    const RADIO_SILENCE_IN_SECS: u64 = 120; // 2 minutes
    /// The duration in seconds after which to expire a failure from a peer.
    const FAILURE_EXPIRY_TIME_IN_SECS: u64 = 7200; // 2 hours

    /// The minimum number of peers required to maintain connections with.
    const MINIMUM_NUMBER_OF_PEERS: usize;
    /// The maximum number of peers permitted to maintain connections with.
    const MAXIMUM_NUMBER_OF_PEERS: usize = 21;
    /// The maximum number of connection failures permitted by an inbound connecting peer.
    const MAXIMUM_CONNECTION_FAILURES: u32 = 5;
    /// The maximum number of candidate peers permitted to be stored in the node.
    const MAXIMUM_CANDIDATE_PEERS: usize = 10_000;

    /// The maximum size of a message that can be transmitted in the network.
    const MAXIMUM_MESSAGE_SIZE: usize = 128 * 1024 * 1024; // 128 MiB
    /// The maximum number of blocks that may be fetched in one request.
    const MAXIMUM_BLOCK_REQUEST: u32 = 100;
    /// The maximum number of failures tolerated before disconnecting from a peer.
    const MAXIMUM_NUMBER_OF_FAILURES: usize = 2400;
}

#[derive(Clone, Debug, Default)]
pub struct Client<N: Network>(PhantomData<N>);

#[rustfmt::skip]
impl<N: Network> Environment for Client<N> {
    type Network = N;
    const NODE_TYPE: NodeType = NodeType::Client;
    const MINIMUM_NUMBER_OF_PEERS: usize = 2;
}

#[derive(Clone, Debug, Default)]
pub struct Miner<N: Network>(PhantomData<N>);

#[rustfmt::skip]
impl<N: Network> Environment for Miner<N> {
    type Network = N;
    const NODE_TYPE: NodeType = NodeType::Miner;
    const COINBASE_IS_PUBLIC: bool = true;
    const MINIMUM_NUMBER_OF_PEERS: usize = 1;
}

#[derive(Clone, Debug, Default)]
pub struct SyncNode<N: Network>(PhantomData<N>);

#[rustfmt::skip]
impl<N: Network> Environment for SyncNode<N> {
    type Network = N;
    const NODE_TYPE: NodeType = NodeType::Sync;
    const MINIMUM_NUMBER_OF_PEERS: usize = 5;
    const MAXIMUM_NUMBER_OF_PEERS: usize = 1024;
}

#[derive(Clone, Debug, Default)]
pub struct ClientTrial<N: Network>(PhantomData<N>);

#[rustfmt::skip]
impl<N: Network> Environment for ClientTrial<N> {
    type Network = N;
    const NODE_TYPE: NodeType = NodeType::Client;
    const SYNC_NODES: [&'static str; 2] = ["144.126.219.193:4132", "165.232.145.194:4132"];
    const MINIMUM_NUMBER_OF_PEERS: usize = 5;
}

#[derive(Clone, Debug, Default)]
pub struct MinerTrial<N: Network>(PhantomData<N>);

#[rustfmt::skip]
impl<N: Network> Environment for MinerTrial<N> {
    type Network = N;
    const NODE_TYPE: NodeType = NodeType::Miner;
    const SYNC_NODES: [&'static str; 2] = ["144.126.219.193:4132", "165.232.145.194:4132"];
    const MINIMUM_NUMBER_OF_PEERS: usize = 5;
    const COINBASE_IS_PUBLIC: bool = true;
}
