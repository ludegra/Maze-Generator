pub fn nodes_in_layer(layer: usize, intensity: usize) -> usize {
    intensity.pow(((layer + 1) ) as u32)
}
