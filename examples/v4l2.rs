extern crate gst;

use gst::ElementT;

fn main(){
    gst::init();
    let mut pipeline = gst::Pipeline::new_from_str("v4l2src ! autovideosink").unwrap();
	let mut mainloop = gst::MainLoop::new();
	let mut bus = pipeline.bus().expect("Couldn't get bus from pipeline");
	let bus_receiver = bus.receiver();
	mainloop.spawn();
	pipeline.play();
	for message in bus_receiver.iter(){
		match message.parse(){
			gst::Message::StateChangedParsed{ref msg, ref old, ref new, ref pending} => {
				println!("element `{}` changed from {:?} to {:?}", message.src_name(), old, new);
			}
			gst::Message::ErrorParsed{ref msg, ref error, ref debug} => {
				println!("error msg from element `{}`: {}, quitting", message.src_name(), error.message());
				break;
			}
			gst::Message::Eos(ref msg) => {
				println!("eos received quiting");
				break;
			}
			_ => {
				println!("msg of type `{}` from element `{}`", message.type_name(), message.src_name());
			}
		}
	}
	mainloop.quit();
}
