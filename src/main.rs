use anyhow::{bail, Context};
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::{
        Input::KeyboardAndMouse::GetKeyboardLayout,
        WindowsAndMessaging::{
            GetForegroundWindow, GetWindowThreadProcessId, PostMessageW, WM_INPUTLANGCHANGEREQUEST,
        },
    },
};

type Locale = isize;

fn get_foreground_window() -> anyhow::Result<HWND> {
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.is_invalid() {
        bail!("foreground is invalid");
    }
    Ok(hwnd)
}

fn get_input_method() -> anyhow::Result<Locale> {
    let hwnd = get_foreground_window()?;
    unsafe {
        let thread_id = GetWindowThreadProcessId(hwnd, None);
        let current_layout = GetKeyboardLayout(thread_id);
        let locale = (current_layout.0 as isize) & 0x0000FFFF;
        Ok(locale)
    }
}

fn set_input_method(locale: isize) -> anyhow::Result<()> {
    let hwnd = Some(get_foreground_window()?);
    let current_layout = LPARAM(locale);
    unsafe {
        PostMessageW(hwnd, WM_INPUTLANGCHANGEREQUEST, WPARAM(0), current_layout)
            .context("failed to set input method")?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    match &args[..] {
        [_] => println!("{}", get_input_method()?),
        [_, locale] => {
            let locale = locale.parse::<Locale>().context("Invalid locale string")?;
            set_input_method(locale)?;
        }
        _ => bail!("usage: {} [locale]", args[0]),
    }

    Ok(())
}
