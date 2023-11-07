// 1713번 후보 추천하기

use std::io;

fn read_from_stdin() -> String {
    let mut raw_input = String::new();

    io::stdin()
        .read_line(&mut raw_input)
        .expect("failed to read");

    raw_input
        .trim()
        .to_string()
}

fn read_usize() -> usize {
    read_from_stdin()
        .parse()
        .expect("expected to input integer")
}

fn read_u64s() -> Vec<u64> {
    read_from_stdin()
        .split_whitespace()
        .map(|x| x
            .parse()
            .expect("expected to input integers")
        )
        .collect()
}

fn main() {
    let num_of_photo_frames     = read_usize();
    let _len_of_recommendations = read_usize();
    let recommendations         = read_u64s();

    let mut interim = recommendations
        .iter()
        .fold(Vec::<(u64, u64)>::with_capacity(num_of_photo_frames), |mut queue, &x| {
            match queue.iter().position(|&(num, _)| num == x) {
                Some(v) => {
                    queue[v].1 += 1
                },
                None => {
                    if queue.len() >= num_of_photo_frames {
                        let min_value = *(
                            queue
                                .iter()
                                .enumerate()
                                .min_by(|&(idx1, &(_, cnt1)), &(idx2, &(_, cnt2))|
                                    cnt1.cmp(&cnt2).then(idx1.cmp(&idx2))
                                )
                                .unwrap()
                                .1
                        );

                        queue.remove(
                            queue
                                .iter()
                                .position(|&x| x == min_value)
                                .unwrap()
                        );
                    }

                    queue.push((x, 1u64))
                },
            }

            queue
        });

    interim.sort_by_key(|&(num, _)| num);

    let result = interim
        .iter()
        .map(|&(num, _)| num.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{result}");
}