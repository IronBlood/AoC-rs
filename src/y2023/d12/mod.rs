// Credit https://github.com/hb0nes/aoc_2023/blob/main/twelve_dp/src/main.rs
//        https://old.reddit.com/r/adventofcode/comments/18ge41g/2023_day_12_solutions/ku9cxmv/
use std::iter::repeat;

#[derive(Debug, Clone)]
struct Record {
    springs: String,
    groups: Vec<usize>,
}

fn solve_record_bottom(springs: &str, groups: &[usize]) -> usize {
    let mut dp: Vec<Vec<usize>> =
        vec![vec![0; groups.len()]; springs.len() + groups[groups.len() - 1] + 1];
    let mut min_j = 0;
    'i: for i in 0..springs.len() {
        // Manage memory
        if i > 0 {
            dp[i - 1].clear();
        }
        for j in 0..groups.len() {
            // If first group is at a broken spring, we skip it from now on
            // The first group decides all the valid starting positions and its placement can never
            // be past the first #.
            let cur_char = &springs[i..i + 1];
            if j < min_j {
                continue;
            }
            if cur_char == "#" && j == 0 {
                min_j = 1;
            }
            // Skip periods
            if cur_char == "." {
                continue 'i;
            }
            // If group can't be placed here according to previous logic, continue
            if j > 0 && dp[i][j - 1] == 0 {
                continue;
            }
            // If remaining groups don't fit in remaining springs, continue
            if groups[j..].iter().sum::<usize>() + groups[j..].len() - 1 > springs[i..].len() {
                continue;
            }
            // if we are at last group and there are springs remaining, group isn't valid
            if (j == groups.len() - 1) && springs[i + groups[j]..].chars().any(|c| c == '#') {
                continue;
            }
            // Check if current group is valid
            let max_idx = springs.len().min(i + groups[j]);
            let end_reached = max_idx == springs.len();
            let subsequent_character = springs.get(max_idx..max_idx + 1).unwrap_or("");
            let group_valid = springs[i..i + groups[j]]
                .chars()
                .all(|x| x == '?' || x == '#')
                && (end_reached || subsequent_character != "#");
            if !group_valid {
                continue;
            }

            // If our current group is valid, we add the amount of ways we can reach the next
            // starting location, to all indices up to and including a broken spring.
            // If there are no broken springs, that means all remaining positions are valid for the
            // next group. During next iterations, we can check if the next group fits there.
            // If it does, we can do the same thing and add the amount of ways we could get to the starting index for the group after that,
            // and so forth.
            // --------------------------------------------------
            //             01234567
            // Scenario 1: ??.??.?? 1,1,1
            // --------------------------------------------------
            //
            //       dp[0]      dp[1]      dp[2]      dp[3]      dp[4]      dp[5]      dp[6]      dp[7]      dp[8]       dp[9]     ]
            //     [ [0, 0, 0], [0, 0, 0], [1, 0, 0], [2, 0, 0], [2, 0, 0], [2, 2, 0], [2, 4, 0], [2, 4, 0], [2, 4, 4],  [2, 4, 8] ]
            // --------------------------------------------------
            //             0123456
            // Scenario 2: ??.#.?? 1,1,1
            // --------------------------------------------------
            //
            //       dp[0]      dp[1]      dp[2]      dp[3]      dp[4]      dp[5]      dp[6]      dp[7]      dp[8]     ]
            //     [ [0, 0, 0], [0, 0, 0], [1, 0, 0], [2, 0, 0], [0, 0, 0], [0, 2, 0], [0, 2, 0], [0, 2, 2], [0, 2, 4] ]
            let next_start_idx = (springs.len()).min(i + groups[j] + 1);
            let next_broken_idx = match springs[next_start_idx..].find('#') {
                Some(n) => next_start_idx + n,
                None => dp.len() - 1,
            };
            for k in next_start_idx..=next_broken_idx {
                if j > 0 {
                    dp[k][j] += dp[i][j - 1];
                } else {
                    dp[k][j] += 1;
                }
            }
        }
    }
    dp[dp.len() - 1][dp[dp.len() - 1].len() - 1]
}

fn parse_input_two(input: &str) -> Vec<Record> {
    let mut records = vec![];
    for line in input.lines() {
        let (springs, groups) = line.split_once(' ').unwrap();
        let springs = repeat(springs).take(5).collect::<Vec<_>>().join("?");
        let groups: Vec<usize> = groups
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .repeat(5);
        records.push(Record { springs, groups });
    }
    records
}

fn solve_records_bottom(records: &[Record]) -> usize {
    let mut total: usize = 0;

    for record in records {
        total += solve_record_bottom(&record.springs, &record.groups);
    }

    total
}

pub fn run(input: &str) {
    let records = parse_input_two(input);
    let solution_two_bottom = solve_records_bottom(&records);
    println!("Solution two: {}", solution_two_bottom);
}
