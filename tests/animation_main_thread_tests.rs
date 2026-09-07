mod common;

use std::time::{Duration, Instant};

use imageio::prelude::*;

unsafe extern "C" {
    fn pthread_main_np() -> i32;
}

fn is_main_thread() -> bool {
    unsafe { pthread_main_np() != 0 }
}

fn finite_gif_bytes() -> Vec<u8> {
    let mut animation = MutableProperties::new().expect("create animation properties");
    animation
        .set_i64("LoopCount", 1)
        .expect("set animation loop count");
    let animation = animation.freeze().expect("freeze animation properties");
    let mut root = MutableProperties::new().expect("create root properties");
    root.set_dictionary("{GIF}", &animation)
        .expect("set GIF properties");
    let root = root.freeze().expect("freeze root properties");

    let mut timing = MutableProperties::new().expect("create frame timing");
    timing.set_f64("DelayTime", 0.1).expect("set frame delay");
    timing
        .set_f64("UnclampedDelayTime", 0.1)
        .expect("set unclamped frame delay");
    let timing = timing.freeze().expect("freeze frame timing");
    let mut frame_properties = MutableProperties::new().expect("create frame properties");
    frame_properties
        .set_dictionary("{GIF}", &timing)
        .expect("set frame GIF properties");
    let frame_properties = frame_properties.freeze().expect("freeze frame properties");

    let mut first = common::sample_image();
    let mut second = first.clone();
    first.bgra.fill(0);
    second.bgra.reverse();

    let mut destination =
        ImageDestination::to_data(ImageFormat::Gif.type_identifier(), 2).expect("create GIF");
    destination
        .set_properties(&root)
        .expect("set GIF destination properties");
    destination
        .add_image(&first, Some(&frame_properties))
        .expect("add first frame");
    destination
        .add_image(&second, Some(&frame_properties))
        .expect("add second frame");
    destination.finalize().expect("finalize GIF");
    destination.data().expect("GIF bytes")
}

fn timed_path_animation_stops_on_callback_request() {
    let started = Instant::now();
    let mut callback_times = Vec::new();
    let mut indices = Vec::new();

    animate_image(common::animated_gif_path(), |index, frame| {
        assert!(
            is_main_thread(),
            "animation callback must use the main queue"
        );
        assert_eq!((frame.width, frame.height), (32, 32));
        indices.push(index);
        callback_times.push(Instant::now());
        indices.len() < 2
    })
    .expect("animate GIF path");

    assert_eq!(indices, vec![0, 1]);
    assert!(
        callback_times[1].duration_since(callback_times[0]) >= Duration::from_millis(70),
        "callbacks must honor frame timing"
    );
    assert!(started.elapsed() < Duration::from_secs(3));
}

fn finite_data_animation_terminates_after_its_loop_count() {
    let bytes = finite_gif_bytes();
    let mut indices = Vec::new();

    animate_image_from_bytes(&bytes, |index, _| {
        assert!(
            is_main_thread(),
            "animation callback must use the main queue"
        );
        indices.push(index);
        true
    })
    .expect("animate finite GIF data");

    assert_eq!(indices, vec![0, 1]);
}

fn callback_panics_are_contained_and_stop_playback() {
    let bytes = finite_gif_bytes();
    let mut callbacks = 0;
    let error = animate_image_from_bytes(&bytes, |_, _| {
        callbacks += 1;
        panic!("animation callback test panic");
    })
    .expect_err("callback panic must become an error");

    assert_eq!(callbacks, 1);
    assert!(matches!(
        error,
        ImageError::DecodeFailed(message) if message.contains("callback panicked")
    ));
    RunLoopPump::for_duration(Duration::from_millis(250));
    assert_eq!(callbacks, 1);
}

struct RunLoopPump;

impl RunLoopPump {
    fn for_duration(duration: Duration) {
        let deadline = Instant::now() + duration;
        while Instant::now() < deadline {
            unsafe {
                CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.01, true);
            }
        }
    }
}

type CFRunLoopMode = *const core::ffi::c_void;

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    static kCFRunLoopDefaultMode: CFRunLoopMode;
    fn CFRunLoopRunInMode(mode: CFRunLoopMode, seconds: f64, return_after_source: bool) -> i32;
}

fn main() {
    assert!(
        is_main_thread(),
        "animation tests must run on the process main thread"
    );
    timed_path_animation_stops_on_callback_request();
    finite_data_animation_terminates_after_its_loop_count();
    callback_panics_are_contained_and_stop_playback();
}
