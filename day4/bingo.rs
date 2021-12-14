use std::io::{self,BufRead};

struct BingoCard {
    // a 5x5 bingo card, values and hits
    vals : [i32; 25],
    hits : [bool; 25],
    wins : bool,
}

fn new_card() -> BingoCard {
    let card = BingoCard {
        vals : [0; 25],
        hits : [false; 25],
        wins : false,
    };
    return card;
}

fn check_win(card : &BingoCard) -> bool {
    // any complete row or column is a winner
    for r in 0..5 {
        if card.hits[5*r] && card.hits[5*r+1] && card.hits[5*r+2] && card.hits[5*r+3] && card.hits[5*r+4] {
            return true;
        }
    }
    for c in 0..5 {
        if card.hits[c] && card.hits[5+c] && card.hits[10+c] && card.hits[15+c] && card.hits[20+c] {
            return true;
        }
    }
    return false;
}

fn mark_card(card : &mut BingoCard, v : i32) -> bool {
    for n in 0..25 {
        if v==card.vals[n] {
            card.hits[n] = true;
            if !card.wins && check_win(card) {
                // we only return true once at point of winning
                card.wins = true;
                return true;
            }
        }
    }
    return false;
}

fn score_card(card : &BingoCard, v : i32) -> i32 {
    // add up all unhit values, multiply by winning value
    let mut score : i32 = 0;
    for n in 0..25 {
        if !card.hits[n] {
            score += card.vals[n];
        }
    }
    return score*v;
}

fn main() {
    let mut calls : Vec<i32> = Vec::new();
    let mut cards : Vec<BingoCard> = Vec::new();
    let mut card : BingoCard = new_card();
    let mut cnt : usize = 0;
    for (idx,line) in io::stdin().lock().lines().enumerate() {
        let txt = line.unwrap();
        // first line is the caller's list
        if 0==idx {
            for v in txt.split(',') {
                let n = i32::from_str_radix(&v,10).unwrap();
                calls.push(n);
            }
        // otherwise we are collecting cards (after first blank line)
        } else if txt.len()>0 {
            for v in txt.split_whitespace() {
                let n = i32::from_str_radix(&v,10).unwrap();
                card.vals[cnt] = n;
                cnt += 1;
            }
            if cnt>=25 {
                // emit a card, start another..
                cards.push(card);
                card = new_card();
                cnt = 0;
            }
        }
    }
    println!("calls: {:?}", calls);
    for card in &cards {
        println!("card: vals: {:?} hits: {:?}", card.vals, card.hits);
    }
    println!("Let's play Bingo!\n");
    // let's play bingo!
    let mut wins : usize = 0;
    for call in calls {
        println!("call: {}", call);
        // check each card for a hit..
        for c in 0..cards.len() {
            if mark_card(&mut cards[c], call) {
                if 0==wins {
                    // first winner
                    println!("1st winner: {} = {:?}/{:?}", c, cards[c].vals, cards[c].hits);
                    println!("Part1: score: {}", score_card(&cards[c], call));
                } else if wins+1 == cards.len() {
                    // last winner
                    println!("last winner: {} = {:?}/{:?}", c, cards[c].vals, cards[c].hits);
                    println!("Part2: score: {}", score_card(&cards[c], call));
                    return;
                }
                wins += 1;
            }
        }
    }
}