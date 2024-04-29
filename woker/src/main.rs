use itertools::Itertools;
use kdam::tqdm;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{clone, collections::HashMap, io::{stdout, Write}, iter::zip, sync::Arc, thread::{self, sleep}, time::{Duration, SystemTime}};

const INT_RANKS: [i64; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
const PRIMES: [i64; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

const MAX_STRAIGHT_FLUSH: i64 = 10;
const MAX_FOUR_OF_A_KIND: i64 = 166;
const MAX_FULL_HOUSE: i64 = 322;
const MAX_FLUSH: i64 = 1599;
const MAX_STRAIGHT: i64 = 1609;
const MAX_THREE_OF_A_KIND: i64 = 2467;
const MAX_TWO_PAIR: i64 = 3325;
const MAX_PAIR: i64 = 6185;
const MAX_HIGH_CARD: i64 = 7462;


// fn init() -> (
//     HashMap<char, i64>,
//     HashMap<char, i64>,
//     HashMap<usize, &'static str>,
//     HashMap<i64, usize>,
//     HashMap<i64, i64>,
//     HashMap<i64, i64>,
//     HashMap<usize, fn(&[i64], &HashMap<i64, i64>, &HashMap<i64, i64>) -> i64>,
// ) {
//     let char_rank_to_int_rank: HashMap<char, i64> =
//         HashMap::from_iter(zip("23456789TJQKA".chars(), INT_RANKS));
//     let char_suit_to_int_suit: HashMap<char, i64> =
//         HashMap::from_iter([('s', 1), ('h', 2), ('d', 4), ('c', 8)]);
//     let rank_class_to_string: HashMap<usize, &str> = HashMap::from_iter(zip(
//         1..10,
//         [
//             "Straight Flush",
//             "Four of a Kind",
//             "Full House",
//             "Flush",
//             "Straight",
//             "Three of a Kind",
//             "Two Pair",
//             "Pair",
//             "High Card",
//         ],
//     ));
//     let max_to_rank_class: HashMap<i64, usize> = HashMap::from_iter(zip(
//         [
//             MAX_STRAIGHT_FLUSH,
//             MAX_FOUR_OF_A_KIND,
//             MAX_FULL_HOUSE,
//             MAX_FLUSH,
//             MAX_STRAIGHT,
//             MAX_THREE_OF_A_KIND,
//             MAX_TWO_PAIR,
//             MAX_PAIR,
//             MAX_HIGH_CARD,
//         ],
//         1..10,
//     ));
//     let mut flush_lookup: HashMap<i64, i64> = HashMap::new();
//     let mut unsuited_lookup: HashMap<i64, i64> = HashMap::new();

//     flushes(&mut flush_lookup, &mut unsuited_lookup);
//     multiples(&mut unsuited_lookup);

//     let hand_sizes: Vec<(
//         usize,
//         fn(&[i64], &HashMap<i64, i64>, &HashMap<i64, i64>) -> i64,
//     )> = vec![(5, _five), (6, _six), (7, _seven)];
//     let hand_size_map: HashMap<usize, fn(&[i64], &HashMap<i64, i64>, &HashMap<i64, i64>) -> i64> =
//         HashMap::from_iter(hand_sizes);

//     (
//         char_rank_to_int_rank,
//         char_suit_to_int_suit,
//             rank_class_to_string,
//                max_to_rank_class,
//                    flush_lookup,
//                       unsuited_lookup,
//        hand_size_map,
//     )
// }

fn init() -> (
    Arc<HashMap<char, i64>>,
    Arc<HashMap<char, i64>>,
    Arc<HashMap<usize, &'static str>>,
    Arc<HashMap<i64, usize>>,
    Arc<HashMap<i64, i64>>,
    Arc<HashMap<i64, i64>>,
    Arc<HashMap<usize, fn(&[i64], &HashMap<i64, i64>, &HashMap<i64, i64>) -> i64>>,
) {
    let char_rank_to_int_rank: HashMap<char, i64> =
        HashMap::from_iter(zip("23456789TJQKA".chars(), INT_RANKS));
    let char_suit_to_int_suit: HashMap<char, i64> =
        HashMap::from_iter([('s', 1), ('h', 2), ('d', 4), ('c', 8)]);
    let rank_class_to_string: HashMap<usize, &str> = HashMap::from_iter(zip(
        1..10,
        [
            "Straight Flush",
            "Four of a Kind",
            "Full House",
            "Flush",
            "Straight",
            "Three of a Kind",
            "Two Pair",
            "Pair",
            "High Card",
        ],
    ));
    let max_to_rank_class: HashMap<i64, usize> = HashMap::from_iter(zip(
        [
            MAX_STRAIGHT_FLUSH,
            MAX_FOUR_OF_A_KIND,
            MAX_FULL_HOUSE,
            MAX_FLUSH,
            MAX_STRAIGHT,
            MAX_THREE_OF_A_KIND,
            MAX_TWO_PAIR,
            MAX_PAIR,
            MAX_HIGH_CARD,
        ],
        1..10,
    ));
    let mut flush_lookup: HashMap<i64, i64> = HashMap::new();
    let mut unsuited_lookup: HashMap<i64, i64> = HashMap::new();

    flushes(&mut flush_lookup, &mut unsuited_lookup);
    multiples(&mut unsuited_lookup);

    let hand_sizes: Vec<(
        usize,
        fn(&[i64], &HashMap<i64, i64>, &HashMap<i64, i64>) -> i64,
    )> = vec![(5, _five), (6, _six), (7, _seven)];
    let hand_size_map: HashMap<usize, fn(&[i64], &HashMap<i64, i64>, &HashMap<i64, i64>) -> i64> =
        HashMap::from_iter(hand_sizes);

    (
        Arc::new(char_rank_to_int_rank),
        Arc::new(char_suit_to_int_suit),
            Arc::new(rank_class_to_string),
                Arc::new(max_to_rank_class),
                    Arc::new(flush_lookup),
                        Arc::new(unsuited_lookup),
        Arc::new(hand_size_map),
    )
}

fn main() {
    // println!("Hello, world!");

    let (
        char_rank_to_int_rank,
        char_suit_to_int_suit,
        rank_class_to_string,
        max_to_rank_class,
        flush_lookup,
        unsuited_lookup,
        hand_size_map,
    ) = init();

    // let c = card(('A', 's'), &char_rank_to_int_rank, &char_suit_to_int_suit);
    // println!("{c}");
    // let c = card(('K', 's'), &char_rank_to_int_rank, &char_suit_to_int_suit);
    // println!("{c}");

    // // println!("{:#?}", unsuited_lookup);

    // let hand1 = evaluate(
    //     &[
    //         card(('A', 'h'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('A', 's'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //     ],
    //     &[
    //         card(('A', 'c'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('A', 'd'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('K', 'd'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('K', 's'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('K', 'c'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //     ],
    //     &flush_lookup,
    //     &unsuited_lookup,
    //     &hand_size_map,
    // );

    // println!("{}", hand1);
    // let hand2 = evaluate(
    //     &[
    //         card(('A', 'h'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('K', 'h'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //     ],
    //     &[
    //         card(('A', 'c'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('K', 's'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('T', 'h'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('J', 'h'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('Q', 'h'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //     ],
    //     &flush_lookup,
    //     &unsuited_lookup,
    //     &hand_size_map,
    // );

    // println!("{}", hand2);
    // let hand3 = evaluate(
    //     &[
    //         card(('A', 'h'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('K', 'h'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //     ],
    //     &[
    //         card(('A', 'c'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('A', 'd'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('K', 'd'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('K', 's'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('K', 'c'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //     ],
    //     &flush_lookup,
    //     &unsuited_lookup,
    //     &hand_size_map,
    // );

    // println!("{}", hand3);
    // let hand3 = evaluate(
    //     &[
    //         card(('Q', 'h'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('Q', 'h'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //     ],
    //     &[
    //         card(('A', 'c'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('A', 'd'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('K', 'd'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('K', 's'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //         card(('K', 'c'), &char_rank_to_int_rank, &char_suit_to_int_suit),
    //     ],
    //     &flush_lookup,
    //     &unsuited_lookup,
    //     &hand_size_map,
    // );

    // println!("{}", hand3);
    let count = 2000000;
    let now = SystemTime::now();
    let res = (0..count).collect::<Vec<usize>>().par_iter().map(|_| {
        let (
            char_rank_to_int_rank,
            char_suit_to_int_suit,
            rank_class_to_string,
            max_to_rank_class,
            flush_lookup,
            unsuited_lookup,
            hand_size_map,
        ) = (
            char_rank_to_int_rank.clone(),
            char_suit_to_int_suit.clone(),
            rank_class_to_string.clone(),
            max_to_rank_class.clone(),
            flush_lookup.clone(),
            unsuited_lookup.clone(),
            hand_size_map.clone(),
        );
        thread::spawn(move || {
            // print!("{}[2J", 27 as char);

            // print!("\x1B[2J\x1B[1;1H");
            hand_summary(
                &[
                    card(('4', 'c'), &char_rank_to_int_rank, &char_suit_to_int_suit),
                    card(('A', 's'), &char_rank_to_int_rank, &char_suit_to_int_suit),
                    card(('5', 'd'), &char_rank_to_int_rank, &char_suit_to_int_suit),
                    card(('K', 'c'), &char_rank_to_int_rank, &char_suit_to_int_suit),
                    card(('2', 's'), &char_rank_to_int_rank, &char_suit_to_int_suit),
                ],
                &[
                    &[
                        card(('6', 'c'), &char_rank_to_int_rank, &char_suit_to_int_suit),
                        card(('7', 'h'), &char_rank_to_int_rank, &char_suit_to_int_suit),
                    ],
                    &[
                        card(('A', 'c'), &char_rank_to_int_rank, &char_suit_to_int_suit),
                        card(('3', 'h'), &char_rank_to_int_rank, &char_suit_to_int_suit),
                    ],
                ],
                &flush_lookup,
                &unsuited_lookup,
                &hand_size_map,
                &max_to_rank_class,
                &rank_class_to_string,
            );
        });

        1
    }).collect::<Vec<usize>>();
    let after = SystemTime::now();
    println!("");
    // println!("{:?}", res);
    sleep(Duration::from_secs(1));
    print!("{}[2J", 27 as char);

    print!("\x1B[2J\x1B[1;1H");
    println!("AVG {}ms", (after.duration_since(now).unwrap().as_millis() as f64 / (count as f64)));
    println!("Elapsed {}s", after.duration_since(now).unwrap().as_secs());
    stdout().flush().unwrap();
}

fn card(
    arg: (char, char),
    char_rank_to_int_rank: &HashMap<char, i64>,
    char_suit_to_int_suit: &HashMap<char, i64>,
) -> ((char, char), i64) {
    // """
    // Converts Card string to binary integer representation of card, inspired by:

    // http://www.suffecool.net/poker/evaluator.html
    // """

    let (rank_char, suit_char) = arg;
    let rank_int = char_rank_to_int_rank[&rank_char];
    let suit_int = char_suit_to_int_suit[&suit_char];
    let rank_prime = PRIMES[rank_int as usize];

    let bitrank = 1 << rank_int << 16;
    let suit = suit_int << 12;
    let rank = rank_int << 8;

    (arg, bitrank | suit | rank | rank_prime)
}

fn evaluate(
    cards: &[((char, char), i64)],
    board: &[((char, char), i64)],
    flush_lookup: &HashMap<i64, i64>,
    unsuited_lookup: &HashMap<i64, i64>,
    hand_size_map: &HashMap<usize, fn(&[i64], &HashMap<i64, i64>, &HashMap<i64, i64>) -> i64>,
) -> i64 {
    // """
    // This is the function that the user calls to get a hand rank.

    // Supports empty board, etc very flexible. No input validation
    // because that's cycles!
    // """
    let mut all_cards = cards.iter().map(|(_, card)| *card).collect::<Vec<i64>>();
    all_cards.extend(board.iter().map(|(_, card)| *card));
    // dbg!(&all_cards);
    hand_size_map[&all_cards.len()](&all_cards, flush_lookup, unsuited_lookup)
}

fn _five(
    cards: &[i64],
    flush_lookup: &HashMap<i64, i64>,
    unsuited_lookup: &HashMap<i64, i64>,
) -> i64 {
    // """
    // Performs an evalution given cards in integer form, mapping them to
    // a rank in the range [1, 7462], with lower ranks being more powerful.

    // Variant of Cactus Kev's 5 card evaluator, though I saved a lot of memory
    // space using a hash table and condensing some of the calculations.
    // """
    // # if flush
    if (cards[0] & cards[1] & cards[2] & cards[3] & cards[4] & 0xF000) != 0 {
        let hand_or = (cards[0] | cards[1] | cards[2] | cards[3] | cards[4]) >> 16;
        let prime = prime_product_from_rankbits(hand_or);
        flush_lookup[&prime]
    }
    // # otherwise
    else {
        let prime = prime_product_from_hand(cards);
        // dbg!(prime);
        unsuited_lookup[&prime]
    }
}

fn _six(
    cards: &[i64],
    flush_lookup: &HashMap<i64, i64>,
    unsuited_lookup: &HashMap<i64, i64>,
) -> i64 {
    let mut minimum = MAX_HIGH_CARD;

    let all5cardcombobs = cards.iter().combinations(5);
    for combo in all5cardcombobs {
        // let combo: &[i64] = &[*combo[0], combo[1], combo[2], combo[3], combo[4]];
        let score = _five(
            &combo.iter().map(|c| **c).collect::<Vec<i64>>()[0..combo.len()],
            flush_lookup,
            unsuited_lookup,
        );
        if score < minimum {
            minimum = score
        }
    }
    minimum
}

fn _seven(
    cards: &[i64],
    flush_lookup: &HashMap<i64, i64>,
    unsuited_lookup: &HashMap<i64, i64>,
) -> i64 {
    _six(cards, flush_lookup, unsuited_lookup)
}

fn prime_product_from_rankbits(rankbits: i64) -> i64 {
    // """
    // Returns the prime product using the bitrank (b)
    // bits of the hand. Each 1 in the sequence is converted
    // to the correct prime and multiplied in.

    // Params:
    //     rankbits = a single 32-bit (only 13-bits set) integer representing
    //             the ranks of 5 _different_ ranked cards
    //             (5 of 13 bits are set)

    // Primarily used for evaulating flushes and straights,
    // two occasions where we know the ranks are *ALL* different.

    // Assumes that the input is in form (set bits):

    //                       rankbits
    //                 +--------+--------+
    //                 |xxxbbbbb|bbbbbbbb|
    //                 +--------+--------+

    // """
    let mut product: i64 = 1;
    for i in INT_RANKS {
        // # if the ith bit is set
        if (rankbits & (1 << i)) != 0 {
            product *= PRIMES[i as usize];
        }
    }
    product
}

fn prime_product_from_hand(card_ints: &[i64]) -> i64 {
    // """
    // Expects a list of cards in integer form.
    // """

    let mut product = 1;
    for c in card_ints {
        product *= c & 0xFF;
    }

    product
}

fn flushes(flush_lookup: &mut HashMap<i64, i64>, unsuited_lookup: &mut HashMap<i64, i64>) {
    // """
    // Straight flushes and flushes.

    // Lookup is done on 13 bit integer (2^13 > 7462):
    // xxxbbbbb bbbbbbbb => integer hand index
    // """

    // # straight flushes in rank order
    const STRAIGHT_FLUSHES: [i64; 10] = [
        7936, // # int('0b1111100000000', 2), # royal flush
        3968, // # int('0b111110000000', 2),
        1984, // # int('0b11111000000', 2),
        992,  // # int('0b1111100000', 2),
        496,  // # int('0b111110000', 2),
        248,  // # int('0b11111000', 2),
        124,  // # int('0b1111100', 2),
        62,   // # int('0b111110', 2),
        31,   // # int('0b11111', 2),
        4111, // # int('0b1000000001111', 2) # 5 high
    ];

    // # now we'll dynamically generate all the other
    // # flushes (including straight flushes)
    let mut flushes = vec![];
    let mut gen = get_lexographically_next_bit_sequence(0b11111);

    // # 1277 = number of high cards
    // # 1277 + len(str_flushes) is number of hands with all cards unique rank
    for _ in 0..1277 + STRAIGHT_FLUSHES.len() - 1 {
        //# we also iterate over SFs
        // # pull the next flush pattern from our generator
        let f = gen;
        gen = get_lexographically_next_bit_sequence(gen);

        // # if this flush matches perfectly any
        // # straight flush, do not add it
        let mut not_sf = true;
        for sf in STRAIGHT_FLUSHES {
            // # if f XOR sf == 0, then bit pattern
            // # is same, and we should not add
            if (f ^ sf) == 0 {
                not_sf = false;
            }
        }

        if not_sf {
            flushes.push(f);
        }
    }

    // # we started from the lowest straight pattern, now we want to start ranking from
    // # the most powerful hands, so we reverse
    flushes.reverse();

    // # now add to the lookup map:
    // # start with straight flushes and the rank of 1
    // # since theyit is the best hand in poker
    // # rank 1 = Royal Flush!
    let mut rank = 1;
    for sf in STRAIGHT_FLUSHES {
        let prime_product = prime_product_from_rankbits(sf);
        flush_lookup.insert(prime_product, rank);
        rank += 1
    }
    // # we start the counting for flushes on max full house, which
    // # is the worst rank that a full house can have (2,2,2,3,3)
    rank = MAX_FULL_HOUSE + 1;
    for f in flushes.iter() {
        let prime_product = prime_product_from_rankbits(*f);
        flush_lookup.insert(prime_product, rank);
        rank += 1;
    }
    // # we can reuse these bit sequences for straights
    // # and high cards since they are inherently related
    // # and differ only by context
    straight_and_highcards(&STRAIGHT_FLUSHES, &flushes, unsuited_lookup)
}

fn get_lexographically_next_bit_sequence(bits: i64) -> i64 {
    let t = (bits | (bits - 1)) + 1;
    t | ((((t & -t) as f64 / (bits & -bits) as f64) as i64 >> 1) - 1)

    // std::iter::from_fn(move || {
    //     let next_value = next;
    //     t = (next | (next - 1)) + 1;
    //     next = t | ((t & -t) / (next & -next) >> 1) - 1;
    //     Some(next_value)
    // })
}

#[cfg(test)]
mod test {
    use crate::get_lexographically_next_bit_sequence;

    #[test]
    fn lex() {
        assert_eq!(get_lexographically_next_bit_sequence(0b11111), 47);
    }
}

fn straight_and_highcards(
    straights: &[i64],
    highcards: &[i64],
    unsuited_lookup: &mut HashMap<i64, i64>,
) {
    // """
    // Unique five card sets. Straights and highcards.

    // Reuses bit sequences from flush calculations.
    // """
    let mut rank = MAX_FLUSH + 1;

    for s in straights {
        let prime_product = prime_product_from_rankbits(*s);
        unsuited_lookup.insert(prime_product, rank);
        rank += 1;
    }
    // dbg!(&straights, &highcards);
    rank = MAX_PAIR + 1;
    for h in highcards {
        let prime_product = prime_product_from_rankbits(*h);
        // dbg!(&prime_product, rank, h);
        unsuited_lookup.insert(prime_product, rank);
        rank += 1;
    }
}

fn multiples(unsuited_lookup: &mut HashMap<i64, i64>) {
    // """
    // Pair, Two Pair, Three of a Kind, Full House, and 4 of a Kind.
    // """
    let backwards_ranks: Vec<i64> = INT_RANKS.iter().copied().rev().collect();

    // # 1) Four of a Kind
    let mut rank = MAX_STRAIGHT_FLUSH + 1;

    // # for each choice of a set of four rank
    for i in backwards_ranks.iter() {
        // # and for each possible kicker rank
        let mut kickers = backwards_ranks.clone();
        kickers.retain(|k| k != i);
        for k in kickers {
            let product = PRIMES[*i as usize].pow(4) * PRIMES[k as usize];
            unsuited_lookup.insert(product, rank);
            rank += 1;
        }
    }
    // # 2) Full House
    rank = MAX_FOUR_OF_A_KIND + 1;

    // # for each three of a kind
    for i in backwards_ranks.iter() {
        // # and for each choice of pair rank
        let mut pairranks = backwards_ranks.clone();
        pairranks.retain(|k| k != i);
        for pr in pairranks {
            let product = PRIMES[*i as usize].pow(3) * PRIMES[pr as usize].pow(2);
            unsuited_lookup.insert(product, rank);
            rank += 1;
        }
    }
    // # 3) Three of a Kind
    rank = MAX_STRAIGHT + 1;

    // # pick three of one rank
    for r in backwards_ranks.iter() {
        let mut kickers = backwards_ranks.clone();
        kickers.retain(|k| k != r);
        // list.iter().combinations(k)
        // let gen = itertools.combinations(kickers, 2);
        let gen = kickers.iter().combinations(2);

        for c in gen {
            let (c1, c2) = (c[0], c[1]);
            let product = PRIMES[*r as usize].pow(3) * PRIMES[*c1 as usize] * PRIMES[*c2 as usize];
            unsuited_lookup.insert(product, rank);
            rank += 1;
        }
    }
    // # 4) Two Pair
    rank = MAX_THREE_OF_A_KIND + 1;

    let tpgen = backwards_ranks.iter().combinations(2);
    for p in tpgen {
        let (pair1, pair2) = (p[0], p[1]);

        let mut kickers = backwards_ranks.clone();
        kickers.retain(|k| k != pair1);
        kickers.retain(|k| k != pair2);
        for kicker in kickers {
            let product = PRIMES[*pair1 as usize].pow(2)
                * PRIMES[*pair2 as usize].pow(2)
                * PRIMES[kicker as usize];
            unsuited_lookup.insert(product, rank);
            rank += 1;
        }
    }
    // # 5) Pair
    rank = MAX_TWO_PAIR + 1;

    // # choose a pair
    for pairrank in backwards_ranks.iter() {
        let mut kickers = backwards_ranks.clone();
        kickers.retain(|k| k != pairrank);
        let kgen = kickers.iter().combinations(3);

        for k in kgen {
            let (k1, k2, k3) = (k[0], k[1], k[2]);
            let product = PRIMES[*pairrank as usize].pow(2)
                * PRIMES[*k1 as usize]
                * PRIMES[*k2 as usize]
                * PRIMES[*k3 as usize];
            unsuited_lookup.insert(product, rank);
            rank += 1;
        }
    }
}

// fn combinations(list: Vec<i32>, k: usize) -> Vec<Vec<i32>> {
//     list.iter().combinations(k).map(|v| v.iter().copied().collect()).collect()
// }

fn get_rank_class(hr: i64, max_to_rank_class: &HashMap<i64, usize>) -> usize {
    // """
    // Returns the class of hand given the hand hand_rank
    // returned from evaluate.
    // """
    if (0..=MAX_STRAIGHT_FLUSH).contains(&hr) {
        max_to_rank_class[&MAX_STRAIGHT_FLUSH]
    } else if hr <= MAX_FOUR_OF_A_KIND {
        max_to_rank_class[&MAX_FOUR_OF_A_KIND]
    } else if hr <= MAX_FULL_HOUSE {
        max_to_rank_class[&MAX_FULL_HOUSE]
    } else if hr <= MAX_FLUSH {
        max_to_rank_class[&MAX_FLUSH]
    } else if hr <= MAX_STRAIGHT {
        max_to_rank_class[&MAX_STRAIGHT]
    } else if hr <= MAX_THREE_OF_A_KIND {
        max_to_rank_class[&MAX_THREE_OF_A_KIND]
    } else if hr <= MAX_TWO_PAIR {
        max_to_rank_class[&MAX_TWO_PAIR]
    } else if hr <= MAX_PAIR {
        max_to_rank_class[&MAX_PAIR]
    } else if hr <= MAX_HIGH_CARD {
        max_to_rank_class[&MAX_HIGH_CARD]
    } else {
        panic!("Invalid hand rank, cannot return rank class")
    }
}

fn get_five_card_rank_percentage(hand_rank: i64) -> u8 {
    // """
    // Scales the hand rank score to the [0.0, 1.0] range.
    // """
    (hand_rank as f64 / MAX_HIGH_CARD as f64 * 100.0).round() as u8
}

fn hand_summary(
    board: &[((char, char), i64)],
    hands: &[&[((char, char), i64)]],
    flush_lookup: &HashMap<i64, i64>,
    unsuited_lookup: &HashMap<i64, i64>,
    hand_size_map: &HashMap<usize, fn(&[i64], &HashMap<i64, i64>, &HashMap<i64, i64>) -> i64>,
    max_to_rank_class: &HashMap<i64, usize>,
    rank_class_to_string: &HashMap<usize, &str>,
) {
    // """
    // Gives a sumamry of the hand with ranks as time proceeds.

    // Requires that the board is in chronological order for the
    // analysis to make sense.
    // """

    assert!(board.len() == 5, "Invalid board length");
    for hand in hands {
        assert!(hand.len() == 2, "Inavlid hand length");
    }
    let line_length = 10;
    let stages = ["FLOP", "TURN", "RIVER"];

    for (i, stage) in stages.iter().enumerate() {
        // let line = ("=".repeat(line_length) + " %s " +  "=" * line_length)
        println!(
            "{stage} ({})\n{}",
            // "=".repeat(line_length),
            &board[..(i + 3)]
                .iter()
                .map(|(arg, _)| format!("{}{}", arg.0, arg.1))
                .join(" "),
            "=".repeat(line_length * 2)
        );

        let mut best_rank = 7463; // # rank one worse than worst hand
        let mut winners = vec![];
        for (player, hand) in hands.iter().enumerate() {
            // # evaluate current board position
            let rank = evaluate(
                hand,
                &board[..(i + 3)],
                flush_lookup,
                unsuited_lookup,
                hand_size_map,
            );
            let rank_class = get_rank_class(rank, max_to_rank_class);
            let class_string = rank_class_to_string[&rank_class];
            let percentage = 100 - get_five_card_rank_percentage(rank); //  # higher better here
            println!(
                "P{} ({})  ->  {}%  {}",
                player + 1,
                hand.iter()
                    .map(|(arg, _)| format!("{}{}", arg.0, arg.1))
                    .join(""),
                percentage,
                class_string,
            );

            // # detect winner
            if rank == best_rank {
                winners.push(player);
                best_rank = rank;
            } else if rank < best_rank {
                winners = vec![player];
                best_rank = rank;
            }
        }
        // # if we're not on the river
        if i != stages.iter().position(|&r| r == "RIVER").unwrap() {
            if winners.len() == 1 {
                println!("Player {} hand is currently winning.\n", winners[0] + 1)
            } else {
                println!(
                    "Players {:?} are tied for the lead.\n",
                    winners
                        .iter()
                        .map(|player| *player + 1)
                        .collect::<Vec<usize>>()
                );
            }
        }
        // # otherwise on all other streets
        else {
            println!(
                "\n{} HAND OVER {}",
                "=".repeat(line_length),
                "=".repeat(line_length)
            );
            if winners.len() == 1 {
                println!(
                    "Player {} is the winner with a {}\n",
                    winners[0] + 1,
                    rank_class_to_string[&get_rank_class(
                        evaluate(
                            hands[winners[0]],
                            board,
                            flush_lookup,
                            unsuited_lookup,
                            hand_size_map,
                        ),
                        max_to_rank_class
                    )]
                );
            } else {
                println!(
                    "Players {:?} tied for the win with a {}\n",
                    winners,
                    rank_class_to_string[&get_rank_class(
                        evaluate(
                            hands[winners[0]],
                            board,
                            flush_lookup,
                            unsuited_lookup,
                            hand_size_map,
                        ),
                        max_to_rank_class
                    )]
                );
            }
        }
    }
}
