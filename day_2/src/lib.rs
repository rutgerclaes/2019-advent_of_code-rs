///
/// ```
/// ```
pub fn part_1(original_input: &Vec<i32>) -> i32 {
  solve(original_input, 12, 2)
}

pub fn part_2(original_input: &Vec<i32>) -> i32 {
  let mut search = (0..=99).flat_map(|noun| (0..=99).map(move |verb| (noun, verb)));
  search
    .find_map(|(noun, verb)| {
      let answer = solve(original_input, noun, verb);
      if answer == 19690720 {
        Some(100 * noun + verb)
      } else {
        None
      }
    })
    .unwrap()
}

/// ```
/// let mut input = vec![ 1,9,10,3,2,3,11,0,99,30,40,50 ];
/// let code = day_2::extract_int_code( 0, &input );
/// assert_eq!( code, ( 1, 9, 10, 3 ) );
/// let exp = vec![1,9,10,70,2,3,11,0,99,30,40,50];
/// let result = day_2::execute_int_code( code, input );
/// let input = result.1;
/// assert_eq!( result.0, true );
/// assert_eq!( input, exp );
///
/// let code = day_2::extract_int_code( 4, &input );
/// assert_eq!( code, (2,3,11,0) );
/// let exp = vec![3500,9,10,70,2,3,11,0,99,30,40,50];
/// let result = day_2::execute_int_code( code, input );
/// let input = result.1;
/// assert_eq!( result.0, true );
/// assert_eq!( input, exp );
/// ```
pub fn execute_int_code(int_code: (i32, i32, i32, i32), mut input: Vec<i32>) -> (bool, Vec<i32>) {
  match int_code {
    (99, _, _, _) => (false, input),
    (1, a, b, res) => {
      input[res as usize] = input[a as usize] + input[b as usize];
      (true, input)
    }
    (2, a, b, res) => {
      input[res as usize] = input[a as usize] * input[b as usize];
      (true, input)
    }
    (code, _, _, _) => panic!("Unknown int code: {}", code),
  }
}

pub fn extract_int_code(position: usize, input: &Vec<i32>) -> (i32, i32, i32, i32) {
  (
    input[position],
    input[position + 1],
    input[position + 2],
    input[position + 3],
  )
}

fn solve(input: &Vec<i32>, noun: i32, verb: i32) -> i32 {
  let mut input: Vec<i32> = input.to_vec();
  input[1] = noun;
  input[2] = verb;
  let mut position: usize = 0;
  let mut cont = true;
  while cont {
    let code = extract_int_code(position, &input);
    let result = execute_int_code(code, input);
    input = result.1;
    cont = result.0;
    position += 4;
  }
  input[0]
}
