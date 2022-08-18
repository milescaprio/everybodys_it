

mod everybodys_it;
use everybodys_it::*;

pub fn main() {
    let mut game = EverybodysIt::new(100);
    game.tag(PlayerAddress::new(5), PlayerAddress::new(6)); //player 5 tags 
    game.tag(PlayerAddress::new(5), PlayerAddress::new(7)); //again
    game.tag(PlayerAddress::new(5), PlayerAddress::new(8)); //man is going on a rampage!
    game.tag(PlayerAddress::new(9), PlayerAddress::new(5)); //ooh! 9 comes to stop it. 678 go up
    game.tag(PlayerAddress::new(10), PlayerAddress::new(9)); //and 10 third-parties 9, sending 5 back up
    game.sitting_info();
}