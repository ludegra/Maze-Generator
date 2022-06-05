macro_rules! connect_node {
    ($node:ident, $condition:expr, $direction:expr, $index:expr) => {
        if $condition {
            $node.connect(Connection {
                direction: $direction,
                index: $index,
                active: true,
            });
        }
    };
}
