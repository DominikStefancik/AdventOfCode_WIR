pub struct Xmas {
    pub horizontal_left_to_right_search_count: usize,
    pub horizontal_right_to_left_search_count: usize,
    pub vertical_top_to_bottom_search_count: usize,
    pub vertical_bottom_to_top_search_count: usize,
    pub diagonal_left_to_right_top_to_bottom_search_count: usize,
    pub diagonal_left_to_right_bottom_to_top_search_count: usize,
    pub diagonal_right_to_left_top_to_bottom_search_count: usize,
    pub diagonal_right_to_left_bottom_to_top_search_count: usize,
    pub total_count: usize,
}

impl Xmas {
    pub fn new(
        horizontal_left_to_right_search_count: usize,
        horizontal_right_to_left_search_count: usize,
        vertical_top_to_bottom_search_count: usize,
        vertical_bottom_to_top_search_count: usize,
        diagonal_left_to_right_top_to_bottom_search_count: usize,
        diagonal_left_to_right_bottom_to_top_search_count: usize,
        diagonal_right_to_left_top_to_bottom_search_count: usize,
        diagonal_right_to_left_bottom_to_top_search_count: usize,
    ) -> Self {
        Xmas {
            horizontal_left_to_right_search_count,
            horizontal_right_to_left_search_count,
            vertical_top_to_bottom_search_count,
            vertical_bottom_to_top_search_count,
            diagonal_left_to_right_top_to_bottom_search_count,
            diagonal_left_to_right_bottom_to_top_search_count,
            diagonal_right_to_left_top_to_bottom_search_count,
            diagonal_right_to_left_bottom_to_top_search_count,
            total_count: horizontal_left_to_right_search_count
                + horizontal_right_to_left_search_count
                + vertical_top_to_bottom_search_count
                + vertical_bottom_to_top_search_count
                + diagonal_left_to_right_top_to_bottom_search_count
                + diagonal_left_to_right_bottom_to_top_search_count
                + diagonal_right_to_left_top_to_bottom_search_count
                + diagonal_right_to_left_bottom_to_top_search_count,
        }
    }
}
