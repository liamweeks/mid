pub(crate) fn extract_op(s: &str) -> (&str, &str) {
   match &s[0..1] {
      "+" | "-" | "*" | "/" => {}
      _ => panic!("bad operator"),
   }

   (&s[1..], &s[0..1])
}


pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> &'b str {
    if s.starts_with(starting_text) {
        &s[starting_text.len()..]
    } else {
        panic!("Error: Expected {}", starting_text);
    }
}
pub(crate) fn extract_digits(s: &str) -> (&str, &str) {

   // The SMART way
   let mut digits_end = 0;

   for (idx, c) in s.char_indices() {
      if c.is_ascii_digit() {
         digits_end = idx + 1;
      } else {
         break;
      }
   }

   /* The RUST way
   let digits_end = s
      .char_indices()
      .find_map(|(idx, c)| if c.is_ascii_digit() { None } else { Some(idx) })
      .unwrap_or_else(|| s.len());

   let digits = &s[..digits_end];
   let remainder = &s[digits_end..];
   (remainder, digits)
   */

   let digits = &s[..digits_end];
   let remainder = &s[digits_end..];
   (remainder, digits)
}


pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
   let mut whitespace_end = 0;

   for (_idx, c) in s.char_indices() {
      if c.is_whitespace() {
         whitespace_end += 1;
      } else {
         break;
      }
   }

   (&s[..whitespace_end], &s[whitespace_end..])
}


mod tests {
   use crate::utils::{extract_digits, extract_op};

   #[test]
   fn extract_plus() {
      assert_eq!(extract_op("+2"), ("2", "+"));
   }

   #[test]
   fn extract_minus() {
      assert_eq!(extract_op("-10"), ("10", "-"));
   }

   #[test]
   fn extract_star() {
      assert_eq!(extract_op("*3"), ("3", "*"));
   }

   #[test]
   fn extract_slash() {
      assert_eq!(extract_op("/4"), ("4", "/"));
   }

   #[test]
   fn do_not_extract_anything_from_empty_input() {
      assert_eq!(extract_digits(""), ("", ""));
   }

   #[test]
   fn extract_digits_with_no_remainder() {
      assert_eq!(extract_digits("100"), ("", "100"));
   }
   #[test]
   fn extract_one_digit() {
      assert_eq!(extract_digits("1+2"), ("+2", "1"));
   }

   #[test]
   fn tag_word() {
   assert_eq!(tag("rn", "rn a"), " a");
   }
}
