use bevy::core::AsBytes;
use bevy::prelude::*;
use bevy::render::{
    render_graph::{Node, ResourceSlots},
    renderer::{
        BufferId, BufferInfo, BufferUsage, RenderContext, RenderResourceBinding,
        RenderResourceBindings, RenderResourceContext,
    },
};
use bevy::window::{WindowCreated, WindowId, WindowResized};

pub const WINDOW_ASPECT_RATIO_NODE: &str = "window_aspect_ratio";
pub const WINDOW_ASPECT_RATIO_UNIFORM: &str = "WindowAspectRatio";

#[derive(Default)]
pub struct WindowAspectRatioNode {
    window_id: WindowId,
    hot_buffer: Option<BufferId>,
    staging_buffer: Option<BufferId>,
    window_created_event_reader: EventReader<WindowCreated>,
    window_resized_event_reader: EventReader<WindowResized>,
}

impl WindowAspectRatioNode {
    fn init(
        &mut self,
        context: &mut dyn RenderResourceContext,
        bindings: &mut RefMut<RenderResourceBindings>,
    ) {
        let buffer_size = std::mem::size_of::<f32>();

        let hot_buffer = context.create_buffer_with_data(
            BufferInfo {
                size: buffer_size,
                buffer_usage: BufferUsage::UNIFORM | BufferUsage::COPY_DST,
                ..Default::default()
            },
            (1.0 as f32).as_bytes(),
        );
        bindings.set(
            WINDOW_ASPECT_RATIO_UNIFORM,
            RenderResourceBinding::Buffer {
                buffer: hot_buffer,
                range: 0..buffer_size as u64,
                dynamic_index: None,
            },
        );
        let staging_buffer = context.create_buffer(BufferInfo {
            size: buffer_size,
            buffer_usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
            ..Default::default()
        });
        self.hot_buffer = Some(hot_buffer);
        self.staging_buffer = Some(staging_buffer);
    }
}

impl Node for WindowAspectRatioNode {
    fn update(
        &mut self,
        _world: &World,
        resources: &Resources,
        render_context: &mut dyn RenderContext,
        _input: &ResourceSlots,
        _output: &mut ResourceSlots,
    ) {
        let buffer_size = std::mem::size_of::<f32>();

        let render_resource_context = render_context.resources_mut();
        let mut render_resource_bindings = resources.get_mut::<RenderResourceBindings>().unwrap();

        if self.hot_buffer.is_none() || self.staging_buffer.is_none() {
            self.init(render_resource_context, &mut render_resource_bindings);
        }
        let window_created_events = resources.get::<Events<WindowCreated>>().unwrap();
        let window_resized_events = resources.get::<Events<WindowResized>>().unwrap();
        let windows = resources.get::<Windows>().unwrap();

        let window = windows
            .get(self.window_id)
            .expect("Received window resized event for non-existent window");

        if self
            .window_created_event_reader
            .find_latest(&window_created_events, |e| e.id == window.id)
            .is_some()
            || self
                .window_resized_event_reader
                .find_latest(&window_resized_events, |e| e.id == window.id)
                .is_some()
        {
            let aspect = (window.width as f32) / (window.height as f32);
            let hot_buffer = self.hot_buffer.unwrap();
            let staging_buffer = self.staging_buffer.unwrap();
            render_resource_context.map_buffer(staging_buffer);
            render_resource_context.write_mapped_buffer(
                staging_buffer,
                0..buffer_size as u64,
                &mut |data, _renderer| {
                    data[0..buffer_size].copy_from_slice(aspect.as_bytes());
                },
            );
            render_resource_context.unmap_buffer(staging_buffer);

            render_context.copy_buffer_to_buffer(
                staging_buffer,
                0,
                hot_buffer,
                0,
                buffer_size as u64,
            );
        }
    }
}
