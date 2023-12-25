// 1343번 폴리오미노

use std::io;

fn partition_by<I>(mut iter: I, pred: fn(I::Item) -> bool) -> Option<Vec<Vec<I::Item>>>
where
    I: Iterator,
    I::Item: Clone,
{
    let fst_item = iter.next()?;

    let mut pre_pred = pred(fst_item.clone());
    let mut result   = vec![vec![fst_item]];

    for i in iter {
        if pred(i.clone()) != pre_pred {
            pre_pred = !pre_pred;
            result.push(Vec::new());
        }

        result.last_mut().unwrap().push(i);
    }

    Some(result)
}

fn solve(partitioned: Vec<Vec<char>>) -> Option<String> {
    let mut result = String::new();

    for v in partitioned {
        let v_len = v.len();

        result.push_str(&match v[0] {
            '.'                   => v.iter().collect::<String>(),
            'X' if v_len & 1 == 0 => {
                let mut init = vec!['A'; v_len >> 2 << 2];
                let mut tail = vec!['B'; v_len & 3];

                init.append(&mut tail);
                init.iter().collect::<String>()
            },
            _                     => return None,
        });
    }

    Some(result)
}

fn main() {
    let mut input_buffer = String::new();

    io::stdin()
        .read_line(&mut input_buffer)
        .expect("failed to read");

    let Some(partitioned) = partition_by(input_buffer.trim().chars(), |x| x == 'X') else {
        panic!("invalid input format");
    };

    println!("{}", solve(partitioned).or(Some(String::from("-1"))).unwrap());
}
