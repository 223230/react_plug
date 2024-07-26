mod params;

use crate::params::*;

use nih_plug::prelude::*;
use react_plug::prelude::*;

use std::sync::Arc;
use crossbeam_channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use include_dir::{Dir, include_dir};

pub struct ExamplePlugin {
    params: Arc<ExampleParams>,
    editor_channel: (Sender<PluginMessage>, Receiver<PluginMessage>),
}

impl Default for ExamplePlugin {
    fn default() -> Self {
        let channel = crossbeam_channel::bounded::<PluginMessage>(64);

        Self {
            params: Arc::new(ExampleParams::new(&Arc::new(channel.0.clone()))),
            editor_channel: channel,
        }
    }
}

#[gui_message(params = ExampleParams)]
pub enum GuiMessage {
    Ping,
}

#[plugin_message(params = ExampleParams)]
pub enum PluginMessage {
    Pong,
}

static EDITOR_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/gui/dist");

impl Plugin for ExamplePlugin {
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> { self.params.clone() }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let gain = self.params.gain.smoothed.next();
            for sample in channel_samples {
                *sample *= gain;
            }
        }

        ProcessStatus::Normal
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        Some(Box::new(create_editor(
            self.params.clone(),
            Some("example"),
            &EDITOR_DIR,
            self.editor_channel(),
            |gui_message: GuiMessage, sender| {
                // Handle GUI messages here
                if let GuiMessage::Ping = gui_message {
                    sender.send(PluginMessage::Pong).unwrap();
                }
            },
        ).with_developer_mode(true)))
    }

    const NAME: &'static str = "Example Plugin";
    const VENDOR: &'static str = "223230";
    const URL: &'static str = "https://github.com/223230";
    const EMAIL: &'static str = "223230@pm.me";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
}

impl RPPlugin for ExamplePlugin {
    type Parameters = ExampleParams;
    type PluginMsg = PluginMessage;
    type GuiMsg = GuiMessage;

    fn editor_channel(&self) -> (Sender<PluginMessage>, Receiver<PluginMessage>) {
        self.editor_channel.clone()
    }
}

impl Vst3Plugin for ExamplePlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"AUDIOPLUGIN\0\0\0\0\0";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_vst3!(ExamplePlugin);