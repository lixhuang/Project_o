#[allow(dead_code)]
#[repr(u8)]
pub enum Color
{
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
struct ColorCode(u8);

impl ColorCode
{
	const fn new(foreground: Color, background: Color) -> ColorCode
	{
		ColorCode((background as u8) << 4 | foreground as u8)
	}
}

#[repr(C)]
struct ScreenChar
{
	ascii_char: u8,
	color_code: ColorCode,
}

const BUFFER_HIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer
{
	chars: [[ScreenChar;BUFFER_WIDTH];BUFFER_HIGHT],
}

use core::ptr::Unique;

pub struct Writer
{
	column_position: usize,
	color_code: ColorCode,
	buffer: Unique<Buffer>,
}

impl Writer
{
	pub fn write_string(&mut self, s: &str)
	{
		for byte in s.bytes()
		{
			self.write_byte(byte);
		}
	}

	pub fn write_byte(&mut self, byte: u8)
	{
		match byte
		{
			b'\n' => self.new_line(),
			byte => {
				if self.column_position >= BUFFER_WIDTH{
					self.new_line();
				}
			
				let row = BUFFER_HIGHT - 1;
				let col = self.column_position;

				self.buffer().chars[row][col] = ScreenChar{
					ascii_char: byte,
					color_code: self.color_code,
				};
				self.column_position += 1;		
			}
		}
	}

	fn buffer(&mut self) -> &mut Buffer
	{
		unsafe{self.buffer.get_mut()}
	}

	fn new_line(&mut self)
	{

	}
}