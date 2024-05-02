use std::collections::HashMap;

pub fn get_starting_tiles() -> Vec<char> {
    [
        " ".repeat(2),

        "E".repeat(12),
        "A".repeat(9),
        "I".repeat(9),
        "O".repeat(8),
        "N".repeat(6),
        "R".repeat(6),
        "T".repeat(6),
        "L".repeat(4),
        "S".repeat(4),
        "U".repeat(4),

        "D".repeat(4),
        "G".repeat(3),

        "B".repeat(2),
        "C".repeat(2),
        "M".repeat(2),
        "P".repeat(2),

        "F".repeat(2),
        "H".repeat(2),
        "V".repeat(2),
        "W".repeat(2),
        "Y".repeat(2),

        "K".repeat(1),

        "J".repeat(1),
        "X".repeat(1),

        "Q".repeat(1),
        "Z".repeat(1),
    ].join("").chars().collect()
}

pub fn get_starting_tile_points() -> HashMap<char, usize> {
    HashMap::from_iter([
        (' ', 0),

        ('E', 1),
        ('A', 1),
        ('I', 1),
        ('O', 1),
        ('N', 1),
        ('R', 1),
        ('T', 1),
        ('L', 1),
        ('S', 1),
        ('U', 1),

        ('D', 2),
        ('G', 2),

        ('B', 3),
        ('C', 3),
        ('M', 3),
        ('P', 3),

        ('F', 4),
        ('H', 4),
        ('V', 4),
        ('W', 4),
        ('Y', 4),

        ('K', 5),

        ('J', 8),
        ('X', 8),

        ('Q', 10),
        ('Z', 10),
    ])
}