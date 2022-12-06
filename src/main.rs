mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    println!("{}", day1::get_max_calories());
    println!("{}", day1::get_top3_calories());
    println!("{}", day2::get_total_score());
    println!("{}", day2::get_total_score_2());
    println!("{}", day3::get_sum_of_priorities());
    println!("{}", day3::get_sum_of_badge_priorities());
    println!("{}", day4::get_num_pairs_with_full_contain());
    println!("{}", day4::get_num_pairs_with_overlap());
    println!("{}", day5::get_tops_of_stacks());
    println!("{}", day5::get_tops_of_stacks_2());
    println!("{}", day6::get_first_pos());
}
