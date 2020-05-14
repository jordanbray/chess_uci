use chess::{Board, ChessMove, File, Piece, Rank, Square};
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::bytes::streaming::tag;
use nom::character::complete::digit1;
use nom::combinator::{complete, map, opt, value};
use nom::sequence::{pair, tuple};
use nom::IResult;
use nom::{FindToken, InputTakeAtPosition};
use std::str::FromStr;

pub fn parse_rank(input: &str) -> IResult<&str, Rank> {
    alt((
        value(Rank::First, tag("1")),
        value(Rank::Second, tag("2")),
        value(Rank::Third, tag("3")),
        value(Rank::Fourth, tag("4")),
        value(Rank::Fifth, tag("5")),
        value(Rank::Sixth, tag("6")),
        value(Rank::Seventh, tag("7")),
        value(Rank::Eighth, tag("8")),
    ))(input)
}

pub fn parse_file(input: &str) -> IResult<&str, File> {
    alt((
        value(File::A, tag("a")),
        value(File::B, tag("b")),
        value(File::C, tag("c")),
        value(File::D, tag("d")),
        value(File::E, tag("e")),
        value(File::F, tag("f")),
        value(File::G, tag("g")),
        value(File::H, tag("h")),
    ))(input)
}

pub fn parse_square(input: &str) -> IResult<&str, Square> {
    map(pair(parse_file, parse_rank), |(f, r)| {
        Square::make_square(r, f)
    })(input)
}

pub fn parse_promotion_piece(input: &str) -> IResult<&str, Option<Piece>> {
    opt(alt((
        complete(value(Piece::Knight, tag("n"))),
        complete(value(Piece::Bishop, tag("b"))),
        complete(value(Piece::Rook, tag("r"))),
        complete(value(Piece::Queen, tag("q"))),
    )))(input)
}

pub fn parse_move(input: &str) -> IResult<&str, ChessMove> {
    map(
        tuple((parse_square, parse_square, parse_promotion_piece)),
        |(s1, s2, promotion)| (ChessMove::new(s1, s2, promotion)),
    )(input)
}

pub fn parse_move_space(input: &str) -> IResult<&str, ChessMove> {
    map(
        tuple((parse_square, parse_square, parse_promotion_piece, space)),
        |(s1, s2, promotion, _)| (ChessMove::new(s1, s2, promotion)),
    )(input)
}

pub fn space(input: &str) -> IResult<&str, &str> {
    input.split_at_position(|c| !(" \t\r\n").find_token(c))
}

pub fn non_newline_space(input: &str) -> IResult<&str, &str> {
    input.split_at_position(|c| !(" \t\r").find_token(c))
}

pub fn parse_fen(input: &str) -> IResult<&str, Board> {
    map(
        tuple((
            take_while(|y| "pPnNbBrRqQkK12345678/".contains(y)),
            space,
            alt((tag("w"), tag("b"))),
            space,
            take_while(|y| "-kKqQ".contains(y)),
            space,
            take_while(|y| "abcdefgh12345678-".contains(y)),
            space,
            take_while(|y| "0123456789".contains(y)),
            space,
            take_while(|y| "0123456789".contains(y)),
        )),
        |(board, _, player, _, castle, _, ep, _, m1, _, m2)| {
            (Board::from_str(&format!(
                "{} {} {} {} {} {}",
                board, player, castle, ep, m1, m2
            ))
            .unwrap()) // we parsed it therefor it must be a valid fen?
                       // this is risky because Board has another fen parser that might disagree,
                       // however I'm not familiar enough with nom to know how to do this
                       // without massive boilerplate
        },
    )(input)
    .map_err(|_| nom::Err::Failure(("Invalid FEN", nom::error::ErrorKind::Verify)))
}

named!(
    pub integer<&str, u64>,
    map_res!(
        digit1,
        u64::from_str
    )
);

named!(
    pub parse_i64<&str, i64>,
    map_res!(
        recognize!(
            do_parse!(
                opt!(tag!("-")) >>
                digit1 >>
                ()
            )
        ),
        |s: &str| s.parse::<i64>()
    )
);

named!(pub parse_movelist<&str, Vec<ChessMove> >, do_parse!(
        moves: fold_many1!(
            alt!(complete!(parse_move_space) | complete!(parse_move)),
            Vec::new(),
            |mut acc: Vec<ChessMove>, item: ChessMove| {
                acc.push(item);
                acc
            }
        ) >>
        (moves.to_vec())
    )
);

#[test]
fn test_parse_fen_success() {
    let parsed = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let want = Board::default();

    assert_eq!(parsed, Ok(("", want)));
}

#[test]
fn test_parse_fen_failure() {
    let res = parse_fen("not a valid fen");
    let want = nom::Err::Failure(("Invalid FEN", nom::error::ErrorKind::Verify));

    assert_eq!(res, Err(want));
}
