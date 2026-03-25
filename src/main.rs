#![no_std]
#![no_main]

// VGA text mode buffer address
const VGA_BUFFER: *mut u16 = 0xB8000 as *mut u16;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

// VGA colors
#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy)]
enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

fn vga_entry(ch: u8, fg: Color, bg: Color) -> u16 {
    let color: u16 = (bg as u16) << 4 | (fg as u16);
    (color << 8) | (ch as u16)
}

fn clear_screen(fg: Color, bg: Color) {
    for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
        unsafe {
            *VGA_BUFFER.add(i) = vga_entry(b' ', fg, bg);
        }
    }
}

fn put_char_at(ch: u8, fg: Color, bg: Color, row: usize, col: usize) {
    if row < VGA_HEIGHT && col < VGA_WIDTH {
        unsafe {
            *VGA_BUFFER.add(row * VGA_WIDTH + col) = vga_entry(ch, fg, bg);
        }
    }
}

fn put_str_at(s: &[u8], fg: Color, bg: Color, row: usize, col: usize) {
    for (i, &ch) in s.iter().enumerate() {
        put_char_at(ch, fg, bg, row, col + i);
    }
}
static ASCII_ART: &[&[u8]] = &[
    b"/$$   /$$ /$$$$$$$$ /$$       /$$        /$$$$$$        /$$   /$$ /$$   /$$ /$$$$$$ /$$   /$$",
    b"| $$  | $$| $$_____/| $$      | $$       /$$__  $$      | $$  | $$| $$$ | $$|_  $$_/| $$  / $$",
    b"| $$  | $$| $$      | $$      | $$      | $$  \\ $$      | $$  | $$| $$$$| $$  | $$  |  $$/ $$/",
    b"| $$$$$$$$| $$$$$   | $$      | $$      | $$  | $$      | $$  | $$| $$ $$ $$  | $$   \\  $$$$/ ",
    b"| $$__  $$| $$__/   | $$      | $$      | $$  | $$      | $$  | $$| $$  $$$$  | $$    >$$  $$ ",
    b"| $$  | $$| $$      | $$      | $$      | $$  | $$      | $$  | $$| $$\\  $$$  | $$   /$$/\\  $$",
    b"| $$  | $$| $$$$$$$$| $$$$$$$$| $$$$$$$$|  $$$$$$/      |  $$$$$$/| $$ \\  $$ /$$$$$$| $$  \\ $$",
    b"|__/  |__/|________/|________/|________/ \\______/        \\______/ |__/  \\__/|______/|__/  |__/",
    b"",
    b"",
    b"",
];


// The ASCII art (Ferris the crab!)
//static ASCII_ART: &[&[u8]] = &[
//    b"",
//    b"",
//    b"   _~^~^~_",
//    b" \\) /  o o  \\ (/",
//    b"   '_   -   _'",
//    b"   / '-----' \\",
//    b"",
//    b"  Ferris  Says:",
//    b"",
//];

static BANNER: &[u8]  = b"  +--------------------------------------------------+";
static BANNER2: &[u8] = b"  |             Welcome to HELLO UNIX                |";
static BANNER3: &[u8] = b"  +--------------------------------------------------+";

static MSG1: &[u8] = b"  Architecture : x86_64";
static MSG2: &[u8] = b"  Boot         : GRUB2 / Multiboot2";
static MSG3: &[u8] = b"  Language     : Rust (no_std, bare metal)";
static MSG4: &[u8] = b"  VGA Mode     : 80x25 Text Mode";
static MSG5: &[u8] = b"";
static MSG6: &[u8] = b"  System halted. Press CTRL+ALT+DEL to reboot.";

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Dark blue background
    clear_screen(Color::White, Color::Black);

    // Top banner
    put_str_at(BANNER,  Color::White, Color::Black, 1, 14);
    put_str_at(BANNER2, Color::White, Color::Black, 2, 14);
    put_str_at(BANNER3, Color::White, Color::Black, 3, 14);

    // ASCII art (Ferris) - centered ish, with color cycling per line
    let art_colors = [
        Color::White,
        
       
    ];

    let art_start_row = 5;
    let art_col = 0;
    for (i, line) in ASCII_ART.iter().enumerate() {
        let color = art_colors[i % art_colors.len()];
        put_str_at(line, color, Color::Black, art_start_row + i, art_col);
    }

    // System info
    let info_row = 15;
    let info_col = 14;
    put_str_at(MSG1, Color::White,  Color::Black, info_row,     info_col);
    put_str_at(MSG2, Color::White,  Color::Black, info_row + 1, info_col);
    put_str_at(MSG3, Color::White,  Color::Black, info_row + 2, info_col);
    put_str_at(MSG4, Color::White,  Color::Black, info_row + 3, info_col);
    put_str_at(MSG5, Color::White,       Color::Black, info_row + 4, info_col);
    put_str_at(MSG6, Color::White,    Color::Black, info_row + 5, info_col);

    // Bottom border
    put_str_at(BANNER, Color::White, Color::Black, 22, 14);

    // Halt
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // On panic: print "KERNEL PANIC" in red
    let msg = b"*** KERNEL PANIC ***";
    for (i, &ch) in msg.iter().enumerate() {
        unsafe {
            *VGA_BUFFER.add(i) = vga_entry(ch, Color::White, Color::Red);
        }
    }
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}
