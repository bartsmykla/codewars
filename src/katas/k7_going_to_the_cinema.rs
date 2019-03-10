#![allow(unused)]

/*
    Kata (7kyu): Going to the cinema
    Url: https://www.codewars.com/kata/going-to-the-cinema/train/rust

    My friend John likes to go to the cinema.
    He can choose between system A and system B.

    System A : buy a ticket (15 dollars) every time
    System B : buy a card (500 dollars) and every time
        buy a ticket the price of which is 0.90 times
        the price he paid for the previous one.

    #Example: If John goes to the cinema 3 times:
    System A : 15 * 3 = 45
    System B : 500 + 15 * 0.90 + (15 * 0.90) * 0.90 + (15 * 0.90 * 0.90) * 0.90
        ( = 536.5849999999999, no rounding for each ticket)
    John wants to know how many times he must go to the cinema so that
    the final result of System B, when rounded up to the next dollar,
    will be cheaper than System A.

    The function movie has 3 parameters: card (price of the card),
    ticket (normal price of a ticket), perc (fraction of what he paid
    for the previous ticket) and returns the first n such that

    ceil(price of System B) < price of System A.
    More examples:

    movie(500, 15, 0.9) should return 43
        (with card the total price is 634, with tickets 645)
    movie(100, 10, 0.95) should return 24
        (with card the total price is 235, with tickets 240)
*/

fn movie(card: i32, ticket: i32, perc: f64) -> i32 {
    let mut i = 0;
    let mut payed = 0.0;
    let mut would_pay = card as f64;

    while would_pay.ceil() >= payed {
        payed += ticket as f64;
        would_pay += ticket as f64 * perc.powi(i + 1);
        i += 1;
    }

    i
}

fn dotest(card: i32, ticket: i32, perc: f64, exp: i32) -> () {
    assert_eq!(exp, movie(card, ticket, perc));
}

#[test]
fn basic_tests() {
    dotest(500, 15, 0.9, 43);
    dotest(100, 10, 0.95, 24);
    dotest(0, 10, 0.95, 2);
}
