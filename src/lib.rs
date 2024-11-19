pub enum Pixel {
    QuadPixel(QuadPixel),
    HexPixel(HexPixel),
}

impl Pixel {
    pub fn quad_pixel(u_l: bool, u_r: bool, l_l: bool, l_r: bool) -> Self {
        Pixel::QuadPixel(
            QuadPixel {
                u_l, u_r, 
                l_l, l_r
            }
        )
    }

    pub fn hex_pixel(u_l: bool, u_r: bool, m_l: bool, m_r: bool, l_l: bool, l_r: bool) -> Self {
        Pixel::HexPixel(
            HexPixel {
                u_l, u_r, 
                m_l, m_r, 
                l_l, l_r
            }
        )
    }

    pub fn get_char(&self) -> char {
        match self {
            Self::QuadPixel(pix) => pix.get_char(),
            Self::HexPixel(pix) => pix.get_char(),
        }
    }
}

pub struct QuadPixel {
    u_l: bool,
    u_r: bool,
    l_l: bool,
    l_r: bool,
}

impl QuadPixel {
    const CHARS: [char; 16] = [
        ' ', '▘', '▝', '▀', 
        '▖', '▌', '▞', '▛', 
        '▗', '▚', '▐', '▜', 
        '▄', '▙', '▟', '█',
    ];

    fn index(&self) -> usize {
        (self.u_l as usize) | 
        (self.u_r as usize) << 1 | 
        (self.l_l as usize) << 2 | 
        (self.l_r as usize) << 3
    }

    /// ```
    /// use display::Pixel;
    /// let pixel = Pixel::quad_pixel (
    ///     true, false, // #_
    ///     false, true, // _#
    /// );
    /// 
    /// let symbol = pixel.get_char();
    /// 
    /// assert_eq!(symbol, '▚')
    /// ```
    fn get_char(&self) -> char {
        Self::CHARS[self.index()]
    }
}

pub struct HexPixel {
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

    fn index(&self) -> usize {
        (self.u_l as usize) | 
        (self.u_r as usize) << 1 | 
        (self.m_l as usize) << 2 | 
        (self.m_r as usize) << 3 | 
        (self.l_l as usize) << 4 | 
        (self.l_r as usize) << 5
    }
    
    /// ```
    /// use display::Pixel;
    /// let pixel = Pixel::hex_pixel (
    ///     true, false, // #_
    ///     false, true, // _#
    ///     true, true,  // ##
    /// );
    /// 
    /// let symbol = pixel.get_char();
    /// 
    /// assert_eq!(symbol, '🬶')
    /// ```
    fn get_char(&self) -> char {
        Self::CHARS[self.index()]
    }
}