use std::collections::HashMap;

pub fn mode(list: &Vec<i32>) -> Vec<i32> {
    if list.len() == 0 {
        return Vec::new();
    }

    let mode_map = build_mode_hashmap(list);
    get_modes(&mode_map)
}

pub fn median(list: &Vec<i32>) -> Option<i32> {
    if list.len() == 0 {
        return None;
    }

    let mut cloned = list.clone();
    cloned.sort();

    Some(cloned[get_median_index(cloned.len())])
}

fn build_mode_hashmap(list: &Vec<i32>) -> HashMap<i32, u32> {
    let mut map = HashMap::new();

    for val in list {
        let prev_count = map.entry(*val).or_insert(1);
        *prev_count += 1;
    }

    map
}

fn get_modes(map: &HashMap<i32, u32>) -> Vec<i32> {
    let mut max_seen = 0;
    let mut modes = Vec::new();

    for (num, times_seen) in map {
        if *times_seen > max_seen {
            max_seen = *times_seen;
            modes = vec![*num];
        } else if *times_seen == max_seen {
            modes.push(*num);
        }
    }

    modes
}

fn get_median_index(element_count: usize) -> usize {
    element_count / 2
}