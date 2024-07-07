use std::hint::black_box;

use to_mut::ToMut;

#[derive(Debug, ToMut)]
struct Player {}

#[test]
fn test() {
    let player = Player {};
    let player_ref: &Player = &player;
    let player_mut: &mut Player = player_ref.to_mut();
    println!("{:?}", player_mut);
    black_box(player);
}
