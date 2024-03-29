use std::ffi::{c_void, CString, CStr};
use crate::calc_ui::{Backend, Callbacks, runGui};
use std::os::raw::c_char;
use std::ptr::{null, null_mut};

mod calc_ui;
mod math;

#[derive(Debug)]
enum Data {
    Init{internal: *mut Backend},
    UpdatedText{text: String},
    Solve,
}

unsafe impl Send for Data {}

unsafe fn get_sender(data: *mut c_void) -> &'static crossbeam::Sender<Data> {
    &*(data as *mut crossbeam::Sender<Data>)
}

extern "C" fn init(data: *mut c_void, internal: *mut Backend) {
    let sender = unsafe { get_sender(data) };

    sender.send(Data::Init{ internal });
}

extern "C" fn updated_text_field(data: *mut c_void, string: *mut c_char) {
    let sender = unsafe { get_sender(data) };

    sender.send(Data::UpdatedText { text: unsafe{ CStr::from_ptr(string) }.to_string_lossy().into_owned() });
}

extern "C" fn solve(data: *mut c_void) {
    let sender = unsafe { get_sender(data) };

    sender.send(Data::Solve);
}

extern "C" fn delete_data(data: *mut c_void) {
    let sender = unsafe { Box::from_raw(data as *mut crossbeam::Sender<Data>) };
}

fn main() {
    let (sender, receiver) = crossbeam::unbounded::<Data>();
    let handle = std::thread::spawn(move || {
        let callbacks = Callbacks {
            init: Some(init),
            updatedTextField: Some(updated_text_field),
            solve: Some(solve),
            deleteData: Some(delete_data),
            data: Box::into_raw(Box::new(sender)) as *mut c_void
        };

        unsafe { runGui(callbacks) };
    });

    let mut internal_text = String::new();
    let mut backend = null_mut();

    while let Ok(data) = receiver.recv() {
        match data {
            Data::Init{internal} =>  {
                println!("{:?}", internal);
                backend = internal;
            },
            Data::UpdatedText { text } => {
                println!("Whoop! Now the text is {}!", text);
                let filtered: String = text.chars().filter(|x| "0123456789.+-−*x×/÷".find(*x).is_some()).collect();
                if filtered != text {
                    let answer_cstr = CString::new(filtered.clone()).unwrap();
                    unsafe{calc_ui::invalidInput(backend, answer_cstr.as_ptr() as *mut c_char)};
                }
                internal_text = filtered;
            },
            Data::Solve => {
                match math::math(&internal_text) {
                    Ok((_, v)) => {
                        let answer = format!("{}", v);
                        let answer_cstr = CString::new(answer).unwrap();
                        println!("{:?}", v);
                        unsafe {calc_ui::setAnswer(backend, answer_cstr.as_ptr() as *mut c_char)}
                    },
                    _ => {}
                }
            }
        }
    }

    handle.join().unwrap();
}
