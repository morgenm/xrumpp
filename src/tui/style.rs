use termbox_sys;

#[derive(Debug)]
pub struct Style {
    /// Termbox fg.
    pub fg  : u16,

    /// Termbox bg.
    pub bg  : u16,

    /// String representation to be used in MsgArea etc. this is how the color
    /// is encoded in (hopefully) most IRC clients.
    pub str : &'static StyleStr<'static>,
}

pub static USER_MSG : Style =
    Style {
        fg: termbox_sys::TB_WHITE,
        bg: termbox_sys::TB_DEFAULT,
        str: &USER_MSG_SS,
    };

pub static SERVER_MSG : Style =
    Style {
        fg: termbox_sys::TB_BLUE | termbox_sys::TB_BOLD,
        bg: termbox_sys::TB_DEFAULT,
        str: &SERVER_MSG_SS,
    };

pub static ERR_MSG : Style =
    Style {
        fg: termbox_sys::TB_WHITE | termbox_sys::TB_BOLD,
        bg: termbox_sys::TB_RED,
        str: &ERR_MSG_SS,
    };


pub static TOPIC : Style =
    Style {
        fg: termbox_sys::TB_BLACK,
        bg: termbox_sys::TB_GREEN,
        str: &TOPIC_SS,
    };

pub static CLEAR : Style =
    Style {
        fg: termbox_sys::TB_WHITE,
        bg: termbox_sys::TB_BLACK,
        str: &CLEAR_SS,
    };

// Colors described in http://en.wikichip.org/wiki/irc/colors
// These need to be macros because it's not possible to concatenate const string
// variables in compile time to get other const strings.
macro_rules! white   { () => { "00" } }
macro_rules! black   { () => { "01" } }
macro_rules! navy    { () => { "02" } }
macro_rules! green   { () => { "03" } }
macro_rules! red     { () => { "04" } }
macro_rules! marron  { () => { "05" } }
macro_rules! purple  { () => { "06" } }
macro_rules! olive   { () => { "07" } }
macro_rules! yellow  { () => { "08" } }
macro_rules! lgreen  { () => { "09" } }
macro_rules! tea     { () => { "10" } }
macro_rules! cyan    { () => { "11" } }
macro_rules! blue    { () => { "12" } }
macro_rules! magenta { () => { "13" } }
macro_rules! gray    { () => { "14" } }
macro_rules! lgray   { () => { "15" } }

macro_rules! reset_prefix { () => { "\x0F" } }
macro_rules! bold_prefix  { () => { "\x02" } }
macro_rules! color_prefix { () => { "\x03" } }

pub const RESET_PREFIX : char = '\x0F';
pub const BOLD_PREFIX  : char = '\x02';
pub const COLOR_PREFIX : char = '\x03';

#[derive(Debug)]
pub struct StyleStr<'a>(pub &'a str);

pub static USER_MSG_SS   : StyleStr<'static> =
    StyleStr(concat!(color_prefix!(), white!()));
pub static SERVER_MSG_SS : StyleStr<'static> =
    StyleStr(concat!(bold_prefix!(), color_prefix!(), blue!()));
pub static ERR_MSG_SS    : StyleStr<'static> =
    StyleStr(concat!(bold_prefix!(), color_prefix!(), white!(), ",", red!()));
pub static TOPIC_SS      : StyleStr<'static> =
    StyleStr(concat!(color_prefix!(), black!(), ",", green!()));
pub static CLEAR_SS      : StyleStr<'static> =
    StyleStr(concat!(color_prefix!(), white!(), ",", black!()));