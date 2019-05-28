use crate::engine::info::Info;
use crate::engine::score::Score;
use crate::timer::timer::Timer;
use chess::ChessMove;

pub struct SearchInfo {
    depth: Option<u64>,
    seldepth: Option<u64>,
    nodes: Option<u64>,
    pv: Vec<ChessMove>,
    multi_pv: Option<u64>,
    score: Option<Score>,
    tb_hits: Option<u64>,
    cur_line: Vec<ChessMove>,
    engine_string: Option<String>,
}

impl SearchInfo {
    pub fn new() -> SearchInfo {
        SearchInfo {
            depth: None,
            seldepth: None,
            nodes: None,
            pv: vec![],
            multi_pv: None,
            score: None,
            tb_hits: None,
            cur_line: vec![],
            engine_string: None,
        }
    }

    pub fn set_depth(&mut self, depth: u64) {
        self.depth = Some(depth);
    }

    pub fn set_seldepth(&mut self, seldepth: u64) {
        self.seldepth = Some(seldepth);
    }

    pub fn set_nodes(&mut self, nodes: u64) {
        self.nodes = Some(nodes);
    }

    pub fn set_pv(&mut self, pv: Vec<ChessMove>) {
        self.pv = pv;
    }

    pub fn set_multi_pv(&mut self, multi_pv: u64) {
        self.multi_pv = Some(multi_pv);
    }

    pub fn set_score(&mut self, score: Score) {
        self.score = Some(score);
    }

    pub fn set_tb_hits(&mut self, tb_hits: u64) {
        self.tb_hits = Some(tb_hits);
    }

    pub fn set_cur_line(&mut self, cur_line: Vec<ChessMove>) {
        self.cur_line = cur_line;
    }

    pub fn set_engine_string(&mut self, engine_string: String) {
        self.engine_string = Some(engine_string);
    }

    pub fn create_engine_info(self, timer: Timer) -> Info {
        let elapsed = timer.elapsed();

        let mut info = Info::default();

        if let Some(depth) = self.depth {
            info = info.combine(&Info::depth(depth));
        }

        if let Some(seldepth) = self.seldepth {
            info = info.combine(&Info::seldepth(seldepth));
        }

        if let Some(nodes) = self.nodes {
            info = info.combine(&Info::nodes(nodes));
            if let Some(e) = elapsed {
                let nanos = e.as_secs() * 1_000_000_000 + (e.subsec_nanos() as u64);
                info = info.combine(&Info::nps(nodes * 1_000_000_000 / nanos));
            }
        }

        if let Some(tb_hits) = self.tb_hits {
            info = info.combine(&Info::tb_hits(tb_hits));
        }

        if self.pv.len() > 0 {
            info = info.combine(&Info::pv(self.pv.clone()));
        }

        if let Some(mpv) = self.multi_pv {
            info = info.combine(&Info::multi_pv(mpv));
        }

        if let Some(score) = self.score {
            info = info.combine(&Info::score(score));
        }

        if self.cur_line.len() > 0 {
            info = info.combine(&Info::cur_line(self.cur_line.clone()));
        }

        if let Some(engine_string) = self.engine_string {
            info = info.combine(&Info::engine_string(engine_string.clone()));
        }

        info
    }
}

#[cfg(test)]
use std::time::Duration;

#[test]
fn convert_to_info() {
    let mut search_info = SearchInfo::new();
    search_info.set_depth(10);
    search_info.set_score(Score::Cp(100));
    search_info.set_nodes(1000);
    search_info.set_pv(vec![ChessMove::default()]);
    search_info.set_multi_pv(0);
    search_info.set_tb_hits(10);
    search_info.set_cur_line(vec![ChessMove::default(), ChessMove::default()]);
    search_info.set_engine_string("Hello, World!!!".to_string());

    let timer = Timer::new_without_increment(Duration::from_millis(1000));

    let info = search_info.create_engine_info(timer);

    let mut desired_info = Info::default();
    desired_info = desired_info.combine(&Info::depth(10));
    desired_info = desired_info.combine(&Info::score(Score::Cp(100)));
    desired_info = desired_info.combine(&Info::nodes(1000));
    desired_info = desired_info.combine(&Info::pv(vec![ChessMove::default()]));
    desired_info = desired_info.combine(&Info::multi_pv(0));
    desired_info = desired_info.combine(&Info::tb_hits(10));
    desired_info = desired_info.combine(&Info::cur_line(vec![
        ChessMove::default(),
        ChessMove::default(),
    ]));
    desired_info = desired_info.combine(&Info::engine_string("Hello, World!!!".to_string()));

    assert_eq!(info, desired_info);
}
