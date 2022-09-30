use std::collections::HashMap;

// Just saw programming excercises from the book:
// The Rust Programming Language
#[allow(dead_code)]
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    let mut compare = |n| {
        if n > largest {
            largest = n
        }
    };

    list.iter().for_each(|n| compare(*n));

    largest
}

#[allow(dead_code)]
fn largest_ref<'a, T: PartialOrd + Copy>(list: &'a [T], mut largest: &'a T) -> &'a T {
    largest = &list[0];

    let mut compare = |n: &'a T| {
        if n > largest {
            largest = n;
        }
    };

    list.iter().for_each(|n| compare(n));

    &largest
}

// The first consonant of each word is moved
// to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”)
#[allow(dead_code)]
fn to_pig_latin(word: &str) -> String {
    let mut word_iter = word.chars();
    let mut return_val = String::new();
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let first = word_iter.next().expect("expected word to be at least one.");

    for c in word_iter {
        return_val.push(c);
    }

    if !vowels.contains(&first) {
        return_val = format!("{}{}ay", return_val, first);
    } else {
        return_val = format!("{}{}", first, return_val);
    }

    return_val
}

// given a list of integers, return the mean, median and mode
#[allow(dead_code)]
fn get_mean_median_mode(int_list: &[u32]) -> (f64, u32, u32) {
    let mut map = HashMap::new();

    for i in int_list {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut map_iter = map.into_iter();
    let (mut mode, mut mode_count) = map_iter.next().expect("List is empty!");

    for (curr_mode, curr_mode_count) in map_iter {
        if curr_mode_count > mode_count {
            mode_count = curr_mode_count;
            mode = curr_mode;
        }
    }

    let sum: u32 = int_list.iter().sum();
    let avg: f64 = sum as f64 / int_list.len() as f64;
    let median: u32 = int_list[int_list.len() / 2];

    (avg, median, *mode)
}

// Using a hash map and vectors, create a text interface to allow a user to
// add employee names to a department in a company. For example, “Add
// Sally to Engineering” or “Add Amir to Sales.”
#[allow(dead_code)]
fn add_to_company<'a>(employee: &'a str, map: &mut HashMap<&'a str, &'a str>) {
    let employee_vec: Vec<&str> = employee.split(' ').collect();

    let _ = match *employee_vec {
        ["Add", name, _, department] => map.insert(name, department),
        _ => None,
    };
}

// let the user retrieve a list of all people in a department or all people
// in the company by department, sorted alphabetically.
#[allow(dead_code)]
fn retrieve_employee_list<'a>(
    search_by: Option<&str>,
    map: &HashMap::<&'a str, &'a str>,
    results: &mut Vec<(&'a str, &'a str)>,
) {
    let mut list: Vec<(&str, &str)> = Vec::new();

    if let Some(department) = search_by {
        list = map
            .iter()
            .map(|(&x, &y)| (x, y))
            .filter(|(_employee_name, emp_department)| *emp_department == department)
            .collect();
    } else {
        list = map.iter().map(|(&x, &y)| (x, y)).collect();
    }

    list.sort_unstable_by(|a, b| a.0.cmp(b.0));
    results.extend_from_slice(&list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_mean_median_mode_test() {
        let list = [0, 1, 1, 2, 3, 5, 8];

        let (avg, median, mode) = get_mean_median_mode(&list);

        assert_eq!(avg, 20.0 / 7.0);
        assert_eq!(median, 2);
        assert_eq!(mode, 1);
    }

    #[test]
    fn pig_latin_test() {
        let word = "first";
        let pig_latin = to_pig_latin(word);
        assert_eq!("irstfay", pig_latin);

        let word = "apple";
        let pig_latin = to_pig_latin(word);
        assert_eq!("apple", pig_latin);
    }

    #[test]
    fn add_to_company_test() {
        let employee_one = "Add Sally to Engineering";
        let employee_two = "Add Amir to Sales";

        let mut map: HashMap<&str, &str> = HashMap::new();

        add_to_company(employee_one, &mut map);
        add_to_company(employee_two, &mut map);
        assert_eq!(map.get("Sally").unwrap(), &"Engineering");
        assert_eq!(map.get("Amir").unwrap(), &"Sales");
    }

    #[test]
    fn retrieve_list_people_test() {
        let map: HashMap<&str, &str> = HashMap::from([
            ("Sally", "Engineering"),
            ("Amir", "Sales"),
            ("Zolo", "Sales"),
        ]);

        let mut list: Vec<(&str, &str)> = Vec::new();

        retrieve_employee_list(Some("Sales"), &map, &mut list);
        assert_eq!(list[0], ("Amir", "Sales"));
        assert_eq!(list[1], ("Zolo", "Sales"));
        list.clear();

        retrieve_employee_list(None, &map, &mut list);
        assert_eq!(list[0], ("Amir", "Sales"));
        assert_eq!(list[1], ("Sally", "Engineering"));
        assert_eq!(list[2], ("Zolo", "Sales"));
    }

    #[test]
    fn largest_test() {
        let number_list = [34, 50, 25, 100, 65];
        let largest_int = largest(&number_list);
        assert_eq!(largest_int, 100);

        let char_list = ['y', 'm', 'a', 'q'];
        let largest_char = largest(&char_list);
        assert_eq!(largest_char, 'y');

    }

}
