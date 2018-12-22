trait Search<T: TimeManager> {
    // these serve as the getters/setters for the impl
    pub fn get_table(&mut self) -> &mut CacheTable;
    pub fn get_pv_lines(&mut self) -> &mut Vec<(Score, Vec<ChessMove>)>;
    pub fn get_pvs(&mut self) -> &mut usize;
    pub fn get_time_manager(&mut self) -> &mut T;
    pub fn root_pos(&mut self) -> &mut Board;

    // these have default implementations, but you can override them
    pub fn search(&mut self, board: Board, alpha: Score, beta: Score, depth: i32) {
        self.pvs_search(board, alpha, beta, depth);
    }

    pub fn qsearch(&mut self, board: Board, alpha: Score, beta: Score, depth: i32) {
        self.pvs_qsearch(board, alpha, beta, depth);
    }



    pub fn id_search(&mut self) {
        
    }

    pub fn evaluate(&mut self) -> Score;
}
