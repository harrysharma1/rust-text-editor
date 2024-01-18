struct Terminal{}

struct Size{
    w:u16,
    h:u16,
}

impl Terminal{
    pub fn default() -> Result<Self, std::io::Error>{
        let size  = termion::terminal_size()?;
        Ok(
            Self{
                size: Size{
                    w: size.0,
                    h: size.1,
                },
            })
    }

    pub fn size(&self)-> &Size{
        &self.size
    }
}