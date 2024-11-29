pub struct Heatmap {
    pub pawns_start: [i32; 64],
    pub pawns_end: [i32; 64],
    pub knights: [i32; 64],
    pub bishops: [i32; 64],
    pub rooks: [i32; 64],
    pub queens: [i32; 64],
    pub kings_start: [i32; 64],
    pub kings_end: [i32; 64],
}

impl Default for Heatmap {
    /// Returns the heatmaps for white.
    /// Use .rev() to get the heatmaps for black.
    fn default() -> Self {
        let pawns_start = [
            0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10,
            5, 5, 10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5,
            10, 10, -20, -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let pawns_end = [
            0, 0, 0, 0, 0, 0, 0, 0, 80, 80, 80, 80, 80, 80, 80, 80, 50, 50, 50, 50, 50, 50, 50, 50,
            30, 30, 30, 30, 30, 30, 30, 30, 20, 20, 20, 20, 20, 20, 20, 20, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let knights = [
            -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15,
            15, 10, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5,
            10, 15, 15, 10, 5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30,
            -40, -50,
        ];

        let bishops = [
            -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10,
            5, 0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10,
            10, 10, 10, 10, -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10,
            -20,
        ];

        let rooks = [
            0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0,
            0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
            0, 0, -5, 0, 0, 0, 5, 5, 0, 0, 0,
        ];

        let queens = [
            -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5,
            0, -10, -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10,
            -10, 0, 5, 0, 0, 0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
        ];

        let kings_start = [
            -80, -70, -70, -70, -70, -70, -70, -80, -60, -60, -60, -60, -60, -60, -60, -60, -40,
            -50, -50, -60, -60, -50, -50, -40, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30,
            -30, -40, -40, -30, -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, -5, -5,
            -5, -5, 20, 20, 20, 30, 10, 0, 0, 10, 30, 20,
        ];

        let kings_end = [
            -20, -10, -10, -10, -10, -10, -10, -20, -5, 0, 5, 5, 5, 5, 0, -5, -10, -5, 20, 30, 30,
            20, -5, -10, -15, -10, 35, 45, 45, 35, -10, -15, -20, -15, 30, 40, 40, 30, -15, -20,
            -25, -20, 20, 25, 25, 20, -20, -25, -30, -25, 0, 0, 0, 0, -25, -30, -50, -30, -30, -30,
            -30, -30, -30, -50,
        ];

        Self {
            pawns_start,
            pawns_end,
            knights,
            bishops,
            rooks,
            queens,
            kings_start,
            kings_end,
        }
    }
}

impl Heatmap {
    /// Returns the heatmap for black, if the provided
    /// heatmap is for white, and vice versa.
    pub fn rev(&self) -> Self {
        Self {
            pawns_start: reverse_array(self.pawns_start),
            pawns_end: reverse_array(self.pawns_end),
            knights: reverse_array(self.knights),
            bishops: reverse_array(self.bishops),
            rooks: reverse_array(self.rooks),
            queens: reverse_array(self.queens),
            kings_start: reverse_array(self.kings_start),
            kings_end: reverse_array(self.kings_end),
        }
    }
}

fn reverse_array(arr: [i32; 64]) -> [i32; 64] {
    let mut ret = [0; 64];

    // All heatmaps are symmetric about the central vertical axis.
    for (idx, v) in arr.into_iter().rev().enumerate() {
        ret[idx] = v;
    }
    ret
}
