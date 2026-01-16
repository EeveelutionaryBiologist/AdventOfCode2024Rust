use std::fs;

fn parse_word_puzzle() -> Vec<Vec<char>> {
    let input = fs::read_to_string("word_search.txt").expect("There is no file, maan.");
    let data: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.replace("\n", "").chars().collect())
        .collect();
    data
}

fn walk_search_xmas(data: &Vec<Vec<char>>, i: usize, j: usize, max_i: usize, max_j: usize) -> i32 {
    let mut local_hits = 0;
    // right
    if j + 3 < max_j {
        if data[i][j + 1] == 'M' && data[i][j + 2] == 'A' && data[i][j + 3] == 'S' {
            local_hits += 1;
        }
    }
    // down-right
    if j + 3 < max_j && i + 3 < max_i {
        if data[i + 1][j + 1] == 'M' && data[i + 2][j + 2] == 'A' && data[i + 3][j + 3] == 'S' {
            local_hits += 1;
        }
    }
    // down
    if i + 3 < max_i {
        if data[i + 1][j] == 'M' && data[i + 2][j] == 'A' && data[i + 3][j] == 'S' {
            local_hits += 1;
        }
    }
    // down-left
    if i + 3 < max_i && j >= 3 {
        if data[i + 1][j - 1] == 'M' && data[i + 2][j - 2] == 'A' && data[i + 3][j - 3] == 'S' {
            local_hits += 1;
        }
    }
    // left
    if j >= 3 {
        if data[i][j - 1] == 'M' && data[i][j - 2] == 'A' && data[i][j - 3] == 'S' {
            local_hits += 1;
        }
    }
    // up-left
    if j >= 3 && i >= 3 {
        if data[i - 1][j - 1] == 'M' && data[i - 2][j - 2] == 'A' && data[i - 3][j - 3] == 'S' {
            local_hits += 1;
        }
    }
    // up
    if i >= 3 {
        if data[i - 1][j] == 'M' && data[i - 2][j] == 'A' && data[i - 3][j] == 'S' {
            local_hits += 1;
        }
    }
    // up-right
    if j + 3 < max_j && i >= 3 {
        if data[i - 1][j + 1] == 'M' && data[i - 2][j + 2] == 'A' && data[i - 3][j + 3] == 'S' {
            local_hits += 1;
        }
    }
    local_hits
}

fn match_search_x_mas(data: &Vec<Vec<char>>, i: usize, j: usize, max_i: usize, max_j: usize) -> i32 {
    if i < 1 || i + 1 >= max_i || j < 1 || j + 1 >= max_j {
        return 0;
    }

    let corners = (
        data[i - 1][j - 1], // up-Left
        data[i - 1][j + 1], // up-Right
        data[i + 1][j - 1], // down-Left
        data[i + 1][j + 1], // down-Right
    );

    match corners {
        ('M', 'M', 'S', 'S') => 1,
        ('M', 'S', 'M', 'S') => 1,
        ('S', 'M', 'S', 'M') => 1,
        ('S', 'S', 'M', 'M') => 1,
        _ => 0,
    }
}

fn main() {
    let data = parse_word_puzzle();
    let start_char: char = 'X';
    let center_char: char = 'A';
    let mut xmas_hits: i32 = 0;
    let mut x_mas_hits: i32 = 0;

    let max_i = data.len();
    let max_j = data[0].len();

    for (i, row) in data.iter().enumerate() {
        for (j, _character) in row.iter().enumerate() {
            if row[j] == start_char {
                xmas_hits += walk_search_xmas(&data, i, j, max_i, max_j)
            }
            if row[j] == center_char {
                x_mas_hits += match_search_x_mas(&data, i, j, max_i, max_j)
            }
        }
    }
    println!("XMAS instances: {}", xmas_hits);
    println!("X-MAS instances: {}", x_mas_hits);
}
