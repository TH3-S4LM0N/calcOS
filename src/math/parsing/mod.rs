use arrayvec::ArrayString;

use crate::{
    math::{parsing::charext::{CharExt, MathType}, functions},
    vga_buffer::BUFFER_WIDTH, error, debug,
};

use super::f32_ext::cF32Ext;

mod charext;

pub fn parse_maths(maths: [char; BUFFER_WIDTH]) -> f32 {
    let mut answer: f32 = 0.0;

    let mut iters: usize = 0;
    loop {
        let char = maths[iters];

        match char.math_type().unwrap() {
            MathType::Action => {
                match char {
                    '+' => {
                        let (new_answer, new_iters) = get_multidigit_num(&maths, iters + 1);
                        answer += new_answer;
                        iters = new_iters - 1;
                    },
                    '-' => {
                        let (new_answer, new_iters) = get_multidigit_num(&maths, iters + 1);
                        answer -= new_answer;
                        iters = new_iters - 1;
                    },
                    '*' => {
                        let (new_answer, new_iters) = get_multidigit_num(&maths, iters + 1);
                        answer *= new_answer;
                        iters = new_iters - 1;
                    },
                    '/' => {
                        let (new_answer, new_iters) = get_multidigit_num(&maths, iters + 1);
                        answer /= new_answer;
                        iters = new_iters - 1;
                    }
                    '^' => {
                        let (new_answer, new_iters) = get_multidigit_num(&maths, iters + 1);
                        answer = answer.c_powf(new_answer);
                        iters = new_iters - 1;
                    }
                    _ => unreachable!()
                }
            }
            MathType::Letter => {
                let mut fn_name = ArrayString::<BUFFER_WIDTH>::new();
                
                loop {
                    if iters == 15 {panic!()}
                    let char = maths[iters];

                    if char == '(' {
                        let fn_name_str = fn_name.as_str();
                        match fn_name_str {
                            "Q_rsqrt" => {
                                iters += 1;
                                let tmp = functions::Arguments::new(&maths, iters);
                                iters = tmp.1;
                                let args = tmp.0;
                                drop(tmp);
                                
                                answer = functions::sqrt::Q_rsqrt(args);
                                break;
                            }
                            "sqrt" => {
                                iters += 1;
                                let tmp = functions::Arguments::new(&maths, iters);
                                iters = tmp.1;
                                let args = tmp.0;
                                drop(tmp);

                                answer = functions::sqrt::sqrt(args);
                                break;
                            }

                            _ => error!("invalid function {fn_name_str}")
                        }
                    }
                    fn_name.push(maths[iters]);
                    iters += 1;
                }
                break;
            }
            MathType::Number => {
                let (new_answer, new_iters) = get_multidigit_num(&maths, iters);

                answer = new_answer;
                iters = new_iters - 1;
            }
            MathType::End => {
                break;
            }
        }

        iters += 1;
    }

    return answer;
}

fn get_multidigit_num(eq: &[char; BUFFER_WIDTH], mut iters: usize) -> (f32, usize) {
    let mut num: f32 = 0.0;

    loop {
        let part = eq[iters];
        if part.is_digit(10) {
            num = (num * 10.0) + part.to_digit(10).unwrap() as f32;
        } else {
            break;
        }
        iters += 1;
    }
    return (num, iters);
}
