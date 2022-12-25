
pub fn execute(input: &Vec<String>) -> usize {

    input.iter().map(| round | {

        let hands = round.split(" ").collect::<Vec<&str>>();
        get_points(eval_hand(hands[0]), eval_hand(hands[1]))

    }).fold(0, |points: usize, round| points + round)
}

/** 
 * tie 3 pts
 * win 6 pts
 * loss 0 pts
 * Rock 1 pt
 * Paper 2 pt
 * Scissors 3 pt
 */

fn eval_hand(a: &str) -> usize {
    match a {
        "A" | "X" => return 1,
        "B" | "Y" => return 2,
        "C" | "Z" => return 3,
        _ => todo!(),
    }
}

fn get_points(hand: usize, hand2: usize) -> usize {
    if hand == 3 && hand2 == 1 {
        return hand2 + 6;
    } else if hand == 1 && hand2 == 3 {
        return hand2 + 0;
    }else if hand2 > hand {
        return hand2 + 6;
    } else if hand2 == hand {
        return hand2 + 3;
    } else {
        return hand2 + 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_execute(){
        let seed_input = vec!["A Y","B X","C Z"];
        let test_input = seed_input.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        assert_eq!(execute(&test_input), 15);
    }
    #[test]
    fn test_eval_hand(){
        assert_eq!(eval_hand("A"), 1);
        assert_eq!(eval_hand("B"), 2);
        assert_eq!(eval_hand("C"), 3);
        assert_eq!(eval_hand("X"), 1);
        assert_eq!(eval_hand("B"), 2);
        assert_eq!(eval_hand("Z"), 3);
    }
    #[test]
    fn test_get_points() {
        assert_eq!(get_points(1,2),8);
        assert_eq!(get_points(1,3),9);
        assert_eq!(get_points(1,1),4);
        assert_eq!(get_points(2,2),5);
        assert_eq!(get_points(3,3),6);
        assert_eq!(get_points(3,1),7);
        assert_eq!(get_points(2,1),1);
        assert_eq!(get_points(3,2),2);
    }
}