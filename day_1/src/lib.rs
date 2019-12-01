/// Calculate the required fuel
///
/// # Example
/// ```
/// use day_1::calculate_fuel;
///
/// assert_eq!( calculate_fuel( &12 ), 2 );
/// assert_eq!( calculate_fuel( &14 ), 2 );
/// assert_eq!( calculate_fuel( &1969 ), 654 );
/// assert_eq!( calculate_fuel( &100756 ), 33583);
/// ```
pub fn calculate_fuel(mass: &i64) -> i64 {
  mass / 3 - 2
}

/// Calculate the required fuel recursively
///
/// The recursion level needed for part_2 isn't deep enough to require a rewrite to tail
/// recursion.
///
/// # Example
/// ```
/// use day_1::calculate_fuel_rec;
///
/// assert_eq!( calculate_fuel_rec( &14 ), 2 );
/// assert_eq!( calculate_fuel_rec( &1969 ), 966 );
/// assert_eq!( calculate_fuel_rec( &100756 ), 50346);
/// ```
pub fn calculate_fuel_rec(mass: &i64) -> i64 {
  let fuel = calculate_fuel(mass);
  if fuel > 2 {
    fuel + calculate_fuel_rec(&fuel)
  } else if fuel >= 0 {
    fuel
  } else {
    0
  }
}

/// Calculate the total required fuel for the list of components
///
/// # Example
/// ```
/// use day_1::part_1;
///
/// assert_eq!( part_1( &vec![ 12, 14 ] ), 4 );
/// assert_eq!( part_1( &vec![ 1969 ] ), 654 );
/// assert_eq!( part_1( &vec![ 100756, 12 ] ), 33585 );
/// ```
pub fn part_1(input: &Vec<i64>) -> i64 {
  input.iter().map(calculate_fuel).sum()
}

/// Calculate the total required fuel for the list of components, also treating fuel as components
///
/// # Example
/// ```
/// use day_1::part_2;
///
/// assert_eq!( part_2( &vec![ 12, 14 ] ), 4 );
/// assert_eq!( part_2( &vec![ 1969, 14 ] ), 968 );
/// ```
pub fn part_2(input: &Vec<i64>) -> i64 {
  input.iter().map(calculate_fuel_rec).sum()
}
