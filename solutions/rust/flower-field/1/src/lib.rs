use core::str;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut gardenmap: Vec<String> = Vec::new();

    if garden.is_empty() { return gardenmap; }

    let rows = garden.len();
    let cols = garden[0].len();

    // Matrix with an extra "border" of blank spaces
    let mut vvv = vec![vec![32u8; cols + 2]; rows + 2];
    // Matrix that will hold the values before being turned into strings
    let mut www = vec![vec![0u8; cols]; rows];

    // (1, 1) is the center; the location we are trying to find how many mines are around it
    // The locations cover the 8 positions aound the central (1, 1)
    let locations = [(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)];

    for r in 0..rows {
        let row = garden[r].as_bytes();
        vvv[r + 1][1..(cols + 1)].copy_from_slice(&row[..cols]);
    }

    for r in 0..rows {
        for c in 0..cols {

            if vvv[r + 1][c + 1] == 42 {
                www[r][c] = 42;
                continue;
            }

            for loc in locations {
                if vvv[r + loc.0][c + loc.1] == 42 { www[r][c] += 1; }
            }

            // If there are no mines around it, make it a blank space
            if www[r][c] == 0 { www[r][c] = 32; } 
            // Otherwise, add 48 to make it an ASCII character for the number
            else { www[r][c] += 48; } 
        }
    }

    for row in www {
        gardenmap.push(str::from_utf8(&row).unwrap().to_string());
    }

    gardenmap
}
