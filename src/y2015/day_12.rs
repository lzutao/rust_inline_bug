use std::str::FromStr;
use serde_json::Value;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/12");

    let value = Value::from_str(INPUT).unwrap();

    let sum = sum_all_recursive(&value, 0, false);

    println!("[PART 1] Sum of all numbers {}", sum);

    let sum = sum_all_recursive(&value, 0, true);

    println!("[PART 2] Sum of not red numbers {}", sum);
}

fn sum_all_recursive(val: &Value, mut sum : i32, without_red : bool) -> i32{
    if val.is_number() {
        sum += val.as_i64().unwrap() as i32;
    }else if val.is_array() {
        for v in val.as_array().unwrap() {
            sum = sum_all_recursive(v, sum, without_red)
        }
    }else if val.is_object() {
        let fields = val.as_object().unwrap();
        if without_red {
            for (_, v) in fields {
                match v.as_str() {
                    None => {},
                    Some(s) => {
                        if s == "red" {
                            return sum;
                        }
                    }
                }
            }
        }
        for (_, v) in fields {
            sum = sum_all_recursive(v, sum, without_red)
        }
    }

    sum
}
