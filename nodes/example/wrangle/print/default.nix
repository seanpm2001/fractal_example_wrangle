{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ NtupQuadrupleU32u32u32f32 ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
