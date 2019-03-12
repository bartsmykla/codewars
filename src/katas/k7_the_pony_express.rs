#![allow(unused)]

/*
    Kata (7kyu): The Pony Express
    Url: https://www.codewars.com/kata/the-pony-express/train/rust

    A History Lesson
    The Pony Express was a mail service operating in the US in 1859-60.

    Pony Express
    It reduced the time for messages to travel between the Atlantic
    and Pacific coasts to about 10 days, before it was made obsolete
    by the transcontinental telegraph.

    How it worked
    There were a number of stations, where:

    The rider switched to a fresh horse and carried on, or
    The mail bag was handed over to the next rider
    Kata Task
    stations is a list/array of distances (miles) from one station
    to the next along the Pony Express route.

    Implement the riders method/function, to return how many riders
    are necessary to get the mail from one end to the other.

    NOTE: Each rider travels as far as he can, but never more than 100 miles.
*/

fn riders(stations: &[u32]) -> u32 {
    let mut sum = 0;
    let mut riders = 1;

    for station in stations {
        if sum + station > 100 {
            sum = 0;

            riders += 1;
        }

        sum += station;
    }

    riders
}

#[test]
fn sample_tests() {
    assert_eq!(riders(&vec![18, 15]), 1);
    assert_eq!(riders(&vec![43, 23, 40, 13]), 2);
    assert_eq!(riders(&vec![33, 8, 16, 47, 30, 30, 46]), 3);
    assert_eq!(
        riders(&vec![6, 24, 6, 8, 28, 8, 23, 47, 17, 29, 37, 18, 40, 49]),
        4
    );
}
