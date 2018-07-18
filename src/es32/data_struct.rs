use super::es20::data_struct::*;

pub const GL_MULTISAMPLE_LINE_WIDTH_RANGE: GLuint = 37761;
pub const GL_MULTISAMPLE_LINE_WIDTH_GRANULARITY: GLuint = 37762;
pub const GL_MULTIPLY: GLuint = 37524;
pub const GL_SCREEN: GLuint = 37525;
pub const GL_OVERLAY: GLuint = 37526;
pub const GL_DARKEN: GLuint = 37527;
pub const GL_LIGHTEN: GLuint = 37528;
pub const GL_COLORDODGE: GLuint = 37529;
pub const GL_COLORBURN: GLuint = 37530;
pub const GL_HARDLIGHT: GLuint = 37531;
pub const GL_SOFTLIGHT: GLuint = 37532;
pub const GL_DIFFERENCE: GLuint = 37534;
pub const GL_EXCLUSION: GLuint = 37536;
pub const GL_HSL_HUE: GLuint = 37549;
pub const GL_HSL_SATURATION: GLuint = 37550;
pub const GL_HSL_COLOR: GLuint = 37551;
pub const GL_HSL_LUMINOSITY: GLuint = 37552;
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: GLuint = 33346;
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLuint = 33347;
pub const GL_DEBUG_CALLBACK_FUNCTION: GLuint = 33348;
pub const GL_DEBUG_CALLBACK_USER_PARAM: GLuint = 33349;
pub const GL_DEBUG_SOURCE_API: GLuint = 33350;
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: GLuint = 33351;
pub const GL_DEBUG_SOURCE_SHADER_COMPILER: GLuint = 33352;
pub const GL_DEBUG_SOURCE_THIRD_PARTY: GLuint = 33353;
pub const GL_DEBUG_SOURCE_APPLICATION: GLuint = 33354;
pub const GL_DEBUG_SOURCE_OTHER: GLuint = 33355;
pub const GL_DEBUG_TYPE_ERROR: GLuint = 33356;
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLuint = 33357;
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLuint = 33358;
pub const GL_DEBUG_TYPE_PORTABILITY: GLuint = 33359;
pub const GL_DEBUG_TYPE_PERFORMANCE: GLuint = 33360;
pub const GL_DEBUG_TYPE_OTHER: GLuint = 33361;
pub const GL_DEBUG_TYPE_MARKER: GLuint = 33384;
pub const GL_DEBUG_TYPE_PUSH_GROUP: GLuint = 33385;
pub const GL_DEBUG_TYPE_POP_GROUP: GLuint = 33386;
pub const GL_DEBUG_SEVERITY_NOTIFICATION: GLuint = 33387;
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: GLuint = 33388;
pub const GL_DEBUG_GROUP_STACK_DEPTH: GLuint = 33389;
pub const GL_BUFFER: GLuint = 33504;
pub const GL_SHADER: GLuint = 33505;
pub const GL_PROGRAM: GLuint = 33506;
pub const GL_VERTEX_ARRAY: GLuint = 32884;
pub const GL_QUERY: GLuint = 33507;
pub const GL_PROGRAM_PIPELINE: GLuint = 33508;
pub const GL_SAMPLER: GLuint = 33510;
pub const GL_MAX_LABEL_LENGTH: GLuint = 33512;
pub const GL_MAX_DEBUG_MESSAGE_LENGTH: GLuint = 37187;
pub const GL_MAX_DEBUG_LOGGED_MESSAGES: GLuint = 37188;
pub const GL_DEBUG_LOGGED_MESSAGES: GLuint = 37189;
pub const GL_DEBUG_SEVERITY_HIGH: GLuint = 37190;
pub const GL_DEBUG_SEVERITY_MEDIUM: GLuint = 37191;
pub const GL_DEBUG_SEVERITY_LOW: GLuint = 37192;
pub const GL_DEBUG_OUTPUT: GLuint = 37600;
pub const GL_CONTEXT_FLAG_DEBUG_BIT: GLuint = 2;
pub const GL_STACK_OVERFLOW: GLuint = 1283;
pub const GL_STACK_UNDERFLOW: GLuint = 1284;
pub const GL_GEOMETRY_SHADER: GLuint = 36313;
pub const GL_GEOMETRY_SHADER_BIT: GLuint = 4;
pub const GL_GEOMETRY_VERTICES_OUT: GLuint = 35094;
pub const GL_GEOMETRY_INPUT_TYPE: GLuint = 35095;
pub const GL_GEOMETRY_OUTPUT_TYPE: GLuint = 35096;
pub const GL_GEOMETRY_SHADER_INVOCATIONS: GLuint = 34943;
pub const GL_LAYER_PROVOKING_VERTEX: GLuint = 33374;
pub const GL_LINES_ADJACENCY: GLuint = 10;
pub const GL_LINE_STRIP_ADJACENCY: GLuint = 11;
pub const GL_TRIANGLES_ADJACENCY: GLuint = 12;
pub const GL_TRIANGLE_STRIP_ADJACENCY: GLuint = 13;
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: GLuint = 36319;
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: GLuint = 35372;
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLuint = 35378;
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: GLuint = 37155;
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: GLuint = 37156;
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: GLuint = 36320;
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLuint = 36321;
pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS: GLuint = 36442;
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLuint = 35881;
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLuint = 37583;
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS: GLuint = 37589;
pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS: GLuint = 37069;
pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLuint = 37079;
pub const GL_FIRST_VERTEX_CONVENTION: GLuint = 36429;
pub const GL_LAST_VERTEX_CONVENTION: GLuint = 36430;
pub const GL_UNDEFINED_VERTEX: GLuint = 33376;
pub const GL_PRIMITIVES_GENERATED: GLuint = 35975;
pub const GL_FRAMEBUFFER_DEFAULT_LAYERS: GLuint = 37650;
pub const GL_MAX_FRAMEBUFFER_LAYERS: GLuint = 37655;
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLuint = 36264;
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: GLuint = 36263;
pub const GL_REFERENCED_BY_GEOMETRY_SHADER: GLuint = 37641;
pub const GL_PRIMITIVE_BOUNDING_BOX: GLuint = 37566;
pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT: GLuint = 4;
pub const GL_CONTEXT_FLAGS: GLuint = 33310;
pub const GL_LOSE_CONTEXT_ON_RESET: GLuint = 33362;
pub const GL_GUILTY_CONTEXT_RESET: GLuint = 33363;
pub const GL_INNOCENT_CONTEXT_RESET: GLuint = 33364;
pub const GL_UNKNOWN_CONTEXT_RESET: GLuint = 33365;
pub const GL_RESET_NOTIFICATION_STRATEGY: GLuint = 33366;
pub const GL_NO_RESET_NOTIFICATION: GLuint = 33377;
pub const GL_CONTEXT_LOST: GLuint = 1287;
pub const GL_SAMPLE_SHADING: GLuint = 35894;
pub const GL_MIN_SAMPLE_SHADING_VALUE: GLuint = 35895;
pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET: GLuint = 36443;
pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET: GLuint = 36444;
pub const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS: GLuint = 36445;
pub const GL_PATCHES: GLuint = 14;
pub const GL_PATCH_VERTICES: GLuint = 36466;
pub const GL_TESS_CONTROL_OUTPUT_VERTICES: GLuint = 36469;
pub const GL_TESS_GEN_MODE: GLuint = 36470;
pub const GL_TESS_GEN_SPACING: GLuint = 36471;
pub const GL_TESS_GEN_VERTEX_ORDER: GLuint = 36472;
pub const GL_TESS_GEN_POINT_MODE: GLuint = 36473;
pub const GL_ISOLINES: GLuint = 36474;
pub const GL_QUADS: GLuint = 7;
pub const GL_FRACTIONAL_ODD: GLuint = 36475;
pub const GL_FRACTIONAL_EVEN: GLuint = 36476;
pub const GL_MAX_PATCH_VERTICES: GLuint = 36477;
pub const GL_MAX_TESS_GEN_LEVEL: GLuint = 36478;
pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLuint = 36479;
pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLuint = 36480;
pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLuint = 36481;
pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLuint = 36482;
pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLuint = 36483;
pub const GL_MAX_TESS_PATCH_COMPONENTS: GLuint = 36484;
pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLuint = 36485;
pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLuint = 36486;
pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLuint = 36489;
pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLuint = 36490;
pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS: GLuint = 34924;
pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLuint = 34925;
pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLuint = 36382;
pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLuint = 36383;
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLuint = 37581;
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLuint = 37582;
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLuint = 37587;
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLuint = 37588;
pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLuint = 37067;
pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLuint = 37068;
pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLuint = 37080;
pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLuint = 37081;
pub const GL_PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: GLuint = 33313;
pub const GL_IS_PER_PATCH: GLuint = 37607;
pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER: GLuint = 37639;
pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER: GLuint = 37640;
pub const GL_TESS_CONTROL_SHADER: GLuint = 36488;
pub const GL_TESS_EVALUATION_SHADER: GLuint = 36487;
pub const GL_TESS_CONTROL_SHADER_BIT: GLuint = 8;
pub const GL_TESS_EVALUATION_SHADER_BIT: GLuint = 16;
pub const GL_TEXTURE_BORDER_COLOR: GLuint = 4100;
pub const GL_CLAMP_TO_BORDER: GLuint = 33069;
pub const GL_TEXTURE_BUFFER: GLuint = 35882;
pub const GL_TEXTURE_BUFFER_BINDING: GLuint = 35882;
pub const GL_MAX_TEXTURE_BUFFER_SIZE: GLuint = 35883;
pub const GL_TEXTURE_BINDING_BUFFER: GLuint = 35884;
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: GLuint = 35885;
pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLuint = 37279;
pub const GL_SAMPLER_BUFFER: GLuint = 36290;
pub const GL_INT_SAMPLER_BUFFER: GLuint = 36304;
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: GLuint = 36312;
pub const GL_IMAGE_BUFFER: GLuint = 36945;
pub const GL_INT_IMAGE_BUFFER: GLuint = 36956;
pub const GL_UNSIGNED_INT_IMAGE_BUFFER: GLuint = 36967;
pub const GL_TEXTURE_BUFFER_OFFSET: GLuint = 37277;
pub const GL_TEXTURE_BUFFER_SIZE: GLuint = 37278;
pub const GL_COMPRESSED_RGBA_ASTC_4x4: GLuint = 37808;
pub const GL_COMPRESSED_RGBA_ASTC_5x4: GLuint = 37809;
pub const GL_COMPRESSED_RGBA_ASTC_5x5: GLuint = 37810;
pub const GL_COMPRESSED_RGBA_ASTC_6x5: GLuint = 37811;
pub const GL_COMPRESSED_RGBA_ASTC_6x6: GLuint = 37812;
pub const GL_COMPRESSED_RGBA_ASTC_8x5: GLuint = 37813;
pub const GL_COMPRESSED_RGBA_ASTC_8x6: GLuint = 37814;
pub const GL_COMPRESSED_RGBA_ASTC_8x8: GLuint = 37815;
pub const GL_COMPRESSED_RGBA_ASTC_10x5: GLuint = 37816;
pub const GL_COMPRESSED_RGBA_ASTC_10x6: GLuint = 37817;
pub const GL_COMPRESSED_RGBA_ASTC_10x8: GLuint = 37818;
pub const GL_COMPRESSED_RGBA_ASTC_10x10: GLuint = 37819;
pub const GL_COMPRESSED_RGBA_ASTC_12x10: GLuint = 37820;
pub const GL_COMPRESSED_RGBA_ASTC_12x12: GLuint = 37821;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4: GLuint = 37840;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4: GLuint = 37841;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5: GLuint = 37842;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5: GLuint = 37843;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6: GLuint = 37844;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5: GLuint = 37845;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6: GLuint = 37846;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8: GLuint = 37847;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5: GLuint = 37848;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6: GLuint = 37849;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8: GLuint = 37850;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10: GLuint = 37851;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10: GLuint = 37852;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12: GLuint = 37853;
pub const GL_TEXTURE_CUBE_MAP_ARRAY: GLuint = 36873;
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY: GLuint = 36874;
pub const GL_SAMPLER_CUBE_MAP_ARRAY: GLuint = 36876;
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLuint = 36877;
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY: GLuint = 36878;
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLuint = 36879;
pub const GL_IMAGE_CUBE_MAP_ARRAY: GLuint = 36948;
pub const GL_INT_IMAGE_CUBE_MAP_ARRAY: GLuint = 36959;
pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLuint = 36970;
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: GLuint = 37122;
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLuint = 37125;
pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: GLuint = 37131;
pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLuint = 37132;
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLuint = 37133;