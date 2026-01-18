use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum MemoryAddress {
    FileNumber(u16),
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct MemorySegment {
    filenumber: Option<u16>,
    length: u16,
}

impl MemorySegment {
    fn print_length(&self) {
        print!("{}", self.length);
    }
    fn print_segment(&self) {
        let token = match self.filenumber {
            Some(val) => val.to_string(),
            None => ".".to_string(),
        };
        for _i in 0..self.length {
            print!("{}", token);
        }
    }
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

fn print_segment_map(segment_map: &Vec<MemorySegment>) {
    for segment in segment_map.iter() {
        segment.print_segment();
    }
    println!();
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

fn expand_disk_map(data: &Vec<u16>) -> Vec<MemoryAddress> {
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

fn refomat_disk_single_address(mut disk_map: Vec<MemoryAddress>) -> Vec<MemoryAddress> {
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
                if let MemoryAddress::FileNumber(_filenumber) = disk_map[j]
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

fn parse_to_memory_segmentation(data: &Vec<u16>) -> Vec<MemorySegment> {
    let mut segment_map: Vec<MemorySegment> = Vec::new();
    let mut filenumber: u16 = 0;
    let mut is_file: bool = true;

    for entry in data.iter() {
        if is_file {
            segment_map.push(MemorySegment {
                filenumber: Some(filenumber),
                length: *entry,
            });
            is_file = false;
            filenumber += 1;
        } else {
            segment_map.push(MemorySegment {
                filenumber: None,
                length: *entry,
            });
            is_file = true;
        }
    }
    segment_map
}

fn reformat_segmentation_map(mut segment_map: Vec<MemorySegment>) -> Vec<MemorySegment> {
    let max_id = segment_map
        .iter()
        .filter_map(|s| s.filenumber)
        .max()
        .unwrap_or(0);

    for id_to_move in (0..=max_id).rev() {
        let file_idx = segment_map
            .iter()
            .position(|s| s.filenumber == Some(id_to_move))
            .unwrap();
        let file_len = segment_map[file_idx].length;

        // Okay, so basically we work our way down from the highest-number
        // file and look for the leftmost gap that would fit it -
        let mut target_gap_idx = None;
        for i in 0..file_idx {
            if segment_map[i].filenumber.is_none() && segment_map[i].length >= file_len {
                target_gap_idx = Some(i);
                break;
            }
        }

        // When we have a gap, we execute the transfer logic, which boils down to:
        // (1) If the file and gap are equal in size, just swap
        // (2) Otherwise, change the gap length to the file length and make the previous gap ID (None) the file ID
        // (3) Mark the previous file position as Empty
        // (4) inserting a new Empty segment on the next index to account for leftover space
        if let Some(gap_idx) = target_gap_idx {
            let gap_len = segment_map[gap_idx].length;

            if gap_len == file_len {
                segment_map[gap_idx].filenumber = Some(id_to_move);
                segment_map[file_idx].filenumber = None;
            } else {
                segment_map[gap_idx].filenumber = Some(id_to_move);
                segment_map[gap_idx].length = file_len;
                segment_map[file_idx].filenumber = None;

                segment_map.insert(
                    gap_idx + 1,
                    MemorySegment {
                        filenumber: None,
                        length: gap_len - file_len,
                    },
                );
            }
        }
    }
    segment_map
}

fn calculate_checksum_diskmap(disk_map: &Vec<MemoryAddress>) -> u64 {
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

fn calculate_checksum_segmentmap(segment_map: &Vec<MemorySegment>) -> u64 {
    let mut checksum: u64 = 0;
    let mut idx: u64 = 0;

    for (_i, segment) in segment_map.iter().enumerate() {
        for _j in 0..segment.length {
            if let Some(filenumber) = segment.filenumber {
                checksum += idx * (filenumber as u64);
            }
            idx += 1;
        }
    }
    checksum
}

fn main() {
    let data = read_disk_map();

    // Part I: "Naive" approach, every cell in memory is treated individually
    let mut disk_map = expand_disk_map(&data);
    //print_disk_map(&disk_map);

    disk_map = refomat_disk_single_address(disk_map);
    //print_disk_map(&disk_map);

    println!(
        "Checksum of file system (single address reordering): {}",
        calculate_checksum_diskmap(&disk_map)
    );

    // Part II: Parse data to a segmentation representation to make
    // swapping based on actual length easier.
    let mut segment_map = parse_to_memory_segmentation(&data);
    //print_segment_map(&segment_map);

    segment_map = reformat_segmentation_map(segment_map);
    //print_segment_map(&segment_map);

    println!(
        "Checksum of file system (segmentation reordering): {}",
        calculate_checksum_segmentmap(&segment_map)
    );
}
