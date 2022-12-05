mod day1;
mod day2;
mod day3;

fn main() {
    println!("{}", day1::get_max_calories());
    println!("{}", day1::get_top3_calories());
    println!("{}", day2::get_total_score());
    println!("{}", day2::get_total_score_2());
    println!("{}", day3::get_sum_of_priorities());
    println!("{}", day3::get_sum_of_badge_priorities());
}
