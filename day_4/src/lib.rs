pub fn part_1(start: u32, end: u32) -> usize {
  find_password(start, end, adjacant_digits)
}

pub fn part_2(start: u32, end: u32) -> usize {
  find_password(start, end, adjacant_digits_not_in_group)
}

pub fn find_password<F>(start: u32, end: u32, validator: F) -> usize
where
  F: Fn(&[u8; 6]) -> bool,
{
  let mut number = make_increasing(to_array(start));
  let mut count = 0;
  while from_array(&number) <= end {
    if validator(&number) {
      count += 1;
    }
    number = make_increasing(increase(number));
  }

  count
}

/// # Increase a number untill all of its digits never decreasing
///
/// ## Example
/// ```
/// use day_4::make_increasing;
///
/// assert_eq!( make_increasing( [ 6, 5, 4, 3, 2, 1 ] ), [ 6, 6, 6, 6, 6, 6 ] );
/// assert_eq!( make_increasing( [ 1, 2, 3, 4, 5, 6 ] ), [ 1, 2, 3, 4, 5, 6 ] );
/// assert_eq!( make_increasing( [ 1, 1, 2, 2, 3, 3 ] ), [ 1, 1, 2, 2, 3, 3 ] );
/// ```
pub fn make_increasing(number: [u8; 6]) -> [u8; 6] {
  (0..5).fold(number, |mut number, index| {
    if number[index] > number[index + 1] {
      number[index + 1] = number[index];
    }
    number
  })
}

/// # Increase the number by 1
///
/// ## Example
/// ```
/// use day_4::increase;
///
/// assert_eq!( increase( [ 1, 9, 9, 9, 9, 9 ] ), [ 2, 0, 0, 0, 0, 0 ] );
/// ```
pub fn increase(mut number: [u8; 6]) -> [u8; 6] {
  number[5] += 1;

  for i in (0..6).rev() {
    if number[i] > 9 {
      number[i] = 0;
      number[i - 1] += 1;
    }
  }

  number
}

/// # Check for adjacant digits in the number
///
/// ## Example
/// ```
/// use day_4::adjacant_digits;
///
/// assert_eq!( adjacant_digits( &[ 1, 2, 3, 5, 5, 6 ] ), true );
/// assert_eq!( adjacant_digits( &[ 1, 1, 3, 4, 5, 6 ] ), true );
/// assert_eq!( adjacant_digits( &[ 1, 2, 3, 4, 5, 5 ] ), true );
/// assert_eq!( adjacant_digits( &[ 1, 2, 3, 4, 5, 6 ] ), false );
/// assert_eq!( adjacant_digits( &[ 1, 2, 3, 4, 5, 1 ] ), false );
/// assert_eq!( adjacant_digits( &[ 1, 2, 3, 4, 3, 6 ] ), false );
/// ```
pub fn adjacant_digits(number: &[u8; 6]) -> bool {
  (0..5).any(|index| number[index] == number[index + 1])
}

/// # Check for adjacant digits, but make sure they are not part of a larger group
///
/// ## Example
/// ```
/// use day_4::adjacant_digits_not_in_group;
///
/// assert_eq!( adjacant_digits_not_in_group( &[ 1, 2, 3, 4, 5, 6 ] ), false );
/// assert_eq!( adjacant_digits_not_in_group( &[ 1, 1, 3, 4, 5, 6 ] ), true );
/// assert_eq!( adjacant_digits_not_in_group( &[ 1, 1, 1, 4, 5, 6 ] ), false );
/// assert_eq!( adjacant_digits_not_in_group( &[ 1, 2, 2, 4, 5, 6 ] ), true );
/// assert_eq!( adjacant_digits_not_in_group( &[ 1, 2, 2, 2, 5, 6 ] ), false );
/// assert_eq!( adjacant_digits_not_in_group( &[ 1, 2, 3, 4, 5, 5 ] ), true );
/// assert_eq!( adjacant_digits_not_in_group( &[ 1, 2, 3, 5, 5, 5 ] ), false );
/// assert_eq!( adjacant_digits_not_in_group( &[ 1, 1, 1, 2, 2, 5 ] ), true );
/// ```
pub fn adjacant_digits_not_in_group(number: &[u8; 6]) -> bool {
  (0..5).any(|index| {
    number[index] == number[index + 1]
      && (index == 0 || number[index - 1] != number[index])
      && (index == 4 || number[index + 2] != number[index])
  })
}

/// # Convert an integer to an array of its digits
///
/// ## Example
/// ```
/// use day_4::to_array;
///
/// assert_eq!( to_array( 123456 ), [ 1, 2, 3, 4, 5, 6 ] );
/// assert_eq!( to_array( 654321 ), [ 6, 5, 4, 3, 2, 1 ] );
/// ```
pub fn to_array(i: u32) -> [u8; 6] {
  [
    (i / 100_000) as u8,
    ((i / 10_000) % 10) as u8,
    ((i / 1000) % 10) as u8,
    ((i / 100) % 10) as u8,
    ((i / 10) % 10) as u8,
    (i % 10) as u8,
  ]
}

/// # Convert an array of digits to an integer
///
/// ## Example
/// ```
/// use day_4::from_array;
///
/// assert_eq!( from_array( &[ 1, 2, 3, 4, 5, 6 ] ), 123456 );
/// assert_eq!( from_array( &[ 6, 5, 4, 3, 2, 1 ] ), 654321 );
/// ```
pub fn from_array(digits: &[u8; 6]) -> u32 {
  digits.iter().fold(0, |sum, &digit| sum * 10 + digit as u32)
}
