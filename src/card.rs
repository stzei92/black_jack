#[derive(Clone,Debug)]
pub enum Color {
    Hearts{name:String},
    Spades{name:String},
    Clubs{name:String},
    Diamonds{name:String}
}

impl Color {

    pub fn get_name(&self) -> String {
        match self {
            Color::Hearts {name}
            | Color::Spades {name}
            | Color::Clubs{name}
            | Color::Diamonds{name}=> name.to_string(),
        }
    }
}

#[derive(Clone,Debug)]
pub enum Number {
    Ace{name:String,value:usize},
    Two{name:String,value:usize},
    Three{name:String,value:usize},
    Four{name:String,value:usize},
    Five{name:String,value:usize},
    Six{name:String,value:usize},
    Seven{name:String,value:usize},
    Eight{name:String,value:usize},
    Nine{name:String,value:usize},
    Ten{name:String,value:usize},
    Jack{name:String,value:usize},
    Queen{name:String,value:usize},
    King{name:String,value:usize},
}

impl Number {

        pub fn get_value(&self, hand:usize) -> usize {
        match self {
            Number::Ace {..} => {if hand<=10 {11} else {1}},
            Number::Two {value,..}
            | Number::Three{value,..}
            | Number::Four{value,..}
            | Number::Five{value,..}
            | Number::Six{value,..}
            | Number::Seven{value,..}
            | Number::Eight{value,..}
            | Number::Nine{value,..}
            | Number::Ten{value,..}
            | Number::Jack{value,..}
            | Number::Queen{value,..}
            | Number::King{value,..}=> *value,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Number::Ace {name,..}
            | Number::Two {name,..}
            | Number::Three{name,..}
            | Number::Four{name,..}
            | Number::Five{name,..}
            | Number::Six{name,..}
            | Number::Seven{name,..}
            | Number::Eight{name,..}
            | Number::Nine{name,..}
            | Number::Ten{name,..}
            | Number::Jack{name,..}
            | Number::Queen{name,..}
            | Number::King{name,..}=> name.to_string(),
        }
    }
}

#[derive(Clone,Debug)]
pub struct Card {
    color:Color,
    number:Number,
}

impl Card {
    pub fn new(color:Color, number:Number) -> Self {
        Card {color, number}
    }

    pub fn to_string(&self) -> String {
        format!("{} {}",self.number.get_name(), self.color.get_name())
    }

    pub fn get_value(&self,hand:usize) -> usize {
        self.number.get_value(hand)
    }
}