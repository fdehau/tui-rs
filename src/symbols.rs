macro_rules! const_set {
    (
      $Name:ident {
          full:           $Full:expr,
          seven_eighths:  $SevenEighths:expr,
          three_quarters: $ThreeQuarters:expr,
          five_eighths:   $FiveEighths:expr,
          half:           $Half:expr,
          three_eighths:  $ThreeEighths:expr,
          one_quarter:    $OneQuarter:expr,
          one_eighth:     $OneEighth:expr,
          empty:          $Empty:expr,
      }
  ) => {
        pub const $Name: Set = Set([
            $Empty,
            $OneEighth,
            $OneQuarter,
            $ThreeEighths,
            $Half,
            $FiveEighths,
            $ThreeQuarters,
            $SevenEighths,
            $Full,
        ]);
        /*pub const $Name: Set = Set {
          full:           $Full,
          seven_eighths:  $SevenEighths,
          three_quarters: $ThreeQuarters,
          five_eighths:   $FiveEighths,
          half:           $Half,
          three_eighths:  $ThreeEighths,
          one_quarter:    $OneQuarter,
          one_eighth:     $OneEighth,
          empty:          $Empty,
        };*/
    };
}

pub mod block {
    pub const FULL: &str = "█";
    pub const SEVEN_EIGHTHS: &str = "▉";
    pub const THREE_QUARTERS: &str = "▊";
    pub const FIVE_EIGHTHS: &str = "▋";
    pub const HALF: &str = "▌";
    pub const THREE_EIGHTHS: &str = "▍";
    pub const ONE_QUARTER: &str = "▎";
    pub const ONE_EIGHTH: &str = "▏";
    pub const EMPTY: &str = " ";
    pub const MAP: [&str; 9] = [
        EMPTY,
        ONE_EIGHTH,
        ONE_QUARTER,
        THREE_EIGHTHS,
        HALF,
        FIVE_EIGHTHS,
        THREE_QUARTERS,
        SEVEN_EIGHTHS,
        FULL,
    ];

    pub fn from_level(level: f64) -> &'static str {
        let level = (level * 8.0).round() as usize;
        if level < 9 {
            MAP[level]
        } else {
            EMPTY
        }
    }

    #[derive(Debug, Clone)]
    pub struct Set([&'static str; 9]);

    const_set!(THREE_LEVELS {
        full: FULL,
        seven_eighths: FULL,
        three_quarters: HALF,
        five_eighths: HALF,
        half: HALF,
        three_eighths: HALF,
        one_quarter: HALF,
        one_eighth: EMPTY,
        empty: EMPTY,
    });

    const_set!(NINE_LEVELS {
        full: FULL,
        seven_eighths: SEVEN_EIGHTHS,
        three_quarters: THREE_QUARTERS,
        five_eighths: FIVE_EIGHTHS,
        half: HALF,
        three_eighths: THREE_EIGHTHS,
        one_quarter: ONE_QUARTER,
        one_eighth: ONE_EIGHTH,
        empty: EMPTY,
    });
}

pub mod bar {
    pub const FULL: &str = "█";
    pub const SEVEN_EIGHTHS: &str = "▇";
    pub const THREE_QUARTERS: &str = "▆";
    pub const FIVE_EIGHTHS: &str = "▅";
    pub const HALF: &str = "▄";
    pub const THREE_EIGHTHS: &str = "▃";
    pub const ONE_QUARTER: &str = "▂";
    pub const ONE_EIGHTH: &str = "▁";
    pub const EMPTY: &str = " ";

    #[derive(Debug, Clone)]
    pub struct Set([&'static str; 9]);

    impl Set {
        pub fn symbol(&self, level: usize) -> &'static str {
            if level < 8 {
                self.0[level]
            } else {
                self.0[8]
            }
        }
    }

    impl Default for Set {
        fn default() -> Self {
            NINE_LEVELS
        }
    }

    const_set!(THREE_LEVELS {
        full: FULL,
        seven_eighths: FULL,
        three_quarters: HALF,
        five_eighths: HALF,
        half: HALF,
        three_eighths: HALF,
        one_quarter: HALF,
        one_eighth: EMPTY,
        empty: EMPTY,
    });

    const_set!(NINE_LEVELS {
        full: FULL,
        seven_eighths: SEVEN_EIGHTHS,
        three_quarters: THREE_QUARTERS,
        five_eighths: FIVE_EIGHTHS,
        half: HALF,
        three_eighths: THREE_EIGHTHS,
        one_quarter: ONE_QUARTER,
        one_eighth: ONE_EIGHTH,
        empty: EMPTY,
    });
}

pub mod line {
    pub const VERTICAL: &str = "│";
    pub const DOUBLE_VERTICAL: &str = "║";
    pub const THICK_VERTICAL: &str = "┃";

    pub const HORIZONTAL: &str = "─";
    pub const DOUBLE_HORIZONTAL: &str = "═";
    pub const THICK_HORIZONTAL: &str = "━";

    pub const TOP_RIGHT: &str = "┐";
    pub const ROUNDED_TOP_RIGHT: &str = "╮";
    pub const DOUBLE_TOP_RIGHT: &str = "╗";
    pub const THICK_TOP_RIGHT: &str = "┓";

    pub const TOP_LEFT: &str = "┌";
    pub const ROUNDED_TOP_LEFT: &str = "╭";
    pub const DOUBLE_TOP_LEFT: &str = "╔";
    pub const THICK_TOP_LEFT: &str = "┏";

    pub const BOTTOM_RIGHT: &str = "┘";
    pub const ROUNDED_BOTTOM_RIGHT: &str = "╯";
    pub const DOUBLE_BOTTOM_RIGHT: &str = "╝";
    pub const THICK_BOTTOM_RIGHT: &str = "┛";

    pub const BOTTOM_LEFT: &str = "└";
    pub const ROUNDED_BOTTOM_LEFT: &str = "╰";
    pub const DOUBLE_BOTTOM_LEFT: &str = "╚";
    pub const THICK_BOTTOM_LEFT: &str = "┗";

    pub const VERTICAL_LEFT: &str = "┤";
    pub const DOUBLE_VERTICAL_LEFT: &str = "╣";
    pub const THICK_VERTICAL_LEFT: &str = "┫";

    pub const VERTICAL_RIGHT: &str = "├";
    pub const DOUBLE_VERTICAL_RIGHT: &str = "╠";
    pub const THICK_VERTICAL_RIGHT: &str = "┣";

    pub const HORIZONTAL_DOWN: &str = "┬";
    pub const DOUBLE_HORIZONTAL_DOWN: &str = "╦";
    pub const THICK_HORIZONTAL_DOWN: &str = "┳";

    pub const HORIZONTAL_UP: &str = "┴";
    pub const DOUBLE_HORIZONTAL_UP: &str = "╩";
    pub const THICK_HORIZONTAL_UP: &str = "┻";

    pub const CROSS: &str = "┼";
    pub const DOUBLE_CROSS: &str = "╬";
    pub const THICK_CROSS: &str = "╋";

    #[derive(Clone, Copy, Debug)]
    pub struct Set {
        pub vertical: &'static str,
        pub horizontal: &'static str,
        pub top_right: &'static str,
        pub top_left: &'static str,
        pub bottom_right: &'static str,
        pub bottom_left: &'static str,
        pub vertical_left: &'static str,
        pub vertical_right: &'static str,
        pub horizontal_down: &'static str,
        pub horizontal_up: &'static str,
        pub cross: &'static str,
    }

    impl Default for Set {
        fn default() -> Self {
            NORMAL
        }
    }

    pub const NORMAL: Set = Set {
        vertical: VERTICAL,
        horizontal: HORIZONTAL,
        top_right: TOP_RIGHT,
        top_left: TOP_LEFT,
        bottom_right: BOTTOM_RIGHT,
        bottom_left: BOTTOM_LEFT,
        vertical_left: VERTICAL_LEFT,
        vertical_right: VERTICAL_RIGHT,
        horizontal_down: HORIZONTAL_DOWN,
        horizontal_up: HORIZONTAL_UP,
        cross: CROSS,
    };

    pub const ROUNDED: Set = Set {
        top_right: ROUNDED_TOP_RIGHT,
        top_left: ROUNDED_TOP_LEFT,
        bottom_right: ROUNDED_BOTTOM_RIGHT,
        bottom_left: ROUNDED_BOTTOM_LEFT,
        ..NORMAL
    };

    pub const DOUBLE: Set = Set {
        vertical: DOUBLE_VERTICAL,
        horizontal: DOUBLE_HORIZONTAL,
        top_right: DOUBLE_TOP_RIGHT,
        top_left: DOUBLE_TOP_LEFT,
        bottom_right: DOUBLE_BOTTOM_RIGHT,
        bottom_left: DOUBLE_BOTTOM_LEFT,
        vertical_left: DOUBLE_VERTICAL_LEFT,
        vertical_right: DOUBLE_VERTICAL_RIGHT,
        horizontal_down: DOUBLE_HORIZONTAL_DOWN,
        horizontal_up: DOUBLE_HORIZONTAL_UP,
        cross: DOUBLE_CROSS,
    };

    pub const THICK: Set = Set {
        vertical: THICK_VERTICAL,
        horizontal: THICK_HORIZONTAL,
        top_right: THICK_TOP_RIGHT,
        top_left: THICK_TOP_LEFT,
        bottom_right: THICK_BOTTOM_RIGHT,
        bottom_left: THICK_BOTTOM_LEFT,
        vertical_left: THICK_VERTICAL_LEFT,
        vertical_right: THICK_VERTICAL_RIGHT,
        horizontal_down: THICK_HORIZONTAL_DOWN,
        horizontal_up: THICK_HORIZONTAL_UP,
        cross: THICK_CROSS,
    };
}

pub const DOT: &str = "•";

pub mod braille {
    pub const BLANK: u16 = 0x2800;
    pub const DOTS: [[u16; 2]; 4] = [
        [0x0001, 0x0008],
        [0x0002, 0x0010],
        [0x0004, 0x0020],
        [0x0040, 0x0080],
    ];
}

/// Marker to use when plotting data points
#[derive(Debug, Clone, Copy)]
pub enum Marker {
    /// One point per cell in shape of dot
    Dot,
    /// One point per cell in shape of a block
    Block,
    /// Up to 8 points per cell
    Braille,
}

impl Default for Marker {
    fn default() -> Self {
        Self::Dot
    }
}
