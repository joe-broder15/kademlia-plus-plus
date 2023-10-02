use crate::routing::NodeId;
enum rpc_type {
    FIND_NODE,
    FIND_VALUE,
    STORE,
    PING,
}

struct rpc_header {
    sender_id: NodeId,
    rpc_type: rpc_type,
}
