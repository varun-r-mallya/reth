//! Implements the Optimism engine API RPC methods.

use alloy_eips::eip7685::Requests;
use alloy_primitives::{BlockHash, B256};
use alloy_rpc_types_engine::{
    ClientVersionV1, ExecutionPayloadBodiesV1, ExecutionPayloadInputV2, ExecutionPayloadV3,
    ForkchoiceState, ForkchoiceUpdated, PayloadId, PayloadStatus,
};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee_core::RpcResult;
use reth_node_api::EngineTypes;

/// Extension trait that gives access to Optimism engine API RPC methods.
///
/// Note:
/// > The provider should use a JWT authentication layer.
///
/// This follows the Optimism specs that can be found at:
/// <https://specs.optimism.io/protocol/exec-engine.html#engine-api>
#[cfg_attr(not(feature = "client"), rpc(server, namespace = "engine"), server_bounds(Engine::PayloadAttributes: jsonrpsee::core::DeserializeOwned))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "engine", client_bounds(Engine::PayloadAttributes: jsonrpsee::core::Serialize + Clone), server_bounds(Engine::PayloadAttributes: jsonrpsee::core::DeserializeOwned)))]
pub trait OpEngineApi<Engine: EngineTypes> {
    /// Sends the given payload to the execution layer client, as specified for the Shanghai fork.
    ///
    /// See also <https://github.com/ethereum/execution-apis/blob/584905270d8ad665718058060267061ecfd79ca5/src/engine/shanghai.md#engine_newpayloadv2>
    ///
    /// No modifications needed for OP compatibility.
    #[method(name = "newPayloadV2")]
    async fn new_payload_v2(&self, payload: ExecutionPayloadInputV2) -> RpcResult<PayloadStatus>;

    /// Sends the given payload to the execution layer client, as specified for the Cancun fork.
    ///
    /// See also <https://github.com/ethereum/execution-apis/blob/main/src/engine/cancun.md#engine_newpayloadv3>
    ///
    /// OP modifications:
    /// - expected versioned hashes MUST be an empty array: therefore the `versioned_hashes`
    ///   parameter is removed.
    /// - parent beacon block root MUST be the parent beacon block root from the L1 origin block of
    ///   the L2 block.
    /// - blob versioned hashes MUST be empty list.
    #[method(name = "newPayloadV3")]
    async fn new_payload_v3(
        &self,
        payload: ExecutionPayloadV3,
        versioned_hashes: Vec<B256>,
        parent_beacon_block_root: B256,
    ) -> RpcResult<PayloadStatus>;

    /// Sends the given payload to the execution layer client, as specified for the Prague fork.
    ///
    /// See also <https://github.com/ethereum/execution-apis/blob/03911ffc053b8b806123f1fc237184b0092a485a/src/engine/prague.md#engine_newpayloadv4>
    ///
    /// - blob versioned hashes MUST be empty list.
    /// - execution layer requests MUST be empty list.
    #[method(name = "newPayloadV4")]
    async fn new_payload_v4(
        &self,
        payload: ExecutionPayloadV3,
        versioned_hashes: Vec<B256>,
        parent_beacon_block_root: B256,
        execution_requests: Requests,
    ) -> RpcResult<PayloadStatus>;

    /// Updates the execution layer client with the given fork choice, as specified for the Shanghai
    /// fork.
    ///
    /// Caution: This should not accept the `parentBeaconBlockRoot` field in the payload attributes.
    ///
    /// See also <https://github.com/ethereum/execution-apis/blob/6709c2a795b707202e93c4f2867fa0bf2640a84f/src/engine/shanghai.md#engine_forkchoiceupdatedv2>
    ///
    /// OP modifications:
    /// - The `payload_attributes` parameter is extended with the [`EngineTypes::PayloadAttributes`](EngineTypes) type as described in <https://specs.optimism.io/protocol/exec-engine.html#extended-payloadattributesv2>
    #[method(name = "forkchoiceUpdatedV2")]
    async fn fork_choice_updated_v2(
        &self,
        fork_choice_state: ForkchoiceState,
        payload_attributes: Option<Engine::PayloadAttributes>,
    ) -> RpcResult<ForkchoiceUpdated>;

    /// Updates the execution layer client with the given fork choice, as specified for the Cancun
    /// fork.
    ///
    /// See also <https://github.com/ethereum/execution-apis/blob/main/src/engine/cancun.md#engine_forkchoiceupdatedv3>
    ///
    /// OP modifications:
    /// - Must be called with an Ecotone payload
    /// - Attributes must contain the parent beacon block root field
    /// - The `payload_attributes` parameter is extended with the [`EngineTypes::PayloadAttributes`](EngineTypes) type as described in <https://specs.optimism.io/protocol/exec-engine.html#extended-payloadattributesv2>
    #[method(name = "forkchoiceUpdatedV3")]
    async fn fork_choice_updated_v3(
        &self,
        fork_choice_state: ForkchoiceState,
        payload_attributes: Option<Engine::PayloadAttributes>,
    ) -> RpcResult<ForkchoiceUpdated>;

    /// Retrieves an execution payload from a previously started build process, as specified for the
    /// Shanghai fork.
    ///
    /// See also <https://github.com/ethereum/execution-apis/blob/6709c2a795b707202e93c4f2867fa0bf2640a84f/src/engine/shanghai.md#engine_getpayloadv2>
    ///
    /// Note:
    /// > Provider software MAY stop the corresponding build process after serving this call.
    ///
    /// No modifications needed for OP compatibility.
    #[method(name = "getPayloadV2")]
    async fn get_payload_v2(
        &self,
        payload_id: PayloadId,
    ) -> RpcResult<Engine::ExecutionPayloadEnvelopeV2>;

    /// Retrieves an execution payload from a previously started build process, as specified for the
    /// Cancun fork.
    ///
    /// See also <https://github.com/ethereum/execution-apis/blob/main/src/engine/cancun.md#engine_getpayloadv3>
    ///
    /// Note:
    /// > Provider software MAY stop the corresponding build process after serving this call.
    ///
    /// OP modifications:
    /// - the response type is extended to [`EngineTypes::ExecutionPayloadEnvelopeV3`].
    #[method(name = "getPayloadV3")]
    async fn get_payload_v3(
        &self,
        payload_id: PayloadId,
    ) -> RpcResult<Engine::ExecutionPayloadEnvelopeV3>;

    /// Returns the most recent version of the payload that is available in the corresponding
    /// payload build process at the time of receiving this call.
    ///
    /// See also <https://github.com/ethereum/execution-apis/blob/main/src/engine/prague.md#engine_getpayloadv4>
    ///
    /// Note:
    /// > Provider software MAY stop the corresponding build process after serving this call.
    ///
    /// OP modifications:
    /// - the response type is extended to [`EngineTypes::ExecutionPayloadEnvelopeV4`].
    #[method(name = "getPayloadV4")]
    async fn get_payload_v4(
        &self,
        payload_id: PayloadId,
    ) -> RpcResult<Engine::ExecutionPayloadEnvelopeV4>;

    /// Returns the execution payload bodies by the given hash.
    ///
    /// See also <https://github.com/ethereum/execution-apis/blob/6452a6b194d7db269bf1dbd087a267251d3cc7f8/src/engine/shanghai.md#engine_getpayloadbodiesbyhashv1>
    #[method(name = "getPayloadBodiesByHashV1")]
    async fn get_payload_bodies_by_hash_v1(
        &self,
        block_hashes: Vec<BlockHash>,
    ) -> RpcResult<ExecutionPayloadBodiesV1>;

    /// Returns the execution payload bodies by the range starting at `start`, containing `count`
    /// blocks.
    ///
    /// WARNING: This method is associated with the BeaconBlocksByRange message in the consensus
    /// layer p2p specification, meaning the input should be treated as untrusted or potentially
    /// adversarial.
    ///
    /// Implementers should take care when acting on the input to this method, specifically
    /// ensuring that the range is limited properly, and that the range boundaries are computed
    /// correctly and without panics.
    ///
    /// See also <https://github.com/ethereum/execution-apis/blob/6452a6b194d7db269bf1dbd087a267251d3cc7f8/src/engine/shanghai.md#engine_getpayloadbodiesbyrangev1>
    #[method(name = "getPayloadBodiesByRangeV1")]
    async fn get_payload_bodies_by_range_v1(
        &self,
        start: u64,
        count: u64,
    ) -> RpcResult<ExecutionPayloadBodiesV1>;

    /// Returns the execution client version information.
    ///
    /// Note:
    /// > The `client_version` parameter identifies the consensus client.
    ///
    /// See also <https://github.com/ethereum/execution-apis/blob/main/src/engine/identification.md#engine_getclientversionv1>
    #[method(name = "getClientVersionV1")]
    async fn get_client_version_v1(
        &self,
        client_version: ClientVersionV1,
    ) -> RpcResult<Vec<ClientVersionV1>>;

    /// Returns the list of Engine API methods supported by the execution layer client software.
    ///
    /// See also <https://github.com/ethereum/execution-apis/blob/6452a6b194d7db269bf1dbd087a267251d3cc7f8/src/engine/common.md#capabilities>
    #[method(name = "exchangeCapabilities")]
    async fn exchange_capabilities(&self, capabilities: Vec<String>) -> RpcResult<Vec<String>>;
}
