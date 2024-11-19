enum Pixel {
    QuadPixel(QuadPixel),
    HexPixel(HexPixel),
}

impl Pixel {
    fn quad_pixel(u_l: bool, u_r: bool, l_l: bool, l_r: bool) -> Self {
        Pixel::QuadPixel(
            QuadPixel {
                u_l, 
                u_r, 
                l_l, 
                l_r
            }
        )
    }

    fn hex_pixel(u_l: bool, u_r: bool, m_l: bool, m_r: bool, l_l: bool, l_r: bool) -> Self {
        Pixel::HexPixel(
            HexPixel {
                u_l, 
                u_r, 
                m_l, 
                m_r, 
                l_l, 
                l_r
            }
        )
    }

    fn get_char(&self) -> char {
        match self {
            Self::QuadPixel(pix) => pix.get_char(),
            Self::HexPixel(pix) => pix.get_char(),
        }
    }
}

struct QuadPixel {
    u_l: bool,
    u_r: bool,
    l_l: bool,
    l_r: bool,
}

impl QuadPixel {
    fn get_char(&self) -> char {
        match (self.u_l, self.u_r, self.l_l, self.l_r) {
            (false, false, false, false) => ' ',
            (true , false, false, false) => '▘',
            (false, true , false, false) => '▝',
            (true , true , false, false) => '▀',
            (false, false, true , false) => '▖',
            (true , false, true , false) => '▌',
            (false, true , true , false) => '▞',
            (true , true , true , false) => '▛',
            (false, false, false, true ) => '▗',
            (true , false, false, true ) => '▚',
            (false, true , false, true ) => '▐',
            (true , true , false, true ) => '▜',
            (false, false, true , true ) => '▄',
            (true , false, true , true ) => '▙',
            (false, true , true , true ) => '▟',
            (true , true , true , true ) => '█',
        }
    }
}

struct HexPixel {
    u_l: bool,
    u_r: bool,
    m_l: bool,
    m_r: bool,
    l_l: bool,
    l_r: bool,
}

impl HexPixel {
    const CHARS: [char; 64] = [
        ' ', '🬀', '🬁', '🬂', '🬃', '🬄', '🬅', '🬆', '🬇', '🬈', '🬉', '🬊', '🬋', '🬌', '🬍', '🬎', 
        '🬏', '🬐', '🬑', '🬒', '🬓', '▌', '🬔', '🬕', '🬖', '🬗', '🬘', '🬙', '🬚', '🬛', '🬜', '🬝', 
        '🬞', '🬟', '🬠', '🬡', '🬢', '🬣', '🬤', '🬥', '🬦', '🬧', '▐', '🬨', '🬩', '🬪', '🬫', '🬬', 
        '🬭', '🬮', '🬯', '🬰', '🬱', '🬲', '🬳', '🬴', '🬵', '🬶', '🬷', '🬸', '🬹', '🬺', '🬻', '█'
    ];

    fn get_char(&self) -> char {
        Self::CHARS[self.index()]
    }

    fn index(&self) -> usize {
        (self.u_l as usize) | 
        (self.u_r as usize) << 1 |
        (self.m_l as usize) << 2 | 
        (self.m_r as usize) << 3 |
        (self.l_l as usize) << 4 | 
        (self.l_r as usize) << 5
    }
}

fn main() {
    let pix1: Pixel = Pixel::quad_pixel(
        true, false, 
        false, true 
    );

    let pix2: Pixel = Pixel::hex_pixel(
        true, false, 
        true, false,
        false, true 
    );
    println!("{}{}", pix1.get_char(), pix2.get_char());
    println!("{}{}", pix2.get_char(), pix1.get_char());
}
