use mutification::ToMut;

#[derive(Debug, ToMut)]
struct Player<'a> {
    name: &'a str,
}

#[test]
fn test() {
    let player: Player = Player { name: "nothing" };
    let player_ref: &Player = &player;
    asdf(player_ref);
    assert_eq!(player.name, "Bruce");
}

fn asdf(player: &Player) {
    player.to_mut().name = "Bruce";
}
