#![deny(invalid_reference_casting)]

use mutification::ToMut;

#[derive(Debug, ToMut)]
struct Player<'a> {
    name: &'a str,
}

#[test]
fn test() {
    let player: Player = Player { name: "nothing" };
    let player: &Player = &player;
    asdf(player);
    assert_eq!(player.name, "Bruce");
}

fn asdf(player: &Player) {
    player.to_mut().name = "Bruce"
}
