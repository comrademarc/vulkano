// Copyright (c) 2017 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use crate::vk;
use std::ops;

macro_rules! pipeline_stages {
    ($($elem:ident, $var:ident => $val:expr, $queue:expr;)+) => (
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct PipelineStages {
            $(
                pub $elem: bool,
            )+
        }

        impl PipelineStages {
            /// Builds an `PipelineStages` struct with none of the stages set.
            pub fn none() -> PipelineStages {
                PipelineStages {
                    $(
                        $elem: false,
                    )+
                }
            }
        }

        impl From<PipelineStages> for vk::PipelineStageFlags {
            #[inline]
            fn from(val: PipelineStages) -> Self {
                let mut result = 0;
                $(
                    if val.$elem { result |= $val }
                )+
                result
            }
        }

        impl ops::BitOr for PipelineStages {
            type Output = PipelineStages;

            #[inline]
            fn bitor(self, rhs: PipelineStages) -> PipelineStages {
                PipelineStages {
                    $(
                        $elem: self.$elem || rhs.$elem,
                    )+
                }
            }
        }

        impl ops::BitOrAssign for PipelineStages {
            #[inline]
            fn bitor_assign(&mut self, rhs: PipelineStages) {
                $(
                    self.$elem = self.$elem || rhs.$elem;
                )+
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u32)]
        pub enum PipelineStage {
            $(
                $var = $val,
            )+
        }

        impl PipelineStage {
            #[inline]
            pub fn required_queue_flags(&self) -> vk::QueueFlags {
                match self {
                    $(
                        Self::$var => $queue,
                    )+
                }
            }
        }
    );
}

pipeline_stages! {
    top_of_pipe, TopOfPipe => vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT, 0;
    draw_indirect, DrawIndirect => vk::PIPELINE_STAGE_DRAW_INDIRECT_BIT, vk::QUEUE_GRAPHICS_BIT | vk::QUEUE_COMPUTE_BIT;
    vertex_input, VertexInput => vk::PIPELINE_STAGE_VERTEX_INPUT_BIT, vk::QUEUE_GRAPHICS_BIT;
    vertex_shader, VertexShader => vk::PIPELINE_STAGE_VERTEX_SHADER_BIT, vk::QUEUE_GRAPHICS_BIT;
    tessellation_control_shader, TessellationControlShader => vk::PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT, vk::QUEUE_GRAPHICS_BIT;
    tessellation_evaluation_shader, TessellationEvaluationShader => vk::PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT, vk::QUEUE_GRAPHICS_BIT;
    geometry_shader, GeometryShader => vk::PIPELINE_STAGE_GEOMETRY_SHADER_BIT, vk::QUEUE_GRAPHICS_BIT;
    fragment_shader, FragmentShader => vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT, vk::QUEUE_GRAPHICS_BIT;
    early_fragment_tests, EarlyFragmentTests => vk::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT, vk::QUEUE_GRAPHICS_BIT;
    late_fragment_tests, LateFragmentTests => vk::PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT, vk::QUEUE_GRAPHICS_BIT;
    color_attachment_output, ColorAttachmentOutput => vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT, vk::QUEUE_GRAPHICS_BIT;
    compute_shader, ComputeShader => vk::PIPELINE_STAGE_COMPUTE_SHADER_BIT, vk::QUEUE_COMPUTE_BIT;
    transfer, Transfer => vk::PIPELINE_STAGE_TRANSFER_BIT, vk::QUEUE_GRAPHICS_BIT | vk::QUEUE_COMPUTE_BIT | vk::QUEUE_TRANSFER_BIT;
    bottom_of_pipe, BottomOfPipe => vk::PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT, 0;
    host, Host => vk::PIPELINE_STAGE_HOST_BIT, 0;
    all_graphics, AllGraphics => vk::PIPELINE_STAGE_ALL_GRAPHICS_BIT, vk::QUEUE_GRAPHICS_BIT;
    all_commands, AllCommands => vk::PIPELINE_STAGE_ALL_COMMANDS_BIT, 0;
}

macro_rules! access_flags {
    ($($elem:ident => $val:expr,)+) => (
        #[derive(Debug, Copy, Clone)]
        #[allow(missing_docs)]
        pub struct AccessFlags {
            $(
                pub $elem: bool,
            )+
        }

        impl AccessFlags {
            /// Builds an `AccessFlags` struct with all bits set.
            pub fn all() -> AccessFlags {
                AccessFlags {
                    $(
                        $elem: true,
                    )+
                }
            }

            /// Builds an `AccessFlags` struct with none of the bits set.
            pub fn none() -> AccessFlags {
                AccessFlags {
                    $(
                        $elem: false,
                    )+
                }
            }
        }

        impl From<AccessFlags> for vk::AccessFlags {
            #[inline]
            fn from(val: AccessFlags) -> Self {
                let mut result = 0;
                $(
                    if val.$elem { result |= $val }
                )+
                result
            }
        }

        impl ops::BitOr for AccessFlags {
            type Output = AccessFlags;

            #[inline]
            fn bitor(self, rhs: AccessFlags) -> AccessFlags {
                AccessFlags {
                    $(
                        $elem: self.$elem || rhs.$elem,
                    )+
                }
            }
        }

        impl ops::BitOrAssign for AccessFlags {
            #[inline]
            fn bitor_assign(&mut self, rhs: AccessFlags) {
                $(
                    self.$elem = self.$elem || rhs.$elem;
                )+
            }
        }
    );
}

access_flags! {
    indirect_command_read => vk::ACCESS_INDIRECT_COMMAND_READ_BIT,
    index_read => vk::ACCESS_INDEX_READ_BIT,
    vertex_attribute_read => vk::ACCESS_VERTEX_ATTRIBUTE_READ_BIT,
    uniform_read => vk::ACCESS_UNIFORM_READ_BIT,
    input_attachment_read => vk::ACCESS_INPUT_ATTACHMENT_READ_BIT,
    shader_read => vk::ACCESS_SHADER_READ_BIT,
    shader_write => vk::ACCESS_SHADER_WRITE_BIT,
    color_attachment_read => vk::ACCESS_COLOR_ATTACHMENT_READ_BIT,
    color_attachment_write => vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
    depth_stencil_attachment_read => vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT,
    depth_stencil_attachment_write => vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
    transfer_read => vk::ACCESS_TRANSFER_READ_BIT,
    transfer_write => vk::ACCESS_TRANSFER_WRITE_BIT,
    host_read => vk::ACCESS_HOST_READ_BIT,
    host_write => vk::ACCESS_HOST_WRITE_BIT,
    memory_read => vk::ACCESS_MEMORY_READ_BIT,
    memory_write => vk::ACCESS_MEMORY_WRITE_BIT,
}

impl AccessFlags {
    /// Returns true if the access flags can be used with the given pipeline stages.
    ///
    /// Corresponds to `Table 4. Supported access types` in section `6.1.3. Access Types` of the
    /// Vulkan specs.
    pub fn is_compatible_with(&self, stages: &PipelineStages) -> bool {
        if stages.all_commands {
            return true;
        }

        if self.indirect_command_read && !stages.draw_indirect && !stages.all_graphics {
            return false;
        }

        if (self.index_read || self.vertex_attribute_read)
            && !stages.vertex_input
            && !stages.all_graphics
        {
            return false;
        }

        if (self.uniform_read || self.shader_read || self.shader_write)
            && !stages.vertex_shader
            && !stages.tessellation_control_shader
            && !stages.tessellation_evaluation_shader
            && !stages.geometry_shader
            && !stages.fragment_shader
            && !stages.compute_shader
            && !stages.all_graphics
        {
            return false;
        }

        if self.input_attachment_read && !stages.fragment_shader && !stages.all_graphics {
            return false;
        }

        if (self.color_attachment_read || self.color_attachment_write)
            && !stages.color_attachment_output
            && !stages.all_graphics
        {
            return false;
        }

        if (self.depth_stencil_attachment_read || self.depth_stencil_attachment_write)
            && !stages.early_fragment_tests
            && !stages.late_fragment_tests
            && !stages.all_graphics
        {
            return false;
        }

        if (self.transfer_read || self.transfer_write) && !stages.transfer {
            return false;
        }

        if (self.host_read || self.host_write) && !stages.host {
            return false;
        }

        true
    }
}

/// The full specification of memory access by the pipeline for a particular resource.
#[derive(Clone, Copy, Debug)]
pub struct PipelineMemoryAccess {
    /// The pipeline stages the resource will be accessed in.
    pub stages: PipelineStages,
    /// The type of memory access that will be performed.
    pub access: AccessFlags,
    /// Whether the resource needs exclusive (mutable) access or can be shared.
    pub exclusive: bool,
}
