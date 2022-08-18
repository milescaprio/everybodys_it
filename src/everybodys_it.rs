#[cfg(debug_assertions)]
const DEBUG : bool = true;
#[cfg(not(debug_assertions))]
const DEBUG : bool = false;

#[derive(Copy, Clone)]
pub struct PlayerAddress {
    addr : isize, //NOT A POINTER, THIS IS AN INDEX
}


impl PlayerAddress {
    pub fn new(addr : isize) -> PlayerAddress {
        PlayerAddress {
            addr : addr,
        }
    }
    pub fn none() -> PlayerAddress {
        PlayerAddress { addr : -1 }
    }
    pub fn index(&self) -> Option<usize> {
        if self.addr >= 0 {
            Some(self.addr as usize)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
struct Player {
    tagged_by   : PlayerAddress,
    tagged      : PlayerAddress,
    tagged_last : PlayerAddress,
}

struct UniquePlayerList {
    list : Vec<PlayerAddress>,
}

impl UniquePlayerList {
    fn new(players : usize) -> UniquePlayerList {
        UniquePlayerList { list : vec![PlayerAddress::none(); players] }
    }
    fn next_from(&self, curr : PlayerAddress) -> PlayerAddress {
        self.list[curr.index().unwrap()]
    }
    fn append(&mut self, last_loc : PlayerAddress, new : PlayerAddress) -> PlayerAddress {
        //assert_eq!(self.list[last_loc.index().unwrap()].index(), None); //item at last_loc is ending //not necessarily we could be overwriting an existing item
        self.list[last_loc.index().unwrap()] = new;
        new
    }
}

pub struct EverybodysIt {
    playerbase : Vec<Player>,
    extra_tagged_list : UniquePlayerList,
}

impl EverybodysIt {
    pub fn new(players : usize) -> EverybodysIt {
        EverybodysIt {
            playerbase : vec![Player {tagged_by : PlayerAddress::none(), tagged : PlayerAddress::none(), tagged_last : PlayerAddress::none()}; players],
            extra_tagged_list : UniquePlayerList::new(players),
        }
    }
    pub fn sitting_info(&self) {
        for i in 0..self.playerbase.len() {
            let player = self.playerbase[i];
            if let Some(tagged_by) = player.tagged_by.index() {
                println!("{} is sitting because of {}", i, tagged_by);
            }
        }
    }
    pub fn tag(&mut self, tagged_by : PlayerAddress, target : PlayerAddress) {
        if DEBUG {
            println!("tagging {} by {}", target.addr, tagged_by.addr);
        }
        match tagged_by.index() {
            Some(index) => {
                if self.playerbase[index].tagged_last.index() == None {
                    self.playerbase[index].tagged = target;
                    self.playerbase[index].tagged_last = target;
                    if DEBUG {
                        println!("{}'s first tag since revive; {}", tagged_by.addr, target.addr);
                    }
                } else {
                    self.playerbase[index].tagged_last = self.extra_tagged_list.append(self.playerbase[index].tagged_last, target);
                    if DEBUG {
                        println!("{} tags another, {}, {} added to UPL", tagged_by.addr, target.addr, self.playerbase[index].tagged_last.addr);
                    }
                }
            },
            None => {
                panic!("Player not found");
            }
        }
        match target.index() {
            Some(index) => {
                self.playerbase[index].tagged_by = tagged_by;
                if DEBUG {
                    println!("{} sits down.", target.addr);
                }
                let mut next = self.playerbase[index].tagged;
                while let Some(next_tagged_index) = next.index() {
                    self.playerbase[next_tagged_index].tagged_by = PlayerAddress::none();
                    self.playerbase[next_tagged_index].tagged = PlayerAddress::none();
                    self.playerbase[next_tagged_index].tagged_last = PlayerAddress::none();
                    next = self.extra_tagged_list.next_from(next);
                    if DEBUG {
                        println!("{} stands back up and resets info", next_tagged_index);
                    }
                }
                self.playerbase[index].tagged = PlayerAddress::none();
                self.playerbase[index].tagged_last = PlayerAddress::none();
            },
            None => {
                panic!("Player not found");
            }
        }
    }
}