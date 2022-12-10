mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
    println!("{}", day6::get_first_pos_2());
    println!("{}", day7::get_sum_of_sizes());
    println!("{}", day7::get_smallest_to_delete());
    println!("{}", day8::get_num_visible());
    println!("{}", day8::get_max_score());
    println!("{}", day9::get_num_positions());
    println!("{}", day9::get_num_positions_2());
    println!("{}", day10::get_sum_strengths());
    println!("{}", day10::render_crt());
}
