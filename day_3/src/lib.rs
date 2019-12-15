extern crate regex;
use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;

/// # Part 1: find the distance to the closest intersection of the two wires
///
/// ```
/// use day_3::part_1;
/// let input = vec!["R8,U5,L5,D3", "U7,R6,D4,L4"];
/// assert_eq!( part_1( &input ), 6 );
/// let input = vec![ "R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83" ];
/// assert_eq!( part_1( &input ), 159 );
/// let input = vec![ "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7" ];
/// assert_eq!( part_1( &input ), 135 );
/// ```
pub fn part_1(input: &Vec<&str>) -> i32 {
    let first = path((0, 0), &split_directions(input[0]));
    let second = path((0, 0), &split_directions(input[1]));

    let first_set: HashSet<(i32, i32)> = HashSet::from_iter(first);
    let second_set: HashSet<(i32, i32)> = HashSet::from_iter(second);

    first_set
        .intersection(&second_set)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

/// # Part 2: find the intersection requiring the minimal number of steps
///
/// ```
/// use day_3::part_2;
/// let input = vec![ "R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83" ];
/// assert_eq!( part_2( &input ), 610 );
/// let input = vec![ "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"];
/// assert_eq!( part_2( &input ), 410 );
/// ```
pub fn part_2(input: &Vec<&str>) -> i32 {
    let first = path((0, 0), &split_directions(input[0]));
    let second = path((0, 0), &split_directions(input[1]));

    let first_set: HashSet<&(i32, i32)> = HashSet::from_iter(first.iter());
    let second_set: HashSet<&(i32, i32)> = HashSet::from_iter(second.iter());

    let first_steps: Vec<(usize, &(i32, i32))> = first.iter().enumerate().collect();
    let second_steps: Vec<(usize, &(i32, i32))> = second.iter().enumerate().collect();

    let crossings = first_set.intersection(&second_set);

    crossings
        .map(|position| {
            let first_step: &usize = first_steps
                .iter()
                .find_map(|(step, pos)| Some(step).filter(|_| pos == position))
                .unwrap();
            let second_step: &usize = second_steps
                .iter()
                .find_map(|(step, pos)| Some(step).filter(|_| pos == position))
                .unwrap();

            first_step + second_step + 2
        })
        .min()
        .unwrap() as i32
}

pub fn split_directions(input: &str) -> Vec<&str> {
    input.split(",").collect()
}

/// # Calculate a wire's path based on a start point and all steps
///
/// ```
/// use day_3::path;
///
/// assert_eq!( path( ( 0, 0 ), &vec!( "R3","U2" ) ), vec!( (1,0), (2,0), (3,0), (3,1), (3,2) ) );
/// ```
pub fn path(start: (i32, i32), input: &Vec<&str>) -> Vec<(i32, i32)> {
    let steps = input.iter().flat_map(|d| parse_direction(d));
    let result = steps.scan(start, move |state, step| {
        state.0 += step.0;
        state.1 += step.1;
        Some(*state)
    });
    result.collect()
}

/// # Parse one instruction
///
/// ```
/// use day_3::parse_direction;
///
/// assert_eq!( parse_direction( "R1" ), vec![(1,0)] );
/// assert_eq!( parse_direction( "L2" ), vec![(-1,0), (-1,0) ] );
/// assert_eq!( parse_direction( "U1" ), vec![(0,1)]);
/// assert_eq!( parse_direction( "D2" ), vec![(0,-1),(0,-1)]);
/// assert_eq!( parse_direction( "D0" ), vec![] );
/// ```
pub fn parse_direction(line: &str) -> Vec<(i32, i32)> {
    let regex = Regex::new("(?P<dir>[UDLR])(?P<dist>[0-9]+)").unwrap();
    let capt = regex.captures(line).unwrap();
    let direction = capt.name("dir").unwrap().as_str();
    let distance = capt.name("dist").unwrap().as_str().parse::<i32>().unwrap();
    let seed = match direction {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("Unknown direction"),
    };

    (0..distance).map(|_| seed).collect()
}
