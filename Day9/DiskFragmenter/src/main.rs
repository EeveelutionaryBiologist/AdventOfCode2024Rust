use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum MemoryAddress {
    FileNumber(u16),
    Empty,
}

fn read_disk_map() -> Vec<u16> {
    let input = fs::read_to_string("disk_map.txt").expect("There is no map file?");
    let data: Vec<u16> = input
        .lines()
        .flat_map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u16))
        })
        .collect();

    data
}

fn expand_disk_map(data: Vec<u16>) -> Vec<MemoryAddress> {
    let mut disk_map: Vec<MemoryAddress> = Vec::new();
    let mut filenumber: u16 = 0;
    let mut is_file: bool = true;

    for entry in data.iter() {
        if is_file {
            for _i in 0..*entry {
                disk_map.push(MemoryAddress::FileNumber(filenumber));
            }
            is_file = false;
            filenumber += 1;
        } else {
            for _i in 0..*entry {
                disk_map.push(MemoryAddress::Empty);
            }
            is_file = true;
        }
    }
    disk_map
}

fn refomat_disk(mut disk_map: Vec<MemoryAddress>) -> Vec<MemoryAddress> {
    let mut i: usize = 0;
    let mut j: usize = disk_map.len() - 1;

    while i < j {
        // print_disk_map(&disk_map);
        let address = disk_map[i];

        match address {
            MemoryAddress::Empty => {
                while disk_map[j] == MemoryAddress::Empty {
                    j -= 1;
                }
                if let MemoryAddress::FileNumber(filenumber) = disk_map[j]
                    && (j > i)
                {
                    disk_map.swap(i, j);
                }
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    disk_map
}

fn calculate_checksum(disk_map: &Vec<MemoryAddress>) -> u64 {
    let mut checksum: u64 = 0;

    for (i, address) in disk_map.iter().enumerate() {
        match address {
            MemoryAddress::Empty => {}
            MemoryAddress::FileNumber(filenumber) => {
                checksum += (*filenumber as u64) * (i as u64);
            }
        }
    }
    checksum
}

fn print_disk_map(disk_map: &Vec<MemoryAddress>) {
    for address in disk_map.iter() {
        match address {
            MemoryAddress::Empty => print!("."),
            MemoryAddress::FileNumber(filenumber) => print!("{}", filenumber),
        }
    }
    println!();
}

fn main() {
    let data = read_disk_map();
    let mut disk_map = expand_disk_map(data);
    print_disk_map(&disk_map);

    disk_map = refomat_disk(disk_map);
    print_disk_map(&disk_map);

    let checksum = calculate_checksum(&disk_map);
    println!("Checksum of file system: {}", checksum);
}
