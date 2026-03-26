#![no_std]
#![no_main]
#[repr(C)]
struct MultibootTag {
    typ: u32,
    size: u32,
}

#[repr(C)]
struct MmapTag {
    typ: u32,      // = 6
    size: u32,
    entry_size: u32,
    entry_version: u32,
    // この後に可変長の mmap_entry が続く
}

#[repr(C)]
struct MmapEntry {
    addr: u64,
    len: u64,
    typ: u32,
    zero: u32,
}
#[repr(C)]
struct MultibootInfo {
    total_size: u32,
    reserved: u32,
    // ここからタグが続く
}
static mut CURSOR: usize = 0;

static mut CURSOR_ROW: usize = 0;
static mut CURSOR_COL: usize = 0;

fn put_char(ch: u8, fg: Color, bg: Color) {
    unsafe {
        // 画面外なら無視（スクロールは後で実装）
        if CURSOR_ROW >= VGA_HEIGHT {
            return;
        }

        let index = CURSOR_ROW * VGA_WIDTH + CURSOR_COL;
        *VGA_BUFFER.add(index) = vga_entry(ch, fg, bg);

        CURSOR_COL += 1;

        // 行末に来たら改行
        if CURSOR_COL >= VGA_WIDTH {
            CURSOR_COL = 0;
            CURSOR_ROW += 1;
        }
    }
}
fn kprint(s: &str) {
    for &b in s.as_bytes() {
        if b == b'\n' {
            unsafe {
                CURSOR_COL = 0;
                CURSOR_ROW += 1;
            }
        } else {
            put_char(b, Color::White, Color::Black);
        }
    }
}
fn kprint_hex(mut value: u64) {
    for i in (0..16).rev() {
        let nibble = ((value >> (i * 4)) & 0xF) as u8;
        let ch = match nibble {
            0..=9 => b'0' + nibble,
            _ => b'A' + (nibble - 10),
        };
        put_char(ch, Color::White, Color::Black);
    }
}

unsafe fn find_mmap_tag(mbi: *const u8) -> Option<*const MmapTag> {
    let total_size = *(mbi as *const u32);
    let mut offset = 8; // total_size + reserved の後

    while offset < total_size as usize {
        let tag = mbi.add(offset) as *const MultibootTag;
        let typ = (*tag).typ;
        let size = (*tag).size as usize;

        if typ == 6 {
            return Some(tag as *const MmapTag);
        }

        // 8バイト境界にアライン
        offset += (size + 7) & !7;
    }

    None
}
unsafe fn parse_mmap(tag: *const MmapTag) {
    let entry_size = (*tag).entry_size as usize;

    // entries の先頭
    let mut entry_ptr = (tag as *const u8).add(16);

    // tag.size はヘッダ + entries 全体
    let end = (tag as *const u8).add((*tag).size as usize);

    while entry_ptr < end {
        let entry = entry_ptr as *const MmapEntry;

        let addr = (*entry).addr;
        let len  = (*entry).len;
        let typ  = (*entry).typ;

        if typ == 1 {
            // usable RAM
            kprint("RAM usable: {:#x} - {:#x}");
            kprint_hex(addr);
            kprint(" - ");
            kprint_hex(addr + len);
        } else {
            kprint("Reserved : {:#x} - {:#x}");
            kprint_hex(addr);
            kprint(" - ");
            kprint_hex(addr + len);
        }

        entry_ptr = entry_ptr.add(entry_size);
    }
}
const MULTIBOOT2_MAGIC: u32 = 0x36d76289;

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
    br"_   _      _ _         _   _       _      ",
    br"| | | | ___| | | ___   | | | |_ __ (_)_  __",
    br"| |_| |/ _ \ | |/ _ \  | | | | '_ \| \ \/ /",
    br"|  _  |  __/ | | (_) | | |_| | | | | |>  < ",
    br"|_| |_|\___|_|_|\___/   \___/|_| |_|_/_/\_\",
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
pub extern "C" fn kernel_main(magic: u32, mbi_phys: u32) -> ! {
    // Dark blue background
    clear_screen(Color::White, Color::Black);
    let art_colors = [
        Color::White,
        
       
    ];

    let art_start_row = 0;
    let art_col = 0;
    unsafe {
    CURSOR_ROW = art_start_row + ASCII_ART.len();
    CURSOR_COL = 0;
}

    for (i, line) in ASCII_ART.iter().enumerate() {
        let color = art_colors[i % art_colors.len()];
        put_str_at(line, color, Color::Black, art_start_row + i, art_col);
    }

    // System info
    let info_row = 15;
    let info_col = 14;
    
    kprint("Architecture : x86_64");
    // Bottom border
    //put_str_at(BANNER, Color::White, Color::Black, 22, 14);
     if magic != MULTIBOOT2_MAGIC {
        panic!("Invalid multiboot2 magic");
        kprint("Invalid multiboot2 magic");
        
    }

    //let mbi = (mbi_phys as u64 + KERNEL_VMA_BASE) as *const u8;
    let mbi = mbi_phys as *const u8;

    unsafe {
        if let Some(tag) = find_mmap_tag(mbi) {
            parse_mmap(tag);
        } else {
            kprint("No memory map tag found");
        }
    }

    kprint("Memory map parsing complete.");
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
