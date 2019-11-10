use std::ffi::{c_void, CString, CStr};
use crate::calc_ui::{Backend, Callbacks, runGui};
use std::os::raw::c_char;

mod calc_ui;
mod math;

#[derive(Debug)]
enum Data {
    Init{internal: *mut Backend},
    UpdatedText{text: String},
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

extern "C" fn delete_data(data: *mut c_void) {
    let sender = unsafe { Box::from_raw(data as *mut crossbeam::Sender<Data>) };
}

fn main() {
    let (sender, receiver) = crossbeam::unbounded::<Data>();
    let handle = std::thread::spawn(move || {
        let callbacks = Callbacks {
            init: Some(init),
            updatedTextField: Some(updated_text_field),
            solve: None,
            deleteData: Some(delete_data),
            data: Box::into_raw(Box::new(sender)) as *mut c_void
        };

        unsafe { runGui(callbacks) };
    });

    while let Ok(data) = receiver.recv() {
        match data {
            d @ Data::Init{..} =>  {
                println!("{:?}", d);
            },
            Data::UpdatedText { text } => {
                println!("Whoop! Now the text is {}!", text);
            }
        }
    }
}
