use std::collections::HashMap;
use std::collections::HashSet;

// maps an a row or col number to a tuple with the extreme value and the list
// of indices that contain it
type ExtremeMap = HashMap<usize, (u64, Vec<(usize, usize)>)>;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut row_maxs: ExtremeMap = HashMap::new();
    let mut col_mins: ExtremeMap = HashMap::new();

    // store the max of each row and the min of each column in the maps defined above
    for (i, row) in input.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            let index = (i, j);
            process_index(&mut row_maxs, &|extreme| value > extreme, i, value, index);
            process_index(&mut col_mins, &|extreme| value < extreme, j, value, index);
        }
    }

    let max_set = map_to_index_set(row_maxs);
    let min_set = map_to_index_set(col_mins);

    // indices that are both row max and column min are saddle points
    max_set.intersection(&min_set).cloned().collect()
}

/// Check if `value` is a new extreme or equals the current one and update
/// the extreme map accordingly
fn process_index(
    map: &mut ExtremeMap,
    new_extreme: &Fn(u64) -> bool,
    key: usize,
    value: u64,
    index: (usize, usize),
) {
    match map.get_mut(&key) {
        None => {
            map.insert(key, (value, vec![index]));
        }
        Some(&mut (extreme_value, ref mut _indices)) if new_extreme(extreme_value) => {
            map.insert(key, (value, vec![index]));
        }
        Some(&mut (extreme_value, ref mut indices)) if extreme_value == value => {
            indices.push(index);
        }
        _ => {}
    }
}

/// Build a set of (row, column) indices out of an extreme map
fn map_to_index_set(map: ExtremeMap) -> HashSet<(usize, usize)> {
    map.values().fold(HashSet::new(), |mut acc, entry| {
        acc.extend(entry.1.iter());
        acc
    })
}
