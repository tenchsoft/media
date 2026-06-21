use std::sync::mpsc;

use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_app as gst_app;
use gstreamer_video as gst_video;

use crate::{MediaEvent, MediaMetadata};

/// GStreamer-backed media playback backend.
///
/// Creates a `playbin` pipeline with an `appsink` for video frame extraction.
/// All GStreamer state lives here; the UI only receives `MediaEvent`s.
pub struct PlayerBackend {
    pipeline: Option<gst::Element>,
    event_tx: mpsc::Sender<MediaEvent>,
    volume: f64,
    muted: bool,
    playback_rate: f64,
    media_path: Option<String>,
    /// Current buffering percentage (0-100).
    buffering_percent: u32,
    /// Whether we're in a buffering state.
    is_buffering: bool,
    /// Next track URI for gapless playback (set by UI when AboutToFinish received).
    next_uri: Option<String>,
    /// Cached thumbnails: (position_seconds) -> RGBA pixels at 160x90.
    thumbnail_cache: std::collections::HashMap<f64, Vec<u8>>,
}

impl PlayerBackend {
    /// Create a new backend with an event channel.
    pub fn new() -> (Self, mpsc::Receiver<MediaEvent>) {
        let (event_tx, event_rx) = mpsc::channel();

        // Initialize GStreamer once
        if let Err(e) = gst::init() {
            // GStreamer may already be initialized
            let _ = e;
        }

        let backend = Self {
            pipeline: None,
            event_tx,
            volume: 1.0,
            muted: false,
            playback_rate: 1.0,
            media_path: None,
            buffering_percent: 100,
            is_buffering: false,
            next_uri: None,
            thumbnail_cache: std::collections::HashMap::new(),
        };
        (backend, event_rx)
    }

    /// Load a media file and prepare for playback.
    pub fn load(&mut self, path: &str) {
        // Stop any existing playback
        self.stop();

        let uri = if path.starts_with("file://") || path.contains("://") {
            path.to_string()
        } else {
            format!("file://{}", path)
        };

        // Create playbin pipeline
        let playbin = match gst::ElementFactory::make("playbin")
            .property("uri", &uri)
            .build()
        {
            Ok(p) => p,
            Err(e) => {
                let _ = self
                    .event_tx
                    .send(MediaEvent::Error(format!("Failed to create playbin: {e}")));
                return;
            }
        };

        // Create appsink for video frame extraction
        let appsink = match gst::ElementFactory::make("appsink")
            .property("emit-signals", true)
            .property("max-buffers", 1u32)
            .property("drop", true)
            .build()
        {
            Ok(a) => a,
            Err(e) => {
                let _ = self
                    .event_tx
                    .send(MediaEvent::Error(format!("Failed to create appsink: {e}")));
                return;
            }
        };

        // Request RGBA format from appsink
        let appsink = appsink
            .dynamic_cast::<gst_app::AppSink>()
            .expect("appsink element is not an AppSink");

        let caps = gst_video::VideoCapsBuilder::new()
            .format(gst_video::VideoFormat::Rgba)
            .build();
        appsink.set_caps(Some(&caps));

        // Set up new-sample callback to extract frames
        let tx = self.event_tx.clone();
        appsink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(move |sink| {
                    let sample = sink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                    let buffer = sample.buffer().ok_or(gst::FlowError::Error)?;

                    let caps = sample.caps().ok_or(gst::FlowError::Error)?;
                    let info =
                        gst_video::VideoInfo::from_caps(caps).map_err(|_| gst::FlowError::Error)?;

                    let width = info.width();
                    let height = info.height();

                    let map = buffer.map_readable().map_err(|_| gst::FlowError::Error)?;

                    // Copy pixel data from the GStreamer buffer
                    let pixels = map.as_slice().to_vec();

                    let _ = tx.send(MediaEvent::VideoFrame {
                        pixels,
                        width,
                        height,
                    });

                    Ok(gst::FlowSuccess::Ok)
                })
                .build(),
        );

        // Set appsink as video sink
        playbin.set_property("video-sink", &appsink);

        // Create an audio filter bin with equalizer + level elements
        let audio_bin = gst::Bin::builder().build();

        // Add equalizer element (5 bands: 60Hz, 250Hz, 1kHz, 4kHz, 16kHz)
        let equalizer = gst::ElementFactory::make("equalizer-nbands")
            .property("num-bands", 5u32)
            .build()
            .unwrap_or_else(|_| gst::ElementFactory::make("identity").build().unwrap());

        // Add audio level element for visualization
        let level = gst::ElementFactory::make("level")
            .property("post-messages", true)
            .property("interval", 100_000_000u64) // 100ms
            .build()
            .unwrap_or_else(|_| gst::ElementFactory::make("identity").build().unwrap());

        let _ = audio_bin.add_many([&equalizer, &level]);
        let _ = gst::Element::link_many([&equalizer, &level]);

        let eq_sink = equalizer.static_pad("sink").ok();
        let level_src = level.static_pad("src").ok();

        if let (Some(sink_pad), Some(src_pad)) = (eq_sink, level_src) {
            let _ = audio_bin
                .add_pad(gst::GhostPad::with_target(&gst::PadDirection::Sink, &sink_pad).unwrap());
            let _ = audio_bin
                .add_pad(gst::GhostPad::with_target(&gst::PadDirection::Src, &src_pad).unwrap());
            playbin.set_property("audio-filter", &audio_bin);
        } else {
            // Fallback: just use level element
            playbin.set_property("audio-filter", &level);
        }

        // Apply current settings
        // Use perceptual cubic volume curve
        let linear_vol = self.volume * self.volume * self.volume;
        playbin.set_property("volume", linear_vol);
        if self.muted {
            playbin.set_property("mute", true);
        }

        // Watch bus for messages
        let tx = self.event_tx.clone();
        if let Some(bus) = playbin.bus() {
            bus.add_watch_local(move |_bus, msg| {
                use gst::MessageView;
                match msg.view() {
                    MessageView::Eos(_) => {
                        let _ = tx.send(MediaEvent::EndOfStream);
                    }
                    MessageView::Error(err) => {
                        let _ = tx.send(MediaEvent::Error(format!("{}", err.error())));
                    }
                    MessageView::DurationChanged(_) => {
                        // Duration will be queried on next tick
                    }
                    MessageView::StateChanged(state_changed) => {
                        if state_changed.current() == gst::State::Playing {
                            // Pipeline started playing - query duration
                        }
                    }
                    MessageView::Warning(_warning) => {}
                    MessageView::Buffering(buffering) => {
                        let percent = buffering.percent() as u32;
                        let _ = tx.send(MediaEvent::Buffering(percent));
                    }
                    MessageView::Element(element) => {
                        // Handle audio level messages from the level element
                        if let Some(structure) = element.structure() {
                            if structure.name() == "level" {
                                if let Ok(rms_db) = structure.get::<gstreamer::glib::List>("rms") {
                                    let levels: Vec<f64> =
                                        rms_db.iter().filter_map(|v| v.get::<f64>().ok()).collect();
                                    if !levels.is_empty() {
                                        let _ = tx.send(MediaEvent::AudioLevels(levels));
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                gstreamer::glib::ControlFlow::Continue
            })
            .ok();
        }

        // Connect about-to-finish signal for gapless playback
        let tx_gapless = self.event_tx.clone();
        playbin.connect("about-to-finish", false, move |_args| {
            let _ = tx_gapless.send(MediaEvent::AboutToFinish);
            None
        });

        // Set to paused to allow preroll (gets duration, dimensions)
        if let Err(e) = playbin.set_state(gst::State::Paused) {
            let _ = self
                .event_tx
                .send(MediaEvent::Error(format!("Failed to pause pipeline: {e}")));
            return;
        }

        // Query duration
        let duration = playbin
            .query_duration::<gst::ClockTime>()
            .map(|ct| ct.seconds() as f64)
            .unwrap_or(0.0);

        // Check if media has video by examining stream info
        let n_video: i32 = playbin.property("n-video");
        let (width, height) = if n_video > 0 {
            (1920u32, 1080u32)
        } else {
            let _ = self.event_tx.send(MediaEvent::AudioOnly);
            (0u32, 0u32)
        };

        let _ = self.event_tx.send(MediaEvent::Loaded {
            duration,
            width,
            height,
        });

        self.media_path = Some(path.to_string());
        self.pipeline = Some(playbin);
    }

    /// Start or resume playback.
    pub fn play(&mut self) {
        if let Some(ref pipeline) = self.pipeline {
            let _ = pipeline.set_state(gst::State::Playing);
        }
    }

    /// Pause playback.
    pub fn pause(&mut self) {
        if let Some(ref pipeline) = self.pipeline {
            let _ = pipeline.set_state(gst::State::Paused);
        }
    }

    /// Seek to a position in seconds.
    pub fn seek(&mut self, position: f64) {
        if let Some(ref pipeline) = self.pipeline {
            let target = gst::ClockTime::from_seconds(position as u64);
            let _ = pipeline.seek_simple(gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT, target);
        }
    }

    /// Set volume (0.0 - 1.0).
    /// Applies perceptual cubic curve so that slider position feels linear to human hearing.
    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume.clamp(0.0, 1.0);
        if let Some(ref pipeline) = self.pipeline {
            let perceptual = self.volume;
            let linear = perceptual * perceptual * perceptual;
            pipeline.set_property("volume", linear);
        }
    }

    /// Toggle mute.
    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
        if let Some(ref pipeline) = self.pipeline {
            pipeline.set_property("mute", muted);
        }
    }

    /// Set playback rate (uses trick-mode seek).
    pub fn set_playback_rate(&mut self, rate: f64) {
        self.playback_rate = rate.clamp(0.1, 4.0);
        if let Some(ref pipeline) = self.pipeline {
            let position = pipeline
                .query_position::<gst::ClockTime>()
                .unwrap_or(gst::ClockTime::ZERO);

            let _ = pipeline.seek(
                self.playback_rate,
                gst::SeekFlags::FLUSH | gst::SeekFlags::ACCURATE,
                gst::SeekType::Set,
                position,
                gst::SeekType::None,
                gst::ClockTime::NONE,
            );
        }
    }

    /// Stop playback and unload media.
    pub fn stop(&mut self) {
        if let Some(ref pipeline) = self.pipeline {
            let _ = pipeline.set_state(gst::State::Null);
        }
        self.pipeline = None;
        self.media_path = None;
    }

    /// Get current position in seconds.
    pub fn position(&self) -> f64 {
        self.pipeline
            .as_ref()
            .and_then(|p| p.query_position::<gst::ClockTime>())
            .map(|ct| ct.seconds() as f64)
            .unwrap_or(0.0)
    }

    /// Get media duration in seconds.
    pub fn duration(&self) -> f64 {
        self.pipeline
            .as_ref()
            .and_then(|p| p.query_duration::<gst::ClockTime>())
            .map(|ct| ct.seconds() as f64)
            .unwrap_or(0.0)
    }

    /// Whether media is currently playing.
    pub fn is_playing(&self) -> bool {
        self.pipeline
            .as_ref()
            .map(|p| {
                let state = p.state(gst::ClockTime::ZERO);
                matches!(state.1, gst::State::Playing)
            })
            .unwrap_or(false)
    }

    /// Get current volume.
    pub fn volume(&self) -> f64 {
        self.volume
    }

    /// Whether audio is muted.
    pub fn is_muted(&self) -> bool {
        self.muted
    }

    /// Get current playback rate.
    pub fn playback_rate(&self) -> f64 {
        self.playback_rate
    }

    /// Whether a media file is loaded.
    pub fn has_media(&self) -> bool {
        self.pipeline.is_some()
    }

    /// Poll the backend for position updates. Call this from the render loop.
    /// Returns the current position and duration.
    pub fn tick(&mut self) -> (f64, f64) {
        let pos = self.position();
        let dur = self.duration();
        (pos, dur)
    }

    /// Set equalizer band gains (dB values for 60Hz, 250Hz, 1kHz, 4kHz, 16kHz).
    /// Applies to the GStreamer `equalizer-nbands` element in the audio filter chain.
    pub fn set_eq_bands(&mut self, bands: &[f64; 5]) {
        if let Some(ref pipeline) = self.pipeline {
            // Try to get the equalizer element from the pipeline's audio-filter bin
            // The equalizer was created during load() as part of a GstBin
            // We store the band values for later application if pipeline exists
            if let Some(eq) = pipeline.property::<Option<gst::Element>>("audio-filter") {
                // Try to find child equalizer elements
                if let Ok(bin) = eq.dynamic_cast::<gst::Bin>() {
                    for child in bin.iterate_elements().iter() {
                        let name = child.name();
                        if name.starts_with("equalizer") {
                            for (i, &gain) in bands.iter().enumerate() {
                                let prop = format!("band{}::gain", i);
                                let _ = child.set_property(&prop, gain);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Get media tags (title, artist, etc.) if available.
    pub fn tags(&self) -> Option<gst::TagList> {
        self.pipeline
            .as_ref()
            .and_then(|p| p.property::<Option<gst::TagList>>("tags"))
    }

    /// Query real metadata from the pipeline.
    pub fn query_metadata(&self) -> MediaMetadata {
        let mut meta = MediaMetadata::default();

        if let Some(ref pipeline) = self.pipeline {
            if let Some(tags) = self.tags() {
                if let Some(codec) = tags.get::<gst::tags::VideoCodec>() {
                    meta.video_codec = codec.get().to_string();
                }
                if let Some(codec) = tags.get::<gst::tags::AudioCodec>() {
                    meta.audio_codec = codec.get().to_string();
                }
                if let Some(br) = tags.get::<gst::tags::Bitrate>() {
                    let kbps = br.get() / 1000;
                    meta.bitrate = format!("{} kbps", kbps);
                }
                if let Some(container) = tags.get::<gst::tags::ContainerFormat>() {
                    meta.container_format = container.get().to_string();
                    if meta.video_codec.is_empty() {
                        meta.video_codec = meta.container_format.clone();
                    }
                }
                if let Some(title) = tags.get::<gst::tags::Title>() {
                    meta.title = title.get().to_string();
                }
                if let Some(artist) = tags.get::<gst::tags::Artist>() {
                    meta.artist = artist.get().to_string();
                }
                if let Some(album) = tags.get::<gst::tags::Album>() {
                    meta.album = album.get().to_string();
                }
            }

            let n_video: i32 = pipeline.property("n-video");
            if n_video > 0 {
                if let Some(tags) = self.tags() {
                    if let Some(nom) = tags.get::<gst::tags::NominalBitrate>() {
                        if meta.bitrate.is_empty() {
                            meta.bitrate = format!("{} kbps", nom.get() / 1000);
                        }
                    }
                }
            }
        }

        meta
    }

    /// Get current buffering percentage (0-100).
    pub fn buffering_percent(&self) -> u32 {
        self.buffering_percent
    }

    /// Whether the pipeline is currently buffering.
    pub fn is_buffering(&self) -> bool {
        self.is_buffering
    }

    /// Set the next URI for gapless playback.
    pub fn set_next_uri(&mut self, uri: String) {
        if let Some(ref pipeline) = self.pipeline {
            pipeline.set_property("uri", &uri);
            self.next_uri = Some(uri);
        }
    }

    /// Get the number of audio streams in the current media.
    pub fn n_audio_streams(&self) -> u32 {
        self.pipeline
            .as_ref()
            .map(|p| p.property::<i32>("n-audio").max(0) as u32)
            .unwrap_or(0)
    }

    /// Get the number of video streams in the current media.
    pub fn n_video_streams(&self) -> u32 {
        self.pipeline
            .as_ref()
            .map(|p| p.property::<i32>("n-video").max(0) as u32)
            .unwrap_or(0)
    }

    /// Select an audio stream by index.
    pub fn set_audio_track(&mut self, index: i32) {
        if let Some(ref pipeline) = self.pipeline {
            pipeline.set_property("current-audio", index);
        }
    }

    /// Get the currently selected audio track index.
    pub fn current_audio_track(&self) -> i32 {
        self.pipeline
            .as_ref()
            .map(|p| p.property::<i32>("current-audio"))
            .unwrap_or(-1)
    }

    /// Select a video stream by index.
    pub fn set_video_track(&mut self, index: i32) {
        if let Some(ref pipeline) = self.pipeline {
            pipeline.set_property("current-video", index);
        }
    }

    /// Select a subtitle stream by index (embedded subtitles).
    pub fn set_subtitle_track(&mut self, index: i32) {
        if let Some(ref pipeline) = self.pipeline {
            pipeline.set_property("current-text", index);
            pipeline.set_property(
                "text-sink",
                &gst::ElementFactory::make("textoverlay")
                    .build()
                    .unwrap_or_else(|_| gst::ElementFactory::make("fakesink").build().unwrap()),
            );
        }
    }

    /// Get the number of subtitle/text streams.
    pub fn n_subtitle_streams(&self) -> u32 {
        self.pipeline
            .as_ref()
            .map(|p| p.property::<i32>("n-text").max(0) as u32)
            .unwrap_or(0)
    }

    /// Enumerate available audio output devices.
    /// Returns a list of (device_name, device_class) pairs.
    pub fn enumerate_audio_devices() -> Vec<(String, String)> {
        let monitor = match gst::DeviceProvider::get_by_name("pulsedeviceprovider")
            .or_else(|| gst::DeviceProvider::get_by_name("alsadeviceprovider"))
            .or_else(|| gst::DeviceProvider::get_by_name("osxaudiodeviceprovider"))
            .or_else(|| gst::DeviceProvider::get_by_name("wasapi2deviceprovider"))
        {
            Some(provider) => provider,
            None => return Vec::new(),
        };

        let devices = match monitor.devices() {
            Some(devs) => devs,
            None => return Vec::new(),
        };

        devices
            .into_iter()
            .filter_map(|dev| {
                let display_name = dev
                    .display_name()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let device_class = dev
                    .device_class()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                if device_class.contains("Sink") || device_class.contains("Output") {
                    Some((display_name, device_class))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Set the audio output device by creating a new audio sink for the given device name.
    pub fn set_audio_device(&mut self, device_name: &str) {
        if let Some(ref pipeline) = self.pipeline {
            let sink = gst::ElementFactory::make("pulsesink")
                .property("device", device_name)
                .build()
                .or_else(|_| {
                    gst::ElementFactory::make("alsasink")
                        .property("device", device_name)
                        .build()
                })
                .or_else(|_| {
                    gst::ElementFactory::make("osxaudiosink")
                        .property("device", device_name)
                        .build()
                })
                .or_else(|_| {
                    gst::ElementFactory::make("wasapi2sink")
                        .property("device", device_name)
                        .build()
                });

            if let Ok(sink) = sink {
                pipeline.set_property("audio-sink", &sink);
            }
        }
    }

    /// Query chapters from the media's TOC (Table of Contents).
    /// Returns a list of (title, start_time_seconds) pairs.
    pub fn query_chapters(&self) -> Vec<(String, f64)> {
        let Some(ref pipeline) = self.pipeline else {
            return Vec::new();
        };

        let toc_query = gst::query::Toc::new(gst::TocScope::Global);
        if !pipeline.query(&toc_query) {
            return Vec::new();
        }

        let (_, toc) = toc_query.result();
        let mut chapters = Vec::new();

        if let Some(toc) = toc {
            for entry in toc.entries() {
                let entry_type = entry.entry_type();
                if entry_type == gst::TocEntryType::Edition {
                    for sub in entry.sub_entries() {
                        if sub.entry_type() == gst::TocEntryType::Chapter {
                            let title = sub
                                .tags()
                                .and_then(|tags| tags.get::<gst::tags::Title>())
                                .map(|t| t.get().to_string())
                                .unwrap_or_else(|| "Untitled Chapter".to_string());
                            let start = sub
                                .start_stop_times()
                                .map(|(start_ns, _)| start_ns as f64 / 1_000_000_000.0)
                                .unwrap_or(0.0);
                            chapters.push((title, start.max(0.0)));
                        }
                    }
                } else if entry_type == gst::TocEntryType::Chapter {
                    let title = entry
                        .tags()
                        .and_then(|tags| tags.get::<gst::tags::Title>())
                        .map(|t| t.get().to_string())
                        .unwrap_or_else(|| "Untitled Chapter".to_string());
                    let start = entry
                        .start_stop_times()
                        .map(|(start_ns, _)| start_ns as f64 / 1_000_000_000.0)
                        .unwrap_or(0.0);
                    chapters.push((title, start.max(0.0)));
                }
            }
        }

        chapters.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        chapters
    }

    /// Generate a thumbnail for a given position (in seconds).
    /// Uses a separate GStreamer pipeline to extract a single frame at the given
    /// position, scaled to 160x90 pixels in RGBA format.
    pub fn generate_thumbnail(&mut self, position_secs: f64) -> Option<Vec<u8>> {
        let rounded = (position_secs * 2.0).round() / 2.0;

        if let Some(cached) = self.thumbnail_cache.get(&rounded) {
            return Some(cached.clone());
        }

        let path = self.media_path.as_ref()?;
        let uri = format!("file://{}", path);

        let appsink = gst::ElementFactory::make("appsink")
            .property("max-buffers", 1u32)
            .property("drop", true)
            .property("emit-signals", true)
            .build()
            .ok()?;

        let caps = gst::Caps::builder("video/x-raw")
            .field("format", gst_video::VideoFormat::Rgba.to_str())
            .field("width", 160i32)
            .field("height", 90i32)
            .build();
        let appsink: gst_app::AppSink = appsink.dynamic_cast().ok()?;
        appsink.set_caps(Some(&caps));

        let (tx, rx) = std::sync::mpsc::channel();
        appsink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(move |sink| {
                    let sample = sink.pull_sample().ok()?;
                    let buffer = sample.buffer().ok()?;
                    let map = buffer.map_readable().ok()?;
                    let pixels = map.as_slice().to_vec();
                    let _ = tx.send(pixels);
                    Ok(gst::FlowSuccess::Ok)
                })
                .build(),
        );

        let appsink_el = appsink.clone().upcast::<gst::Element>();
        let videoconvert = gst::ElementFactory::make("videoconvert").build().ok()?;
        let videoscale = gst::ElementFactory::make("videoscale").build().ok()?;
        let capsfilter = gst::ElementFactory::make("capsfilter").build().ok()?;
        let thumb_caps = gst::Caps::builder("video/x-raw")
            .field("width", 160i32)
            .field("height", 90i32)
            .build();
        capsfilter.set_property("caps", &thumb_caps);

        let video_bin = gst::Bin::builder().build();
        video_bin
            .add_many([&videoconvert, &videoscale, &capsfilter, &appsink_el])
            .ok()?;
        gst::Element::link_many([&videoconvert, &videoscale, &capsfilter, &appsink_el]).ok()?;
        video_bin
            .add_pad(
                gst::GhostPad::with_target(
                    &gst::PadDirection::Sink,
                    &videoconvert.static_pad("sink").ok()?,
                )
                .ok()?,
            )
            .ok()?;

        let fakesink = gst::ElementFactory::make("fakesink").build().ok()?;

        let pipeline = gst::ElementFactory::make("playbin")
            .property("uri", &uri)
            .property("video-sink", &video_bin)
            .property("audio-sink", &fakesink)
            .property("flags", gst::PlayFlags::VIDEO & gst::PlayFlags::FLUSH)
            .build()
            .ok()?;

        let pipeline_el: gst::Element = pipeline.clone();
        let bus = pipeline_el.bus()?;

        let pipeline_ref = pipeline.clone();
        pipeline_ref.set_state(gst::State::Paused).ok()?;

        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(3);
        loop {
            if start.elapsed() > timeout {
                pipeline_ref.set_state(gst::State::Null).ok()?;
                return None;
            }
            while let Some(msg) = bus.timed_pop(gst::ClockTime::from_mseconds(50)) {
                use gst::MessageView;
                match msg.view() {
                    MessageView::AsyncDone(_) | MessageView::StateChanged(_) => {
                        let seek_pos =
                            gst::ClockTime::from_nseconds((rounded * 1_000_000_000.0) as u64);
                        let _ = pipeline_ref.seek_simple(
                            gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                            seek_pos,
                        );
                        break;
                    }
                    MessageView::Error(_) => {
                        pipeline_ref.set_state(gst::State::Null).ok()?;
                        return None;
                    }
                    _ => {}
                }
            }
            if let Ok(pixels) = rx.recv_timeout(std::time::Duration::from_millis(200)) {
                pipeline_ref.set_state(gst::State::Null).ok()?;
                self.thumbnail_cache.insert(rounded, pixels.clone());
                if self.thumbnail_cache.len() > 200 {
                    if let Some(&furthest) = self.thumbnail_cache.keys().max_by(|a, b| {
                        let da = (a - rounded).abs();
                        let db = (b - rounded).abs();
                        da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                    }) {
                        self.thumbnail_cache.remove(&furthest);
                    }
                }
                return Some(pixels);
            }
        }
    }

    /// Get a cached thumbnail for a position, without generating a new one.
    pub fn get_cached_thumbnail(&self, position_secs: f64) -> Option<&[u8]> {
        let rounded = (position_secs * 2.0).round() / 2.0;
        self.thumbnail_cache.get(&rounded).map(|v| v.as_slice())
    }

    /// Query subtitle stream labels (language/title) from the container.
    /// Returns a list of labels like "Track 1 (en)", "Track 2 (ko)".
    pub fn query_subtitle_labels(&self) -> Vec<String> {
        let Some(ref pipeline) = self.pipeline else {
            return Vec::new();
        };

        let n_text: i32 = pipeline.property("n-text");
        let mut labels = Vec::new();

        for i in 0..n_text {
            // Try to get stream tags for each text stream
            let label = if let Some(tags) =
                pipeline.emit_by_name::<Option<gst::TagList>>("get-text-tags", &[&i])
            {
                let lang = tags
                    .get::<gst::tags::LanguageCode>()
                    .map(|l| l.get().to_string());
                let title = tags.get::<gst::tags::Title>().map(|t| t.get().to_string());

                match (lang, title) {
                    (Some(l), Some(t)) => format!("Track {} ({}) - {}", i + 1, l, t),
                    (Some(l), None) => format!("Track {} ({})", i + 1, l),
                    (None, Some(t)) => format!("Track {} - {}", i + 1, t),
                    (None, None) => format!("Track {}", i + 1),
                }
            } else {
                format!("Track {}", i + 1)
            };
            labels.push(label);
        }

        labels
    }
}

impl Drop for PlayerBackend {
    fn drop(&mut self) {
        self.stop();
    }
}
