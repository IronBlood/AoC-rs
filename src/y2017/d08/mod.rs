use std::collections::HashMap;

// ChatGPT
fn get_val(mem: &mut HashMap<String, i32>, key: &str) -> i32 {
    *mem.entry(key.to_string()).or_insert(0)
}

fn do_add(mem: &mut HashMap<String, i32>, key: &str, val: i32) {
    *mem.entry(key.to_string()).or_insert(0) += val;
}

fn exec(data: &str, part: u8) -> i32 {
    let mut mem: HashMap<String, i32> = HashMap::new();
    let mut val_max: i32 = 0;

    for line in data.lines() {
        if let Some((ins, cond)) = line.split_once(" if ") {
            let arr: Vec<_> = ins.split_whitespace().collect();
            let t = arr[0];
            let opcode = arr[1];
            let str_oprand = arr[2];

            let arr: Vec<_> = cond.split_whitespace().collect();
            let s = arr[0];
            let cmp = arr[1];
            let str_cmp = arr[2];

            get_val(&mut mem, t);
            let src_val = get_val(&mut mem, s);

            let evaluated_cond: bool;
            let val_cmp: i32 = str_cmp.parse().expect("not a valid cmp number");
            let val_oprand: i32 = str_oprand.parse().expect("not a valid oprand number");

            match cmp {
                ">" => evaluated_cond = src_val > val_cmp,
                "<" => evaluated_cond = src_val < val_cmp,
                ">=" => evaluated_cond = src_val >= val_cmp,
                "<=" => evaluated_cond = src_val <= val_cmp,
                "==" => evaluated_cond = src_val == val_cmp,
                "!=" => evaluated_cond = src_val != val_cmp,
                _ => panic!("invalid cmp"),
            }

            if evaluated_cond {
                match opcode {
                    "inc" => do_add(&mut mem, t, val_oprand),
                    "dec" => do_add(&mut mem, t, -val_oprand),
                    _ => panic!("invalid opcode"),
                }

                val_max = val_max.max(get_val(&mut mem, t));
            }
        } else {
            panic!("invalid input");
        }
    }

    if part == 1 {
        // ChatGPT
        mem.values().copied().max().unwrap_or(0)
    } else {
        val_max
    }
}

pub fn run(input: &str) {
    println!("{}", exec(input, 1));
    println!("{}", exec(input, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = "
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"
            .trim();
        assert_eq!(exec(data, 1), 1);
        assert_eq!(exec(data, 2), 10);
    }
}
