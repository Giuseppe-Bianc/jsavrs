//! Formatting utilities for common display patterns.
//!
//! This module provides helper functions that simplify common formatting operations
//! when implementing the [`std::fmt::Display`] trait. These utilities handle
//! repetitive formatting logic such as joining collections with separators.
//!
//! # Use Cases
//!
//! - Implementing `Display` for custom collection types
//! - Formatting lists, tuples, or sequences with consistent separators
//! - Reducing boilerplate in `fmt` implementations
//!
//! # Examples
//!
//! ```rust
//! use std::fmt;
//! use jsavrs::fmtlike::write_comma_separated;
//!
//! struct MyList(Vec<i32>);
//!
//! impl fmt::Display for MyList {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         write_comma_separated(f, &self.0)
//!     }
//! }
//!
//! let list = MyList(vec![1, 2, 3]);
//! assert_eq!(format!("{}", list), "1, 2, 3");
//! ```

use std::fmt;

/// Writes a slice of items to a formatter, separated by commas and spaces.
///
/// This utility function formats each element in the provided slice using its
/// [`Display`](std::fmt::Display) implementation, inserting `", "` between
/// consecutive elements. It is designed to be used within `fmt` method
/// implementations to simplify the formatting of sequences.
///
/// The function iterates through the items, writing the first element without
/// a preceding separator, then prepending `", "` to all subsequent elements.
///
/// # Arguments
///
/// * `f` - A mutable reference to a [`Formatter`](std::fmt::Formatter) that
///   will receive the formatted output. This is typically the formatter passed
///   into a `Display::fmt` or `Debug::fmt` implementation.
/// * `items` - A slice of items to format. Each item must implement the
///   [`Display`](std::fmt::Display) trait. The slice can be empty.
///
/// # Returns
///
/// * `Ok(())` - If all items were successfully formatted and written to the formatter
/// * `Err(fmt::Error)` - If any write operation to the formatter fails
///
/// # Errors
///
/// This function returns an error if:
/// - Writing the separator string `", "` fails
/// - Formatting any individual item via `Display::fmt` fails
///
/// These errors originate from the underlying formatter and typically occur
/// when writing to a finite buffer that runs out of space, though most formatters
/// have unlimited capacity.
///
/// # Examples
///
/// ## Basic usage with integers
///
/// ```rust
/// use std::fmt;
/// use jsavrs::fmtlike::write_comma_separated;
///
/// struct IntList(Vec<i32>);
///
/// impl fmt::Display for IntList {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         f.write_str("[")?;
///         write_comma_separated(f, &self.0)?;
///         f.write_str("]")
///     }
/// }
///
/// let list = IntList(vec![1, 2, 3, 4, 5]);
/// assert_eq!(format!("{}", list), "[1, 2, 3, 4, 5]");
/// ```
///
/// ## Empty slice handling
///
/// ```rust
/// use std::fmt;
/// use jsavrs::fmtlike::write_comma_separated;
///
/// struct EmptyList;
///
/// impl fmt::Display for EmptyList {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write_comma_separated(f, &[] as &[i32])
///     }
/// }
///
/// let empty = EmptyList;
/// assert_eq!(format!("{}", empty), "");
/// ```
///
/// ## With custom types
///
/// ```rust
/// use std::fmt;
/// use jsavrs::fmtlike::write_comma_separated;
///
/// #[derive(Debug)]
/// struct Person { name: String, age: u32 }
///
/// impl fmt::Display for Person {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "{}({})", self.name, self.age)
///     }
/// }
///
/// struct Team(Vec<Person>);
///
/// impl fmt::Display for Team {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write_comma_separated(f, &self.0)
///     }
/// }
///
/// let team = Team(vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
/// ]);
/// assert_eq!(format!("{}", team), "Alice(30), Bob(25)");
/// ```
pub fn write_comma_separated<T: fmt::Display>(f: &mut fmt::Formatter<'_>, items: &[T]) -> fmt::Result {
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            f.write_str(", ")?;
        }
        item.fmt(f)?;
    }
    Ok(())
}
