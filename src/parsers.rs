use std::str::FromStr;
use nom::digit;
use chess::{ChessMove, Square, Piece, Rank, File, Board};

named!(pub parse_rank<&str, Rank>, do_parse!(
        r: alt!(
            value!(Rank::First, tag!("1")) |
            value!(Rank::Second, tag!("2")) |
            value!(Rank::Third, tag!("3")) |
            value!(Rank::Fourth, tag!("4")) |
            value!(Rank::Fifth, tag!("5")) |
            value!(Rank::Sixth, tag!("6")) |
            value!(Rank::Seventh, tag!("7")) |
            value!(Rank::Eighth, tag!("8"))
        ) >>
        (r)
    )
);

named!(pub parse_file<&str, File>, do_parse!(
        f: alt!(
            value!(File::A, tag!("a")) |
            value!(File::B, tag!("b")) |
            value!(File::C, tag!("c")) |
            value!(File::D, tag!("d")) |
            value!(File::E, tag!("e")) |
            value!(File::F, tag!("f")) |
            value!(File::G, tag!("g")) |
            value!(File::H, tag!("h"))
        ) >>
        (f)
    )
);

named!(pub parse_square<&str, Square>, do_parse!(
        f: parse_file >>
        r: parse_rank >>
        (Square::make_square(r, f))
    )
);

named!(pub parse_promotion_piece<&str, Option<Piece>>, do_parse!(
        p: opt!(alt_complete!(
            value!(Piece::Knight, tag!("n")) |
            value!(Piece::Bishop, tag!("b")) |
            value!(Piece::Rook, tag!("r")) |
            value!(Piece::Queen, tag!("q"))
        )) >>
        (p)
    )
);



named!(pub parse_move<&str, ChessMove>, do_parse!(
        s1: parse_square >>
        s2: parse_square >>
        promotion: parse_promotion_piece >>
        (ChessMove::new(s1, s2, promotion))
    )
);

named!(pub parse_move_space<&str, ChessMove>, do_parse!(
        s1: parse_square >>
        s2: parse_square >>
        promotion: parse_promotion_piece >>
        space >>
        (ChessMove::new(s1, s2, promotion))
    )
);

named!(pub space<&str, &str>, eat_separator!(" \t\r\n"));
named!(pub non_newline_space<&str, &str>, eat_separator!(" \t\r"));

named!(pub parse_fen<&str, Board>, do_parse!(
        x: do_parse!(
            board: take_while_s!(|y| "pPnNbBrRqQkK12345678/".contains(y)) >>
            space >>
            player: alt!(tag!("w") | tag!("b")) >>
            space >>
            castle: take_while_s!(|y| "-kKqQ".contains(y)) >>
            space >>
            ep: take_while_s!(|y| "abcdefgh12345678-".contains(y)) >>
            space >>
            m1: take_while_s!(|y| "0123456789".contains(y)) >>
            space >>
            m2: take_while_s!(|y| "0123456789".contains(y)) >>
            board: expr_opt!(Board::from_fen(board.to_owned() +
                                             " " +
                                             player +
                                             " " +
                                             castle +
                                             " " +
                                             ep +
                                             " " +
                                             m1 +
                                             " " +
                                             m2)) >>
            (board)
        ) >>
        (x)
    )
);

named!(
    pub integer<&str, u64>,
    map_res!(
        digit,
        u64::from_str
    )
);

named!(
    pub parse_i64<&str, i64>,
    map_res!(
        recognize!(
            do_parse!(
                opt!(tag!("-")) >>
                digit >>
                ()
            )
        ),
        |s: &str| s.parse::<i64>()
    )
);

named!(pub parse_movelist<&str, Vec<ChessMove> >, do_parse!(
        moves: fold_many1!(
            alt_complete!(parse_move_space | parse_move),
            Vec::new(),
            |mut acc: Vec<ChessMove>, item: ChessMove| {
                acc.push(item);
                acc
            }
        ) >>
        (moves.to_vec())
    )
);

