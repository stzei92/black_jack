pub mod card;

use card::Card;
use card::Color;
use card::Number;
use rand::{rng,seq::SliceRandom};
use std::io;

//Use this function to create a new deck (52 Cards). Create as many new Decks as you want.
fn create_deck() -> Vec<Card>
{
    //Define the colors and their readable substrings.
    //Readable Strings will be stitched together later.
    let colors = [
        Color::Hearts{name:String::from("of Hearts")},
        Color::Spades{name:String::from("of Spades")},
        Color::Clubs{name:String::from("of Clubs")},
        Color::Diamonds{name:String::from("of Diamonds")},
        ];

    //Define card numbers (Values) together with their nominal values (Ace=1 default) as well as their readable substrings
    //Readable Strings will be stitched together later. The special case for Ace is handled in card.rs
    let numbers = [
        Number::Ace{name:String::from("Ace"),value:1},
        Number::Two{name:String::from("Two"),value:2},
        Number::Three{name:String::from("Three"),value:3},
        Number::Four{name:String::from("Four"),value:4},
        Number::Five{name:String::from("Five"),value:5},
        Number::Six{name:String::from("Six"),value:6},
        Number::Seven{name:String::from("Seven"),value:7},
        Number::Eight{name:String::from("Eight"),value:8},
        Number::Nine{name:String::from("Nine"),value:9},
        Number::Ten{name:String::from("Ten"),value:10},
        Number::Jack{name:String::from("Jack"),value:10},
        Number::Queen{name:String::from("Queen"),value:10}, 
        Number::King{name:String::from("King"),value:10},
    ];

    let mut deck: Vec<Card> = vec![];

    for c in &colors {
        for n in &numbers {
            deck.push(Card::new(c.clone(),n.clone()));
        }
    }
    deck
}

fn hand_to_string(hand: &Vec<Card>) -> String{
    let mut buffer = String::new();
    for card in hand {
        buffer.push_str(&card.to_string());
        buffer.push_str(", ")
    }
    buffer.split_off(buffer.len()-2);
    buffer
}

//Deal one Card from Deck into Hand
fn deal(hand: &mut Vec<Card>, deck: &mut Vec<Card>) -> Card {
    let mut dealed_card = deck.split_off(deck.len()-1);
    let card:Card = dealed_card.get(0).expect("Oops!").clone();
    hand.append(&mut dealed_card);
    card
}

fn shuffle(deck: &mut Vec<Card>) {
    let mut rng = rng();
    deck.shuffle(&mut rng);
}

fn count_values(hand:&Vec<Card>) ->usize {
    let mut current_value:usize = 0;
    for card in hand {
        current_value += card.get_value(current_value);
    }
    current_value
}

fn game_loop() {

    let mut deck = create_deck();
    shuffle(&mut deck);
    prompt(String::from("The deck has been shuffled. Game Start. Press Enter to move forward!."));

    //Create the Dealers Hand
    let mut hand_dealer:Vec<Card> = vec![];

    //Create the Players Hand
    let mut hand_player:Vec<Card> = vec![];

    //Dealer Draws first card
    let mut card = deal(&mut hand_dealer, &mut deck);
    prompt(String::from(format!("Dealers open card: {}",card.to_string())));


    //Dealer Draws second card
    deal(&mut hand_dealer, &mut deck);
    let mut dealer_value:usize = count_values(&hand_dealer);
    prompt(String::from("Dealers draws his closed card."));

    //Player draws first card
    card = deal(&mut hand_player, &mut deck);
    prompt(String::from(format!("Players draws a card: {}",card.to_string())));

    //Player draws second card
    card = deal(&mut hand_player, &mut deck);
    prompt(String::from(format!("Players draws a card: {}",card.to_string())));

    println!("Players hand: {}",hand_to_string(&hand_player));
    //Count Players Hand
    let mut player_value:usize = count_values(&hand_player);
    prompt(String::from(format!("The players hand value is {}.",player_value)));

    let mut next = String::new();


    loop {
        if player_value < 21 {
            next = prompt(String::from("Deal or Stand? "));
        }
        else {
            next = String::from("Stand");
        }
        if next == "Deal" {
            //Dealer Draws card
            card = deal(&mut hand_dealer, &mut deck);
            prompt(String::from("Players Turn."));

            //Player draws card
            card = deal(&mut hand_player, &mut deck);
            println!("Players draws a card: {}",card.to_string());
            println!("Players hand: {}",hand_to_string(&hand_player));
            //Count Players Hand
            player_value = count_values(&hand_player);
            println!("The players hand value is {}.",player_value);
            prompt(String::from("Next Round."));
        }
        else{
            dealer_value = count_values(&hand_dealer);
            player_value = count_values(&hand_player);
            println!("The dealers hand is {}. Value {}.",hand_to_string(&hand_dealer),dealer_value);
            println!("The players hand is {}. Value {}.",hand_to_string(&hand_player),player_value);
            if player_value > 21 {
                println!("Dealer wins!");
            }
            else if dealer_value > player_value {
                    println!("Dealer wins!");
                }
            else if player_value > dealer_value {
                println!("Player wins!");
            }
            else {
                println!("It's a tie!");
            }
            break;
        }
    }
}

fn prompt(text:String) -> String {
    
    println!("{}",text);
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("Opps2");
    buffer = buffer.trim().to_string();
    buffer
}

fn main() {
    game_loop();
}
