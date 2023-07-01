use super::consts;

#[repr(u32)]
pub enum AlphaFunction {
    Always = consts::GL_ALWAYS,
    Equal = consts::GL_EQUAL,
    Gequal = consts::GL_GEQUAL,
    Greater = consts::GL_GREATER,
    Lequal = consts::GL_LEQUAL,
    Less = consts::GL_LESS,
    Never = consts::GL_NEVER,
    Notequal = consts::GL_NOTEQUAL,
}

impl From<u32> for AlphaFunction {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ALWAYS => Self::Always,
            consts::GL_EQUAL => Self::Equal,
            consts::GL_GEQUAL => Self::Gequal,
            consts::GL_GREATER => Self::Greater,
            consts::GL_LEQUAL => Self::Lequal,
            consts::GL_LESS => Self::Less,
            consts::GL_NEVER => Self::Never,
            consts::GL_NOTEQUAL => Self::Notequal,
            v => panic!("Unknow value {v} for AlphaFunction"),
        }
    }
}

#[repr(u32)]
pub enum AttributeType {
    Bool = consts::GL_BOOL,
    BoolVec2 = consts::GL_BOOL_VEC2,
    BoolVec3 = consts::GL_BOOL_VEC3,
    BoolVec4 = consts::GL_BOOL_VEC4,
    Double = consts::GL_DOUBLE,
    Float = consts::GL_FLOAT,
    FloatMat2 = consts::GL_FLOAT_MAT2,
    FloatMat2x3 = consts::GL_FLOAT_MAT2x3,
    FloatMat2x4 = consts::GL_FLOAT_MAT2x4,
    FloatMat3 = consts::GL_FLOAT_MAT3,
    FloatMat3x2 = consts::GL_FLOAT_MAT3x2,
    FloatMat3x4 = consts::GL_FLOAT_MAT3x4,
    FloatMat4 = consts::GL_FLOAT_MAT4,
    FloatMat4x2 = consts::GL_FLOAT_MAT4x2,
    FloatMat4x3 = consts::GL_FLOAT_MAT4x3,
    FloatVec2 = consts::GL_FLOAT_VEC2,
    FloatVec3 = consts::GL_FLOAT_VEC3,
    FloatVec4 = consts::GL_FLOAT_VEC4,
    Image2d = consts::GL_IMAGE_2D,
    Image2dArray = consts::GL_IMAGE_2D_ARRAY,
    Image3d = consts::GL_IMAGE_3D,
    ImageBuffer = consts::GL_IMAGE_BUFFER,
    ImageCube = consts::GL_IMAGE_CUBE,
    ImageCubeMapArray = consts::GL_IMAGE_CUBE_MAP_ARRAY,
    Int = consts::GL_INT,
    IntImage2d = consts::GL_INT_IMAGE_2D,
    IntImage2dArray = consts::GL_INT_IMAGE_2D_ARRAY,
    IntImage3d = consts::GL_INT_IMAGE_3D,
    IntImageBuffer = consts::GL_INT_IMAGE_BUFFER,
    IntImageCube = consts::GL_INT_IMAGE_CUBE,
    IntImageCubeMapArray = consts::GL_INT_IMAGE_CUBE_MAP_ARRAY,
    IntSampler1d = consts::GL_INT_SAMPLER_1D,
    IntSampler1dArray = consts::GL_INT_SAMPLER_1D_ARRAY,
    IntSampler2d = consts::GL_INT_SAMPLER_2D,
    IntSampler2dArray = consts::GL_INT_SAMPLER_2D_ARRAY,
    IntSampler2dMultisample = consts::GL_INT_SAMPLER_2D_MULTISAMPLE,
    IntSampler2dMultisampleArray = consts::GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY,
    IntSampler2dRect = consts::GL_INT_SAMPLER_2D_RECT,
    IntSampler3d = consts::GL_INT_SAMPLER_3D,
    IntSamplerBuffer = consts::GL_INT_SAMPLER_BUFFER,
    IntSamplerCube = consts::GL_INT_SAMPLER_CUBE,
    IntSamplerCubeMapArray = consts::GL_INT_SAMPLER_CUBE_MAP_ARRAY,
    IntVec2 = consts::GL_INT_VEC2,
    IntVec3 = consts::GL_INT_VEC3,
    IntVec4 = consts::GL_INT_VEC4,
    Sampler1d = consts::GL_SAMPLER_1D,
    Sampler1dArrayShadow = consts::GL_SAMPLER_1D_ARRAY_SHADOW,
    Sampler1dShadow = consts::GL_SAMPLER_1D_SHADOW,
    Sampler2d = consts::GL_SAMPLER_2D,
    Sampler2dArrayShadow = consts::GL_SAMPLER_2D_ARRAY_SHADOW,
    Sampler2dMultisample = consts::GL_SAMPLER_2D_MULTISAMPLE,
    Sampler2dMultisampleArray = consts::GL_SAMPLER_2D_MULTISAMPLE_ARRAY,
    Sampler2dRect = consts::GL_SAMPLER_2D_RECT,
    Sampler2dRectShadow = consts::GL_SAMPLER_2D_RECT_SHADOW,
    Sampler2dShadow = consts::GL_SAMPLER_2D_SHADOW,
    Sampler3d = consts::GL_SAMPLER_3D,
    SamplerBuffer = consts::GL_SAMPLER_BUFFER,
    SamplerCube = consts::GL_SAMPLER_CUBE,
    SamplerCubeMapArray = consts::GL_SAMPLER_CUBE_MAP_ARRAY,
    SamplerCubeMapArrayShadow = consts::GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW,
    SamplerCubeShadow = consts::GL_SAMPLER_CUBE_SHADOW,
    UnsignedInt = consts::GL_UNSIGNED_INT,
    UnsignedIntImage2d = consts::GL_UNSIGNED_INT_IMAGE_2D,
    UnsignedIntImage2dArray = consts::GL_UNSIGNED_INT_IMAGE_2D_ARRAY,
    UnsignedIntImage3d = consts::GL_UNSIGNED_INT_IMAGE_3D,
    UnsignedIntImageBuffer = consts::GL_UNSIGNED_INT_IMAGE_BUFFER,
    UnsignedIntImageCube = consts::GL_UNSIGNED_INT_IMAGE_CUBE,
    UnsignedIntImageCubeMapArray = consts::GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY,
    UnsignedIntSampler1d = consts::GL_UNSIGNED_INT_SAMPLER_1D,
    UnsignedIntSampler1dArray = consts::GL_UNSIGNED_INT_SAMPLER_1D_ARRAY,
    UnsignedIntSampler2d = consts::GL_UNSIGNED_INT_SAMPLER_2D,
    UnsignedIntSampler2dArray = consts::GL_UNSIGNED_INT_SAMPLER_2D_ARRAY,
    UnsignedIntSampler2dMultisample = consts::GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE,
    UnsignedIntSampler2dMultisampleArray = consts::GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY,
    UnsignedIntSampler2dRect = consts::GL_UNSIGNED_INT_SAMPLER_2D_RECT,
    UnsignedIntSampler3d = consts::GL_UNSIGNED_INT_SAMPLER_3D,
    UnsignedIntSamplerBuffer = consts::GL_UNSIGNED_INT_SAMPLER_BUFFER,
    UnsignedIntSamplerCube = consts::GL_UNSIGNED_INT_SAMPLER_CUBE,
    UnsignedIntSamplerCubeMapArray = consts::GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY,
    UnsignedIntVec2 = consts::GL_UNSIGNED_INT_VEC2,
    UnsignedIntVec3 = consts::GL_UNSIGNED_INT_VEC3,
    UnsignedIntVec4 = consts::GL_UNSIGNED_INT_VEC4,
}

impl From<u32> for AttributeType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BOOL => Self::Bool,
            consts::GL_BOOL_VEC2 => Self::BoolVec2,
            consts::GL_BOOL_VEC3 => Self::BoolVec3,
            consts::GL_BOOL_VEC4 => Self::BoolVec4,
            consts::GL_DOUBLE => Self::Double,
            consts::GL_FLOAT => Self::Float,
            consts::GL_FLOAT_MAT2 => Self::FloatMat2,
            consts::GL_FLOAT_MAT2x3 => Self::FloatMat2x3,
            consts::GL_FLOAT_MAT2x4 => Self::FloatMat2x4,
            consts::GL_FLOAT_MAT3 => Self::FloatMat3,
            consts::GL_FLOAT_MAT3x2 => Self::FloatMat3x2,
            consts::GL_FLOAT_MAT3x4 => Self::FloatMat3x4,
            consts::GL_FLOAT_MAT4 => Self::FloatMat4,
            consts::GL_FLOAT_MAT4x2 => Self::FloatMat4x2,
            consts::GL_FLOAT_MAT4x3 => Self::FloatMat4x3,
            consts::GL_FLOAT_VEC2 => Self::FloatVec2,
            consts::GL_FLOAT_VEC3 => Self::FloatVec3,
            consts::GL_FLOAT_VEC4 => Self::FloatVec4,
            consts::GL_IMAGE_2D => Self::Image2d,
            consts::GL_IMAGE_2D_ARRAY => Self::Image2dArray,
            consts::GL_IMAGE_3D => Self::Image3d,
            consts::GL_IMAGE_BUFFER => Self::ImageBuffer,
            consts::GL_IMAGE_CUBE => Self::ImageCube,
            consts::GL_IMAGE_CUBE_MAP_ARRAY => Self::ImageCubeMapArray,
            consts::GL_INT => Self::Int,
            consts::GL_INT_IMAGE_2D => Self::IntImage2d,
            consts::GL_INT_IMAGE_2D_ARRAY => Self::IntImage2dArray,
            consts::GL_INT_IMAGE_3D => Self::IntImage3d,
            consts::GL_INT_IMAGE_BUFFER => Self::IntImageBuffer,
            consts::GL_INT_IMAGE_CUBE => Self::IntImageCube,
            consts::GL_INT_IMAGE_CUBE_MAP_ARRAY => Self::IntImageCubeMapArray,
            consts::GL_INT_SAMPLER_1D => Self::IntSampler1d,
            consts::GL_INT_SAMPLER_1D_ARRAY => Self::IntSampler1dArray,
            consts::GL_INT_SAMPLER_2D => Self::IntSampler2d,
            consts::GL_INT_SAMPLER_2D_ARRAY => Self::IntSampler2dArray,
            consts::GL_INT_SAMPLER_2D_MULTISAMPLE => Self::IntSampler2dMultisample,
            consts::GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY => Self::IntSampler2dMultisampleArray,
            consts::GL_INT_SAMPLER_2D_RECT => Self::IntSampler2dRect,
            consts::GL_INT_SAMPLER_3D => Self::IntSampler3d,
            consts::GL_INT_SAMPLER_BUFFER => Self::IntSamplerBuffer,
            consts::GL_INT_SAMPLER_CUBE => Self::IntSamplerCube,
            consts::GL_INT_SAMPLER_CUBE_MAP_ARRAY => Self::IntSamplerCubeMapArray,
            consts::GL_INT_VEC2 => Self::IntVec2,
            consts::GL_INT_VEC3 => Self::IntVec3,
            consts::GL_INT_VEC4 => Self::IntVec4,
            consts::GL_SAMPLER_1D => Self::Sampler1d,
            consts::GL_SAMPLER_1D_ARRAY_SHADOW => Self::Sampler1dArrayShadow,
            consts::GL_SAMPLER_1D_SHADOW => Self::Sampler1dShadow,
            consts::GL_SAMPLER_2D => Self::Sampler2d,
            consts::GL_SAMPLER_2D_ARRAY_SHADOW => Self::Sampler2dArrayShadow,
            consts::GL_SAMPLER_2D_MULTISAMPLE => Self::Sampler2dMultisample,
            consts::GL_SAMPLER_2D_MULTISAMPLE_ARRAY => Self::Sampler2dMultisampleArray,
            consts::GL_SAMPLER_2D_RECT => Self::Sampler2dRect,
            consts::GL_SAMPLER_2D_RECT_SHADOW => Self::Sampler2dRectShadow,
            consts::GL_SAMPLER_2D_SHADOW => Self::Sampler2dShadow,
            consts::GL_SAMPLER_3D => Self::Sampler3d,
            consts::GL_SAMPLER_BUFFER => Self::SamplerBuffer,
            consts::GL_SAMPLER_CUBE => Self::SamplerCube,
            consts::GL_SAMPLER_CUBE_MAP_ARRAY => Self::SamplerCubeMapArray,
            consts::GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW => Self::SamplerCubeMapArrayShadow,
            consts::GL_SAMPLER_CUBE_SHADOW => Self::SamplerCubeShadow,
            consts::GL_UNSIGNED_INT => Self::UnsignedInt,
            consts::GL_UNSIGNED_INT_IMAGE_2D => Self::UnsignedIntImage2d,
            consts::GL_UNSIGNED_INT_IMAGE_2D_ARRAY => Self::UnsignedIntImage2dArray,
            consts::GL_UNSIGNED_INT_IMAGE_3D => Self::UnsignedIntImage3d,
            consts::GL_UNSIGNED_INT_IMAGE_BUFFER => Self::UnsignedIntImageBuffer,
            consts::GL_UNSIGNED_INT_IMAGE_CUBE => Self::UnsignedIntImageCube,
            consts::GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY => Self::UnsignedIntImageCubeMapArray,
            consts::GL_UNSIGNED_INT_SAMPLER_1D => Self::UnsignedIntSampler1d,
            consts::GL_UNSIGNED_INT_SAMPLER_1D_ARRAY => Self::UnsignedIntSampler1dArray,
            consts::GL_UNSIGNED_INT_SAMPLER_2D => Self::UnsignedIntSampler2d,
            consts::GL_UNSIGNED_INT_SAMPLER_2D_ARRAY => Self::UnsignedIntSampler2dArray,
            consts::GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE => Self::UnsignedIntSampler2dMultisample,
            consts::GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY => {
                Self::UnsignedIntSampler2dMultisampleArray
            }
            consts::GL_UNSIGNED_INT_SAMPLER_2D_RECT => Self::UnsignedIntSampler2dRect,
            consts::GL_UNSIGNED_INT_SAMPLER_3D => Self::UnsignedIntSampler3d,
            consts::GL_UNSIGNED_INT_SAMPLER_BUFFER => Self::UnsignedIntSamplerBuffer,
            consts::GL_UNSIGNED_INT_SAMPLER_CUBE => Self::UnsignedIntSamplerCube,
            consts::GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY => Self::UnsignedIntSamplerCubeMapArray,
            consts::GL_UNSIGNED_INT_VEC2 => Self::UnsignedIntVec2,
            consts::GL_UNSIGNED_INT_VEC3 => Self::UnsignedIntVec3,
            consts::GL_UNSIGNED_INT_VEC4 => Self::UnsignedIntVec4,
            v => panic!("Unknow value {v} for AttributeType"),
        }
    }
}

#[repr(u32)]
pub enum BindTransformFeedbackTarget {
    TransformFeedback = consts::GL_TRANSFORM_FEEDBACK,
}

impl From<u32> for BindTransformFeedbackTarget {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_TRANSFORM_FEEDBACK => Self::TransformFeedback,
            v => panic!("Unknow value {v} for BindTransformFeedbackTarget"),
        }
    }
}

#[repr(u32)]
pub enum BlendEquationModeEXT {
    FuncAdd = consts::GL_FUNC_ADD,
    FuncReverseSubtract = consts::GL_FUNC_REVERSE_SUBTRACT,
    FuncSubtract = consts::GL_FUNC_SUBTRACT,
    Max = consts::GL_MAX,
    Min = consts::GL_MIN,
}

impl From<u32> for BlendEquationModeEXT {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_FUNC_ADD => Self::FuncAdd,
            consts::GL_FUNC_REVERSE_SUBTRACT => Self::FuncReverseSubtract,
            consts::GL_FUNC_SUBTRACT => Self::FuncSubtract,
            consts::GL_MAX => Self::Max,
            consts::GL_MIN => Self::Min,
            v => panic!("Unknow value {v} for BlendEquationModeEXT"),
        }
    }
}

#[repr(u32)]
pub enum BlendingFactor {
    ConstantAlpha = consts::GL_CONSTANT_ALPHA,
    ConstantColor = consts::GL_CONSTANT_COLOR,
    DstAlpha = consts::GL_DST_ALPHA,
    DstColor = consts::GL_DST_COLOR,
    One = consts::GL_ONE,
    OneMinusConstantAlpha = consts::GL_ONE_MINUS_CONSTANT_ALPHA,
    OneMinusConstantColor = consts::GL_ONE_MINUS_CONSTANT_COLOR,
    OneMinusDstAlpha = consts::GL_ONE_MINUS_DST_ALPHA,
    OneMinusDstColor = consts::GL_ONE_MINUS_DST_COLOR,
    OneMinusSrc1Alpha = consts::GL_ONE_MINUS_SRC1_ALPHA,
    OneMinusSrc1Color = consts::GL_ONE_MINUS_SRC1_COLOR,
    OneMinusSrcAlpha = consts::GL_ONE_MINUS_SRC_ALPHA,
    OneMinusSrcColor = consts::GL_ONE_MINUS_SRC_COLOR,
    Src1Alpha = consts::GL_SRC1_ALPHA,
    Src1Color = consts::GL_SRC1_COLOR,
    SrcAlpha = consts::GL_SRC_ALPHA,
    SrcAlphaSaturate = consts::GL_SRC_ALPHA_SATURATE,
    SrcColor = consts::GL_SRC_COLOR,
    Zero = consts::GL_ZERO,
}

impl From<u32> for BlendingFactor {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_CONSTANT_ALPHA => Self::ConstantAlpha,
            consts::GL_CONSTANT_COLOR => Self::ConstantColor,
            consts::GL_DST_ALPHA => Self::DstAlpha,
            consts::GL_DST_COLOR => Self::DstColor,
            consts::GL_ONE => Self::One,
            consts::GL_ONE_MINUS_CONSTANT_ALPHA => Self::OneMinusConstantAlpha,
            consts::GL_ONE_MINUS_CONSTANT_COLOR => Self::OneMinusConstantColor,
            consts::GL_ONE_MINUS_DST_ALPHA => Self::OneMinusDstAlpha,
            consts::GL_ONE_MINUS_DST_COLOR => Self::OneMinusDstColor,
            consts::GL_ONE_MINUS_SRC1_ALPHA => Self::OneMinusSrc1Alpha,
            consts::GL_ONE_MINUS_SRC1_COLOR => Self::OneMinusSrc1Color,
            consts::GL_ONE_MINUS_SRC_ALPHA => Self::OneMinusSrcAlpha,
            consts::GL_ONE_MINUS_SRC_COLOR => Self::OneMinusSrcColor,
            consts::GL_SRC1_ALPHA => Self::Src1Alpha,
            consts::GL_SRC1_COLOR => Self::Src1Color,
            consts::GL_SRC_ALPHA => Self::SrcAlpha,
            consts::GL_SRC_ALPHA_SATURATE => Self::SrcAlphaSaturate,
            consts::GL_SRC_COLOR => Self::SrcColor,
            consts::GL_ZERO => Self::Zero,
            v => panic!("Unknow value {v} for BlendingFactor"),
        }
    }
}

#[repr(u32)]
pub enum BlitFramebufferFilter {
    Linear = consts::GL_LINEAR,
    Nearest = consts::GL_NEAREST,
}

impl From<u32> for BlitFramebufferFilter {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_LINEAR => Self::Linear,
            consts::GL_NEAREST => Self::Nearest,
            v => panic!("Unknow value {v} for BlitFramebufferFilter"),
        }
    }
}

#[repr(u8)]
pub enum Boolean {
    False = consts::GL_FALSE as u8,
    True = consts::GL_TRUE as u8,
}

impl From<u8> for Boolean {
    fn from(value: u8) -> Self {
        match value as u32 {
            consts::GL_FALSE => Self::False,
            consts::GL_TRUE => Self::True,
            v => panic!("Unknow value {v} for Boolean"),
        }
    }
}

#[repr(u32)]
pub enum Buffer {
    Color = consts::GL_COLOR,
    Depth = consts::GL_DEPTH,
    Stencil = consts::GL_STENCIL,
}

impl From<u32> for Buffer {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_COLOR => Self::Color,
            consts::GL_DEPTH => Self::Depth,
            consts::GL_STENCIL => Self::Stencil,
            v => panic!("Unknow value {v} for Buffer"),
        }
    }
}

#[repr(u32)]
pub enum BufferAccessARB {
    ReadOnly = consts::GL_READ_ONLY,
    ReadWrite = consts::GL_READ_WRITE,
    WriteOnly = consts::GL_WRITE_ONLY,
}

impl From<u32> for BufferAccessARB {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_READ_ONLY => Self::ReadOnly,
            consts::GL_READ_WRITE => Self::ReadWrite,
            consts::GL_WRITE_ONLY => Self::WriteOnly,
            v => panic!("Unknow value {v} for BufferAccessARB"),
        }
    }
}

#[repr(u32)]
pub enum BufferPNameARB {
    BufferAccess = consts::GL_BUFFER_ACCESS,
    BufferAccessFlags = consts::GL_BUFFER_ACCESS_FLAGS,
    BufferMapped = consts::GL_BUFFER_MAPPED,
    BufferMapLength = consts::GL_BUFFER_MAP_LENGTH,
    BufferMapOffset = consts::GL_BUFFER_MAP_OFFSET,
    BufferSize = consts::GL_BUFFER_SIZE,
    BufferUsage = consts::GL_BUFFER_USAGE,
}

impl From<u32> for BufferPNameARB {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BUFFER_ACCESS => Self::BufferAccess,
            consts::GL_BUFFER_ACCESS_FLAGS => Self::BufferAccessFlags,
            consts::GL_BUFFER_MAPPED => Self::BufferMapped,
            consts::GL_BUFFER_MAP_LENGTH => Self::BufferMapLength,
            consts::GL_BUFFER_MAP_OFFSET => Self::BufferMapOffset,
            consts::GL_BUFFER_SIZE => Self::BufferSize,
            consts::GL_BUFFER_USAGE => Self::BufferUsage,
            v => panic!("Unknow value {v} for BufferPNameARB"),
        }
    }
}

#[repr(u32)]
pub enum BufferPointerNameARB {
    BufferMapPointer = consts::GL_BUFFER_MAP_POINTER,
}

impl From<u32> for BufferPointerNameARB {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BUFFER_MAP_POINTER => Self::BufferMapPointer,
            v => panic!("Unknow value {v} for BufferPointerNameARB"),
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum BufferTargetARB {
    ArrayBuffer = consts::GL_ARRAY_BUFFER,
    AtomicCounterBuffer = consts::GL_ATOMIC_COUNTER_BUFFER,
    CopyReadBuffer = consts::GL_COPY_READ_BUFFER,
    CopyWriteBuffer = consts::GL_COPY_WRITE_BUFFER,
    DispatchIndirectBuffer = consts::GL_DISPATCH_INDIRECT_BUFFER,
    DrawIndirectBuffer = consts::GL_DRAW_INDIRECT_BUFFER,
    ElementArrayBuffer = consts::GL_ELEMENT_ARRAY_BUFFER,
    PixelPackBuffer = consts::GL_PIXEL_PACK_BUFFER,
    PixelUnpackBuffer = consts::GL_PIXEL_UNPACK_BUFFER,
    ShaderStorageBuffer = consts::GL_SHADER_STORAGE_BUFFER,
    TextureBuffer = consts::GL_TEXTURE_BUFFER,
    TransformFeedbackBuffer = consts::GL_TRANSFORM_FEEDBACK_BUFFER,
    UniformBuffer = consts::GL_UNIFORM_BUFFER,
}

impl From<u32> for BufferTargetARB {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ARRAY_BUFFER => Self::ArrayBuffer,
            consts::GL_ATOMIC_COUNTER_BUFFER => Self::AtomicCounterBuffer,
            consts::GL_COPY_READ_BUFFER => Self::CopyReadBuffer,
            consts::GL_COPY_WRITE_BUFFER => Self::CopyWriteBuffer,
            consts::GL_DISPATCH_INDIRECT_BUFFER => Self::DispatchIndirectBuffer,
            consts::GL_DRAW_INDIRECT_BUFFER => Self::DrawIndirectBuffer,
            consts::GL_ELEMENT_ARRAY_BUFFER => Self::ElementArrayBuffer,
            consts::GL_PIXEL_PACK_BUFFER => Self::PixelPackBuffer,
            consts::GL_PIXEL_UNPACK_BUFFER => Self::PixelUnpackBuffer,
            consts::GL_SHADER_STORAGE_BUFFER => Self::ShaderStorageBuffer,
            consts::GL_TEXTURE_BUFFER => Self::TextureBuffer,
            consts::GL_TRANSFORM_FEEDBACK_BUFFER => Self::TransformFeedbackBuffer,
            consts::GL_UNIFORM_BUFFER => Self::UniformBuffer,
            v => panic!("Unknow value {v} for BufferTargetARB"),
        }
    }
}

#[repr(u32)]
pub enum BufferUsageARB {
    DynamicCopy = consts::GL_DYNAMIC_COPY,
    DynamicDraw = consts::GL_DYNAMIC_DRAW,
    DynamicRead = consts::GL_DYNAMIC_READ,
    StaticCopy = consts::GL_STATIC_COPY,
    StaticDraw = consts::GL_STATIC_DRAW,
    StaticRead = consts::GL_STATIC_READ,
    StreamCopy = consts::GL_STREAM_COPY,
    StreamDraw = consts::GL_STREAM_DRAW,
    StreamRead = consts::GL_STREAM_READ,
}

impl From<u32> for BufferUsageARB {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_DYNAMIC_COPY => Self::DynamicCopy,
            consts::GL_DYNAMIC_DRAW => Self::DynamicDraw,
            consts::GL_DYNAMIC_READ => Self::DynamicRead,
            consts::GL_STATIC_COPY => Self::StaticCopy,
            consts::GL_STATIC_DRAW => Self::StaticDraw,
            consts::GL_STATIC_READ => Self::StaticRead,
            consts::GL_STREAM_COPY => Self::StreamCopy,
            consts::GL_STREAM_DRAW => Self::StreamDraw,
            consts::GL_STREAM_READ => Self::StreamRead,
            v => panic!("Unknow value {v} for BufferUsageARB"),
        }
    }
}

#[repr(u32)]
pub enum ClampColorModeARB {
    False = consts::GL_FALSE,
    FixedOnly = consts::GL_FIXED_ONLY,
    True = consts::GL_TRUE,
}

impl From<u32> for ClampColorModeARB {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_FALSE => Self::False,
            consts::GL_FIXED_ONLY => Self::FixedOnly,
            consts::GL_TRUE => Self::True,
            v => panic!("Unknow value {v} for ClampColorModeARB"),
        }
    }
}

#[repr(u32)]
pub enum ClampColorTargetARB {
    ClampReadColor = consts::GL_CLAMP_READ_COLOR,
}

impl From<u32> for ClampColorTargetARB {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_CLAMP_READ_COLOR => Self::ClampReadColor,
            v => panic!("Unknow value {v} for ClampColorTargetARB"),
        }
    }
}

#[repr(u32)]
pub enum ClearBufferMask {
    ColorBufferBit = consts::GL_COLOR_BUFFER_BIT,
    DepthBufferBit = consts::GL_DEPTH_BUFFER_BIT,
    StencilBufferBit = consts::GL_STENCIL_BUFFER_BIT,
}

impl From<u32> for ClearBufferMask {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_COLOR_BUFFER_BIT => Self::ColorBufferBit,
            consts::GL_DEPTH_BUFFER_BIT => Self::DepthBufferBit,
            consts::GL_STENCIL_BUFFER_BIT => Self::StencilBufferBit,
            v => panic!("Unknow value {v} for ClearBufferMask"),
        }
    }
}

#[repr(u32)]
pub enum ClipPlaneName {
    ClipDistance6 = consts::GL_CLIP_DISTANCE6,
    ClipDistance7 = consts::GL_CLIP_DISTANCE7,
    ClipPlane0 = consts::GL_CLIP_PLANE0,
    ClipPlane1 = consts::GL_CLIP_PLANE1,
    ClipPlane2 = consts::GL_CLIP_PLANE2,
    ClipPlane3 = consts::GL_CLIP_PLANE3,
    ClipPlane4 = consts::GL_CLIP_PLANE4,
    ClipPlane5 = consts::GL_CLIP_PLANE5,
}

impl ClipPlaneName {
    pub const ClipDistance0: Self = Self::ClipPlane0;
    pub const ClipDistance1: Self = Self::ClipPlane1;
    pub const ClipDistance2: Self = Self::ClipPlane2;
    pub const ClipDistance3: Self = Self::ClipPlane3;
    pub const ClipDistance4: Self = Self::ClipPlane4;
    pub const ClipDistance5: Self = Self::ClipPlane5;
}

impl From<u32> for ClipPlaneName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_CLIP_DISTANCE6 => Self::ClipDistance6,
            consts::GL_CLIP_DISTANCE7 => Self::ClipDistance7,
            consts::GL_CLIP_PLANE0 => Self::ClipPlane0,
            consts::GL_CLIP_PLANE1 => Self::ClipPlane1,
            consts::GL_CLIP_PLANE2 => Self::ClipPlane2,
            consts::GL_CLIP_PLANE3 => Self::ClipPlane3,
            consts::GL_CLIP_PLANE4 => Self::ClipPlane4,
            consts::GL_CLIP_PLANE5 => Self::ClipPlane5,
            v => panic!("Unknow value {v} for ClipPlaneName"),
        }
    }
}

#[repr(u32)]
pub enum ColorPointerType {
    Byte = consts::GL_BYTE,
    UnsignedByte = consts::GL_UNSIGNED_BYTE,
    UnsignedInt = consts::GL_UNSIGNED_INT,
    UnsignedShort = consts::GL_UNSIGNED_SHORT,
}

impl From<u32> for ColorPointerType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BYTE => Self::Byte,
            consts::GL_UNSIGNED_BYTE => Self::UnsignedByte,
            consts::GL_UNSIGNED_INT => Self::UnsignedInt,
            consts::GL_UNSIGNED_SHORT => Self::UnsignedShort,
            v => panic!("Unknow value {v} for ColorPointerType"),
        }
    }
}

#[repr(u32)]
pub enum ConditionalRenderMode {
    QueryByRegionNoWait = consts::GL_QUERY_BY_REGION_NO_WAIT,
    QueryByRegionWait = consts::GL_QUERY_BY_REGION_WAIT,
    QueryNoWait = consts::GL_QUERY_NO_WAIT,
    QueryWait = consts::GL_QUERY_WAIT,
}

impl From<u32> for ConditionalRenderMode {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_QUERY_BY_REGION_NO_WAIT => Self::QueryByRegionNoWait,
            consts::GL_QUERY_BY_REGION_WAIT => Self::QueryByRegionWait,
            consts::GL_QUERY_NO_WAIT => Self::QueryNoWait,
            consts::GL_QUERY_WAIT => Self::QueryWait,
            v => panic!("Unknow value {v} for ConditionalRenderMode"),
        }
    }
}

#[repr(u32)]
pub enum CopyBufferSubDataTarget {
    ArrayBuffer = consts::GL_ARRAY_BUFFER,
    AtomicCounterBuffer = consts::GL_ATOMIC_COUNTER_BUFFER,
    CopyReadBuffer = consts::GL_COPY_READ_BUFFER,
    CopyWriteBuffer = consts::GL_COPY_WRITE_BUFFER,
    DispatchIndirectBuffer = consts::GL_DISPATCH_INDIRECT_BUFFER,
    DrawIndirectBuffer = consts::GL_DRAW_INDIRECT_BUFFER,
    ElementArrayBuffer = consts::GL_ELEMENT_ARRAY_BUFFER,
    PixelPackBuffer = consts::GL_PIXEL_PACK_BUFFER,
    PixelUnpackBuffer = consts::GL_PIXEL_UNPACK_BUFFER,
    ShaderStorageBuffer = consts::GL_SHADER_STORAGE_BUFFER,
    TextureBuffer = consts::GL_TEXTURE_BUFFER,
    TransformFeedbackBuffer = consts::GL_TRANSFORM_FEEDBACK_BUFFER,
    UniformBuffer = consts::GL_UNIFORM_BUFFER,
}

impl From<u32> for CopyBufferSubDataTarget {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ARRAY_BUFFER => Self::ArrayBuffer,
            consts::GL_ATOMIC_COUNTER_BUFFER => Self::AtomicCounterBuffer,
            consts::GL_COPY_READ_BUFFER => Self::CopyReadBuffer,
            consts::GL_COPY_WRITE_BUFFER => Self::CopyWriteBuffer,
            consts::GL_DISPATCH_INDIRECT_BUFFER => Self::DispatchIndirectBuffer,
            consts::GL_DRAW_INDIRECT_BUFFER => Self::DrawIndirectBuffer,
            consts::GL_ELEMENT_ARRAY_BUFFER => Self::ElementArrayBuffer,
            consts::GL_PIXEL_PACK_BUFFER => Self::PixelPackBuffer,
            consts::GL_PIXEL_UNPACK_BUFFER => Self::PixelUnpackBuffer,
            consts::GL_SHADER_STORAGE_BUFFER => Self::ShaderStorageBuffer,
            consts::GL_TEXTURE_BUFFER => Self::TextureBuffer,
            consts::GL_TRANSFORM_FEEDBACK_BUFFER => Self::TransformFeedbackBuffer,
            consts::GL_UNIFORM_BUFFER => Self::UniformBuffer,
            v => panic!("Unknow value {v} for CopyBufferSubDataTarget"),
        }
    }
}

#[repr(u32)]
pub enum CopyImageSubDataTarget {
    Renderbuffer = consts::GL_RENDERBUFFER,
    Texture1d = consts::GL_TEXTURE_1D,
    Texture1dArray = consts::GL_TEXTURE_1D_ARRAY,
    Texture2d = consts::GL_TEXTURE_2D,
    Texture2dArray = consts::GL_TEXTURE_2D_ARRAY,
    Texture2dMultisample = consts::GL_TEXTURE_2D_MULTISAMPLE,
    Texture2dMultisampleArray = consts::GL_TEXTURE_2D_MULTISAMPLE_ARRAY,
    Texture3d = consts::GL_TEXTURE_3D,
    TextureCubeMap = consts::GL_TEXTURE_CUBE_MAP,
    TextureCubeMapArray = consts::GL_TEXTURE_CUBE_MAP_ARRAY,
    TextureRectangle = consts::GL_TEXTURE_RECTANGLE,
}

impl From<u32> for CopyImageSubDataTarget {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_RENDERBUFFER => Self::Renderbuffer,
            consts::GL_TEXTURE_1D => Self::Texture1d,
            consts::GL_TEXTURE_1D_ARRAY => Self::Texture1dArray,
            consts::GL_TEXTURE_2D => Self::Texture2d,
            consts::GL_TEXTURE_2D_ARRAY => Self::Texture2dArray,
            consts::GL_TEXTURE_2D_MULTISAMPLE => Self::Texture2dMultisample,
            consts::GL_TEXTURE_2D_MULTISAMPLE_ARRAY => Self::Texture2dMultisampleArray,
            consts::GL_TEXTURE_3D => Self::Texture3d,
            consts::GL_TEXTURE_CUBE_MAP => Self::TextureCubeMap,
            consts::GL_TEXTURE_CUBE_MAP_ARRAY => Self::TextureCubeMapArray,
            consts::GL_TEXTURE_RECTANGLE => Self::TextureRectangle,
            v => panic!("Unknow value {v} for CopyImageSubDataTarget"),
        }
    }
}

#[repr(u32)]
pub enum DebugSeverity {
    DebugSeverityHigh = consts::GL_DEBUG_SEVERITY_HIGH,
    DebugSeverityLow = consts::GL_DEBUG_SEVERITY_LOW,
    DebugSeverityMedium = consts::GL_DEBUG_SEVERITY_MEDIUM,
    DebugSeverityNotification = consts::GL_DEBUG_SEVERITY_NOTIFICATION,
    DontCare = consts::GL_DONT_CARE,
}

impl From<u32> for DebugSeverity {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_DEBUG_SEVERITY_HIGH => Self::DebugSeverityHigh,
            consts::GL_DEBUG_SEVERITY_LOW => Self::DebugSeverityLow,
            consts::GL_DEBUG_SEVERITY_MEDIUM => Self::DebugSeverityMedium,
            consts::GL_DEBUG_SEVERITY_NOTIFICATION => Self::DebugSeverityNotification,
            consts::GL_DONT_CARE => Self::DontCare,
            v => panic!("Unknow value {v} for DebugSeverity"),
        }
    }
}

#[repr(u32)]
pub enum DebugSource {
    DebugSourceApi = consts::GL_DEBUG_SOURCE_API,
    DebugSourceApplication = consts::GL_DEBUG_SOURCE_APPLICATION,
    DebugSourceOther = consts::GL_DEBUG_SOURCE_OTHER,
    DebugSourceShaderCompiler = consts::GL_DEBUG_SOURCE_SHADER_COMPILER,
    DebugSourceThirdParty = consts::GL_DEBUG_SOURCE_THIRD_PARTY,
    DebugSourceWindowSystem = consts::GL_DEBUG_SOURCE_WINDOW_SYSTEM,
    DontCare = consts::GL_DONT_CARE,
}

impl From<u32> for DebugSource {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_DEBUG_SOURCE_API => Self::DebugSourceApi,
            consts::GL_DEBUG_SOURCE_APPLICATION => Self::DebugSourceApplication,
            consts::GL_DEBUG_SOURCE_OTHER => Self::DebugSourceOther,
            consts::GL_DEBUG_SOURCE_SHADER_COMPILER => Self::DebugSourceShaderCompiler,
            consts::GL_DEBUG_SOURCE_THIRD_PARTY => Self::DebugSourceThirdParty,
            consts::GL_DEBUG_SOURCE_WINDOW_SYSTEM => Self::DebugSourceWindowSystem,
            consts::GL_DONT_CARE => Self::DontCare,
            v => panic!("Unknow value {v} for DebugSource"),
        }
    }
}

#[repr(u32)]
pub enum DebugType {
    DebugTypeDeprecatedBehavior = consts::GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR,
    DebugTypeError = consts::GL_DEBUG_TYPE_ERROR,
    DebugTypeMarker = consts::GL_DEBUG_TYPE_MARKER,
    DebugTypeOther = consts::GL_DEBUG_TYPE_OTHER,
    DebugTypePerformance = consts::GL_DEBUG_TYPE_PERFORMANCE,
    DebugTypePopGroup = consts::GL_DEBUG_TYPE_POP_GROUP,
    DebugTypePortability = consts::GL_DEBUG_TYPE_PORTABILITY,
    DebugTypePushGroup = consts::GL_DEBUG_TYPE_PUSH_GROUP,
    DebugTypeUndefinedBehavior = consts::GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR,
    DontCare = consts::GL_DONT_CARE,
}

impl From<u32> for DebugType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR => Self::DebugTypeDeprecatedBehavior,
            consts::GL_DEBUG_TYPE_ERROR => Self::DebugTypeError,
            consts::GL_DEBUG_TYPE_MARKER => Self::DebugTypeMarker,
            consts::GL_DEBUG_TYPE_OTHER => Self::DebugTypeOther,
            consts::GL_DEBUG_TYPE_PERFORMANCE => Self::DebugTypePerformance,
            consts::GL_DEBUG_TYPE_POP_GROUP => Self::DebugTypePopGroup,
            consts::GL_DEBUG_TYPE_PORTABILITY => Self::DebugTypePortability,
            consts::GL_DEBUG_TYPE_PUSH_GROUP => Self::DebugTypePushGroup,
            consts::GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR => Self::DebugTypeUndefinedBehavior,
            consts::GL_DONT_CARE => Self::DontCare,
            v => panic!("Unknow value {v} for DebugType"),
        }
    }
}

#[repr(u32)]
pub enum DepthFunction {
    Always = consts::GL_ALWAYS,
    Equal = consts::GL_EQUAL,
    Gequal = consts::GL_GEQUAL,
    Greater = consts::GL_GREATER,
    Lequal = consts::GL_LEQUAL,
    Less = consts::GL_LESS,
    Never = consts::GL_NEVER,
    Notequal = consts::GL_NOTEQUAL,
}

impl From<u32> for DepthFunction {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ALWAYS => Self::Always,
            consts::GL_EQUAL => Self::Equal,
            consts::GL_GEQUAL => Self::Gequal,
            consts::GL_GREATER => Self::Greater,
            consts::GL_LEQUAL => Self::Lequal,
            consts::GL_LESS => Self::Less,
            consts::GL_NEVER => Self::Never,
            consts::GL_NOTEQUAL => Self::Notequal,
            v => panic!("Unknow value {v} for DepthFunction"),
        }
    }
}

#[repr(u32)]
pub enum DrawBufferMode {
    Back = consts::GL_BACK,
    BackLeft = consts::GL_BACK_LEFT,
    BackRight = consts::GL_BACK_RIGHT,
    ColorAttachment0 = consts::GL_COLOR_ATTACHMENT0,
    ColorAttachment1 = consts::GL_COLOR_ATTACHMENT1,
    ColorAttachment10 = consts::GL_COLOR_ATTACHMENT10,
    ColorAttachment11 = consts::GL_COLOR_ATTACHMENT11,
    ColorAttachment12 = consts::GL_COLOR_ATTACHMENT12,
    ColorAttachment13 = consts::GL_COLOR_ATTACHMENT13,
    ColorAttachment14 = consts::GL_COLOR_ATTACHMENT14,
    ColorAttachment15 = consts::GL_COLOR_ATTACHMENT15,
    ColorAttachment16 = consts::GL_COLOR_ATTACHMENT16,
    ColorAttachment17 = consts::GL_COLOR_ATTACHMENT17,
    ColorAttachment18 = consts::GL_COLOR_ATTACHMENT18,
    ColorAttachment19 = consts::GL_COLOR_ATTACHMENT19,
    ColorAttachment2 = consts::GL_COLOR_ATTACHMENT2,
    ColorAttachment20 = consts::GL_COLOR_ATTACHMENT20,
    ColorAttachment21 = consts::GL_COLOR_ATTACHMENT21,
    ColorAttachment22 = consts::GL_COLOR_ATTACHMENT22,
    ColorAttachment23 = consts::GL_COLOR_ATTACHMENT23,
    ColorAttachment24 = consts::GL_COLOR_ATTACHMENT24,
    ColorAttachment25 = consts::GL_COLOR_ATTACHMENT25,
    ColorAttachment26 = consts::GL_COLOR_ATTACHMENT26,
    ColorAttachment27 = consts::GL_COLOR_ATTACHMENT27,
    ColorAttachment28 = consts::GL_COLOR_ATTACHMENT28,
    ColorAttachment29 = consts::GL_COLOR_ATTACHMENT29,
    ColorAttachment3 = consts::GL_COLOR_ATTACHMENT3,
    ColorAttachment30 = consts::GL_COLOR_ATTACHMENT30,
    ColorAttachment31 = consts::GL_COLOR_ATTACHMENT31,
    ColorAttachment4 = consts::GL_COLOR_ATTACHMENT4,
    ColorAttachment5 = consts::GL_COLOR_ATTACHMENT5,
    ColorAttachment6 = consts::GL_COLOR_ATTACHMENT6,
    ColorAttachment7 = consts::GL_COLOR_ATTACHMENT7,
    ColorAttachment8 = consts::GL_COLOR_ATTACHMENT8,
    ColorAttachment9 = consts::GL_COLOR_ATTACHMENT9,
    Front = consts::GL_FRONT,
    FrontAndBack = consts::GL_FRONT_AND_BACK,
    FrontLeft = consts::GL_FRONT_LEFT,
    FrontRight = consts::GL_FRONT_RIGHT,
    Left = consts::GL_LEFT,
    None = consts::GL_NONE,
    Right = consts::GL_RIGHT,
}

impl From<u32> for DrawBufferMode {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BACK => Self::Back,
            consts::GL_BACK_LEFT => Self::BackLeft,
            consts::GL_BACK_RIGHT => Self::BackRight,
            consts::GL_COLOR_ATTACHMENT0 => Self::ColorAttachment0,
            consts::GL_COLOR_ATTACHMENT1 => Self::ColorAttachment1,
            consts::GL_COLOR_ATTACHMENT10 => Self::ColorAttachment10,
            consts::GL_COLOR_ATTACHMENT11 => Self::ColorAttachment11,
            consts::GL_COLOR_ATTACHMENT12 => Self::ColorAttachment12,
            consts::GL_COLOR_ATTACHMENT13 => Self::ColorAttachment13,
            consts::GL_COLOR_ATTACHMENT14 => Self::ColorAttachment14,
            consts::GL_COLOR_ATTACHMENT15 => Self::ColorAttachment15,
            consts::GL_COLOR_ATTACHMENT16 => Self::ColorAttachment16,
            consts::GL_COLOR_ATTACHMENT17 => Self::ColorAttachment17,
            consts::GL_COLOR_ATTACHMENT18 => Self::ColorAttachment18,
            consts::GL_COLOR_ATTACHMENT19 => Self::ColorAttachment19,
            consts::GL_COLOR_ATTACHMENT2 => Self::ColorAttachment2,
            consts::GL_COLOR_ATTACHMENT20 => Self::ColorAttachment20,
            consts::GL_COLOR_ATTACHMENT21 => Self::ColorAttachment21,
            consts::GL_COLOR_ATTACHMENT22 => Self::ColorAttachment22,
            consts::GL_COLOR_ATTACHMENT23 => Self::ColorAttachment23,
            consts::GL_COLOR_ATTACHMENT24 => Self::ColorAttachment24,
            consts::GL_COLOR_ATTACHMENT25 => Self::ColorAttachment25,
            consts::GL_COLOR_ATTACHMENT26 => Self::ColorAttachment26,
            consts::GL_COLOR_ATTACHMENT27 => Self::ColorAttachment27,
            consts::GL_COLOR_ATTACHMENT28 => Self::ColorAttachment28,
            consts::GL_COLOR_ATTACHMENT29 => Self::ColorAttachment29,
            consts::GL_COLOR_ATTACHMENT3 => Self::ColorAttachment3,
            consts::GL_COLOR_ATTACHMENT30 => Self::ColorAttachment30,
            consts::GL_COLOR_ATTACHMENT31 => Self::ColorAttachment31,
            consts::GL_COLOR_ATTACHMENT4 => Self::ColorAttachment4,
            consts::GL_COLOR_ATTACHMENT5 => Self::ColorAttachment5,
            consts::GL_COLOR_ATTACHMENT6 => Self::ColorAttachment6,
            consts::GL_COLOR_ATTACHMENT7 => Self::ColorAttachment7,
            consts::GL_COLOR_ATTACHMENT8 => Self::ColorAttachment8,
            consts::GL_COLOR_ATTACHMENT9 => Self::ColorAttachment9,
            consts::GL_FRONT => Self::Front,
            consts::GL_FRONT_AND_BACK => Self::FrontAndBack,
            consts::GL_FRONT_LEFT => Self::FrontLeft,
            consts::GL_FRONT_RIGHT => Self::FrontRight,
            consts::GL_LEFT => Self::Left,
            consts::GL_NONE => Self::None,
            consts::GL_RIGHT => Self::Right,
            v => panic!("Unknow value {v} for DrawBufferMode"),
        }
    }
}

#[repr(u32)]
pub enum DrawElementsType {
    UnsignedByte = consts::GL_UNSIGNED_BYTE,
    UnsignedInt = consts::GL_UNSIGNED_INT,
    UnsignedShort = consts::GL_UNSIGNED_SHORT,
}

impl From<u32> for DrawElementsType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_UNSIGNED_BYTE => Self::UnsignedByte,
            consts::GL_UNSIGNED_INT => Self::UnsignedInt,
            consts::GL_UNSIGNED_SHORT => Self::UnsignedShort,
            v => panic!("Unknow value {v} for DrawElementsType"),
        }
    }
}

#[repr(u32)]
pub enum EnableCap {
    AlphaTest = consts::GL_ALPHA_TEST,
    Blend = consts::GL_BLEND,
    ClipDistance6 = consts::GL_CLIP_DISTANCE6,
    ClipDistance7 = consts::GL_CLIP_DISTANCE7,
    ClipPlane0 = consts::GL_CLIP_PLANE0,
    ClipPlane1 = consts::GL_CLIP_PLANE1,
    ClipPlane2 = consts::GL_CLIP_PLANE2,
    ClipPlane3 = consts::GL_CLIP_PLANE3,
    ClipPlane4 = consts::GL_CLIP_PLANE4,
    ClipPlane5 = consts::GL_CLIP_PLANE5,
    ColorArray = consts::GL_COLOR_ARRAY,
    ColorLogicOp = consts::GL_COLOR_LOGIC_OP,
    ColorMaterial = consts::GL_COLOR_MATERIAL,
    CullFace = consts::GL_CULL_FACE,
    DebugOutput = consts::GL_DEBUG_OUTPUT,
    DebugOutputSynchronous = consts::GL_DEBUG_OUTPUT_SYNCHRONOUS,
    DepthClamp = consts::GL_DEPTH_CLAMP,
    DepthTest = consts::GL_DEPTH_TEST,
    Dither = consts::GL_DITHER,
    Fog = consts::GL_FOG,
    FramebufferSrgb = consts::GL_FRAMEBUFFER_SRGB,
    Light0 = consts::GL_LIGHT0,
    Light1 = consts::GL_LIGHT1,
    Light2 = consts::GL_LIGHT2,
    Light3 = consts::GL_LIGHT3,
    Light4 = consts::GL_LIGHT4,
    Light5 = consts::GL_LIGHT5,
    Light6 = consts::GL_LIGHT6,
    Light7 = consts::GL_LIGHT7,
    Lighting = consts::GL_LIGHTING,
    LineSmooth = consts::GL_LINE_SMOOTH,
    Multisample = consts::GL_MULTISAMPLE,
    Normalize = consts::GL_NORMALIZE,
    NormalArray = consts::GL_NORMAL_ARRAY,
    PointSmooth = consts::GL_POINT_SMOOTH,
    PolygonOffsetFill = consts::GL_POLYGON_OFFSET_FILL,
    PolygonOffsetLine = consts::GL_POLYGON_OFFSET_LINE,
    PolygonOffsetPoint = consts::GL_POLYGON_OFFSET_POINT,
    PolygonSmooth = consts::GL_POLYGON_SMOOTH,
    PrimitiveRestart = consts::GL_PRIMITIVE_RESTART,
    PrimitiveRestartFixedIndex = consts::GL_PRIMITIVE_RESTART_FIXED_INDEX,
    ProgramPointSize = consts::GL_PROGRAM_POINT_SIZE,
    RasterizerDiscard = consts::GL_RASTERIZER_DISCARD,
    SampleAlphaToCoverage = consts::GL_SAMPLE_ALPHA_TO_COVERAGE,
    SampleAlphaToOne = consts::GL_SAMPLE_ALPHA_TO_ONE,
    SampleCoverage = consts::GL_SAMPLE_COVERAGE,
    SampleMask = consts::GL_SAMPLE_MASK,
    SampleShading = consts::GL_SAMPLE_SHADING,
    ScissorTest = consts::GL_SCISSOR_TEST,
    StencilTest = consts::GL_STENCIL_TEST,
    Texture1d = consts::GL_TEXTURE_1D,
    Texture2d = consts::GL_TEXTURE_2D,
    TextureCoordArray = consts::GL_TEXTURE_COORD_ARRAY,
    TextureCubeMap = consts::GL_TEXTURE_CUBE_MAP,
    TextureCubeMapSeamless = consts::GL_TEXTURE_CUBE_MAP_SEAMLESS,
    TextureRectangle = consts::GL_TEXTURE_RECTANGLE,
    VertexArray = consts::GL_VERTEX_ARRAY,
}

impl EnableCap {
    pub const ClipDistance0: Self = Self::ClipPlane0;
    pub const ClipDistance1: Self = Self::ClipPlane1;
    pub const ClipDistance2: Self = Self::ClipPlane2;
    pub const ClipDistance3: Self = Self::ClipPlane3;
    pub const ClipDistance4: Self = Self::ClipPlane4;
    pub const ClipDistance5: Self = Self::ClipPlane5;
}

impl From<u32> for EnableCap {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ALPHA_TEST => Self::AlphaTest,
            consts::GL_BLEND => Self::Blend,
            consts::GL_CLIP_DISTANCE6 => Self::ClipDistance6,
            consts::GL_CLIP_DISTANCE7 => Self::ClipDistance7,
            consts::GL_CLIP_PLANE0 => Self::ClipPlane0,
            consts::GL_CLIP_PLANE1 => Self::ClipPlane1,
            consts::GL_CLIP_PLANE2 => Self::ClipPlane2,
            consts::GL_CLIP_PLANE3 => Self::ClipPlane3,
            consts::GL_CLIP_PLANE4 => Self::ClipPlane4,
            consts::GL_CLIP_PLANE5 => Self::ClipPlane5,
            consts::GL_COLOR_ARRAY => Self::ColorArray,
            consts::GL_COLOR_LOGIC_OP => Self::ColorLogicOp,
            consts::GL_COLOR_MATERIAL => Self::ColorMaterial,
            consts::GL_CULL_FACE => Self::CullFace,
            consts::GL_DEBUG_OUTPUT => Self::DebugOutput,
            consts::GL_DEBUG_OUTPUT_SYNCHRONOUS => Self::DebugOutputSynchronous,
            consts::GL_DEPTH_CLAMP => Self::DepthClamp,
            consts::GL_DEPTH_TEST => Self::DepthTest,
            consts::GL_DITHER => Self::Dither,
            consts::GL_FOG => Self::Fog,
            consts::GL_FRAMEBUFFER_SRGB => Self::FramebufferSrgb,
            consts::GL_LIGHT0 => Self::Light0,
            consts::GL_LIGHT1 => Self::Light1,
            consts::GL_LIGHT2 => Self::Light2,
            consts::GL_LIGHT3 => Self::Light3,
            consts::GL_LIGHT4 => Self::Light4,
            consts::GL_LIGHT5 => Self::Light5,
            consts::GL_LIGHT6 => Self::Light6,
            consts::GL_LIGHT7 => Self::Light7,
            consts::GL_LIGHTING => Self::Lighting,
            consts::GL_LINE_SMOOTH => Self::LineSmooth,
            consts::GL_MULTISAMPLE => Self::Multisample,
            consts::GL_NORMALIZE => Self::Normalize,
            consts::GL_NORMAL_ARRAY => Self::NormalArray,
            consts::GL_POINT_SMOOTH => Self::PointSmooth,
            consts::GL_POLYGON_OFFSET_FILL => Self::PolygonOffsetFill,
            consts::GL_POLYGON_OFFSET_LINE => Self::PolygonOffsetLine,
            consts::GL_POLYGON_OFFSET_POINT => Self::PolygonOffsetPoint,
            consts::GL_POLYGON_SMOOTH => Self::PolygonSmooth,
            consts::GL_PRIMITIVE_RESTART => Self::PrimitiveRestart,
            consts::GL_PRIMITIVE_RESTART_FIXED_INDEX => Self::PrimitiveRestartFixedIndex,
            consts::GL_PROGRAM_POINT_SIZE => Self::ProgramPointSize,
            consts::GL_RASTERIZER_DISCARD => Self::RasterizerDiscard,
            consts::GL_SAMPLE_ALPHA_TO_COVERAGE => Self::SampleAlphaToCoverage,
            consts::GL_SAMPLE_ALPHA_TO_ONE => Self::SampleAlphaToOne,
            consts::GL_SAMPLE_COVERAGE => Self::SampleCoverage,
            consts::GL_SAMPLE_MASK => Self::SampleMask,
            consts::GL_SAMPLE_SHADING => Self::SampleShading,
            consts::GL_SCISSOR_TEST => Self::ScissorTest,
            consts::GL_STENCIL_TEST => Self::StencilTest,
            consts::GL_TEXTURE_1D => Self::Texture1d,
            consts::GL_TEXTURE_2D => Self::Texture2d,
            consts::GL_TEXTURE_COORD_ARRAY => Self::TextureCoordArray,
            consts::GL_TEXTURE_CUBE_MAP => Self::TextureCubeMap,
            consts::GL_TEXTURE_CUBE_MAP_SEAMLESS => Self::TextureCubeMapSeamless,
            consts::GL_TEXTURE_RECTANGLE => Self::TextureRectangle,
            consts::GL_VERTEX_ARRAY => Self::VertexArray,
            v => panic!("Unknow value {v} for EnableCap"),
        }
    }
}

#[repr(u32)]
pub enum ErrorCode {
    InvalidEnum = consts::GL_INVALID_ENUM,
    InvalidFramebufferOperation = consts::GL_INVALID_FRAMEBUFFER_OPERATION,
    InvalidOperation = consts::GL_INVALID_OPERATION,
    InvalidValue = consts::GL_INVALID_VALUE,
    NoError = consts::GL_NO_ERROR,
    OutOfMemory = consts::GL_OUT_OF_MEMORY,
    StackOverflow = consts::GL_STACK_OVERFLOW,
    StackUnderflow = consts::GL_STACK_UNDERFLOW,
}

impl From<u32> for ErrorCode {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_INVALID_ENUM => Self::InvalidEnum,
            consts::GL_INVALID_FRAMEBUFFER_OPERATION => Self::InvalidFramebufferOperation,
            consts::GL_INVALID_OPERATION => Self::InvalidOperation,
            consts::GL_INVALID_VALUE => Self::InvalidValue,
            consts::GL_NO_ERROR => Self::NoError,
            consts::GL_OUT_OF_MEMORY => Self::OutOfMemory,
            consts::GL_STACK_OVERFLOW => Self::StackOverflow,
            consts::GL_STACK_UNDERFLOW => Self::StackUnderflow,
            v => panic!("Unknow value {v} for ErrorCode"),
        }
    }
}

#[repr(u32)]
pub enum FogPName {
    FogDensity = consts::GL_FOG_DENSITY,
    FogEnd = consts::GL_FOG_END,
    FogMode = consts::GL_FOG_MODE,
    FogStart = consts::GL_FOG_START,
}

impl From<u32> for FogPName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_FOG_DENSITY => Self::FogDensity,
            consts::GL_FOG_END => Self::FogEnd,
            consts::GL_FOG_MODE => Self::FogMode,
            consts::GL_FOG_START => Self::FogStart,
            v => panic!("Unknow value {v} for FogPName"),
        }
    }
}

#[repr(u32)]
pub enum FogParameter {
    FogColor = consts::GL_FOG_COLOR,
    FogDensity = consts::GL_FOG_DENSITY,
    FogEnd = consts::GL_FOG_END,
    FogMode = consts::GL_FOG_MODE,
    FogStart = consts::GL_FOG_START,
}

impl From<u32> for FogParameter {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_FOG_COLOR => Self::FogColor,
            consts::GL_FOG_DENSITY => Self::FogDensity,
            consts::GL_FOG_END => Self::FogEnd,
            consts::GL_FOG_MODE => Self::FogMode,
            consts::GL_FOG_START => Self::FogStart,
            v => panic!("Unknow value {v} for FogParameter"),
        }
    }
}

#[repr(u32)]
pub enum FramebufferAttachment {
    ColorAttachment0 = consts::GL_COLOR_ATTACHMENT0,
    ColorAttachment1 = consts::GL_COLOR_ATTACHMENT1,
    ColorAttachment10 = consts::GL_COLOR_ATTACHMENT10,
    ColorAttachment11 = consts::GL_COLOR_ATTACHMENT11,
    ColorAttachment12 = consts::GL_COLOR_ATTACHMENT12,
    ColorAttachment13 = consts::GL_COLOR_ATTACHMENT13,
    ColorAttachment14 = consts::GL_COLOR_ATTACHMENT14,
    ColorAttachment15 = consts::GL_COLOR_ATTACHMENT15,
    ColorAttachment16 = consts::GL_COLOR_ATTACHMENT16,
    ColorAttachment17 = consts::GL_COLOR_ATTACHMENT17,
    ColorAttachment18 = consts::GL_COLOR_ATTACHMENT18,
    ColorAttachment19 = consts::GL_COLOR_ATTACHMENT19,
    ColorAttachment2 = consts::GL_COLOR_ATTACHMENT2,
    ColorAttachment20 = consts::GL_COLOR_ATTACHMENT20,
    ColorAttachment21 = consts::GL_COLOR_ATTACHMENT21,
    ColorAttachment22 = consts::GL_COLOR_ATTACHMENT22,
    ColorAttachment23 = consts::GL_COLOR_ATTACHMENT23,
    ColorAttachment24 = consts::GL_COLOR_ATTACHMENT24,
    ColorAttachment25 = consts::GL_COLOR_ATTACHMENT25,
    ColorAttachment26 = consts::GL_COLOR_ATTACHMENT26,
    ColorAttachment27 = consts::GL_COLOR_ATTACHMENT27,
    ColorAttachment28 = consts::GL_COLOR_ATTACHMENT28,
    ColorAttachment29 = consts::GL_COLOR_ATTACHMENT29,
    ColorAttachment3 = consts::GL_COLOR_ATTACHMENT3,
    ColorAttachment30 = consts::GL_COLOR_ATTACHMENT30,
    ColorAttachment31 = consts::GL_COLOR_ATTACHMENT31,
    ColorAttachment4 = consts::GL_COLOR_ATTACHMENT4,
    ColorAttachment5 = consts::GL_COLOR_ATTACHMENT5,
    ColorAttachment6 = consts::GL_COLOR_ATTACHMENT6,
    ColorAttachment7 = consts::GL_COLOR_ATTACHMENT7,
    ColorAttachment8 = consts::GL_COLOR_ATTACHMENT8,
    ColorAttachment9 = consts::GL_COLOR_ATTACHMENT9,
    DepthAttachment = consts::GL_DEPTH_ATTACHMENT,
    DepthStencilAttachment = consts::GL_DEPTH_STENCIL_ATTACHMENT,
    StencilAttachment = consts::GL_STENCIL_ATTACHMENT,
}

impl From<u32> for FramebufferAttachment {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_COLOR_ATTACHMENT0 => Self::ColorAttachment0,
            consts::GL_COLOR_ATTACHMENT1 => Self::ColorAttachment1,
            consts::GL_COLOR_ATTACHMENT10 => Self::ColorAttachment10,
            consts::GL_COLOR_ATTACHMENT11 => Self::ColorAttachment11,
            consts::GL_COLOR_ATTACHMENT12 => Self::ColorAttachment12,
            consts::GL_COLOR_ATTACHMENT13 => Self::ColorAttachment13,
            consts::GL_COLOR_ATTACHMENT14 => Self::ColorAttachment14,
            consts::GL_COLOR_ATTACHMENT15 => Self::ColorAttachment15,
            consts::GL_COLOR_ATTACHMENT16 => Self::ColorAttachment16,
            consts::GL_COLOR_ATTACHMENT17 => Self::ColorAttachment17,
            consts::GL_COLOR_ATTACHMENT18 => Self::ColorAttachment18,
            consts::GL_COLOR_ATTACHMENT19 => Self::ColorAttachment19,
            consts::GL_COLOR_ATTACHMENT2 => Self::ColorAttachment2,
            consts::GL_COLOR_ATTACHMENT20 => Self::ColorAttachment20,
            consts::GL_COLOR_ATTACHMENT21 => Self::ColorAttachment21,
            consts::GL_COLOR_ATTACHMENT22 => Self::ColorAttachment22,
            consts::GL_COLOR_ATTACHMENT23 => Self::ColorAttachment23,
            consts::GL_COLOR_ATTACHMENT24 => Self::ColorAttachment24,
            consts::GL_COLOR_ATTACHMENT25 => Self::ColorAttachment25,
            consts::GL_COLOR_ATTACHMENT26 => Self::ColorAttachment26,
            consts::GL_COLOR_ATTACHMENT27 => Self::ColorAttachment27,
            consts::GL_COLOR_ATTACHMENT28 => Self::ColorAttachment28,
            consts::GL_COLOR_ATTACHMENT29 => Self::ColorAttachment29,
            consts::GL_COLOR_ATTACHMENT3 => Self::ColorAttachment3,
            consts::GL_COLOR_ATTACHMENT30 => Self::ColorAttachment30,
            consts::GL_COLOR_ATTACHMENT31 => Self::ColorAttachment31,
            consts::GL_COLOR_ATTACHMENT4 => Self::ColorAttachment4,
            consts::GL_COLOR_ATTACHMENT5 => Self::ColorAttachment5,
            consts::GL_COLOR_ATTACHMENT6 => Self::ColorAttachment6,
            consts::GL_COLOR_ATTACHMENT7 => Self::ColorAttachment7,
            consts::GL_COLOR_ATTACHMENT8 => Self::ColorAttachment8,
            consts::GL_COLOR_ATTACHMENT9 => Self::ColorAttachment9,
            consts::GL_DEPTH_ATTACHMENT => Self::DepthAttachment,
            consts::GL_DEPTH_STENCIL_ATTACHMENT => Self::DepthStencilAttachment,
            consts::GL_STENCIL_ATTACHMENT => Self::StencilAttachment,
            v => panic!("Unknow value {v} for FramebufferAttachment"),
        }
    }
}

#[repr(u32)]
pub enum FramebufferAttachmentParameterName {
    FramebufferAttachmentAlphaSize = consts::GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE,
    FramebufferAttachmentBlueSize = consts::GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE,
    FramebufferAttachmentColorEncoding = consts::GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING,
    FramebufferAttachmentComponentType = consts::GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE,
    FramebufferAttachmentDepthSize = consts::GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE,
    FramebufferAttachmentGreenSize = consts::GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE,
    FramebufferAttachmentLayered = consts::GL_FRAMEBUFFER_ATTACHMENT_LAYERED,
    FramebufferAttachmentObjectName = consts::GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME,
    FramebufferAttachmentObjectType = consts::GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE,
    FramebufferAttachmentRedSize = consts::GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE,
    FramebufferAttachmentStencilSize = consts::GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE,
    FramebufferAttachmentTextureCubeMapFace =
        consts::GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE,
    FramebufferAttachmentTextureLayer = consts::GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER,
    FramebufferAttachmentTextureLevel = consts::GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL,
}

impl From<u32> for FramebufferAttachmentParameterName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE => Self::FramebufferAttachmentAlphaSize,
            consts::GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE => Self::FramebufferAttachmentBlueSize,
            consts::GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING => {
                Self::FramebufferAttachmentColorEncoding
            }
            consts::GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE => {
                Self::FramebufferAttachmentComponentType
            }
            consts::GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE => Self::FramebufferAttachmentDepthSize,
            consts::GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE => Self::FramebufferAttachmentGreenSize,
            consts::GL_FRAMEBUFFER_ATTACHMENT_LAYERED => Self::FramebufferAttachmentLayered,
            consts::GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME => Self::FramebufferAttachmentObjectName,
            consts::GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE => Self::FramebufferAttachmentObjectType,
            consts::GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE => Self::FramebufferAttachmentRedSize,
            consts::GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE => {
                Self::FramebufferAttachmentStencilSize
            }
            consts::GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE => {
                Self::FramebufferAttachmentTextureCubeMapFace
            }
            consts::GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER => {
                Self::FramebufferAttachmentTextureLayer
            }
            consts::GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL => {
                Self::FramebufferAttachmentTextureLevel
            }
            v => panic!("Unknow value {v} for FramebufferAttachmentParameterName"),
        }
    }
}

#[repr(u32)]
pub enum FramebufferParameterName {
    FramebufferDefaultFixedSampleLocations = consts::GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS,
    FramebufferDefaultHeight = consts::GL_FRAMEBUFFER_DEFAULT_HEIGHT,
    FramebufferDefaultLayers = consts::GL_FRAMEBUFFER_DEFAULT_LAYERS,
    FramebufferDefaultSamples = consts::GL_FRAMEBUFFER_DEFAULT_SAMPLES,
    FramebufferDefaultWidth = consts::GL_FRAMEBUFFER_DEFAULT_WIDTH,
}

impl From<u32> for FramebufferParameterName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS => {
                Self::FramebufferDefaultFixedSampleLocations
            }
            consts::GL_FRAMEBUFFER_DEFAULT_HEIGHT => Self::FramebufferDefaultHeight,
            consts::GL_FRAMEBUFFER_DEFAULT_LAYERS => Self::FramebufferDefaultLayers,
            consts::GL_FRAMEBUFFER_DEFAULT_SAMPLES => Self::FramebufferDefaultSamples,
            consts::GL_FRAMEBUFFER_DEFAULT_WIDTH => Self::FramebufferDefaultWidth,
            v => panic!("Unknow value {v} for FramebufferParameterName"),
        }
    }
}

#[repr(u32)]
pub enum FramebufferStatus {
    FramebufferComplete = consts::GL_FRAMEBUFFER_COMPLETE,
    FramebufferIncompleteAttachment = consts::GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT,
    FramebufferIncompleteDrawBuffer = consts::GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER,
    FramebufferIncompleteLayerTargets = consts::GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS,
    FramebufferIncompleteMissingAttachment = consts::GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,
    FramebufferIncompleteMultisample = consts::GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,
    FramebufferIncompleteReadBuffer = consts::GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER,
    FramebufferUndefined = consts::GL_FRAMEBUFFER_UNDEFINED,
    FramebufferUnsupported = consts::GL_FRAMEBUFFER_UNSUPPORTED,
}

impl From<u32> for FramebufferStatus {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_FRAMEBUFFER_COMPLETE => Self::FramebufferComplete,
            consts::GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT => Self::FramebufferIncompleteAttachment,
            consts::GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => Self::FramebufferIncompleteDrawBuffer,
            consts::GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => {
                Self::FramebufferIncompleteLayerTargets
            }
            consts::GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                Self::FramebufferIncompleteMissingAttachment
            }
            consts::GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => Self::FramebufferIncompleteMultisample,
            consts::GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER => Self::FramebufferIncompleteReadBuffer,
            consts::GL_FRAMEBUFFER_UNDEFINED => Self::FramebufferUndefined,
            consts::GL_FRAMEBUFFER_UNSUPPORTED => Self::FramebufferUnsupported,
            v => panic!("Unknow value {v} for FramebufferStatus"),
        }
    }
}

#[repr(u32)]
pub enum FramebufferTarget {
    DrawFramebuffer = consts::GL_DRAW_FRAMEBUFFER,
    Framebuffer = consts::GL_FRAMEBUFFER,
    ReadFramebuffer = consts::GL_READ_FRAMEBUFFER,
}

impl From<u32> for FramebufferTarget {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_DRAW_FRAMEBUFFER => Self::DrawFramebuffer,
            consts::GL_FRAMEBUFFER => Self::Framebuffer,
            consts::GL_READ_FRAMEBUFFER => Self::ReadFramebuffer,
            v => panic!("Unknow value {v} for FramebufferTarget"),
        }
    }
}

#[repr(u32)]
pub enum FrontFaceDirection {
    Ccw = consts::GL_CCW,
    Cw = consts::GL_CW,
}

impl From<u32> for FrontFaceDirection {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_CCW => Self::Ccw,
            consts::GL_CW => Self::Cw,
            v => panic!("Unknow value {v} for FrontFaceDirection"),
        }
    }
}

#[repr(u32)]
pub enum GetMultisamplePNameNV {
    SamplePosition = consts::GL_SAMPLE_POSITION,
}

impl From<u32> for GetMultisamplePNameNV {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_SAMPLE_POSITION => Self::SamplePosition,
            v => panic!("Unknow value {v} for GetMultisamplePNameNV"),
        }
    }
}

#[repr(u32)]
pub enum GetPName {
    ActiveTexture = consts::GL_ACTIVE_TEXTURE,
    AliasedLineWidthRange = consts::GL_ALIASED_LINE_WIDTH_RANGE,
    AliasedPointSizeRange = consts::GL_ALIASED_POINT_SIZE_RANGE,
    AlphaBits = consts::GL_ALPHA_BITS,
    AlphaScale = consts::GL_ALPHA_SCALE,
    AlphaTest = consts::GL_ALPHA_TEST,
    AlphaTestFunc = consts::GL_ALPHA_TEST_FUNC,
    AlphaTestRef = consts::GL_ALPHA_TEST_REF,
    ArrayBufferBinding = consts::GL_ARRAY_BUFFER_BINDING,
    Blend = consts::GL_BLEND,
    BlendColor = consts::GL_BLEND_COLOR,
    BlendDst = consts::GL_BLEND_DST,
    BlendDstAlpha = consts::GL_BLEND_DST_ALPHA,
    BlendDstRgb = consts::GL_BLEND_DST_RGB,
    BlendEquation = consts::GL_BLEND_EQUATION,
    BlendEquationAlpha = consts::GL_BLEND_EQUATION_ALPHA,
    BlendSrc = consts::GL_BLEND_SRC,
    BlendSrcAlpha = consts::GL_BLEND_SRC_ALPHA,
    BlendSrcRgb = consts::GL_BLEND_SRC_RGB,
    BlueBits = consts::GL_BLUE_BITS,
    ClipPlane0 = consts::GL_CLIP_PLANE0,
    ClipPlane1 = consts::GL_CLIP_PLANE1,
    ClipPlane2 = consts::GL_CLIP_PLANE2,
    ClipPlane3 = consts::GL_CLIP_PLANE3,
    ClipPlane4 = consts::GL_CLIP_PLANE4,
    ClipPlane5 = consts::GL_CLIP_PLANE5,
    ColorArray = consts::GL_COLOR_ARRAY,
    ColorArraySize = consts::GL_COLOR_ARRAY_SIZE,
    ColorArrayStride = consts::GL_COLOR_ARRAY_STRIDE,
    ColorArrayType = consts::GL_COLOR_ARRAY_TYPE,
    ColorClearValue = consts::GL_COLOR_CLEAR_VALUE,
    ColorLogicOp = consts::GL_COLOR_LOGIC_OP,
    ColorMaterial = consts::GL_COLOR_MATERIAL,
    ColorWritemask = consts::GL_COLOR_WRITEMASK,
    CompressedTextureFormats = consts::GL_COMPRESSED_TEXTURE_FORMATS,
    ContextFlags = consts::GL_CONTEXT_FLAGS,
    ContextProfileMask = consts::GL_CONTEXT_PROFILE_MASK,
    CullFace = consts::GL_CULL_FACE,
    CullFaceMode = consts::GL_CULL_FACE_MODE,
    CurrentColor = consts::GL_CURRENT_COLOR,
    CurrentNormal = consts::GL_CURRENT_NORMAL,
    CurrentProgram = consts::GL_CURRENT_PROGRAM,
    CurrentTextureCoords = consts::GL_CURRENT_TEXTURE_COORDS,
    DebugGroupStackDepth = consts::GL_DEBUG_GROUP_STACK_DEPTH,
    DepthBits = consts::GL_DEPTH_BITS,
    DepthClearValue = consts::GL_DEPTH_CLEAR_VALUE,
    DepthFunc = consts::GL_DEPTH_FUNC,
    DepthRange = consts::GL_DEPTH_RANGE,
    DepthTest = consts::GL_DEPTH_TEST,
    DepthWritemask = consts::GL_DEPTH_WRITEMASK,
    DispatchIndirectBufferBinding = consts::GL_DISPATCH_INDIRECT_BUFFER_BINDING,
    Dither = consts::GL_DITHER,
    Doublebuffer = consts::GL_DOUBLEBUFFER,
    DrawBuffer = consts::GL_DRAW_BUFFER,
    DrawFramebufferBinding = consts::GL_DRAW_FRAMEBUFFER_BINDING,
    ElementArrayBufferBinding = consts::GL_ELEMENT_ARRAY_BUFFER_BINDING,
    Fog = consts::GL_FOG,
    FogColor = consts::GL_FOG_COLOR,
    FogDensity = consts::GL_FOG_DENSITY,
    FogEnd = consts::GL_FOG_END,
    FogHint = consts::GL_FOG_HINT,
    FogMode = consts::GL_FOG_MODE,
    FogStart = consts::GL_FOG_START,
    FragmentShaderDerivativeHint = consts::GL_FRAGMENT_SHADER_DERIVATIVE_HINT,
    FrontFace = consts::GL_FRONT_FACE,
    GreenBits = consts::GL_GREEN_BITS,
    ImplementationColorReadFormat = consts::GL_IMPLEMENTATION_COLOR_READ_FORMAT,
    ImplementationColorReadType = consts::GL_IMPLEMENTATION_COLOR_READ_TYPE,
    LayerProvokingVertex = consts::GL_LAYER_PROVOKING_VERTEX,
    Light0 = consts::GL_LIGHT0,
    Light1 = consts::GL_LIGHT1,
    Light2 = consts::GL_LIGHT2,
    Light3 = consts::GL_LIGHT3,
    Light4 = consts::GL_LIGHT4,
    Light5 = consts::GL_LIGHT5,
    Light6 = consts::GL_LIGHT6,
    Light7 = consts::GL_LIGHT7,
    Lighting = consts::GL_LIGHTING,
    LightModelAmbient = consts::GL_LIGHT_MODEL_AMBIENT,
    LightModelTwoSide = consts::GL_LIGHT_MODEL_TWO_SIDE,
    LineSmooth = consts::GL_LINE_SMOOTH,
    LineSmoothHint = consts::GL_LINE_SMOOTH_HINT,
    LineWidth = consts::GL_LINE_WIDTH,
    LineWidthGranularity = consts::GL_LINE_WIDTH_GRANULARITY,
    LineWidthRange = consts::GL_LINE_WIDTH_RANGE,
    LogicOpMode = consts::GL_LOGIC_OP_MODE,
    MajorVersion = consts::GL_MAJOR_VERSION,
    MatrixMode = consts::GL_MATRIX_MODE,
    Max3dTextureSize = consts::GL_MAX_3D_TEXTURE_SIZE,
    MaxArrayTextureLayers = consts::GL_MAX_ARRAY_TEXTURE_LAYERS,
    MaxClipPlanes = consts::GL_MAX_CLIP_PLANES,
    MaxColorAttachments = consts::GL_MAX_COLOR_ATTACHMENTS,
    MaxColorTextureSamples = consts::GL_MAX_COLOR_TEXTURE_SAMPLES,
    MaxCombinedAtomicCounters = consts::GL_MAX_COMBINED_ATOMIC_COUNTERS,
    MaxCombinedComputeUniformComponents = consts::GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS,
    MaxCombinedFragmentUniformComponents = consts::GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS,
    MaxCombinedGeometryUniformComponents = consts::GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS,
    MaxCombinedShaderStorageBlocks = consts::GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS,
    MaxCombinedTextureImageUnits = consts::GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS,
    MaxCombinedUniformBlocks = consts::GL_MAX_COMBINED_UNIFORM_BLOCKS,
    MaxCombinedVertexUniformComponents = consts::GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS,
    MaxComputeAtomicCounters = consts::GL_MAX_COMPUTE_ATOMIC_COUNTERS,
    MaxComputeAtomicCounterBuffers = consts::GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS,
    MaxComputeShaderStorageBlocks = consts::GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS,
    MaxComputeTextureImageUnits = consts::GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS,
    MaxComputeUniformBlocks = consts::GL_MAX_COMPUTE_UNIFORM_BLOCKS,
    MaxComputeUniformComponents = consts::GL_MAX_COMPUTE_UNIFORM_COMPONENTS,
    MaxComputeWorkGroupCount = consts::GL_MAX_COMPUTE_WORK_GROUP_COUNT,
    MaxComputeWorkGroupInvocations = consts::GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS,
    MaxComputeWorkGroupSize = consts::GL_MAX_COMPUTE_WORK_GROUP_SIZE,
    MaxCubeMapTextureSize = consts::GL_MAX_CUBE_MAP_TEXTURE_SIZE,
    MaxDebugGroupStackDepth = consts::GL_MAX_DEBUG_GROUP_STACK_DEPTH,
    MaxDepthTextureSamples = consts::GL_MAX_DEPTH_TEXTURE_SAMPLES,
    MaxDrawBuffers = consts::GL_MAX_DRAW_BUFFERS,
    MaxDualSourceDrawBuffers = consts::GL_MAX_DUAL_SOURCE_DRAW_BUFFERS,
    MaxElementsIndices = consts::GL_MAX_ELEMENTS_INDICES,
    MaxElementsVertices = consts::GL_MAX_ELEMENTS_VERTICES,
    MaxElementIndex = consts::GL_MAX_ELEMENT_INDEX,
    MaxFragmentAtomicCounters = consts::GL_MAX_FRAGMENT_ATOMIC_COUNTERS,
    MaxFragmentInputComponents = consts::GL_MAX_FRAGMENT_INPUT_COMPONENTS,
    MaxFragmentShaderStorageBlocks = consts::GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS,
    MaxFragmentUniformBlocks = consts::GL_MAX_FRAGMENT_UNIFORM_BLOCKS,
    MaxFragmentUniformComponents = consts::GL_MAX_FRAGMENT_UNIFORM_COMPONENTS,
    MaxFragmentUniformVectors = consts::GL_MAX_FRAGMENT_UNIFORM_VECTORS,
    MaxFramebufferHeight = consts::GL_MAX_FRAMEBUFFER_HEIGHT,
    MaxFramebufferLayers = consts::GL_MAX_FRAMEBUFFER_LAYERS,
    MaxFramebufferSamples = consts::GL_MAX_FRAMEBUFFER_SAMPLES,
    MaxFramebufferWidth = consts::GL_MAX_FRAMEBUFFER_WIDTH,
    MaxGeometryAtomicCounters = consts::GL_MAX_GEOMETRY_ATOMIC_COUNTERS,
    MaxGeometryInputComponents = consts::GL_MAX_GEOMETRY_INPUT_COMPONENTS,
    MaxGeometryOutputComponents = consts::GL_MAX_GEOMETRY_OUTPUT_COMPONENTS,
    MaxGeometryShaderStorageBlocks = consts::GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS,
    MaxGeometryTextureImageUnits = consts::GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS,
    MaxGeometryUniformBlocks = consts::GL_MAX_GEOMETRY_UNIFORM_BLOCKS,
    MaxGeometryUniformComponents = consts::GL_MAX_GEOMETRY_UNIFORM_COMPONENTS,
    MaxIntegerSamples = consts::GL_MAX_INTEGER_SAMPLES,
    MaxLabelLength = consts::GL_MAX_LABEL_LENGTH,
    MaxLights = consts::GL_MAX_LIGHTS,
    MaxModelviewStackDepth = consts::GL_MAX_MODELVIEW_STACK_DEPTH,
    MaxProgramTexelOffset = consts::GL_MAX_PROGRAM_TEXEL_OFFSET,
    MaxProjectionStackDepth = consts::GL_MAX_PROJECTION_STACK_DEPTH,
    MaxRectangleTextureSize = consts::GL_MAX_RECTANGLE_TEXTURE_SIZE,
    MaxRenderbufferSize = consts::GL_MAX_RENDERBUFFER_SIZE,
    MaxSampleMaskWords = consts::GL_MAX_SAMPLE_MASK_WORDS,
    MaxServerWaitTimeout = consts::GL_MAX_SERVER_WAIT_TIMEOUT,
    MaxShaderStorageBufferBindings = consts::GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS,
    MaxTessControlAtomicCounters = consts::GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS,
    MaxTessControlShaderStorageBlocks = consts::GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS,
    MaxTessControlUniformBlocks = consts::GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS,
    MaxTessEvaluationAtomicCounters = consts::GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS,
    MaxTessEvaluationShaderStorageBlocks = consts::GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS,
    MaxTessEvaluationUniformBlocks = consts::GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS,
    MaxTextureBufferSize = consts::GL_MAX_TEXTURE_BUFFER_SIZE,
    MaxTextureImageUnits = consts::GL_MAX_TEXTURE_IMAGE_UNITS,
    MaxTextureLodBias = consts::GL_MAX_TEXTURE_LOD_BIAS,
    MaxTextureSize = consts::GL_MAX_TEXTURE_SIZE,
    MaxTextureStackDepth = consts::GL_MAX_TEXTURE_STACK_DEPTH,
    MaxUniformBlockSize = consts::GL_MAX_UNIFORM_BLOCK_SIZE,
    MaxUniformBufferBindings = consts::GL_MAX_UNIFORM_BUFFER_BINDINGS,
    MaxUniformLocations = consts::GL_MAX_UNIFORM_LOCATIONS,
    MaxVaryingFloats = consts::GL_MAX_VARYING_FLOATS,
    MaxVaryingVectors = consts::GL_MAX_VARYING_VECTORS,
    MaxVertexAtomicCounters = consts::GL_MAX_VERTEX_ATOMIC_COUNTERS,
    MaxVertexAttribs = consts::GL_MAX_VERTEX_ATTRIBS,
    MaxVertexAttribBindings = consts::GL_MAX_VERTEX_ATTRIB_BINDINGS,
    MaxVertexAttribRelativeOffset = consts::GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET,
    MaxVertexOutputComponents = consts::GL_MAX_VERTEX_OUTPUT_COMPONENTS,
    MaxVertexShaderStorageBlocks = consts::GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS,
    MaxVertexTextureImageUnits = consts::GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS,
    MaxVertexUniformBlocks = consts::GL_MAX_VERTEX_UNIFORM_BLOCKS,
    MaxVertexUniformComponents = consts::GL_MAX_VERTEX_UNIFORM_COMPONENTS,
    MaxVertexUniformVectors = consts::GL_MAX_VERTEX_UNIFORM_VECTORS,
    MaxViewportDims = consts::GL_MAX_VIEWPORT_DIMS,
    MinorVersion = consts::GL_MINOR_VERSION,
    MinProgramTexelOffset = consts::GL_MIN_PROGRAM_TEXEL_OFFSET,
    ModelviewMatrix = consts::GL_MODELVIEW_MATRIX,
    ModelviewStackDepth = consts::GL_MODELVIEW_STACK_DEPTH,
    Normalize = consts::GL_NORMALIZE,
    NormalArray = consts::GL_NORMAL_ARRAY,
    NormalArrayStride = consts::GL_NORMAL_ARRAY_STRIDE,
    NormalArrayType = consts::GL_NORMAL_ARRAY_TYPE,
    NumCompressedTextureFormats = consts::GL_NUM_COMPRESSED_TEXTURE_FORMATS,
    NumExtensions = consts::GL_NUM_EXTENSIONS,
    NumProgramBinaryFormats = consts::GL_NUM_PROGRAM_BINARY_FORMATS,
    NumShaderBinaryFormats = consts::GL_NUM_SHADER_BINARY_FORMATS,
    PackAlignment = consts::GL_PACK_ALIGNMENT,
    PackImageHeight = consts::GL_PACK_IMAGE_HEIGHT,
    PackLsbFirst = consts::GL_PACK_LSB_FIRST,
    PackRowLength = consts::GL_PACK_ROW_LENGTH,
    PackSkipImages = consts::GL_PACK_SKIP_IMAGES,
    PackSkipPixels = consts::GL_PACK_SKIP_PIXELS,
    PackSkipRows = consts::GL_PACK_SKIP_ROWS,
    PackSwapBytes = consts::GL_PACK_SWAP_BYTES,
    PerspectiveCorrectionHint = consts::GL_PERSPECTIVE_CORRECTION_HINT,
    PixelPackBufferBinding = consts::GL_PIXEL_PACK_BUFFER_BINDING,
    PixelUnpackBufferBinding = consts::GL_PIXEL_UNPACK_BUFFER_BINDING,
    PointDistanceAttenuation = consts::GL_POINT_DISTANCE_ATTENUATION,
    PointFadeThresholdSize = consts::GL_POINT_FADE_THRESHOLD_SIZE,
    PointSize = consts::GL_POINT_SIZE,
    PointSizeGranularity = consts::GL_POINT_SIZE_GRANULARITY,
    PointSizeMax = consts::GL_POINT_SIZE_MAX,
    PointSizeMin = consts::GL_POINT_SIZE_MIN,
    PointSizeRange = consts::GL_POINT_SIZE_RANGE,
    PointSmooth = consts::GL_POINT_SMOOTH,
    PointSmoothHint = consts::GL_POINT_SMOOTH_HINT,
    PolygonMode = consts::GL_POLYGON_MODE,
    PolygonOffsetFactor = consts::GL_POLYGON_OFFSET_FACTOR,
    PolygonOffsetFill = consts::GL_POLYGON_OFFSET_FILL,
    PolygonOffsetLine = consts::GL_POLYGON_OFFSET_LINE,
    PolygonOffsetPoint = consts::GL_POLYGON_OFFSET_POINT,
    PolygonOffsetUnits = consts::GL_POLYGON_OFFSET_UNITS,
    PolygonSmooth = consts::GL_POLYGON_SMOOTH,
    PolygonSmoothHint = consts::GL_POLYGON_SMOOTH_HINT,
    PrimitiveRestartIndex = consts::GL_PRIMITIVE_RESTART_INDEX,
    ProgramBinaryFormats = consts::GL_PROGRAM_BINARY_FORMATS,
    ProgramPipelineBinding = consts::GL_PROGRAM_PIPELINE_BINDING,
    ProgramPointSize = consts::GL_PROGRAM_POINT_SIZE,
    ProjectionMatrix = consts::GL_PROJECTION_MATRIX,
    ProjectionStackDepth = consts::GL_PROJECTION_STACK_DEPTH,
    ProvokingVertex = consts::GL_PROVOKING_VERTEX,
    ReadBuffer = consts::GL_READ_BUFFER,
    ReadFramebufferBinding = consts::GL_READ_FRAMEBUFFER_BINDING,
    RedBits = consts::GL_RED_BITS,
    RenderbufferBinding = consts::GL_RENDERBUFFER_BINDING,
    SamplerBinding = consts::GL_SAMPLER_BINDING,
    Samples = consts::GL_SAMPLES,
    SampleBuffers = consts::GL_SAMPLE_BUFFERS,
    SampleCoverageInvert = consts::GL_SAMPLE_COVERAGE_INVERT,
    SampleCoverageValue = consts::GL_SAMPLE_COVERAGE_VALUE,
    ScissorBox = consts::GL_SCISSOR_BOX,
    ScissorTest = consts::GL_SCISSOR_TEST,
    ShaderBinaryFormats = consts::GL_SHADER_BINARY_FORMATS,
    ShaderCompiler = consts::GL_SHADER_COMPILER,
    ShaderStorageBufferBinding = consts::GL_SHADER_STORAGE_BUFFER_BINDING,
    ShaderStorageBufferOffsetAlignment = consts::GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT,
    ShaderStorageBufferSize = consts::GL_SHADER_STORAGE_BUFFER_SIZE,
    ShaderStorageBufferStart = consts::GL_SHADER_STORAGE_BUFFER_START,
    ShadeModel = consts::GL_SHADE_MODEL,
    StencilBackFail = consts::GL_STENCIL_BACK_FAIL,
    StencilBackFunc = consts::GL_STENCIL_BACK_FUNC,
    StencilBackPassDepthFail = consts::GL_STENCIL_BACK_PASS_DEPTH_FAIL,
    StencilBackPassDepthPass = consts::GL_STENCIL_BACK_PASS_DEPTH_PASS,
    StencilBackRef = consts::GL_STENCIL_BACK_REF,
    StencilBackValueMask = consts::GL_STENCIL_BACK_VALUE_MASK,
    StencilBackWritemask = consts::GL_STENCIL_BACK_WRITEMASK,
    StencilBits = consts::GL_STENCIL_BITS,
    StencilClearValue = consts::GL_STENCIL_CLEAR_VALUE,
    StencilFail = consts::GL_STENCIL_FAIL,
    StencilFunc = consts::GL_STENCIL_FUNC,
    StencilPassDepthFail = consts::GL_STENCIL_PASS_DEPTH_FAIL,
    StencilPassDepthPass = consts::GL_STENCIL_PASS_DEPTH_PASS,
    StencilRef = consts::GL_STENCIL_REF,
    StencilTest = consts::GL_STENCIL_TEST,
    StencilValueMask = consts::GL_STENCIL_VALUE_MASK,
    StencilWritemask = consts::GL_STENCIL_WRITEMASK,
    Stereo = consts::GL_STEREO,
    SubpixelBits = consts::GL_SUBPIXEL_BITS,
    Texture1d = consts::GL_TEXTURE_1D,
    Texture2d = consts::GL_TEXTURE_2D,
    TextureBinding1d = consts::GL_TEXTURE_BINDING_1D,
    TextureBinding1dArray = consts::GL_TEXTURE_BINDING_1D_ARRAY,
    TextureBinding2d = consts::GL_TEXTURE_BINDING_2D,
    TextureBinding2dArray = consts::GL_TEXTURE_BINDING_2D_ARRAY,
    TextureBinding2dMultisample = consts::GL_TEXTURE_BINDING_2D_MULTISAMPLE,
    TextureBinding2dMultisampleArray = consts::GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY,
    TextureBinding3d = consts::GL_TEXTURE_BINDING_3D,
    TextureBindingBuffer = consts::GL_TEXTURE_BINDING_BUFFER,
    TextureBindingCubeMap = consts::GL_TEXTURE_BINDING_CUBE_MAP,
    TextureBindingRectangle = consts::GL_TEXTURE_BINDING_RECTANGLE,
    TextureBufferOffsetAlignment = consts::GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT,
    TextureCompressionHint = consts::GL_TEXTURE_COMPRESSION_HINT,
    TextureCoordArray = consts::GL_TEXTURE_COORD_ARRAY,
    TextureCoordArraySize = consts::GL_TEXTURE_COORD_ARRAY_SIZE,
    TextureCoordArrayStride = consts::GL_TEXTURE_COORD_ARRAY_STRIDE,
    TextureCoordArrayType = consts::GL_TEXTURE_COORD_ARRAY_TYPE,
    TextureMatrix = consts::GL_TEXTURE_MATRIX,
    TextureStackDepth = consts::GL_TEXTURE_STACK_DEPTH,
    Timestamp = consts::GL_TIMESTAMP,
    TransformFeedbackBufferBinding = consts::GL_TRANSFORM_FEEDBACK_BUFFER_BINDING,
    TransformFeedbackBufferSize = consts::GL_TRANSFORM_FEEDBACK_BUFFER_SIZE,
    TransformFeedbackBufferStart = consts::GL_TRANSFORM_FEEDBACK_BUFFER_START,
    UniformBufferBinding = consts::GL_UNIFORM_BUFFER_BINDING,
    UniformBufferOffsetAlignment = consts::GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT,
    UniformBufferSize = consts::GL_UNIFORM_BUFFER_SIZE,
    UniformBufferStart = consts::GL_UNIFORM_BUFFER_START,
    UnpackAlignment = consts::GL_UNPACK_ALIGNMENT,
    UnpackImageHeight = consts::GL_UNPACK_IMAGE_HEIGHT,
    UnpackLsbFirst = consts::GL_UNPACK_LSB_FIRST,
    UnpackRowLength = consts::GL_UNPACK_ROW_LENGTH,
    UnpackSkipImages = consts::GL_UNPACK_SKIP_IMAGES,
    UnpackSkipPixels = consts::GL_UNPACK_SKIP_PIXELS,
    UnpackSkipRows = consts::GL_UNPACK_SKIP_ROWS,
    UnpackSwapBytes = consts::GL_UNPACK_SWAP_BYTES,
    VertexArray = consts::GL_VERTEX_ARRAY,
    VertexArrayBinding = consts::GL_VERTEX_ARRAY_BINDING,
    VertexArraySize = consts::GL_VERTEX_ARRAY_SIZE,
    VertexArrayStride = consts::GL_VERTEX_ARRAY_STRIDE,
    VertexArrayType = consts::GL_VERTEX_ARRAY_TYPE,
    VertexBindingDivisor = consts::GL_VERTEX_BINDING_DIVISOR,
    VertexBindingOffset = consts::GL_VERTEX_BINDING_OFFSET,
    VertexBindingStride = consts::GL_VERTEX_BINDING_STRIDE,
    Viewport = consts::GL_VIEWPORT,
}

impl GetPName {
    pub const BlendEquationRgb: Self = Self::BlendEquation;
    pub const MaxClipDistances: Self = Self::MaxClipPlanes;
    pub const MaxVaryingComponents: Self = Self::MaxVaryingFloats;
    pub const SmoothLineWidthGranularity: Self = Self::LineWidthGranularity;
    pub const SmoothLineWidthRange: Self = Self::LineWidthRange;
    pub const SmoothPointSizeGranularity: Self = Self::PointSizeGranularity;
    pub const SmoothPointSizeRange: Self = Self::PointSizeRange;
}

impl From<u32> for GetPName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ACTIVE_TEXTURE => Self::ActiveTexture,
            consts::GL_ALIASED_LINE_WIDTH_RANGE => Self::AliasedLineWidthRange,
            consts::GL_ALIASED_POINT_SIZE_RANGE => Self::AliasedPointSizeRange,
            consts::GL_ALPHA_BITS => Self::AlphaBits,
            consts::GL_ALPHA_SCALE => Self::AlphaScale,
            consts::GL_ALPHA_TEST => Self::AlphaTest,
            consts::GL_ALPHA_TEST_FUNC => Self::AlphaTestFunc,
            consts::GL_ALPHA_TEST_REF => Self::AlphaTestRef,
            consts::GL_ARRAY_BUFFER_BINDING => Self::ArrayBufferBinding,
            consts::GL_BLEND => Self::Blend,
            consts::GL_BLEND_COLOR => Self::BlendColor,
            consts::GL_BLEND_DST => Self::BlendDst,
            consts::GL_BLEND_DST_ALPHA => Self::BlendDstAlpha,
            consts::GL_BLEND_DST_RGB => Self::BlendDstRgb,
            consts::GL_BLEND_EQUATION => Self::BlendEquation,
            consts::GL_BLEND_EQUATION_ALPHA => Self::BlendEquationAlpha,
            consts::GL_BLEND_SRC => Self::BlendSrc,
            consts::GL_BLEND_SRC_ALPHA => Self::BlendSrcAlpha,
            consts::GL_BLEND_SRC_RGB => Self::BlendSrcRgb,
            consts::GL_BLUE_BITS => Self::BlueBits,
            consts::GL_CLIP_PLANE0 => Self::ClipPlane0,
            consts::GL_CLIP_PLANE1 => Self::ClipPlane1,
            consts::GL_CLIP_PLANE2 => Self::ClipPlane2,
            consts::GL_CLIP_PLANE3 => Self::ClipPlane3,
            consts::GL_CLIP_PLANE4 => Self::ClipPlane4,
            consts::GL_CLIP_PLANE5 => Self::ClipPlane5,
            consts::GL_COLOR_ARRAY => Self::ColorArray,
            consts::GL_COLOR_ARRAY_SIZE => Self::ColorArraySize,
            consts::GL_COLOR_ARRAY_STRIDE => Self::ColorArrayStride,
            consts::GL_COLOR_ARRAY_TYPE => Self::ColorArrayType,
            consts::GL_COLOR_CLEAR_VALUE => Self::ColorClearValue,
            consts::GL_COLOR_LOGIC_OP => Self::ColorLogicOp,
            consts::GL_COLOR_MATERIAL => Self::ColorMaterial,
            consts::GL_COLOR_WRITEMASK => Self::ColorWritemask,
            consts::GL_COMPRESSED_TEXTURE_FORMATS => Self::CompressedTextureFormats,
            consts::GL_CONTEXT_FLAGS => Self::ContextFlags,
            consts::GL_CONTEXT_PROFILE_MASK => Self::ContextProfileMask,
            consts::GL_CULL_FACE => Self::CullFace,
            consts::GL_CULL_FACE_MODE => Self::CullFaceMode,
            consts::GL_CURRENT_COLOR => Self::CurrentColor,
            consts::GL_CURRENT_NORMAL => Self::CurrentNormal,
            consts::GL_CURRENT_PROGRAM => Self::CurrentProgram,
            consts::GL_CURRENT_TEXTURE_COORDS => Self::CurrentTextureCoords,
            consts::GL_DEBUG_GROUP_STACK_DEPTH => Self::DebugGroupStackDepth,
            consts::GL_DEPTH_BITS => Self::DepthBits,
            consts::GL_DEPTH_CLEAR_VALUE => Self::DepthClearValue,
            consts::GL_DEPTH_FUNC => Self::DepthFunc,
            consts::GL_DEPTH_RANGE => Self::DepthRange,
            consts::GL_DEPTH_TEST => Self::DepthTest,
            consts::GL_DEPTH_WRITEMASK => Self::DepthWritemask,
            consts::GL_DISPATCH_INDIRECT_BUFFER_BINDING => Self::DispatchIndirectBufferBinding,
            consts::GL_DITHER => Self::Dither,
            consts::GL_DOUBLEBUFFER => Self::Doublebuffer,
            consts::GL_DRAW_BUFFER => Self::DrawBuffer,
            consts::GL_DRAW_FRAMEBUFFER_BINDING => Self::DrawFramebufferBinding,
            consts::GL_ELEMENT_ARRAY_BUFFER_BINDING => Self::ElementArrayBufferBinding,
            consts::GL_FOG => Self::Fog,
            consts::GL_FOG_COLOR => Self::FogColor,
            consts::GL_FOG_DENSITY => Self::FogDensity,
            consts::GL_FOG_END => Self::FogEnd,
            consts::GL_FOG_HINT => Self::FogHint,
            consts::GL_FOG_MODE => Self::FogMode,
            consts::GL_FOG_START => Self::FogStart,
            consts::GL_FRAGMENT_SHADER_DERIVATIVE_HINT => Self::FragmentShaderDerivativeHint,
            consts::GL_FRONT_FACE => Self::FrontFace,
            consts::GL_GREEN_BITS => Self::GreenBits,
            consts::GL_IMPLEMENTATION_COLOR_READ_FORMAT => Self::ImplementationColorReadFormat,
            consts::GL_IMPLEMENTATION_COLOR_READ_TYPE => Self::ImplementationColorReadType,
            consts::GL_LAYER_PROVOKING_VERTEX => Self::LayerProvokingVertex,
            consts::GL_LIGHT0 => Self::Light0,
            consts::GL_LIGHT1 => Self::Light1,
            consts::GL_LIGHT2 => Self::Light2,
            consts::GL_LIGHT3 => Self::Light3,
            consts::GL_LIGHT4 => Self::Light4,
            consts::GL_LIGHT5 => Self::Light5,
            consts::GL_LIGHT6 => Self::Light6,
            consts::GL_LIGHT7 => Self::Light7,
            consts::GL_LIGHTING => Self::Lighting,
            consts::GL_LIGHT_MODEL_AMBIENT => Self::LightModelAmbient,
            consts::GL_LIGHT_MODEL_TWO_SIDE => Self::LightModelTwoSide,
            consts::GL_LINE_SMOOTH => Self::LineSmooth,
            consts::GL_LINE_SMOOTH_HINT => Self::LineSmoothHint,
            consts::GL_LINE_WIDTH => Self::LineWidth,
            consts::GL_LINE_WIDTH_GRANULARITY => Self::LineWidthGranularity,
            consts::GL_LINE_WIDTH_RANGE => Self::LineWidthRange,
            consts::GL_LOGIC_OP_MODE => Self::LogicOpMode,
            consts::GL_MAJOR_VERSION => Self::MajorVersion,
            consts::GL_MATRIX_MODE => Self::MatrixMode,
            consts::GL_MAX_3D_TEXTURE_SIZE => Self::Max3dTextureSize,
            consts::GL_MAX_ARRAY_TEXTURE_LAYERS => Self::MaxArrayTextureLayers,
            consts::GL_MAX_CLIP_PLANES => Self::MaxClipPlanes,
            consts::GL_MAX_COLOR_ATTACHMENTS => Self::MaxColorAttachments,
            consts::GL_MAX_COLOR_TEXTURE_SAMPLES => Self::MaxColorTextureSamples,
            consts::GL_MAX_COMBINED_ATOMIC_COUNTERS => Self::MaxCombinedAtomicCounters,
            consts::GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS => {
                Self::MaxCombinedComputeUniformComponents
            }
            consts::GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS => {
                Self::MaxCombinedFragmentUniformComponents
            }
            consts::GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS => {
                Self::MaxCombinedGeometryUniformComponents
            }
            consts::GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS => Self::MaxCombinedShaderStorageBlocks,
            consts::GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS => Self::MaxCombinedTextureImageUnits,
            consts::GL_MAX_COMBINED_UNIFORM_BLOCKS => Self::MaxCombinedUniformBlocks,
            consts::GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS => {
                Self::MaxCombinedVertexUniformComponents
            }
            consts::GL_MAX_COMPUTE_ATOMIC_COUNTERS => Self::MaxComputeAtomicCounters,
            consts::GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS => Self::MaxComputeAtomicCounterBuffers,
            consts::GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS => Self::MaxComputeShaderStorageBlocks,
            consts::GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS => Self::MaxComputeTextureImageUnits,
            consts::GL_MAX_COMPUTE_UNIFORM_BLOCKS => Self::MaxComputeUniformBlocks,
            consts::GL_MAX_COMPUTE_UNIFORM_COMPONENTS => Self::MaxComputeUniformComponents,
            consts::GL_MAX_COMPUTE_WORK_GROUP_COUNT => Self::MaxComputeWorkGroupCount,
            consts::GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS => Self::MaxComputeWorkGroupInvocations,
            consts::GL_MAX_COMPUTE_WORK_GROUP_SIZE => Self::MaxComputeWorkGroupSize,
            consts::GL_MAX_CUBE_MAP_TEXTURE_SIZE => Self::MaxCubeMapTextureSize,
            consts::GL_MAX_DEBUG_GROUP_STACK_DEPTH => Self::MaxDebugGroupStackDepth,
            consts::GL_MAX_DEPTH_TEXTURE_SAMPLES => Self::MaxDepthTextureSamples,
            consts::GL_MAX_DRAW_BUFFERS => Self::MaxDrawBuffers,
            consts::GL_MAX_DUAL_SOURCE_DRAW_BUFFERS => Self::MaxDualSourceDrawBuffers,
            consts::GL_MAX_ELEMENTS_INDICES => Self::MaxElementsIndices,
            consts::GL_MAX_ELEMENTS_VERTICES => Self::MaxElementsVertices,
            consts::GL_MAX_ELEMENT_INDEX => Self::MaxElementIndex,
            consts::GL_MAX_FRAGMENT_ATOMIC_COUNTERS => Self::MaxFragmentAtomicCounters,
            consts::GL_MAX_FRAGMENT_INPUT_COMPONENTS => Self::MaxFragmentInputComponents,
            consts::GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS => Self::MaxFragmentShaderStorageBlocks,
            consts::GL_MAX_FRAGMENT_UNIFORM_BLOCKS => Self::MaxFragmentUniformBlocks,
            consts::GL_MAX_FRAGMENT_UNIFORM_COMPONENTS => Self::MaxFragmentUniformComponents,
            consts::GL_MAX_FRAGMENT_UNIFORM_VECTORS => Self::MaxFragmentUniformVectors,
            consts::GL_MAX_FRAMEBUFFER_HEIGHT => Self::MaxFramebufferHeight,
            consts::GL_MAX_FRAMEBUFFER_LAYERS => Self::MaxFramebufferLayers,
            consts::GL_MAX_FRAMEBUFFER_SAMPLES => Self::MaxFramebufferSamples,
            consts::GL_MAX_FRAMEBUFFER_WIDTH => Self::MaxFramebufferWidth,
            consts::GL_MAX_GEOMETRY_ATOMIC_COUNTERS => Self::MaxGeometryAtomicCounters,
            consts::GL_MAX_GEOMETRY_INPUT_COMPONENTS => Self::MaxGeometryInputComponents,
            consts::GL_MAX_GEOMETRY_OUTPUT_COMPONENTS => Self::MaxGeometryOutputComponents,
            consts::GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS => Self::MaxGeometryShaderStorageBlocks,
            consts::GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS => Self::MaxGeometryTextureImageUnits,
            consts::GL_MAX_GEOMETRY_UNIFORM_BLOCKS => Self::MaxGeometryUniformBlocks,
            consts::GL_MAX_GEOMETRY_UNIFORM_COMPONENTS => Self::MaxGeometryUniformComponents,
            consts::GL_MAX_INTEGER_SAMPLES => Self::MaxIntegerSamples,
            consts::GL_MAX_LABEL_LENGTH => Self::MaxLabelLength,
            consts::GL_MAX_LIGHTS => Self::MaxLights,
            consts::GL_MAX_MODELVIEW_STACK_DEPTH => Self::MaxModelviewStackDepth,
            consts::GL_MAX_PROGRAM_TEXEL_OFFSET => Self::MaxProgramTexelOffset,
            consts::GL_MAX_PROJECTION_STACK_DEPTH => Self::MaxProjectionStackDepth,
            consts::GL_MAX_RECTANGLE_TEXTURE_SIZE => Self::MaxRectangleTextureSize,
            consts::GL_MAX_RENDERBUFFER_SIZE => Self::MaxRenderbufferSize,
            consts::GL_MAX_SAMPLE_MASK_WORDS => Self::MaxSampleMaskWords,
            consts::GL_MAX_SERVER_WAIT_TIMEOUT => Self::MaxServerWaitTimeout,
            consts::GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS => Self::MaxShaderStorageBufferBindings,
            consts::GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS => Self::MaxTessControlAtomicCounters,
            consts::GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS => {
                Self::MaxTessControlShaderStorageBlocks
            }
            consts::GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS => Self::MaxTessControlUniformBlocks,
            consts::GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS => Self::MaxTessEvaluationAtomicCounters,
            consts::GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS => {
                Self::MaxTessEvaluationShaderStorageBlocks
            }
            consts::GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS => Self::MaxTessEvaluationUniformBlocks,
            consts::GL_MAX_TEXTURE_BUFFER_SIZE => Self::MaxTextureBufferSize,
            consts::GL_MAX_TEXTURE_IMAGE_UNITS => Self::MaxTextureImageUnits,
            consts::GL_MAX_TEXTURE_LOD_BIAS => Self::MaxTextureLodBias,
            consts::GL_MAX_TEXTURE_SIZE => Self::MaxTextureSize,
            consts::GL_MAX_TEXTURE_STACK_DEPTH => Self::MaxTextureStackDepth,
            consts::GL_MAX_UNIFORM_BLOCK_SIZE => Self::MaxUniformBlockSize,
            consts::GL_MAX_UNIFORM_BUFFER_BINDINGS => Self::MaxUniformBufferBindings,
            consts::GL_MAX_UNIFORM_LOCATIONS => Self::MaxUniformLocations,
            consts::GL_MAX_VARYING_FLOATS => Self::MaxVaryingFloats,
            consts::GL_MAX_VARYING_VECTORS => Self::MaxVaryingVectors,
            consts::GL_MAX_VERTEX_ATOMIC_COUNTERS => Self::MaxVertexAtomicCounters,
            consts::GL_MAX_VERTEX_ATTRIBS => Self::MaxVertexAttribs,
            consts::GL_MAX_VERTEX_ATTRIB_BINDINGS => Self::MaxVertexAttribBindings,
            consts::GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET => Self::MaxVertexAttribRelativeOffset,
            consts::GL_MAX_VERTEX_OUTPUT_COMPONENTS => Self::MaxVertexOutputComponents,
            consts::GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS => Self::MaxVertexShaderStorageBlocks,
            consts::GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS => Self::MaxVertexTextureImageUnits,
            consts::GL_MAX_VERTEX_UNIFORM_BLOCKS => Self::MaxVertexUniformBlocks,
            consts::GL_MAX_VERTEX_UNIFORM_COMPONENTS => Self::MaxVertexUniformComponents,
            consts::GL_MAX_VERTEX_UNIFORM_VECTORS => Self::MaxVertexUniformVectors,
            consts::GL_MAX_VIEWPORT_DIMS => Self::MaxViewportDims,
            consts::GL_MINOR_VERSION => Self::MinorVersion,
            consts::GL_MIN_PROGRAM_TEXEL_OFFSET => Self::MinProgramTexelOffset,
            consts::GL_MODELVIEW_MATRIX => Self::ModelviewMatrix,
            consts::GL_MODELVIEW_STACK_DEPTH => Self::ModelviewStackDepth,
            consts::GL_NORMALIZE => Self::Normalize,
            consts::GL_NORMAL_ARRAY => Self::NormalArray,
            consts::GL_NORMAL_ARRAY_STRIDE => Self::NormalArrayStride,
            consts::GL_NORMAL_ARRAY_TYPE => Self::NormalArrayType,
            consts::GL_NUM_COMPRESSED_TEXTURE_FORMATS => Self::NumCompressedTextureFormats,
            consts::GL_NUM_EXTENSIONS => Self::NumExtensions,
            consts::GL_NUM_PROGRAM_BINARY_FORMATS => Self::NumProgramBinaryFormats,
            consts::GL_NUM_SHADER_BINARY_FORMATS => Self::NumShaderBinaryFormats,
            consts::GL_PACK_ALIGNMENT => Self::PackAlignment,
            consts::GL_PACK_IMAGE_HEIGHT => Self::PackImageHeight,
            consts::GL_PACK_LSB_FIRST => Self::PackLsbFirst,
            consts::GL_PACK_ROW_LENGTH => Self::PackRowLength,
            consts::GL_PACK_SKIP_IMAGES => Self::PackSkipImages,
            consts::GL_PACK_SKIP_PIXELS => Self::PackSkipPixels,
            consts::GL_PACK_SKIP_ROWS => Self::PackSkipRows,
            consts::GL_PACK_SWAP_BYTES => Self::PackSwapBytes,
            consts::GL_PERSPECTIVE_CORRECTION_HINT => Self::PerspectiveCorrectionHint,
            consts::GL_PIXEL_PACK_BUFFER_BINDING => Self::PixelPackBufferBinding,
            consts::GL_PIXEL_UNPACK_BUFFER_BINDING => Self::PixelUnpackBufferBinding,
            consts::GL_POINT_DISTANCE_ATTENUATION => Self::PointDistanceAttenuation,
            consts::GL_POINT_FADE_THRESHOLD_SIZE => Self::PointFadeThresholdSize,
            consts::GL_POINT_SIZE => Self::PointSize,
            consts::GL_POINT_SIZE_GRANULARITY => Self::PointSizeGranularity,
            consts::GL_POINT_SIZE_MAX => Self::PointSizeMax,
            consts::GL_POINT_SIZE_MIN => Self::PointSizeMin,
            consts::GL_POINT_SIZE_RANGE => Self::PointSizeRange,
            consts::GL_POINT_SMOOTH => Self::PointSmooth,
            consts::GL_POINT_SMOOTH_HINT => Self::PointSmoothHint,
            consts::GL_POLYGON_MODE => Self::PolygonMode,
            consts::GL_POLYGON_OFFSET_FACTOR => Self::PolygonOffsetFactor,
            consts::GL_POLYGON_OFFSET_FILL => Self::PolygonOffsetFill,
            consts::GL_POLYGON_OFFSET_LINE => Self::PolygonOffsetLine,
            consts::GL_POLYGON_OFFSET_POINT => Self::PolygonOffsetPoint,
            consts::GL_POLYGON_OFFSET_UNITS => Self::PolygonOffsetUnits,
            consts::GL_POLYGON_SMOOTH => Self::PolygonSmooth,
            consts::GL_POLYGON_SMOOTH_HINT => Self::PolygonSmoothHint,
            consts::GL_PRIMITIVE_RESTART_INDEX => Self::PrimitiveRestartIndex,
            consts::GL_PROGRAM_BINARY_FORMATS => Self::ProgramBinaryFormats,
            consts::GL_PROGRAM_PIPELINE_BINDING => Self::ProgramPipelineBinding,
            consts::GL_PROGRAM_POINT_SIZE => Self::ProgramPointSize,
            consts::GL_PROJECTION_MATRIX => Self::ProjectionMatrix,
            consts::GL_PROJECTION_STACK_DEPTH => Self::ProjectionStackDepth,
            consts::GL_PROVOKING_VERTEX => Self::ProvokingVertex,
            consts::GL_READ_BUFFER => Self::ReadBuffer,
            consts::GL_READ_FRAMEBUFFER_BINDING => Self::ReadFramebufferBinding,
            consts::GL_RED_BITS => Self::RedBits,
            consts::GL_RENDERBUFFER_BINDING => Self::RenderbufferBinding,
            consts::GL_SAMPLER_BINDING => Self::SamplerBinding,
            consts::GL_SAMPLES => Self::Samples,
            consts::GL_SAMPLE_BUFFERS => Self::SampleBuffers,
            consts::GL_SAMPLE_COVERAGE_INVERT => Self::SampleCoverageInvert,
            consts::GL_SAMPLE_COVERAGE_VALUE => Self::SampleCoverageValue,
            consts::GL_SCISSOR_BOX => Self::ScissorBox,
            consts::GL_SCISSOR_TEST => Self::ScissorTest,
            consts::GL_SHADER_BINARY_FORMATS => Self::ShaderBinaryFormats,
            consts::GL_SHADER_COMPILER => Self::ShaderCompiler,
            consts::GL_SHADER_STORAGE_BUFFER_BINDING => Self::ShaderStorageBufferBinding,
            consts::GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT => {
                Self::ShaderStorageBufferOffsetAlignment
            }
            consts::GL_SHADER_STORAGE_BUFFER_SIZE => Self::ShaderStorageBufferSize,
            consts::GL_SHADER_STORAGE_BUFFER_START => Self::ShaderStorageBufferStart,
            consts::GL_SHADE_MODEL => Self::ShadeModel,
            consts::GL_STENCIL_BACK_FAIL => Self::StencilBackFail,
            consts::GL_STENCIL_BACK_FUNC => Self::StencilBackFunc,
            consts::GL_STENCIL_BACK_PASS_DEPTH_FAIL => Self::StencilBackPassDepthFail,
            consts::GL_STENCIL_BACK_PASS_DEPTH_PASS => Self::StencilBackPassDepthPass,
            consts::GL_STENCIL_BACK_REF => Self::StencilBackRef,
            consts::GL_STENCIL_BACK_VALUE_MASK => Self::StencilBackValueMask,
            consts::GL_STENCIL_BACK_WRITEMASK => Self::StencilBackWritemask,
            consts::GL_STENCIL_BITS => Self::StencilBits,
            consts::GL_STENCIL_CLEAR_VALUE => Self::StencilClearValue,
            consts::GL_STENCIL_FAIL => Self::StencilFail,
            consts::GL_STENCIL_FUNC => Self::StencilFunc,
            consts::GL_STENCIL_PASS_DEPTH_FAIL => Self::StencilPassDepthFail,
            consts::GL_STENCIL_PASS_DEPTH_PASS => Self::StencilPassDepthPass,
            consts::GL_STENCIL_REF => Self::StencilRef,
            consts::GL_STENCIL_TEST => Self::StencilTest,
            consts::GL_STENCIL_VALUE_MASK => Self::StencilValueMask,
            consts::GL_STENCIL_WRITEMASK => Self::StencilWritemask,
            consts::GL_STEREO => Self::Stereo,
            consts::GL_SUBPIXEL_BITS => Self::SubpixelBits,
            consts::GL_TEXTURE_1D => Self::Texture1d,
            consts::GL_TEXTURE_2D => Self::Texture2d,
            consts::GL_TEXTURE_BINDING_1D => Self::TextureBinding1d,
            consts::GL_TEXTURE_BINDING_1D_ARRAY => Self::TextureBinding1dArray,
            consts::GL_TEXTURE_BINDING_2D => Self::TextureBinding2d,
            consts::GL_TEXTURE_BINDING_2D_ARRAY => Self::TextureBinding2dArray,
            consts::GL_TEXTURE_BINDING_2D_MULTISAMPLE => Self::TextureBinding2dMultisample,
            consts::GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY => {
                Self::TextureBinding2dMultisampleArray
            }
            consts::GL_TEXTURE_BINDING_3D => Self::TextureBinding3d,
            consts::GL_TEXTURE_BINDING_BUFFER => Self::TextureBindingBuffer,
            consts::GL_TEXTURE_BINDING_CUBE_MAP => Self::TextureBindingCubeMap,
            consts::GL_TEXTURE_BINDING_RECTANGLE => Self::TextureBindingRectangle,
            consts::GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT => Self::TextureBufferOffsetAlignment,
            consts::GL_TEXTURE_COMPRESSION_HINT => Self::TextureCompressionHint,
            consts::GL_TEXTURE_COORD_ARRAY => Self::TextureCoordArray,
            consts::GL_TEXTURE_COORD_ARRAY_SIZE => Self::TextureCoordArraySize,
            consts::GL_TEXTURE_COORD_ARRAY_STRIDE => Self::TextureCoordArrayStride,
            consts::GL_TEXTURE_COORD_ARRAY_TYPE => Self::TextureCoordArrayType,
            consts::GL_TEXTURE_MATRIX => Self::TextureMatrix,
            consts::GL_TEXTURE_STACK_DEPTH => Self::TextureStackDepth,
            consts::GL_TIMESTAMP => Self::Timestamp,
            consts::GL_TRANSFORM_FEEDBACK_BUFFER_BINDING => Self::TransformFeedbackBufferBinding,
            consts::GL_TRANSFORM_FEEDBACK_BUFFER_SIZE => Self::TransformFeedbackBufferSize,
            consts::GL_TRANSFORM_FEEDBACK_BUFFER_START => Self::TransformFeedbackBufferStart,
            consts::GL_UNIFORM_BUFFER_BINDING => Self::UniformBufferBinding,
            consts::GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT => Self::UniformBufferOffsetAlignment,
            consts::GL_UNIFORM_BUFFER_SIZE => Self::UniformBufferSize,
            consts::GL_UNIFORM_BUFFER_START => Self::UniformBufferStart,
            consts::GL_UNPACK_ALIGNMENT => Self::UnpackAlignment,
            consts::GL_UNPACK_IMAGE_HEIGHT => Self::UnpackImageHeight,
            consts::GL_UNPACK_LSB_FIRST => Self::UnpackLsbFirst,
            consts::GL_UNPACK_ROW_LENGTH => Self::UnpackRowLength,
            consts::GL_UNPACK_SKIP_IMAGES => Self::UnpackSkipImages,
            consts::GL_UNPACK_SKIP_PIXELS => Self::UnpackSkipPixels,
            consts::GL_UNPACK_SKIP_ROWS => Self::UnpackSkipRows,
            consts::GL_UNPACK_SWAP_BYTES => Self::UnpackSwapBytes,
            consts::GL_VERTEX_ARRAY => Self::VertexArray,
            consts::GL_VERTEX_ARRAY_BINDING => Self::VertexArrayBinding,
            consts::GL_VERTEX_ARRAY_SIZE => Self::VertexArraySize,
            consts::GL_VERTEX_ARRAY_STRIDE => Self::VertexArrayStride,
            consts::GL_VERTEX_ARRAY_TYPE => Self::VertexArrayType,
            consts::GL_VERTEX_BINDING_DIVISOR => Self::VertexBindingDivisor,
            consts::GL_VERTEX_BINDING_OFFSET => Self::VertexBindingOffset,
            consts::GL_VERTEX_BINDING_STRIDE => Self::VertexBindingStride,
            consts::GL_VIEWPORT => Self::Viewport,
            v => panic!("Unknow value {v} for GetPName"),
        }
    }
}

#[repr(u32)]
pub enum GetPointervPName {
    ColorArrayPointer = consts::GL_COLOR_ARRAY_POINTER,
    DebugCallbackFunction = consts::GL_DEBUG_CALLBACK_FUNCTION,
    DebugCallbackUserParam = consts::GL_DEBUG_CALLBACK_USER_PARAM,
    NormalArrayPointer = consts::GL_NORMAL_ARRAY_POINTER,
    TextureCoordArrayPointer = consts::GL_TEXTURE_COORD_ARRAY_POINTER,
    VertexArrayPointer = consts::GL_VERTEX_ARRAY_POINTER,
}

impl From<u32> for GetPointervPName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_COLOR_ARRAY_POINTER => Self::ColorArrayPointer,
            consts::GL_DEBUG_CALLBACK_FUNCTION => Self::DebugCallbackFunction,
            consts::GL_DEBUG_CALLBACK_USER_PARAM => Self::DebugCallbackUserParam,
            consts::GL_NORMAL_ARRAY_POINTER => Self::NormalArrayPointer,
            consts::GL_TEXTURE_COORD_ARRAY_POINTER => Self::TextureCoordArrayPointer,
            consts::GL_VERTEX_ARRAY_POINTER => Self::VertexArrayPointer,
            v => panic!("Unknow value {v} for GetPointervPName"),
        }
    }
}

#[repr(u32)]
pub enum GetTextureParameter {
    TextureAlphaSize = consts::GL_TEXTURE_ALPHA_SIZE,
    TextureBlueSize = consts::GL_TEXTURE_BLUE_SIZE,
    TextureBorderColor = consts::GL_TEXTURE_BORDER_COLOR,
    TextureGreenSize = consts::GL_TEXTURE_GREEN_SIZE,
    TextureHeight = consts::GL_TEXTURE_HEIGHT,
    TextureInternalFormat = consts::GL_TEXTURE_INTERNAL_FORMAT,
    TextureMagFilter = consts::GL_TEXTURE_MAG_FILTER,
    TextureMinFilter = consts::GL_TEXTURE_MIN_FILTER,
    TextureRedSize = consts::GL_TEXTURE_RED_SIZE,
    TextureWidth = consts::GL_TEXTURE_WIDTH,
    TextureWrapS = consts::GL_TEXTURE_WRAP_S,
    TextureWrapT = consts::GL_TEXTURE_WRAP_T,
}

impl From<u32> for GetTextureParameter {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_TEXTURE_ALPHA_SIZE => Self::TextureAlphaSize,
            consts::GL_TEXTURE_BLUE_SIZE => Self::TextureBlueSize,
            consts::GL_TEXTURE_BORDER_COLOR => Self::TextureBorderColor,
            consts::GL_TEXTURE_GREEN_SIZE => Self::TextureGreenSize,
            consts::GL_TEXTURE_HEIGHT => Self::TextureHeight,
            consts::GL_TEXTURE_INTERNAL_FORMAT => Self::TextureInternalFormat,
            consts::GL_TEXTURE_MAG_FILTER => Self::TextureMagFilter,
            consts::GL_TEXTURE_MIN_FILTER => Self::TextureMinFilter,
            consts::GL_TEXTURE_RED_SIZE => Self::TextureRedSize,
            consts::GL_TEXTURE_WIDTH => Self::TextureWidth,
            consts::GL_TEXTURE_WRAP_S => Self::TextureWrapS,
            consts::GL_TEXTURE_WRAP_T => Self::TextureWrapT,
            v => panic!("Unknow value {v} for GetTextureParameter"),
        }
    }
}

#[repr(u32)]
pub enum GraphicsResetStatus {
    GuiltyContextReset = consts::GL_GUILTY_CONTEXT_RESET,
    InnocentContextReset = consts::GL_INNOCENT_CONTEXT_RESET,
    NoError = consts::GL_NO_ERROR,
    UnknownContextReset = consts::GL_UNKNOWN_CONTEXT_RESET,
}

impl From<u32> for GraphicsResetStatus {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_GUILTY_CONTEXT_RESET => Self::GuiltyContextReset,
            consts::GL_INNOCENT_CONTEXT_RESET => Self::InnocentContextReset,
            consts::GL_NO_ERROR => Self::NoError,
            consts::GL_UNKNOWN_CONTEXT_RESET => Self::UnknownContextReset,
            v => panic!("Unknow value {v} for GraphicsResetStatus"),
        }
    }
}

#[repr(u32)]
pub enum HintMode {
    DontCare = consts::GL_DONT_CARE,
    Fastest = consts::GL_FASTEST,
    Nicest = consts::GL_NICEST,
}

impl From<u32> for HintMode {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_DONT_CARE => Self::DontCare,
            consts::GL_FASTEST => Self::Fastest,
            consts::GL_NICEST => Self::Nicest,
            v => panic!("Unknow value {v} for HintMode"),
        }
    }
}

#[repr(u32)]
pub enum HintTarget {
    FogHint = consts::GL_FOG_HINT,
    FragmentShaderDerivativeHint = consts::GL_FRAGMENT_SHADER_DERIVATIVE_HINT,
    GenerateMipmapHint = consts::GL_GENERATE_MIPMAP_HINT,
    LineSmoothHint = consts::GL_LINE_SMOOTH_HINT,
    PerspectiveCorrectionHint = consts::GL_PERSPECTIVE_CORRECTION_HINT,
    PointSmoothHint = consts::GL_POINT_SMOOTH_HINT,
    PolygonSmoothHint = consts::GL_POLYGON_SMOOTH_HINT,
    ProgramBinaryRetrievableHint = consts::GL_PROGRAM_BINARY_RETRIEVABLE_HINT,
    TextureCompressionHint = consts::GL_TEXTURE_COMPRESSION_HINT,
}

impl From<u32> for HintTarget {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_FOG_HINT => Self::FogHint,
            consts::GL_FRAGMENT_SHADER_DERIVATIVE_HINT => Self::FragmentShaderDerivativeHint,
            consts::GL_GENERATE_MIPMAP_HINT => Self::GenerateMipmapHint,
            consts::GL_LINE_SMOOTH_HINT => Self::LineSmoothHint,
            consts::GL_PERSPECTIVE_CORRECTION_HINT => Self::PerspectiveCorrectionHint,
            consts::GL_POINT_SMOOTH_HINT => Self::PointSmoothHint,
            consts::GL_POLYGON_SMOOTH_HINT => Self::PolygonSmoothHint,
            consts::GL_PROGRAM_BINARY_RETRIEVABLE_HINT => Self::ProgramBinaryRetrievableHint,
            consts::GL_TEXTURE_COMPRESSION_HINT => Self::TextureCompressionHint,
            v => panic!("Unknow value {v} for HintTarget"),
        }
    }
}

#[repr(u32)]
pub enum InternalFormat {
    CompressedR11Eac = consts::GL_COMPRESSED_R11_EAC,
    CompressedRed = consts::GL_COMPRESSED_RED,
    CompressedRedRgtc1 = consts::GL_COMPRESSED_RED_RGTC1,
    CompressedRg = consts::GL_COMPRESSED_RG,
    CompressedRg11Eac = consts::GL_COMPRESSED_RG11_EAC,
    CompressedRgb = consts::GL_COMPRESSED_RGB,
    CompressedRgb8Etc2 = consts::GL_COMPRESSED_RGB8_ETC2,
    CompressedRgb8PunchthroughAlpha1Etc2 = consts::GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2,
    CompressedRgba = consts::GL_COMPRESSED_RGBA,
    CompressedRgba8Etc2Eac = consts::GL_COMPRESSED_RGBA8_ETC2_EAC,
    CompressedRgbaAstc10x10 = consts::GL_COMPRESSED_RGBA_ASTC_10x10,
    CompressedRgbaAstc10x5 = consts::GL_COMPRESSED_RGBA_ASTC_10x5,
    CompressedRgbaAstc10x6 = consts::GL_COMPRESSED_RGBA_ASTC_10x6,
    CompressedRgbaAstc10x8 = consts::GL_COMPRESSED_RGBA_ASTC_10x8,
    CompressedRgbaAstc12x10 = consts::GL_COMPRESSED_RGBA_ASTC_12x10,
    CompressedRgbaAstc12x12 = consts::GL_COMPRESSED_RGBA_ASTC_12x12,
    CompressedRgbaAstc4x4 = consts::GL_COMPRESSED_RGBA_ASTC_4x4,
    CompressedRgbaAstc5x4 = consts::GL_COMPRESSED_RGBA_ASTC_5x4,
    CompressedRgbaAstc5x5 = consts::GL_COMPRESSED_RGBA_ASTC_5x5,
    CompressedRgbaAstc6x5 = consts::GL_COMPRESSED_RGBA_ASTC_6x5,
    CompressedRgbaAstc6x6 = consts::GL_COMPRESSED_RGBA_ASTC_6x6,
    CompressedRgbaAstc8x5 = consts::GL_COMPRESSED_RGBA_ASTC_8x5,
    CompressedRgbaAstc8x6 = consts::GL_COMPRESSED_RGBA_ASTC_8x6,
    CompressedRgbaAstc8x8 = consts::GL_COMPRESSED_RGBA_ASTC_8x8,
    CompressedRgRgtc2 = consts::GL_COMPRESSED_RG_RGTC2,
    CompressedSignedR11Eac = consts::GL_COMPRESSED_SIGNED_R11_EAC,
    CompressedSignedRedRgtc1 = consts::GL_COMPRESSED_SIGNED_RED_RGTC1,
    CompressedSignedRg11Eac = consts::GL_COMPRESSED_SIGNED_RG11_EAC,
    CompressedSignedRgRgtc2 = consts::GL_COMPRESSED_SIGNED_RG_RGTC2,
    CompressedSrgb = consts::GL_COMPRESSED_SRGB,
    CompressedSrgb8Alpha8Astc10x10 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10,
    CompressedSrgb8Alpha8Astc10x5 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5,
    CompressedSrgb8Alpha8Astc10x6 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6,
    CompressedSrgb8Alpha8Astc10x8 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8,
    CompressedSrgb8Alpha8Astc12x10 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10,
    CompressedSrgb8Alpha8Astc12x12 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12,
    CompressedSrgb8Alpha8Astc4x4 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4,
    CompressedSrgb8Alpha8Astc5x4 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4,
    CompressedSrgb8Alpha8Astc5x5 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5,
    CompressedSrgb8Alpha8Astc6x5 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5,
    CompressedSrgb8Alpha8Astc6x6 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6,
    CompressedSrgb8Alpha8Astc8x5 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5,
    CompressedSrgb8Alpha8Astc8x6 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6,
    CompressedSrgb8Alpha8Astc8x8 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8,
    CompressedSrgb8Alpha8Etc2Eac = consts::GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC,
    CompressedSrgb8Etc2 = consts::GL_COMPRESSED_SRGB8_ETC2,
    CompressedSrgb8PunchthroughAlpha1Etc2 = consts::GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2,
    CompressedSrgbAlpha = consts::GL_COMPRESSED_SRGB_ALPHA,
    Depth24Stencil8 = consts::GL_DEPTH24_STENCIL8,
    Depth32fStencil8 = consts::GL_DEPTH32F_STENCIL8,
    DepthComponent = consts::GL_DEPTH_COMPONENT,
    DepthComponent16 = consts::GL_DEPTH_COMPONENT16,
    DepthComponent24 = consts::GL_DEPTH_COMPONENT24,
    DepthComponent32 = consts::GL_DEPTH_COMPONENT32,
    DepthComponent32f = consts::GL_DEPTH_COMPONENT32F,
    DepthStencil = consts::GL_DEPTH_STENCIL,
    R11fG11fB10f = consts::GL_R11F_G11F_B10F,
    R16 = consts::GL_R16,
    R16f = consts::GL_R16F,
    R16i = consts::GL_R16I,
    R16ui = consts::GL_R16UI,
    R16Snorm = consts::GL_R16_SNORM,
    R32f = consts::GL_R32F,
    R32i = consts::GL_R32I,
    R32ui = consts::GL_R32UI,
    R3G3B2 = consts::GL_R3_G3_B2,
    R8 = consts::GL_R8,
    R8i = consts::GL_R8I,
    R8ui = consts::GL_R8UI,
    R8Snorm = consts::GL_R8_SNORM,
    Red = consts::GL_RED,
    Rg = consts::GL_RG,
    Rg16 = consts::GL_RG16,
    Rg16f = consts::GL_RG16F,
    Rg16i = consts::GL_RG16I,
    Rg16ui = consts::GL_RG16UI,
    Rg16Snorm = consts::GL_RG16_SNORM,
    Rg32f = consts::GL_RG32F,
    Rg32i = consts::GL_RG32I,
    Rg32ui = consts::GL_RG32UI,
    Rg8 = consts::GL_RG8,
    Rg8i = consts::GL_RG8I,
    Rg8ui = consts::GL_RG8UI,
    Rg8Snorm = consts::GL_RG8_SNORM,
    Rgb = consts::GL_RGB,
    Rgb10 = consts::GL_RGB10,
    Rgb10A2 = consts::GL_RGB10_A2,
    Rgb10A2ui = consts::GL_RGB10_A2UI,
    Rgb12 = consts::GL_RGB12,
    Rgb16 = consts::GL_RGB16,
    Rgb16f = consts::GL_RGB16F,
    Rgb16i = consts::GL_RGB16I,
    Rgb16ui = consts::GL_RGB16UI,
    Rgb16Snorm = consts::GL_RGB16_SNORM,
    Rgb32f = consts::GL_RGB32F,
    Rgb32i = consts::GL_RGB32I,
    Rgb32ui = consts::GL_RGB32UI,
    Rgb4 = consts::GL_RGB4,
    Rgb5 = consts::GL_RGB5,
    Rgb565 = consts::GL_RGB565,
    Rgb5A1 = consts::GL_RGB5_A1,
    Rgb8 = consts::GL_RGB8,
    Rgb8i = consts::GL_RGB8I,
    Rgb8ui = consts::GL_RGB8UI,
    Rgb8Snorm = consts::GL_RGB8_SNORM,
    Rgb9E5 = consts::GL_RGB9_E5,
    Rgba = consts::GL_RGBA,
    Rgba12 = consts::GL_RGBA12,
    Rgba16 = consts::GL_RGBA16,
    Rgba16f = consts::GL_RGBA16F,
    Rgba16i = consts::GL_RGBA16I,
    Rgba16ui = consts::GL_RGBA16UI,
    Rgba16Snorm = consts::GL_RGBA16_SNORM,
    Rgba2 = consts::GL_RGBA2,
    Rgba32f = consts::GL_RGBA32F,
    Rgba32i = consts::GL_RGBA32I,
    Rgba32ui = consts::GL_RGBA32UI,
    Rgba4 = consts::GL_RGBA4,
    Rgba8 = consts::GL_RGBA8,
    Rgba8i = consts::GL_RGBA8I,
    Rgba8ui = consts::GL_RGBA8UI,
    Rgba8Snorm = consts::GL_RGBA8_SNORM,
    Srgb = consts::GL_SRGB,
    Srgb8 = consts::GL_SRGB8,
    Srgb8Alpha8 = consts::GL_SRGB8_ALPHA8,
    SrgbAlpha = consts::GL_SRGB_ALPHA,
    StencilIndex = consts::GL_STENCIL_INDEX,
    StencilIndex1 = consts::GL_STENCIL_INDEX1,
    StencilIndex16 = consts::GL_STENCIL_INDEX16,
    StencilIndex4 = consts::GL_STENCIL_INDEX4,
    StencilIndex8 = consts::GL_STENCIL_INDEX8,
}

impl From<u32> for InternalFormat {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_COMPRESSED_R11_EAC => Self::CompressedR11Eac,
            consts::GL_COMPRESSED_RED => Self::CompressedRed,
            consts::GL_COMPRESSED_RED_RGTC1 => Self::CompressedRedRgtc1,
            consts::GL_COMPRESSED_RG => Self::CompressedRg,
            consts::GL_COMPRESSED_RG11_EAC => Self::CompressedRg11Eac,
            consts::GL_COMPRESSED_RGB => Self::CompressedRgb,
            consts::GL_COMPRESSED_RGB8_ETC2 => Self::CompressedRgb8Etc2,
            consts::GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2 => {
                Self::CompressedRgb8PunchthroughAlpha1Etc2
            }
            consts::GL_COMPRESSED_RGBA => Self::CompressedRgba,
            consts::GL_COMPRESSED_RGBA8_ETC2_EAC => Self::CompressedRgba8Etc2Eac,
            consts::GL_COMPRESSED_RGBA_ASTC_10x10 => Self::CompressedRgbaAstc10x10,
            consts::GL_COMPRESSED_RGBA_ASTC_10x5 => Self::CompressedRgbaAstc10x5,
            consts::GL_COMPRESSED_RGBA_ASTC_10x6 => Self::CompressedRgbaAstc10x6,
            consts::GL_COMPRESSED_RGBA_ASTC_10x8 => Self::CompressedRgbaAstc10x8,
            consts::GL_COMPRESSED_RGBA_ASTC_12x10 => Self::CompressedRgbaAstc12x10,
            consts::GL_COMPRESSED_RGBA_ASTC_12x12 => Self::CompressedRgbaAstc12x12,
            consts::GL_COMPRESSED_RGBA_ASTC_4x4 => Self::CompressedRgbaAstc4x4,
            consts::GL_COMPRESSED_RGBA_ASTC_5x4 => Self::CompressedRgbaAstc5x4,
            consts::GL_COMPRESSED_RGBA_ASTC_5x5 => Self::CompressedRgbaAstc5x5,
            consts::GL_COMPRESSED_RGBA_ASTC_6x5 => Self::CompressedRgbaAstc6x5,
            consts::GL_COMPRESSED_RGBA_ASTC_6x6 => Self::CompressedRgbaAstc6x6,
            consts::GL_COMPRESSED_RGBA_ASTC_8x5 => Self::CompressedRgbaAstc8x5,
            consts::GL_COMPRESSED_RGBA_ASTC_8x6 => Self::CompressedRgbaAstc8x6,
            consts::GL_COMPRESSED_RGBA_ASTC_8x8 => Self::CompressedRgbaAstc8x8,
            consts::GL_COMPRESSED_RG_RGTC2 => Self::CompressedRgRgtc2,
            consts::GL_COMPRESSED_SIGNED_R11_EAC => Self::CompressedSignedR11Eac,
            consts::GL_COMPRESSED_SIGNED_RED_RGTC1 => Self::CompressedSignedRedRgtc1,
            consts::GL_COMPRESSED_SIGNED_RG11_EAC => Self::CompressedSignedRg11Eac,
            consts::GL_COMPRESSED_SIGNED_RG_RGTC2 => Self::CompressedSignedRgRgtc2,
            consts::GL_COMPRESSED_SRGB => Self::CompressedSrgb,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10 => Self::CompressedSrgb8Alpha8Astc10x10,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5 => Self::CompressedSrgb8Alpha8Astc10x5,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6 => Self::CompressedSrgb8Alpha8Astc10x6,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8 => Self::CompressedSrgb8Alpha8Astc10x8,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10 => Self::CompressedSrgb8Alpha8Astc12x10,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12 => Self::CompressedSrgb8Alpha8Astc12x12,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4 => Self::CompressedSrgb8Alpha8Astc4x4,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4 => Self::CompressedSrgb8Alpha8Astc5x4,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5 => Self::CompressedSrgb8Alpha8Astc5x5,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5 => Self::CompressedSrgb8Alpha8Astc6x5,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6 => Self::CompressedSrgb8Alpha8Astc6x6,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5 => Self::CompressedSrgb8Alpha8Astc8x5,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6 => Self::CompressedSrgb8Alpha8Astc8x6,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8 => Self::CompressedSrgb8Alpha8Astc8x8,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC => Self::CompressedSrgb8Alpha8Etc2Eac,
            consts::GL_COMPRESSED_SRGB8_ETC2 => Self::CompressedSrgb8Etc2,
            consts::GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2 => {
                Self::CompressedSrgb8PunchthroughAlpha1Etc2
            }
            consts::GL_COMPRESSED_SRGB_ALPHA => Self::CompressedSrgbAlpha,
            consts::GL_DEPTH24_STENCIL8 => Self::Depth24Stencil8,
            consts::GL_DEPTH32F_STENCIL8 => Self::Depth32fStencil8,
            consts::GL_DEPTH_COMPONENT => Self::DepthComponent,
            consts::GL_DEPTH_COMPONENT16 => Self::DepthComponent16,
            consts::GL_DEPTH_COMPONENT24 => Self::DepthComponent24,
            consts::GL_DEPTH_COMPONENT32 => Self::DepthComponent32,
            consts::GL_DEPTH_COMPONENT32F => Self::DepthComponent32f,
            consts::GL_DEPTH_STENCIL => Self::DepthStencil,
            consts::GL_R11F_G11F_B10F => Self::R11fG11fB10f,
            consts::GL_R16 => Self::R16,
            consts::GL_R16F => Self::R16f,
            consts::GL_R16I => Self::R16i,
            consts::GL_R16UI => Self::R16ui,
            consts::GL_R16_SNORM => Self::R16Snorm,
            consts::GL_R32F => Self::R32f,
            consts::GL_R32I => Self::R32i,
            consts::GL_R32UI => Self::R32ui,
            consts::GL_R3_G3_B2 => Self::R3G3B2,
            consts::GL_R8 => Self::R8,
            consts::GL_R8I => Self::R8i,
            consts::GL_R8UI => Self::R8ui,
            consts::GL_R8_SNORM => Self::R8Snorm,
            consts::GL_RED => Self::Red,
            consts::GL_RG => Self::Rg,
            consts::GL_RG16 => Self::Rg16,
            consts::GL_RG16F => Self::Rg16f,
            consts::GL_RG16I => Self::Rg16i,
            consts::GL_RG16UI => Self::Rg16ui,
            consts::GL_RG16_SNORM => Self::Rg16Snorm,
            consts::GL_RG32F => Self::Rg32f,
            consts::GL_RG32I => Self::Rg32i,
            consts::GL_RG32UI => Self::Rg32ui,
            consts::GL_RG8 => Self::Rg8,
            consts::GL_RG8I => Self::Rg8i,
            consts::GL_RG8UI => Self::Rg8ui,
            consts::GL_RG8_SNORM => Self::Rg8Snorm,
            consts::GL_RGB => Self::Rgb,
            consts::GL_RGB10 => Self::Rgb10,
            consts::GL_RGB10_A2 => Self::Rgb10A2,
            consts::GL_RGB10_A2UI => Self::Rgb10A2ui,
            consts::GL_RGB12 => Self::Rgb12,
            consts::GL_RGB16 => Self::Rgb16,
            consts::GL_RGB16F => Self::Rgb16f,
            consts::GL_RGB16I => Self::Rgb16i,
            consts::GL_RGB16UI => Self::Rgb16ui,
            consts::GL_RGB16_SNORM => Self::Rgb16Snorm,
            consts::GL_RGB32F => Self::Rgb32f,
            consts::GL_RGB32I => Self::Rgb32i,
            consts::GL_RGB32UI => Self::Rgb32ui,
            consts::GL_RGB4 => Self::Rgb4,
            consts::GL_RGB5 => Self::Rgb5,
            consts::GL_RGB565 => Self::Rgb565,
            consts::GL_RGB5_A1 => Self::Rgb5A1,
            consts::GL_RGB8 => Self::Rgb8,
            consts::GL_RGB8I => Self::Rgb8i,
            consts::GL_RGB8UI => Self::Rgb8ui,
            consts::GL_RGB8_SNORM => Self::Rgb8Snorm,
            consts::GL_RGB9_E5 => Self::Rgb9E5,
            consts::GL_RGBA => Self::Rgba,
            consts::GL_RGBA12 => Self::Rgba12,
            consts::GL_RGBA16 => Self::Rgba16,
            consts::GL_RGBA16F => Self::Rgba16f,
            consts::GL_RGBA16I => Self::Rgba16i,
            consts::GL_RGBA16UI => Self::Rgba16ui,
            consts::GL_RGBA16_SNORM => Self::Rgba16Snorm,
            consts::GL_RGBA2 => Self::Rgba2,
            consts::GL_RGBA32F => Self::Rgba32f,
            consts::GL_RGBA32I => Self::Rgba32i,
            consts::GL_RGBA32UI => Self::Rgba32ui,
            consts::GL_RGBA4 => Self::Rgba4,
            consts::GL_RGBA8 => Self::Rgba8,
            consts::GL_RGBA8I => Self::Rgba8i,
            consts::GL_RGBA8UI => Self::Rgba8ui,
            consts::GL_RGBA8_SNORM => Self::Rgba8Snorm,
            consts::GL_SRGB => Self::Srgb,
            consts::GL_SRGB8 => Self::Srgb8,
            consts::GL_SRGB8_ALPHA8 => Self::Srgb8Alpha8,
            consts::GL_SRGB_ALPHA => Self::SrgbAlpha,
            consts::GL_STENCIL_INDEX => Self::StencilIndex,
            consts::GL_STENCIL_INDEX1 => Self::StencilIndex1,
            consts::GL_STENCIL_INDEX16 => Self::StencilIndex16,
            consts::GL_STENCIL_INDEX4 => Self::StencilIndex4,
            consts::GL_STENCIL_INDEX8 => Self::StencilIndex8,
            v => panic!("Unknow value {v} for InternalFormat"),
        }
    }
}

#[repr(u32)]
pub enum InternalFormatPName {
    GenerateMipmap = consts::GL_GENERATE_MIPMAP,
    ImageFormatCompatibilityType = consts::GL_IMAGE_FORMAT_COMPATIBILITY_TYPE,
    NumSampleCounts = consts::GL_NUM_SAMPLE_COUNTS,
    Samples = consts::GL_SAMPLES,
    TextureCompressed = consts::GL_TEXTURE_COMPRESSED,
}

impl From<u32> for InternalFormatPName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_GENERATE_MIPMAP => Self::GenerateMipmap,
            consts::GL_IMAGE_FORMAT_COMPATIBILITY_TYPE => Self::ImageFormatCompatibilityType,
            consts::GL_NUM_SAMPLE_COUNTS => Self::NumSampleCounts,
            consts::GL_SAMPLES => Self::Samples,
            consts::GL_TEXTURE_COMPRESSED => Self::TextureCompressed,
            v => panic!("Unknow value {v} for InternalFormatPName"),
        }
    }
}

#[repr(u32)]
pub enum InvalidateFramebufferAttachment {
    Color = consts::GL_COLOR,
    ColorAttachment0 = consts::GL_COLOR_ATTACHMENT0,
    ColorAttachment1 = consts::GL_COLOR_ATTACHMENT1,
    ColorAttachment10 = consts::GL_COLOR_ATTACHMENT10,
    ColorAttachment11 = consts::GL_COLOR_ATTACHMENT11,
    ColorAttachment12 = consts::GL_COLOR_ATTACHMENT12,
    ColorAttachment13 = consts::GL_COLOR_ATTACHMENT13,
    ColorAttachment14 = consts::GL_COLOR_ATTACHMENT14,
    ColorAttachment15 = consts::GL_COLOR_ATTACHMENT15,
    ColorAttachment16 = consts::GL_COLOR_ATTACHMENT16,
    ColorAttachment17 = consts::GL_COLOR_ATTACHMENT17,
    ColorAttachment18 = consts::GL_COLOR_ATTACHMENT18,
    ColorAttachment19 = consts::GL_COLOR_ATTACHMENT19,
    ColorAttachment2 = consts::GL_COLOR_ATTACHMENT2,
    ColorAttachment20 = consts::GL_COLOR_ATTACHMENT20,
    ColorAttachment21 = consts::GL_COLOR_ATTACHMENT21,
    ColorAttachment22 = consts::GL_COLOR_ATTACHMENT22,
    ColorAttachment23 = consts::GL_COLOR_ATTACHMENT23,
    ColorAttachment24 = consts::GL_COLOR_ATTACHMENT24,
    ColorAttachment25 = consts::GL_COLOR_ATTACHMENT25,
    ColorAttachment26 = consts::GL_COLOR_ATTACHMENT26,
    ColorAttachment27 = consts::GL_COLOR_ATTACHMENT27,
    ColorAttachment28 = consts::GL_COLOR_ATTACHMENT28,
    ColorAttachment29 = consts::GL_COLOR_ATTACHMENT29,
    ColorAttachment3 = consts::GL_COLOR_ATTACHMENT3,
    ColorAttachment30 = consts::GL_COLOR_ATTACHMENT30,
    ColorAttachment31 = consts::GL_COLOR_ATTACHMENT31,
    ColorAttachment4 = consts::GL_COLOR_ATTACHMENT4,
    ColorAttachment5 = consts::GL_COLOR_ATTACHMENT5,
    ColorAttachment6 = consts::GL_COLOR_ATTACHMENT6,
    ColorAttachment7 = consts::GL_COLOR_ATTACHMENT7,
    ColorAttachment8 = consts::GL_COLOR_ATTACHMENT8,
    ColorAttachment9 = consts::GL_COLOR_ATTACHMENT9,
    Depth = consts::GL_DEPTH,
    DepthAttachment = consts::GL_DEPTH_ATTACHMENT,
    DepthStencilAttachment = consts::GL_DEPTH_STENCIL_ATTACHMENT,
    Stencil = consts::GL_STENCIL,
}

impl From<u32> for InvalidateFramebufferAttachment {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_COLOR => Self::Color,
            consts::GL_COLOR_ATTACHMENT0 => Self::ColorAttachment0,
            consts::GL_COLOR_ATTACHMENT1 => Self::ColorAttachment1,
            consts::GL_COLOR_ATTACHMENT10 => Self::ColorAttachment10,
            consts::GL_COLOR_ATTACHMENT11 => Self::ColorAttachment11,
            consts::GL_COLOR_ATTACHMENT12 => Self::ColorAttachment12,
            consts::GL_COLOR_ATTACHMENT13 => Self::ColorAttachment13,
            consts::GL_COLOR_ATTACHMENT14 => Self::ColorAttachment14,
            consts::GL_COLOR_ATTACHMENT15 => Self::ColorAttachment15,
            consts::GL_COLOR_ATTACHMENT16 => Self::ColorAttachment16,
            consts::GL_COLOR_ATTACHMENT17 => Self::ColorAttachment17,
            consts::GL_COLOR_ATTACHMENT18 => Self::ColorAttachment18,
            consts::GL_COLOR_ATTACHMENT19 => Self::ColorAttachment19,
            consts::GL_COLOR_ATTACHMENT2 => Self::ColorAttachment2,
            consts::GL_COLOR_ATTACHMENT20 => Self::ColorAttachment20,
            consts::GL_COLOR_ATTACHMENT21 => Self::ColorAttachment21,
            consts::GL_COLOR_ATTACHMENT22 => Self::ColorAttachment22,
            consts::GL_COLOR_ATTACHMENT23 => Self::ColorAttachment23,
            consts::GL_COLOR_ATTACHMENT24 => Self::ColorAttachment24,
            consts::GL_COLOR_ATTACHMENT25 => Self::ColorAttachment25,
            consts::GL_COLOR_ATTACHMENT26 => Self::ColorAttachment26,
            consts::GL_COLOR_ATTACHMENT27 => Self::ColorAttachment27,
            consts::GL_COLOR_ATTACHMENT28 => Self::ColorAttachment28,
            consts::GL_COLOR_ATTACHMENT29 => Self::ColorAttachment29,
            consts::GL_COLOR_ATTACHMENT3 => Self::ColorAttachment3,
            consts::GL_COLOR_ATTACHMENT30 => Self::ColorAttachment30,
            consts::GL_COLOR_ATTACHMENT31 => Self::ColorAttachment31,
            consts::GL_COLOR_ATTACHMENT4 => Self::ColorAttachment4,
            consts::GL_COLOR_ATTACHMENT5 => Self::ColorAttachment5,
            consts::GL_COLOR_ATTACHMENT6 => Self::ColorAttachment6,
            consts::GL_COLOR_ATTACHMENT7 => Self::ColorAttachment7,
            consts::GL_COLOR_ATTACHMENT8 => Self::ColorAttachment8,
            consts::GL_COLOR_ATTACHMENT9 => Self::ColorAttachment9,
            consts::GL_DEPTH => Self::Depth,
            consts::GL_DEPTH_ATTACHMENT => Self::DepthAttachment,
            consts::GL_DEPTH_STENCIL_ATTACHMENT => Self::DepthStencilAttachment,
            consts::GL_STENCIL => Self::Stencil,
            v => panic!("Unknow value {v} for InvalidateFramebufferAttachment"),
        }
    }
}

#[repr(u32)]
pub enum LightModelParameter {
    LightModelAmbient = consts::GL_LIGHT_MODEL_AMBIENT,
    LightModelTwoSide = consts::GL_LIGHT_MODEL_TWO_SIDE,
}

impl From<u32> for LightModelParameter {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_LIGHT_MODEL_AMBIENT => Self::LightModelAmbient,
            consts::GL_LIGHT_MODEL_TWO_SIDE => Self::LightModelTwoSide,
            v => panic!("Unknow value {v} for LightModelParameter"),
        }
    }
}

#[repr(u32)]
pub enum LightName {
    Light0 = consts::GL_LIGHT0,
    Light1 = consts::GL_LIGHT1,
    Light2 = consts::GL_LIGHT2,
    Light3 = consts::GL_LIGHT3,
    Light4 = consts::GL_LIGHT4,
    Light5 = consts::GL_LIGHT5,
    Light6 = consts::GL_LIGHT6,
    Light7 = consts::GL_LIGHT7,
}

impl From<u32> for LightName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_LIGHT0 => Self::Light0,
            consts::GL_LIGHT1 => Self::Light1,
            consts::GL_LIGHT2 => Self::Light2,
            consts::GL_LIGHT3 => Self::Light3,
            consts::GL_LIGHT4 => Self::Light4,
            consts::GL_LIGHT5 => Self::Light5,
            consts::GL_LIGHT6 => Self::Light6,
            consts::GL_LIGHT7 => Self::Light7,
            v => panic!("Unknow value {v} for LightName"),
        }
    }
}

#[repr(u32)]
pub enum LightParameter {
    ConstantAttenuation = consts::GL_CONSTANT_ATTENUATION,
    LinearAttenuation = consts::GL_LINEAR_ATTENUATION,
    Position = consts::GL_POSITION,
    QuadraticAttenuation = consts::GL_QUADRATIC_ATTENUATION,
    SpotCutoff = consts::GL_SPOT_CUTOFF,
    SpotDirection = consts::GL_SPOT_DIRECTION,
    SpotExponent = consts::GL_SPOT_EXPONENT,
}

impl From<u32> for LightParameter {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_CONSTANT_ATTENUATION => Self::ConstantAttenuation,
            consts::GL_LINEAR_ATTENUATION => Self::LinearAttenuation,
            consts::GL_POSITION => Self::Position,
            consts::GL_QUADRATIC_ATTENUATION => Self::QuadraticAttenuation,
            consts::GL_SPOT_CUTOFF => Self::SpotCutoff,
            consts::GL_SPOT_DIRECTION => Self::SpotDirection,
            consts::GL_SPOT_EXPONENT => Self::SpotExponent,
            v => panic!("Unknow value {v} for LightParameter"),
        }
    }
}

#[repr(u32)]
pub enum LogicOp {
    And = consts::GL_AND,
    AndInverted = consts::GL_AND_INVERTED,
    AndReverse = consts::GL_AND_REVERSE,
    Clear = consts::GL_CLEAR,
    Copy = consts::GL_COPY,
    CopyInverted = consts::GL_COPY_INVERTED,
    Equiv = consts::GL_EQUIV,
    Invert = consts::GL_INVERT,
    Nand = consts::GL_NAND,
    Noop = consts::GL_NOOP,
    Nor = consts::GL_NOR,
    Or = consts::GL_OR,
    OrInverted = consts::GL_OR_INVERTED,
    OrReverse = consts::GL_OR_REVERSE,
    Set = consts::GL_SET,
    Xor = consts::GL_XOR,
}

impl From<u32> for LogicOp {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_AND => Self::And,
            consts::GL_AND_INVERTED => Self::AndInverted,
            consts::GL_AND_REVERSE => Self::AndReverse,
            consts::GL_CLEAR => Self::Clear,
            consts::GL_COPY => Self::Copy,
            consts::GL_COPY_INVERTED => Self::CopyInverted,
            consts::GL_EQUIV => Self::Equiv,
            consts::GL_INVERT => Self::Invert,
            consts::GL_NAND => Self::Nand,
            consts::GL_NOOP => Self::Noop,
            consts::GL_NOR => Self::Nor,
            consts::GL_OR => Self::Or,
            consts::GL_OR_INVERTED => Self::OrInverted,
            consts::GL_OR_REVERSE => Self::OrReverse,
            consts::GL_SET => Self::Set,
            consts::GL_XOR => Self::Xor,
            v => panic!("Unknow value {v} for LogicOp"),
        }
    }
}

#[repr(u32)]
pub enum MapBufferAccessMask {
    MapFlushExplicitBit = consts::GL_MAP_FLUSH_EXPLICIT_BIT,
    MapInvalidateBufferBit = consts::GL_MAP_INVALIDATE_BUFFER_BIT,
    MapInvalidateRangeBit = consts::GL_MAP_INVALIDATE_RANGE_BIT,
    MapReadBit = consts::GL_MAP_READ_BIT,
    MapUnsynchronizedBit = consts::GL_MAP_UNSYNCHRONIZED_BIT,
    MapWriteBit = consts::GL_MAP_WRITE_BIT,
}

impl From<u32> for MapBufferAccessMask {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_MAP_FLUSH_EXPLICIT_BIT => Self::MapFlushExplicitBit,
            consts::GL_MAP_INVALIDATE_BUFFER_BIT => Self::MapInvalidateBufferBit,
            consts::GL_MAP_INVALIDATE_RANGE_BIT => Self::MapInvalidateRangeBit,
            consts::GL_MAP_READ_BIT => Self::MapReadBit,
            consts::GL_MAP_UNSYNCHRONIZED_BIT => Self::MapUnsynchronizedBit,
            consts::GL_MAP_WRITE_BIT => Self::MapWriteBit,
            v => panic!("Unknow value {v} for MapBufferAccessMask"),
        }
    }
}

#[repr(u32)]
pub enum MaterialParameter {
    Ambient = consts::GL_AMBIENT,
    AmbientAndDiffuse = consts::GL_AMBIENT_AND_DIFFUSE,
    Diffuse = consts::GL_DIFFUSE,
    Emission = consts::GL_EMISSION,
    Shininess = consts::GL_SHININESS,
    Specular = consts::GL_SPECULAR,
}

impl From<u32> for MaterialParameter {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_AMBIENT => Self::Ambient,
            consts::GL_AMBIENT_AND_DIFFUSE => Self::AmbientAndDiffuse,
            consts::GL_DIFFUSE => Self::Diffuse,
            consts::GL_EMISSION => Self::Emission,
            consts::GL_SHININESS => Self::Shininess,
            consts::GL_SPECULAR => Self::Specular,
            v => panic!("Unknow value {v} for MaterialParameter"),
        }
    }
}

#[repr(u32)]
pub enum MatrixMode {
    Modelview = consts::GL_MODELVIEW,
    Projection = consts::GL_PROJECTION,
    Texture = consts::GL_TEXTURE,
}

impl From<u32> for MatrixMode {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_MODELVIEW => Self::Modelview,
            consts::GL_PROJECTION => Self::Projection,
            consts::GL_TEXTURE => Self::Texture,
            v => panic!("Unknow value {v} for MatrixMode"),
        }
    }
}

#[repr(u32)]
pub enum MemoryBarrierMask {
    AllBarrierBits = consts::GL_ALL_BARRIER_BITS,
    AtomicCounterBarrierBit = consts::GL_ATOMIC_COUNTER_BARRIER_BIT,
    BufferUpdateBarrierBit = consts::GL_BUFFER_UPDATE_BARRIER_BIT,
    CommandBarrierBit = consts::GL_COMMAND_BARRIER_BIT,
    ElementArrayBarrierBit = consts::GL_ELEMENT_ARRAY_BARRIER_BIT,
    FramebufferBarrierBit = consts::GL_FRAMEBUFFER_BARRIER_BIT,
    PixelBufferBarrierBit = consts::GL_PIXEL_BUFFER_BARRIER_BIT,
    ShaderImageAccessBarrierBit = consts::GL_SHADER_IMAGE_ACCESS_BARRIER_BIT,
    ShaderStorageBarrierBit = consts::GL_SHADER_STORAGE_BARRIER_BIT,
    TextureFetchBarrierBit = consts::GL_TEXTURE_FETCH_BARRIER_BIT,
    TextureUpdateBarrierBit = consts::GL_TEXTURE_UPDATE_BARRIER_BIT,
    TransformFeedbackBarrierBit = consts::GL_TRANSFORM_FEEDBACK_BARRIER_BIT,
    UniformBarrierBit = consts::GL_UNIFORM_BARRIER_BIT,
    VertexAttribArrayBarrierBit = consts::GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT,
}

impl From<u32> for MemoryBarrierMask {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ALL_BARRIER_BITS => Self::AllBarrierBits,
            consts::GL_ATOMIC_COUNTER_BARRIER_BIT => Self::AtomicCounterBarrierBit,
            consts::GL_BUFFER_UPDATE_BARRIER_BIT => Self::BufferUpdateBarrierBit,
            consts::GL_COMMAND_BARRIER_BIT => Self::CommandBarrierBit,
            consts::GL_ELEMENT_ARRAY_BARRIER_BIT => Self::ElementArrayBarrierBit,
            consts::GL_FRAMEBUFFER_BARRIER_BIT => Self::FramebufferBarrierBit,
            consts::GL_PIXEL_BUFFER_BARRIER_BIT => Self::PixelBufferBarrierBit,
            consts::GL_SHADER_IMAGE_ACCESS_BARRIER_BIT => Self::ShaderImageAccessBarrierBit,
            consts::GL_SHADER_STORAGE_BARRIER_BIT => Self::ShaderStorageBarrierBit,
            consts::GL_TEXTURE_FETCH_BARRIER_BIT => Self::TextureFetchBarrierBit,
            consts::GL_TEXTURE_UPDATE_BARRIER_BIT => Self::TextureUpdateBarrierBit,
            consts::GL_TRANSFORM_FEEDBACK_BARRIER_BIT => Self::TransformFeedbackBarrierBit,
            consts::GL_UNIFORM_BARRIER_BIT => Self::UniformBarrierBit,
            consts::GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT => Self::VertexAttribArrayBarrierBit,
            v => panic!("Unknow value {v} for MemoryBarrierMask"),
        }
    }
}

#[repr(u32)]
pub enum NormalPointerType {
    Byte = consts::GL_BYTE,
    Double = consts::GL_DOUBLE,
    Float = consts::GL_FLOAT,
    Int = consts::GL_INT,
    Short = consts::GL_SHORT,
}

impl From<u32> for NormalPointerType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BYTE => Self::Byte,
            consts::GL_DOUBLE => Self::Double,
            consts::GL_FLOAT => Self::Float,
            consts::GL_INT => Self::Int,
            consts::GL_SHORT => Self::Short,
            v => panic!("Unknow value {v} for NormalPointerType"),
        }
    }
}

#[repr(u32)]
pub enum ObjectIdentifier {
    Buffer = consts::GL_BUFFER,
    Framebuffer = consts::GL_FRAMEBUFFER,
    Program = consts::GL_PROGRAM,
    ProgramPipeline = consts::GL_PROGRAM_PIPELINE,
    Query = consts::GL_QUERY,
    Renderbuffer = consts::GL_RENDERBUFFER,
    Sampler = consts::GL_SAMPLER,
    Shader = consts::GL_SHADER,
    Texture = consts::GL_TEXTURE,
    TransformFeedback = consts::GL_TRANSFORM_FEEDBACK,
    VertexArray = consts::GL_VERTEX_ARRAY,
}

impl From<u32> for ObjectIdentifier {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BUFFER => Self::Buffer,
            consts::GL_FRAMEBUFFER => Self::Framebuffer,
            consts::GL_PROGRAM => Self::Program,
            consts::GL_PROGRAM_PIPELINE => Self::ProgramPipeline,
            consts::GL_QUERY => Self::Query,
            consts::GL_RENDERBUFFER => Self::Renderbuffer,
            consts::GL_SAMPLER => Self::Sampler,
            consts::GL_SHADER => Self::Shader,
            consts::GL_TEXTURE => Self::Texture,
            consts::GL_TRANSFORM_FEEDBACK => Self::TransformFeedback,
            consts::GL_VERTEX_ARRAY => Self::VertexArray,
            v => panic!("Unknow value {v} for ObjectIdentifier"),
        }
    }
}

#[repr(u32)]
pub enum PatchParameterName {
    PatchVertices = consts::GL_PATCH_VERTICES,
}

impl From<u32> for PatchParameterName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_PATCH_VERTICES => Self::PatchVertices,
            v => panic!("Unknow value {v} for PatchParameterName"),
        }
    }
}

#[repr(u32)]
pub enum PipelineParameterName {
    ActiveProgram = consts::GL_ACTIVE_PROGRAM,
    FragmentShader = consts::GL_FRAGMENT_SHADER,
    GeometryShader = consts::GL_GEOMETRY_SHADER,
    InfoLogLength = consts::GL_INFO_LOG_LENGTH,
    TessControlShader = consts::GL_TESS_CONTROL_SHADER,
    TessEvaluationShader = consts::GL_TESS_EVALUATION_SHADER,
    VertexShader = consts::GL_VERTEX_SHADER,
}

impl From<u32> for PipelineParameterName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ACTIVE_PROGRAM => Self::ActiveProgram,
            consts::GL_FRAGMENT_SHADER => Self::FragmentShader,
            consts::GL_GEOMETRY_SHADER => Self::GeometryShader,
            consts::GL_INFO_LOG_LENGTH => Self::InfoLogLength,
            consts::GL_TESS_CONTROL_SHADER => Self::TessControlShader,
            consts::GL_TESS_EVALUATION_SHADER => Self::TessEvaluationShader,
            consts::GL_VERTEX_SHADER => Self::VertexShader,
            v => panic!("Unknow value {v} for PipelineParameterName"),
        }
    }
}

#[repr(u32)]
pub enum PixelFormat {
    Alpha = consts::GL_ALPHA,
    Bgr = consts::GL_BGR,
    Bgra = consts::GL_BGRA,
    BgraInteger = consts::GL_BGRA_INTEGER,
    BgrInteger = consts::GL_BGR_INTEGER,
    Blue = consts::GL_BLUE,
    BlueInteger = consts::GL_BLUE_INTEGER,
    DepthComponent = consts::GL_DEPTH_COMPONENT,
    DepthStencil = consts::GL_DEPTH_STENCIL,
    Green = consts::GL_GREEN,
    GreenInteger = consts::GL_GREEN_INTEGER,
    Luminance = consts::GL_LUMINANCE,
    LuminanceAlpha = consts::GL_LUMINANCE_ALPHA,
    Red = consts::GL_RED,
    RedInteger = consts::GL_RED_INTEGER,
    Rg = consts::GL_RG,
    Rgb = consts::GL_RGB,
    Rgba = consts::GL_RGBA,
    RgbaInteger = consts::GL_RGBA_INTEGER,
    RgbInteger = consts::GL_RGB_INTEGER,
    RgInteger = consts::GL_RG_INTEGER,
    StencilIndex = consts::GL_STENCIL_INDEX,
    UnsignedInt = consts::GL_UNSIGNED_INT,
    UnsignedShort = consts::GL_UNSIGNED_SHORT,
}

impl From<u32> for PixelFormat {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ALPHA => Self::Alpha,
            consts::GL_BGR => Self::Bgr,
            consts::GL_BGRA => Self::Bgra,
            consts::GL_BGRA_INTEGER => Self::BgraInteger,
            consts::GL_BGR_INTEGER => Self::BgrInteger,
            consts::GL_BLUE => Self::Blue,
            consts::GL_BLUE_INTEGER => Self::BlueInteger,
            consts::GL_DEPTH_COMPONENT => Self::DepthComponent,
            consts::GL_DEPTH_STENCIL => Self::DepthStencil,
            consts::GL_GREEN => Self::Green,
            consts::GL_GREEN_INTEGER => Self::GreenInteger,
            consts::GL_LUMINANCE => Self::Luminance,
            consts::GL_LUMINANCE_ALPHA => Self::LuminanceAlpha,
            consts::GL_RED => Self::Red,
            consts::GL_RED_INTEGER => Self::RedInteger,
            consts::GL_RG => Self::Rg,
            consts::GL_RGB => Self::Rgb,
            consts::GL_RGBA => Self::Rgba,
            consts::GL_RGBA_INTEGER => Self::RgbaInteger,
            consts::GL_RGB_INTEGER => Self::RgbInteger,
            consts::GL_RG_INTEGER => Self::RgInteger,
            consts::GL_STENCIL_INDEX => Self::StencilIndex,
            consts::GL_UNSIGNED_INT => Self::UnsignedInt,
            consts::GL_UNSIGNED_SHORT => Self::UnsignedShort,
            v => panic!("Unknow value {v} for PixelFormat"),
        }
    }
}

#[repr(u32)]
pub enum PixelStoreParameter {
    PackAlignment = consts::GL_PACK_ALIGNMENT,
    PackImageHeight = consts::GL_PACK_IMAGE_HEIGHT,
    PackLsbFirst = consts::GL_PACK_LSB_FIRST,
    PackRowLength = consts::GL_PACK_ROW_LENGTH,
    PackSkipImages = consts::GL_PACK_SKIP_IMAGES,
    PackSkipPixels = consts::GL_PACK_SKIP_PIXELS,
    PackSkipRows = consts::GL_PACK_SKIP_ROWS,
    PackSwapBytes = consts::GL_PACK_SWAP_BYTES,
    UnpackAlignment = consts::GL_UNPACK_ALIGNMENT,
    UnpackImageHeight = consts::GL_UNPACK_IMAGE_HEIGHT,
    UnpackLsbFirst = consts::GL_UNPACK_LSB_FIRST,
    UnpackRowLength = consts::GL_UNPACK_ROW_LENGTH,
    UnpackSkipImages = consts::GL_UNPACK_SKIP_IMAGES,
    UnpackSkipPixels = consts::GL_UNPACK_SKIP_PIXELS,
    UnpackSkipRows = consts::GL_UNPACK_SKIP_ROWS,
    UnpackSwapBytes = consts::GL_UNPACK_SWAP_BYTES,
}

impl From<u32> for PixelStoreParameter {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_PACK_ALIGNMENT => Self::PackAlignment,
            consts::GL_PACK_IMAGE_HEIGHT => Self::PackImageHeight,
            consts::GL_PACK_LSB_FIRST => Self::PackLsbFirst,
            consts::GL_PACK_ROW_LENGTH => Self::PackRowLength,
            consts::GL_PACK_SKIP_IMAGES => Self::PackSkipImages,
            consts::GL_PACK_SKIP_PIXELS => Self::PackSkipPixels,
            consts::GL_PACK_SKIP_ROWS => Self::PackSkipRows,
            consts::GL_PACK_SWAP_BYTES => Self::PackSwapBytes,
            consts::GL_UNPACK_ALIGNMENT => Self::UnpackAlignment,
            consts::GL_UNPACK_IMAGE_HEIGHT => Self::UnpackImageHeight,
            consts::GL_UNPACK_LSB_FIRST => Self::UnpackLsbFirst,
            consts::GL_UNPACK_ROW_LENGTH => Self::UnpackRowLength,
            consts::GL_UNPACK_SKIP_IMAGES => Self::UnpackSkipImages,
            consts::GL_UNPACK_SKIP_PIXELS => Self::UnpackSkipPixels,
            consts::GL_UNPACK_SKIP_ROWS => Self::UnpackSkipRows,
            consts::GL_UNPACK_SWAP_BYTES => Self::UnpackSwapBytes,
            v => panic!("Unknow value {v} for PixelStoreParameter"),
        }
    }
}

#[repr(u32)]
pub enum PixelType {
    Byte = consts::GL_BYTE,
    Float = consts::GL_FLOAT,
    Float32UnsignedInt248Rev = consts::GL_FLOAT_32_UNSIGNED_INT_24_8_REV,
    HalfFloat = consts::GL_HALF_FLOAT,
    Int = consts::GL_INT,
    Short = consts::GL_SHORT,
    UnsignedByte = consts::GL_UNSIGNED_BYTE,
    UnsignedByte233Rev = consts::GL_UNSIGNED_BYTE_2_3_3_REV,
    UnsignedByte332 = consts::GL_UNSIGNED_BYTE_3_3_2,
    UnsignedInt = consts::GL_UNSIGNED_INT,
    UnsignedInt10f11f11fRev = consts::GL_UNSIGNED_INT_10F_11F_11F_REV,
    UnsignedInt1010102 = consts::GL_UNSIGNED_INT_10_10_10_2,
    UnsignedInt248 = consts::GL_UNSIGNED_INT_24_8,
    UnsignedInt2101010Rev = consts::GL_UNSIGNED_INT_2_10_10_10_REV,
    UnsignedInt5999Rev = consts::GL_UNSIGNED_INT_5_9_9_9_REV,
    UnsignedInt8888 = consts::GL_UNSIGNED_INT_8_8_8_8,
    UnsignedInt8888Rev = consts::GL_UNSIGNED_INT_8_8_8_8_REV,
    UnsignedShort = consts::GL_UNSIGNED_SHORT,
    UnsignedShort1555Rev = consts::GL_UNSIGNED_SHORT_1_5_5_5_REV,
    UnsignedShort4444 = consts::GL_UNSIGNED_SHORT_4_4_4_4,
    UnsignedShort4444Rev = consts::GL_UNSIGNED_SHORT_4_4_4_4_REV,
    UnsignedShort5551 = consts::GL_UNSIGNED_SHORT_5_5_5_1,
    UnsignedShort565 = consts::GL_UNSIGNED_SHORT_5_6_5,
    UnsignedShort565Rev = consts::GL_UNSIGNED_SHORT_5_6_5_REV,
}

impl From<u32> for PixelType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BYTE => Self::Byte,
            consts::GL_FLOAT => Self::Float,
            consts::GL_FLOAT_32_UNSIGNED_INT_24_8_REV => Self::Float32UnsignedInt248Rev,
            consts::GL_HALF_FLOAT => Self::HalfFloat,
            consts::GL_INT => Self::Int,
            consts::GL_SHORT => Self::Short,
            consts::GL_UNSIGNED_BYTE => Self::UnsignedByte,
            consts::GL_UNSIGNED_BYTE_2_3_3_REV => Self::UnsignedByte233Rev,
            consts::GL_UNSIGNED_BYTE_3_3_2 => Self::UnsignedByte332,
            consts::GL_UNSIGNED_INT => Self::UnsignedInt,
            consts::GL_UNSIGNED_INT_10F_11F_11F_REV => Self::UnsignedInt10f11f11fRev,
            consts::GL_UNSIGNED_INT_10_10_10_2 => Self::UnsignedInt1010102,
            consts::GL_UNSIGNED_INT_24_8 => Self::UnsignedInt248,
            consts::GL_UNSIGNED_INT_2_10_10_10_REV => Self::UnsignedInt2101010Rev,
            consts::GL_UNSIGNED_INT_5_9_9_9_REV => Self::UnsignedInt5999Rev,
            consts::GL_UNSIGNED_INT_8_8_8_8 => Self::UnsignedInt8888,
            consts::GL_UNSIGNED_INT_8_8_8_8_REV => Self::UnsignedInt8888Rev,
            consts::GL_UNSIGNED_SHORT => Self::UnsignedShort,
            consts::GL_UNSIGNED_SHORT_1_5_5_5_REV => Self::UnsignedShort1555Rev,
            consts::GL_UNSIGNED_SHORT_4_4_4_4 => Self::UnsignedShort4444,
            consts::GL_UNSIGNED_SHORT_4_4_4_4_REV => Self::UnsignedShort4444Rev,
            consts::GL_UNSIGNED_SHORT_5_5_5_1 => Self::UnsignedShort5551,
            consts::GL_UNSIGNED_SHORT_5_6_5 => Self::UnsignedShort565,
            consts::GL_UNSIGNED_SHORT_5_6_5_REV => Self::UnsignedShort565Rev,
            v => panic!("Unknow value {v} for PixelType"),
        }
    }
}

#[repr(u32)]
pub enum PointParameterNameARB {
    PointDistanceAttenuation = consts::GL_POINT_DISTANCE_ATTENUATION,
    PointFadeThresholdSize = consts::GL_POINT_FADE_THRESHOLD_SIZE,
    PointSizeMax = consts::GL_POINT_SIZE_MAX,
    PointSizeMin = consts::GL_POINT_SIZE_MIN,
}

impl From<u32> for PointParameterNameARB {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_POINT_DISTANCE_ATTENUATION => Self::PointDistanceAttenuation,
            consts::GL_POINT_FADE_THRESHOLD_SIZE => Self::PointFadeThresholdSize,
            consts::GL_POINT_SIZE_MAX => Self::PointSizeMax,
            consts::GL_POINT_SIZE_MIN => Self::PointSizeMin,
            v => panic!("Unknow value {v} for PointParameterNameARB"),
        }
    }
}

#[repr(u32)]
pub enum PolygonMode {
    Fill = consts::GL_FILL,
    Line = consts::GL_LINE,
    Point = consts::GL_POINT,
}

impl From<u32> for PolygonMode {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_FILL => Self::Fill,
            consts::GL_LINE => Self::Line,
            consts::GL_POINT => Self::Point,
            v => panic!("Unknow value {v} for PolygonMode"),
        }
    }
}

#[repr(u32)]
pub enum PrecisionType {
    HighFloat = consts::GL_HIGH_FLOAT,
    HighInt = consts::GL_HIGH_INT,
    LowFloat = consts::GL_LOW_FLOAT,
    LowInt = consts::GL_LOW_INT,
    MediumFloat = consts::GL_MEDIUM_FLOAT,
    MediumInt = consts::GL_MEDIUM_INT,
}

impl From<u32> for PrecisionType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_HIGH_FLOAT => Self::HighFloat,
            consts::GL_HIGH_INT => Self::HighInt,
            consts::GL_LOW_FLOAT => Self::LowFloat,
            consts::GL_LOW_INT => Self::LowInt,
            consts::GL_MEDIUM_FLOAT => Self::MediumFloat,
            consts::GL_MEDIUM_INT => Self::MediumInt,
            v => panic!("Unknow value {v} for PrecisionType"),
        }
    }
}

#[repr(u32)]
pub enum PrimitiveType {
    Lines = consts::GL_LINES,
    LinesAdjacency = consts::GL_LINES_ADJACENCY,
    LineLoop = consts::GL_LINE_LOOP,
    LineStrip = consts::GL_LINE_STRIP,
    LineStripAdjacency = consts::GL_LINE_STRIP_ADJACENCY,
    Patches = consts::GL_PATCHES,
    Points = consts::GL_POINTS,
    Quads = consts::GL_QUADS,
    Triangles = consts::GL_TRIANGLES,
    TrianglesAdjacency = consts::GL_TRIANGLES_ADJACENCY,
    TriangleFan = consts::GL_TRIANGLE_FAN,
    TriangleStrip = consts::GL_TRIANGLE_STRIP,
    TriangleStripAdjacency = consts::GL_TRIANGLE_STRIP_ADJACENCY,
}

impl From<u32> for PrimitiveType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_LINES => Self::Lines,
            consts::GL_LINES_ADJACENCY => Self::LinesAdjacency,
            consts::GL_LINE_LOOP => Self::LineLoop,
            consts::GL_LINE_STRIP => Self::LineStrip,
            consts::GL_LINE_STRIP_ADJACENCY => Self::LineStripAdjacency,
            consts::GL_PATCHES => Self::Patches,
            consts::GL_POINTS => Self::Points,
            consts::GL_QUADS => Self::Quads,
            consts::GL_TRIANGLES => Self::Triangles,
            consts::GL_TRIANGLES_ADJACENCY => Self::TrianglesAdjacency,
            consts::GL_TRIANGLE_FAN => Self::TriangleFan,
            consts::GL_TRIANGLE_STRIP => Self::TriangleStrip,
            consts::GL_TRIANGLE_STRIP_ADJACENCY => Self::TriangleStripAdjacency,
            v => panic!("Unknow value {v} for PrimitiveType"),
        }
    }
}

#[repr(u32)]
pub enum ProgramInterface {
    BufferVariable = consts::GL_BUFFER_VARIABLE,
    ProgramInput = consts::GL_PROGRAM_INPUT,
    ProgramOutput = consts::GL_PROGRAM_OUTPUT,
    ShaderStorageBlock = consts::GL_SHADER_STORAGE_BLOCK,
    TransformFeedbackBuffer = consts::GL_TRANSFORM_FEEDBACK_BUFFER,
    TransformFeedbackVarying = consts::GL_TRANSFORM_FEEDBACK_VARYING,
    Uniform = consts::GL_UNIFORM,
    UniformBlock = consts::GL_UNIFORM_BLOCK,
}

impl From<u32> for ProgramInterface {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BUFFER_VARIABLE => Self::BufferVariable,
            consts::GL_PROGRAM_INPUT => Self::ProgramInput,
            consts::GL_PROGRAM_OUTPUT => Self::ProgramOutput,
            consts::GL_SHADER_STORAGE_BLOCK => Self::ShaderStorageBlock,
            consts::GL_TRANSFORM_FEEDBACK_BUFFER => Self::TransformFeedbackBuffer,
            consts::GL_TRANSFORM_FEEDBACK_VARYING => Self::TransformFeedbackVarying,
            consts::GL_UNIFORM => Self::Uniform,
            consts::GL_UNIFORM_BLOCK => Self::UniformBlock,
            v => panic!("Unknow value {v} for ProgramInterface"),
        }
    }
}

#[repr(u32)]
pub enum ProgramInterfacePName {
    ActiveResources = consts::GL_ACTIVE_RESOURCES,
    MaxNameLength = consts::GL_MAX_NAME_LENGTH,
    MaxNumActiveVariables = consts::GL_MAX_NUM_ACTIVE_VARIABLES,
}

impl From<u32> for ProgramInterfacePName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ACTIVE_RESOURCES => Self::ActiveResources,
            consts::GL_MAX_NAME_LENGTH => Self::MaxNameLength,
            consts::GL_MAX_NUM_ACTIVE_VARIABLES => Self::MaxNumActiveVariables,
            v => panic!("Unknow value {v} for ProgramInterfacePName"),
        }
    }
}

#[repr(u32)]
pub enum ProgramParameterPName {
    ProgramBinaryRetrievableHint = consts::GL_PROGRAM_BINARY_RETRIEVABLE_HINT,
    ProgramSeparable = consts::GL_PROGRAM_SEPARABLE,
}

impl From<u32> for ProgramParameterPName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_PROGRAM_BINARY_RETRIEVABLE_HINT => Self::ProgramBinaryRetrievableHint,
            consts::GL_PROGRAM_SEPARABLE => Self::ProgramSeparable,
            v => panic!("Unknow value {v} for ProgramParameterPName"),
        }
    }
}

#[repr(u32)]
pub enum ProgramPropertyARB {
    ActiveAtomicCounterBuffers = consts::GL_ACTIVE_ATOMIC_COUNTER_BUFFERS,
    ActiveAttributes = consts::GL_ACTIVE_ATTRIBUTES,
    ActiveAttributeMaxLength = consts::GL_ACTIVE_ATTRIBUTE_MAX_LENGTH,
    ActiveUniforms = consts::GL_ACTIVE_UNIFORMS,
    ActiveUniformBlocks = consts::GL_ACTIVE_UNIFORM_BLOCKS,
    ActiveUniformBlockMaxNameLength = consts::GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH,
    ActiveUniformMaxLength = consts::GL_ACTIVE_UNIFORM_MAX_LENGTH,
    AttachedShaders = consts::GL_ATTACHED_SHADERS,
    ComputeWorkGroupSize = consts::GL_COMPUTE_WORK_GROUP_SIZE,
    DeleteStatus = consts::GL_DELETE_STATUS,
    GeometryInputType = consts::GL_GEOMETRY_INPUT_TYPE,
    GeometryOutputType = consts::GL_GEOMETRY_OUTPUT_TYPE,
    GeometryVerticesOut = consts::GL_GEOMETRY_VERTICES_OUT,
    InfoLogLength = consts::GL_INFO_LOG_LENGTH,
    LinkStatus = consts::GL_LINK_STATUS,
    ProgramBinaryLength = consts::GL_PROGRAM_BINARY_LENGTH,
    TransformFeedbackBufferMode = consts::GL_TRANSFORM_FEEDBACK_BUFFER_MODE,
    TransformFeedbackVaryings = consts::GL_TRANSFORM_FEEDBACK_VARYINGS,
    TransformFeedbackVaryingMaxLength = consts::GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH,
    ValidateStatus = consts::GL_VALIDATE_STATUS,
}

impl From<u32> for ProgramPropertyARB {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ACTIVE_ATOMIC_COUNTER_BUFFERS => Self::ActiveAtomicCounterBuffers,
            consts::GL_ACTIVE_ATTRIBUTES => Self::ActiveAttributes,
            consts::GL_ACTIVE_ATTRIBUTE_MAX_LENGTH => Self::ActiveAttributeMaxLength,
            consts::GL_ACTIVE_UNIFORMS => Self::ActiveUniforms,
            consts::GL_ACTIVE_UNIFORM_BLOCKS => Self::ActiveUniformBlocks,
            consts::GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH => {
                Self::ActiveUniformBlockMaxNameLength
            }
            consts::GL_ACTIVE_UNIFORM_MAX_LENGTH => Self::ActiveUniformMaxLength,
            consts::GL_ATTACHED_SHADERS => Self::AttachedShaders,
            consts::GL_COMPUTE_WORK_GROUP_SIZE => Self::ComputeWorkGroupSize,
            consts::GL_DELETE_STATUS => Self::DeleteStatus,
            consts::GL_GEOMETRY_INPUT_TYPE => Self::GeometryInputType,
            consts::GL_GEOMETRY_OUTPUT_TYPE => Self::GeometryOutputType,
            consts::GL_GEOMETRY_VERTICES_OUT => Self::GeometryVerticesOut,
            consts::GL_INFO_LOG_LENGTH => Self::InfoLogLength,
            consts::GL_LINK_STATUS => Self::LinkStatus,
            consts::GL_PROGRAM_BINARY_LENGTH => Self::ProgramBinaryLength,
            consts::GL_TRANSFORM_FEEDBACK_BUFFER_MODE => Self::TransformFeedbackBufferMode,
            consts::GL_TRANSFORM_FEEDBACK_VARYINGS => Self::TransformFeedbackVaryings,
            consts::GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH => {
                Self::TransformFeedbackVaryingMaxLength
            }
            consts::GL_VALIDATE_STATUS => Self::ValidateStatus,
            v => panic!("Unknow value {v} for ProgramPropertyARB"),
        }
    }
}

#[repr(u32)]
pub enum ProgramResourceProperty {
    ActiveVariables = consts::GL_ACTIVE_VARIABLES,
    ArraySize = consts::GL_ARRAY_SIZE,
    ArrayStride = consts::GL_ARRAY_STRIDE,
    AtomicCounterBufferIndex = consts::GL_ATOMIC_COUNTER_BUFFER_INDEX,
    BlockIndex = consts::GL_BLOCK_INDEX,
    BufferBinding = consts::GL_BUFFER_BINDING,
    BufferDataSize = consts::GL_BUFFER_DATA_SIZE,
    IsPerPatch = consts::GL_IS_PER_PATCH,
    IsRowMajor = consts::GL_IS_ROW_MAJOR,
    Location = consts::GL_LOCATION,
    MatrixStride = consts::GL_MATRIX_STRIDE,
    NameLength = consts::GL_NAME_LENGTH,
    NumActiveVariables = consts::GL_NUM_ACTIVE_VARIABLES,
    Offset = consts::GL_OFFSET,
    ReferencedByComputeShader = consts::GL_REFERENCED_BY_COMPUTE_SHADER,
    ReferencedByFragmentShader = consts::GL_REFERENCED_BY_FRAGMENT_SHADER,
    ReferencedByGeometryShader = consts::GL_REFERENCED_BY_GEOMETRY_SHADER,
    ReferencedByTessControlShader = consts::GL_REFERENCED_BY_TESS_CONTROL_SHADER,
    ReferencedByTessEvaluationShader = consts::GL_REFERENCED_BY_TESS_EVALUATION_SHADER,
    ReferencedByVertexShader = consts::GL_REFERENCED_BY_VERTEX_SHADER,
    TopLevelArraySize = consts::GL_TOP_LEVEL_ARRAY_SIZE,
    TopLevelArrayStride = consts::GL_TOP_LEVEL_ARRAY_STRIDE,
    Type = consts::GL_TYPE,
    Uniform = consts::GL_UNIFORM,
}

impl From<u32> for ProgramResourceProperty {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ACTIVE_VARIABLES => Self::ActiveVariables,
            consts::GL_ARRAY_SIZE => Self::ArraySize,
            consts::GL_ARRAY_STRIDE => Self::ArrayStride,
            consts::GL_ATOMIC_COUNTER_BUFFER_INDEX => Self::AtomicCounterBufferIndex,
            consts::GL_BLOCK_INDEX => Self::BlockIndex,
            consts::GL_BUFFER_BINDING => Self::BufferBinding,
            consts::GL_BUFFER_DATA_SIZE => Self::BufferDataSize,
            consts::GL_IS_PER_PATCH => Self::IsPerPatch,
            consts::GL_IS_ROW_MAJOR => Self::IsRowMajor,
            consts::GL_LOCATION => Self::Location,
            consts::GL_MATRIX_STRIDE => Self::MatrixStride,
            consts::GL_NAME_LENGTH => Self::NameLength,
            consts::GL_NUM_ACTIVE_VARIABLES => Self::NumActiveVariables,
            consts::GL_OFFSET => Self::Offset,
            consts::GL_REFERENCED_BY_COMPUTE_SHADER => Self::ReferencedByComputeShader,
            consts::GL_REFERENCED_BY_FRAGMENT_SHADER => Self::ReferencedByFragmentShader,
            consts::GL_REFERENCED_BY_GEOMETRY_SHADER => Self::ReferencedByGeometryShader,
            consts::GL_REFERENCED_BY_TESS_CONTROL_SHADER => Self::ReferencedByTessControlShader,
            consts::GL_REFERENCED_BY_TESS_EVALUATION_SHADER => {
                Self::ReferencedByTessEvaluationShader
            }
            consts::GL_REFERENCED_BY_VERTEX_SHADER => Self::ReferencedByVertexShader,
            consts::GL_TOP_LEVEL_ARRAY_SIZE => Self::TopLevelArraySize,
            consts::GL_TOP_LEVEL_ARRAY_STRIDE => Self::TopLevelArrayStride,
            consts::GL_TYPE => Self::Type,
            consts::GL_UNIFORM => Self::Uniform,
            v => panic!("Unknow value {v} for ProgramResourceProperty"),
        }
    }
}

#[repr(u32)]
pub enum QueryCounterTarget {
    Timestamp = consts::GL_TIMESTAMP,
}

impl From<u32> for QueryCounterTarget {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_TIMESTAMP => Self::Timestamp,
            v => panic!("Unknow value {v} for QueryCounterTarget"),
        }
    }
}

#[repr(u32)]
pub enum QueryObjectParameterName {
    QueryResult = consts::GL_QUERY_RESULT,
    QueryResultAvailable = consts::GL_QUERY_RESULT_AVAILABLE,
}

impl From<u32> for QueryObjectParameterName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_QUERY_RESULT => Self::QueryResult,
            consts::GL_QUERY_RESULT_AVAILABLE => Self::QueryResultAvailable,
            v => panic!("Unknow value {v} for QueryObjectParameterName"),
        }
    }
}

#[repr(u32)]
pub enum QueryParameterName {
    CurrentQuery = consts::GL_CURRENT_QUERY,
    QueryCounterBits = consts::GL_QUERY_COUNTER_BITS,
}

impl From<u32> for QueryParameterName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_CURRENT_QUERY => Self::CurrentQuery,
            consts::GL_QUERY_COUNTER_BITS => Self::QueryCounterBits,
            v => panic!("Unknow value {v} for QueryParameterName"),
        }
    }
}

#[repr(u32)]
pub enum QueryTarget {
    AnySamplesPassed = consts::GL_ANY_SAMPLES_PASSED,
    AnySamplesPassedConservative = consts::GL_ANY_SAMPLES_PASSED_CONSERVATIVE,
    PrimitivesGenerated = consts::GL_PRIMITIVES_GENERATED,
    SamplesPassed = consts::GL_SAMPLES_PASSED,
    TimeElapsed = consts::GL_TIME_ELAPSED,
    TransformFeedbackPrimitivesWritten = consts::GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN,
}

impl From<u32> for QueryTarget {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ANY_SAMPLES_PASSED => Self::AnySamplesPassed,
            consts::GL_ANY_SAMPLES_PASSED_CONSERVATIVE => Self::AnySamplesPassedConservative,
            consts::GL_PRIMITIVES_GENERATED => Self::PrimitivesGenerated,
            consts::GL_SAMPLES_PASSED => Self::SamplesPassed,
            consts::GL_TIME_ELAPSED => Self::TimeElapsed,
            consts::GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN => {
                Self::TransformFeedbackPrimitivesWritten
            }
            v => panic!("Unknow value {v} for QueryTarget"),
        }
    }
}

#[repr(u32)]
pub enum ReadBufferMode {
    Back = consts::GL_BACK,
    BackLeft = consts::GL_BACK_LEFT,
    BackRight = consts::GL_BACK_RIGHT,
    ColorAttachment0 = consts::GL_COLOR_ATTACHMENT0,
    ColorAttachment1 = consts::GL_COLOR_ATTACHMENT1,
    ColorAttachment10 = consts::GL_COLOR_ATTACHMENT10,
    ColorAttachment11 = consts::GL_COLOR_ATTACHMENT11,
    ColorAttachment12 = consts::GL_COLOR_ATTACHMENT12,
    ColorAttachment13 = consts::GL_COLOR_ATTACHMENT13,
    ColorAttachment14 = consts::GL_COLOR_ATTACHMENT14,
    ColorAttachment15 = consts::GL_COLOR_ATTACHMENT15,
    ColorAttachment2 = consts::GL_COLOR_ATTACHMENT2,
    ColorAttachment3 = consts::GL_COLOR_ATTACHMENT3,
    ColorAttachment4 = consts::GL_COLOR_ATTACHMENT4,
    ColorAttachment5 = consts::GL_COLOR_ATTACHMENT5,
    ColorAttachment6 = consts::GL_COLOR_ATTACHMENT6,
    ColorAttachment7 = consts::GL_COLOR_ATTACHMENT7,
    ColorAttachment8 = consts::GL_COLOR_ATTACHMENT8,
    ColorAttachment9 = consts::GL_COLOR_ATTACHMENT9,
    Front = consts::GL_FRONT,
    FrontLeft = consts::GL_FRONT_LEFT,
    FrontRight = consts::GL_FRONT_RIGHT,
    Left = consts::GL_LEFT,
    None = consts::GL_NONE,
    Right = consts::GL_RIGHT,
}

impl From<u32> for ReadBufferMode {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BACK => Self::Back,
            consts::GL_BACK_LEFT => Self::BackLeft,
            consts::GL_BACK_RIGHT => Self::BackRight,
            consts::GL_COLOR_ATTACHMENT0 => Self::ColorAttachment0,
            consts::GL_COLOR_ATTACHMENT1 => Self::ColorAttachment1,
            consts::GL_COLOR_ATTACHMENT10 => Self::ColorAttachment10,
            consts::GL_COLOR_ATTACHMENT11 => Self::ColorAttachment11,
            consts::GL_COLOR_ATTACHMENT12 => Self::ColorAttachment12,
            consts::GL_COLOR_ATTACHMENT13 => Self::ColorAttachment13,
            consts::GL_COLOR_ATTACHMENT14 => Self::ColorAttachment14,
            consts::GL_COLOR_ATTACHMENT15 => Self::ColorAttachment15,
            consts::GL_COLOR_ATTACHMENT2 => Self::ColorAttachment2,
            consts::GL_COLOR_ATTACHMENT3 => Self::ColorAttachment3,
            consts::GL_COLOR_ATTACHMENT4 => Self::ColorAttachment4,
            consts::GL_COLOR_ATTACHMENT5 => Self::ColorAttachment5,
            consts::GL_COLOR_ATTACHMENT6 => Self::ColorAttachment6,
            consts::GL_COLOR_ATTACHMENT7 => Self::ColorAttachment7,
            consts::GL_COLOR_ATTACHMENT8 => Self::ColorAttachment8,
            consts::GL_COLOR_ATTACHMENT9 => Self::ColorAttachment9,
            consts::GL_FRONT => Self::Front,
            consts::GL_FRONT_LEFT => Self::FrontLeft,
            consts::GL_FRONT_RIGHT => Self::FrontRight,
            consts::GL_LEFT => Self::Left,
            consts::GL_NONE => Self::None,
            consts::GL_RIGHT => Self::Right,
            v => panic!("Unknow value {v} for ReadBufferMode"),
        }
    }
}

#[repr(u32)]
pub enum RenderbufferParameterName {
    RenderbufferAlphaSize = consts::GL_RENDERBUFFER_ALPHA_SIZE,
    RenderbufferBlueSize = consts::GL_RENDERBUFFER_BLUE_SIZE,
    RenderbufferDepthSize = consts::GL_RENDERBUFFER_DEPTH_SIZE,
    RenderbufferGreenSize = consts::GL_RENDERBUFFER_GREEN_SIZE,
    RenderbufferHeight = consts::GL_RENDERBUFFER_HEIGHT,
    RenderbufferInternalFormat = consts::GL_RENDERBUFFER_INTERNAL_FORMAT,
    RenderbufferRedSize = consts::GL_RENDERBUFFER_RED_SIZE,
    RenderbufferSamples = consts::GL_RENDERBUFFER_SAMPLES,
    RenderbufferStencilSize = consts::GL_RENDERBUFFER_STENCIL_SIZE,
    RenderbufferWidth = consts::GL_RENDERBUFFER_WIDTH,
}

impl From<u32> for RenderbufferParameterName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_RENDERBUFFER_ALPHA_SIZE => Self::RenderbufferAlphaSize,
            consts::GL_RENDERBUFFER_BLUE_SIZE => Self::RenderbufferBlueSize,
            consts::GL_RENDERBUFFER_DEPTH_SIZE => Self::RenderbufferDepthSize,
            consts::GL_RENDERBUFFER_GREEN_SIZE => Self::RenderbufferGreenSize,
            consts::GL_RENDERBUFFER_HEIGHT => Self::RenderbufferHeight,
            consts::GL_RENDERBUFFER_INTERNAL_FORMAT => Self::RenderbufferInternalFormat,
            consts::GL_RENDERBUFFER_RED_SIZE => Self::RenderbufferRedSize,
            consts::GL_RENDERBUFFER_SAMPLES => Self::RenderbufferSamples,
            consts::GL_RENDERBUFFER_STENCIL_SIZE => Self::RenderbufferStencilSize,
            consts::GL_RENDERBUFFER_WIDTH => Self::RenderbufferWidth,
            v => panic!("Unknow value {v} for RenderbufferParameterName"),
        }
    }
}

#[repr(u32)]
pub enum RenderbufferTarget {
    Renderbuffer = consts::GL_RENDERBUFFER,
}

impl From<u32> for RenderbufferTarget {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_RENDERBUFFER => Self::Renderbuffer,
            v => panic!("Unknow value {v} for RenderbufferTarget"),
        }
    }
}

#[repr(u32)]
pub enum SamplerParameterF {
    TextureBorderColor = consts::GL_TEXTURE_BORDER_COLOR,
    TextureLodBias = consts::GL_TEXTURE_LOD_BIAS,
    TextureMaxLod = consts::GL_TEXTURE_MAX_LOD,
    TextureMinLod = consts::GL_TEXTURE_MIN_LOD,
}

impl From<u32> for SamplerParameterF {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_TEXTURE_BORDER_COLOR => Self::TextureBorderColor,
            consts::GL_TEXTURE_LOD_BIAS => Self::TextureLodBias,
            consts::GL_TEXTURE_MAX_LOD => Self::TextureMaxLod,
            consts::GL_TEXTURE_MIN_LOD => Self::TextureMinLod,
            v => panic!("Unknow value {v} for SamplerParameterF"),
        }
    }
}

#[repr(u32)]
pub enum SamplerParameterI {
    TextureCompareFunc = consts::GL_TEXTURE_COMPARE_FUNC,
    TextureCompareMode = consts::GL_TEXTURE_COMPARE_MODE,
    TextureMagFilter = consts::GL_TEXTURE_MAG_FILTER,
    TextureMinFilter = consts::GL_TEXTURE_MIN_FILTER,
    TextureWrapR = consts::GL_TEXTURE_WRAP_R,
    TextureWrapS = consts::GL_TEXTURE_WRAP_S,
    TextureWrapT = consts::GL_TEXTURE_WRAP_T,
}

impl From<u32> for SamplerParameterI {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_TEXTURE_COMPARE_FUNC => Self::TextureCompareFunc,
            consts::GL_TEXTURE_COMPARE_MODE => Self::TextureCompareMode,
            consts::GL_TEXTURE_MAG_FILTER => Self::TextureMagFilter,
            consts::GL_TEXTURE_MIN_FILTER => Self::TextureMinFilter,
            consts::GL_TEXTURE_WRAP_R => Self::TextureWrapR,
            consts::GL_TEXTURE_WRAP_S => Self::TextureWrapS,
            consts::GL_TEXTURE_WRAP_T => Self::TextureWrapT,
            v => panic!("Unknow value {v} for SamplerParameterI"),
        }
    }
}

#[repr(u32)]
pub enum ShaderParameterName {
    CompileStatus = consts::GL_COMPILE_STATUS,
    DeleteStatus = consts::GL_DELETE_STATUS,
    InfoLogLength = consts::GL_INFO_LOG_LENGTH,
    ShaderSourceLength = consts::GL_SHADER_SOURCE_LENGTH,
    ShaderType = consts::GL_SHADER_TYPE,
}

impl From<u32> for ShaderParameterName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_COMPILE_STATUS => Self::CompileStatus,
            consts::GL_DELETE_STATUS => Self::DeleteStatus,
            consts::GL_INFO_LOG_LENGTH => Self::InfoLogLength,
            consts::GL_SHADER_SOURCE_LENGTH => Self::ShaderSourceLength,
            consts::GL_SHADER_TYPE => Self::ShaderType,
            v => panic!("Unknow value {v} for ShaderParameterName"),
        }
    }
}

#[repr(u32)]
pub enum ShaderType {
    ComputeShader = consts::GL_COMPUTE_SHADER,
    FragmentShader = consts::GL_FRAGMENT_SHADER,
    GeometryShader = consts::GL_GEOMETRY_SHADER,
    TessControlShader = consts::GL_TESS_CONTROL_SHADER,
    TessEvaluationShader = consts::GL_TESS_EVALUATION_SHADER,
    VertexShader = consts::GL_VERTEX_SHADER,
}

impl From<u32> for ShaderType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_COMPUTE_SHADER => Self::ComputeShader,
            consts::GL_FRAGMENT_SHADER => Self::FragmentShader,
            consts::GL_GEOMETRY_SHADER => Self::GeometryShader,
            consts::GL_TESS_CONTROL_SHADER => Self::TessControlShader,
            consts::GL_TESS_EVALUATION_SHADER => Self::TessEvaluationShader,
            consts::GL_VERTEX_SHADER => Self::VertexShader,
            v => panic!("Unknow value {v} for ShaderType"),
        }
    }
}

#[repr(u32)]
pub enum ShadingModel {
    Flat = consts::GL_FLAT,
    Smooth = consts::GL_SMOOTH,
}

impl From<u32> for ShadingModel {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_FLAT => Self::Flat,
            consts::GL_SMOOTH => Self::Smooth,
            v => panic!("Unknow value {v} for ShadingModel"),
        }
    }
}

#[repr(u32)]
pub enum SizedInternalFormat {
    CompressedR11Eac = consts::GL_COMPRESSED_R11_EAC,
    CompressedRedRgtc1 = consts::GL_COMPRESSED_RED_RGTC1,
    CompressedRg11Eac = consts::GL_COMPRESSED_RG11_EAC,
    CompressedRgb8Etc2 = consts::GL_COMPRESSED_RGB8_ETC2,
    CompressedRgb8PunchthroughAlpha1Etc2 = consts::GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2,
    CompressedRgba8Etc2Eac = consts::GL_COMPRESSED_RGBA8_ETC2_EAC,
    CompressedRgbaAstc10x10 = consts::GL_COMPRESSED_RGBA_ASTC_10x10,
    CompressedRgbaAstc10x5 = consts::GL_COMPRESSED_RGBA_ASTC_10x5,
    CompressedRgbaAstc10x6 = consts::GL_COMPRESSED_RGBA_ASTC_10x6,
    CompressedRgbaAstc10x8 = consts::GL_COMPRESSED_RGBA_ASTC_10x8,
    CompressedRgbaAstc12x10 = consts::GL_COMPRESSED_RGBA_ASTC_12x10,
    CompressedRgbaAstc12x12 = consts::GL_COMPRESSED_RGBA_ASTC_12x12,
    CompressedRgbaAstc4x4 = consts::GL_COMPRESSED_RGBA_ASTC_4x4,
    CompressedRgbaAstc5x4 = consts::GL_COMPRESSED_RGBA_ASTC_5x4,
    CompressedRgbaAstc5x5 = consts::GL_COMPRESSED_RGBA_ASTC_5x5,
    CompressedRgbaAstc6x5 = consts::GL_COMPRESSED_RGBA_ASTC_6x5,
    CompressedRgbaAstc6x6 = consts::GL_COMPRESSED_RGBA_ASTC_6x6,
    CompressedRgbaAstc8x5 = consts::GL_COMPRESSED_RGBA_ASTC_8x5,
    CompressedRgbaAstc8x6 = consts::GL_COMPRESSED_RGBA_ASTC_8x6,
    CompressedRgbaAstc8x8 = consts::GL_COMPRESSED_RGBA_ASTC_8x8,
    CompressedRgRgtc2 = consts::GL_COMPRESSED_RG_RGTC2,
    CompressedSignedR11Eac = consts::GL_COMPRESSED_SIGNED_R11_EAC,
    CompressedSignedRedRgtc1 = consts::GL_COMPRESSED_SIGNED_RED_RGTC1,
    CompressedSignedRg11Eac = consts::GL_COMPRESSED_SIGNED_RG11_EAC,
    CompressedSignedRgRgtc2 = consts::GL_COMPRESSED_SIGNED_RG_RGTC2,
    CompressedSrgb8Alpha8Astc10x10 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10,
    CompressedSrgb8Alpha8Astc10x5 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5,
    CompressedSrgb8Alpha8Astc10x6 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6,
    CompressedSrgb8Alpha8Astc10x8 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8,
    CompressedSrgb8Alpha8Astc12x10 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10,
    CompressedSrgb8Alpha8Astc12x12 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12,
    CompressedSrgb8Alpha8Astc4x4 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4,
    CompressedSrgb8Alpha8Astc5x4 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4,
    CompressedSrgb8Alpha8Astc5x5 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5,
    CompressedSrgb8Alpha8Astc6x5 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5,
    CompressedSrgb8Alpha8Astc6x6 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6,
    CompressedSrgb8Alpha8Astc8x5 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5,
    CompressedSrgb8Alpha8Astc8x6 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6,
    CompressedSrgb8Alpha8Astc8x8 = consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8,
    CompressedSrgb8Alpha8Etc2Eac = consts::GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC,
    CompressedSrgb8Etc2 = consts::GL_COMPRESSED_SRGB8_ETC2,
    CompressedSrgb8PunchthroughAlpha1Etc2 = consts::GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2,
    Depth24Stencil8 = consts::GL_DEPTH24_STENCIL8,
    Depth32fStencil8 = consts::GL_DEPTH32F_STENCIL8,
    DepthComponent16 = consts::GL_DEPTH_COMPONENT16,
    DepthComponent24 = consts::GL_DEPTH_COMPONENT24,
    DepthComponent32 = consts::GL_DEPTH_COMPONENT32,
    DepthComponent32f = consts::GL_DEPTH_COMPONENT32F,
    R11fG11fB10f = consts::GL_R11F_G11F_B10F,
    R16 = consts::GL_R16,
    R16f = consts::GL_R16F,
    R16i = consts::GL_R16I,
    R16ui = consts::GL_R16UI,
    R16Snorm = consts::GL_R16_SNORM,
    R32f = consts::GL_R32F,
    R32i = consts::GL_R32I,
    R32ui = consts::GL_R32UI,
    R3G3B2 = consts::GL_R3_G3_B2,
    R8 = consts::GL_R8,
    R8i = consts::GL_R8I,
    R8ui = consts::GL_R8UI,
    R8Snorm = consts::GL_R8_SNORM,
    Rg16 = consts::GL_RG16,
    Rg16f = consts::GL_RG16F,
    Rg16i = consts::GL_RG16I,
    Rg16ui = consts::GL_RG16UI,
    Rg16Snorm = consts::GL_RG16_SNORM,
    Rg32f = consts::GL_RG32F,
    Rg32i = consts::GL_RG32I,
    Rg32ui = consts::GL_RG32UI,
    Rg8 = consts::GL_RG8,
    Rg8i = consts::GL_RG8I,
    Rg8ui = consts::GL_RG8UI,
    Rg8Snorm = consts::GL_RG8_SNORM,
    Rgb10 = consts::GL_RGB10,
    Rgb10A2 = consts::GL_RGB10_A2,
    Rgb10A2ui = consts::GL_RGB10_A2UI,
    Rgb12 = consts::GL_RGB12,
    Rgb16 = consts::GL_RGB16,
    Rgb16f = consts::GL_RGB16F,
    Rgb16i = consts::GL_RGB16I,
    Rgb16ui = consts::GL_RGB16UI,
    Rgb16Snorm = consts::GL_RGB16_SNORM,
    Rgb32f = consts::GL_RGB32F,
    Rgb32i = consts::GL_RGB32I,
    Rgb32ui = consts::GL_RGB32UI,
    Rgb4 = consts::GL_RGB4,
    Rgb5 = consts::GL_RGB5,
    Rgb565 = consts::GL_RGB565,
    Rgb5A1 = consts::GL_RGB5_A1,
    Rgb8 = consts::GL_RGB8,
    Rgb8i = consts::GL_RGB8I,
    Rgb8ui = consts::GL_RGB8UI,
    Rgb8Snorm = consts::GL_RGB8_SNORM,
    Rgb9E5 = consts::GL_RGB9_E5,
    Rgba12 = consts::GL_RGBA12,
    Rgba16 = consts::GL_RGBA16,
    Rgba16f = consts::GL_RGBA16F,
    Rgba16i = consts::GL_RGBA16I,
    Rgba16ui = consts::GL_RGBA16UI,
    Rgba16Snorm = consts::GL_RGBA16_SNORM,
    Rgba2 = consts::GL_RGBA2,
    Rgba32f = consts::GL_RGBA32F,
    Rgba32i = consts::GL_RGBA32I,
    Rgba32ui = consts::GL_RGBA32UI,
    Rgba4 = consts::GL_RGBA4,
    Rgba8 = consts::GL_RGBA8,
    Rgba8i = consts::GL_RGBA8I,
    Rgba8ui = consts::GL_RGBA8UI,
    Rgba8Snorm = consts::GL_RGBA8_SNORM,
    Srgb8 = consts::GL_SRGB8,
    Srgb8Alpha8 = consts::GL_SRGB8_ALPHA8,
    StencilIndex1 = consts::GL_STENCIL_INDEX1,
    StencilIndex16 = consts::GL_STENCIL_INDEX16,
    StencilIndex4 = consts::GL_STENCIL_INDEX4,
    StencilIndex8 = consts::GL_STENCIL_INDEX8,
}

impl From<u32> for SizedInternalFormat {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_COMPRESSED_R11_EAC => Self::CompressedR11Eac,
            consts::GL_COMPRESSED_RED_RGTC1 => Self::CompressedRedRgtc1,
            consts::GL_COMPRESSED_RG11_EAC => Self::CompressedRg11Eac,
            consts::GL_COMPRESSED_RGB8_ETC2 => Self::CompressedRgb8Etc2,
            consts::GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2 => {
                Self::CompressedRgb8PunchthroughAlpha1Etc2
            }
            consts::GL_COMPRESSED_RGBA8_ETC2_EAC => Self::CompressedRgba8Etc2Eac,
            consts::GL_COMPRESSED_RGBA_ASTC_10x10 => Self::CompressedRgbaAstc10x10,
            consts::GL_COMPRESSED_RGBA_ASTC_10x5 => Self::CompressedRgbaAstc10x5,
            consts::GL_COMPRESSED_RGBA_ASTC_10x6 => Self::CompressedRgbaAstc10x6,
            consts::GL_COMPRESSED_RGBA_ASTC_10x8 => Self::CompressedRgbaAstc10x8,
            consts::GL_COMPRESSED_RGBA_ASTC_12x10 => Self::CompressedRgbaAstc12x10,
            consts::GL_COMPRESSED_RGBA_ASTC_12x12 => Self::CompressedRgbaAstc12x12,
            consts::GL_COMPRESSED_RGBA_ASTC_4x4 => Self::CompressedRgbaAstc4x4,
            consts::GL_COMPRESSED_RGBA_ASTC_5x4 => Self::CompressedRgbaAstc5x4,
            consts::GL_COMPRESSED_RGBA_ASTC_5x5 => Self::CompressedRgbaAstc5x5,
            consts::GL_COMPRESSED_RGBA_ASTC_6x5 => Self::CompressedRgbaAstc6x5,
            consts::GL_COMPRESSED_RGBA_ASTC_6x6 => Self::CompressedRgbaAstc6x6,
            consts::GL_COMPRESSED_RGBA_ASTC_8x5 => Self::CompressedRgbaAstc8x5,
            consts::GL_COMPRESSED_RGBA_ASTC_8x6 => Self::CompressedRgbaAstc8x6,
            consts::GL_COMPRESSED_RGBA_ASTC_8x8 => Self::CompressedRgbaAstc8x8,
            consts::GL_COMPRESSED_RG_RGTC2 => Self::CompressedRgRgtc2,
            consts::GL_COMPRESSED_SIGNED_R11_EAC => Self::CompressedSignedR11Eac,
            consts::GL_COMPRESSED_SIGNED_RED_RGTC1 => Self::CompressedSignedRedRgtc1,
            consts::GL_COMPRESSED_SIGNED_RG11_EAC => Self::CompressedSignedRg11Eac,
            consts::GL_COMPRESSED_SIGNED_RG_RGTC2 => Self::CompressedSignedRgRgtc2,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10 => Self::CompressedSrgb8Alpha8Astc10x10,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5 => Self::CompressedSrgb8Alpha8Astc10x5,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6 => Self::CompressedSrgb8Alpha8Astc10x6,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8 => Self::CompressedSrgb8Alpha8Astc10x8,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10 => Self::CompressedSrgb8Alpha8Astc12x10,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12 => Self::CompressedSrgb8Alpha8Astc12x12,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4 => Self::CompressedSrgb8Alpha8Astc4x4,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4 => Self::CompressedSrgb8Alpha8Astc5x4,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5 => Self::CompressedSrgb8Alpha8Astc5x5,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5 => Self::CompressedSrgb8Alpha8Astc6x5,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6 => Self::CompressedSrgb8Alpha8Astc6x6,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5 => Self::CompressedSrgb8Alpha8Astc8x5,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6 => Self::CompressedSrgb8Alpha8Astc8x6,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8 => Self::CompressedSrgb8Alpha8Astc8x8,
            consts::GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC => Self::CompressedSrgb8Alpha8Etc2Eac,
            consts::GL_COMPRESSED_SRGB8_ETC2 => Self::CompressedSrgb8Etc2,
            consts::GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2 => {
                Self::CompressedSrgb8PunchthroughAlpha1Etc2
            }
            consts::GL_DEPTH24_STENCIL8 => Self::Depth24Stencil8,
            consts::GL_DEPTH32F_STENCIL8 => Self::Depth32fStencil8,
            consts::GL_DEPTH_COMPONENT16 => Self::DepthComponent16,
            consts::GL_DEPTH_COMPONENT24 => Self::DepthComponent24,
            consts::GL_DEPTH_COMPONENT32 => Self::DepthComponent32,
            consts::GL_DEPTH_COMPONENT32F => Self::DepthComponent32f,
            consts::GL_R11F_G11F_B10F => Self::R11fG11fB10f,
            consts::GL_R16 => Self::R16,
            consts::GL_R16F => Self::R16f,
            consts::GL_R16I => Self::R16i,
            consts::GL_R16UI => Self::R16ui,
            consts::GL_R16_SNORM => Self::R16Snorm,
            consts::GL_R32F => Self::R32f,
            consts::GL_R32I => Self::R32i,
            consts::GL_R32UI => Self::R32ui,
            consts::GL_R3_G3_B2 => Self::R3G3B2,
            consts::GL_R8 => Self::R8,
            consts::GL_R8I => Self::R8i,
            consts::GL_R8UI => Self::R8ui,
            consts::GL_R8_SNORM => Self::R8Snorm,
            consts::GL_RG16 => Self::Rg16,
            consts::GL_RG16F => Self::Rg16f,
            consts::GL_RG16I => Self::Rg16i,
            consts::GL_RG16UI => Self::Rg16ui,
            consts::GL_RG16_SNORM => Self::Rg16Snorm,
            consts::GL_RG32F => Self::Rg32f,
            consts::GL_RG32I => Self::Rg32i,
            consts::GL_RG32UI => Self::Rg32ui,
            consts::GL_RG8 => Self::Rg8,
            consts::GL_RG8I => Self::Rg8i,
            consts::GL_RG8UI => Self::Rg8ui,
            consts::GL_RG8_SNORM => Self::Rg8Snorm,
            consts::GL_RGB10 => Self::Rgb10,
            consts::GL_RGB10_A2 => Self::Rgb10A2,
            consts::GL_RGB10_A2UI => Self::Rgb10A2ui,
            consts::GL_RGB12 => Self::Rgb12,
            consts::GL_RGB16 => Self::Rgb16,
            consts::GL_RGB16F => Self::Rgb16f,
            consts::GL_RGB16I => Self::Rgb16i,
            consts::GL_RGB16UI => Self::Rgb16ui,
            consts::GL_RGB16_SNORM => Self::Rgb16Snorm,
            consts::GL_RGB32F => Self::Rgb32f,
            consts::GL_RGB32I => Self::Rgb32i,
            consts::GL_RGB32UI => Self::Rgb32ui,
            consts::GL_RGB4 => Self::Rgb4,
            consts::GL_RGB5 => Self::Rgb5,
            consts::GL_RGB565 => Self::Rgb565,
            consts::GL_RGB5_A1 => Self::Rgb5A1,
            consts::GL_RGB8 => Self::Rgb8,
            consts::GL_RGB8I => Self::Rgb8i,
            consts::GL_RGB8UI => Self::Rgb8ui,
            consts::GL_RGB8_SNORM => Self::Rgb8Snorm,
            consts::GL_RGB9_E5 => Self::Rgb9E5,
            consts::GL_RGBA12 => Self::Rgba12,
            consts::GL_RGBA16 => Self::Rgba16,
            consts::GL_RGBA16F => Self::Rgba16f,
            consts::GL_RGBA16I => Self::Rgba16i,
            consts::GL_RGBA16UI => Self::Rgba16ui,
            consts::GL_RGBA16_SNORM => Self::Rgba16Snorm,
            consts::GL_RGBA2 => Self::Rgba2,
            consts::GL_RGBA32F => Self::Rgba32f,
            consts::GL_RGBA32I => Self::Rgba32i,
            consts::GL_RGBA32UI => Self::Rgba32ui,
            consts::GL_RGBA4 => Self::Rgba4,
            consts::GL_RGBA8 => Self::Rgba8,
            consts::GL_RGBA8I => Self::Rgba8i,
            consts::GL_RGBA8UI => Self::Rgba8ui,
            consts::GL_RGBA8_SNORM => Self::Rgba8Snorm,
            consts::GL_SRGB8 => Self::Srgb8,
            consts::GL_SRGB8_ALPHA8 => Self::Srgb8Alpha8,
            consts::GL_STENCIL_INDEX1 => Self::StencilIndex1,
            consts::GL_STENCIL_INDEX16 => Self::StencilIndex16,
            consts::GL_STENCIL_INDEX4 => Self::StencilIndex4,
            consts::GL_STENCIL_INDEX8 => Self::StencilIndex8,
            v => panic!("Unknow value {v} for SizedInternalFormat"),
        }
    }
}

#[repr(u32)]
pub enum StencilFunction {
    Always = consts::GL_ALWAYS,
    Equal = consts::GL_EQUAL,
    Gequal = consts::GL_GEQUAL,
    Greater = consts::GL_GREATER,
    Lequal = consts::GL_LEQUAL,
    Less = consts::GL_LESS,
    Never = consts::GL_NEVER,
    Notequal = consts::GL_NOTEQUAL,
}

impl From<u32> for StencilFunction {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ALWAYS => Self::Always,
            consts::GL_EQUAL => Self::Equal,
            consts::GL_GEQUAL => Self::Gequal,
            consts::GL_GREATER => Self::Greater,
            consts::GL_LEQUAL => Self::Lequal,
            consts::GL_LESS => Self::Less,
            consts::GL_NEVER => Self::Never,
            consts::GL_NOTEQUAL => Self::Notequal,
            v => panic!("Unknow value {v} for StencilFunction"),
        }
    }
}

#[repr(u32)]
pub enum StencilOp {
    Decr = consts::GL_DECR,
    DecrWrap = consts::GL_DECR_WRAP,
    Incr = consts::GL_INCR,
    IncrWrap = consts::GL_INCR_WRAP,
    Invert = consts::GL_INVERT,
    Keep = consts::GL_KEEP,
    Replace = consts::GL_REPLACE,
    Zero = consts::GL_ZERO,
}

impl From<u32> for StencilOp {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_DECR => Self::Decr,
            consts::GL_DECR_WRAP => Self::DecrWrap,
            consts::GL_INCR => Self::Incr,
            consts::GL_INCR_WRAP => Self::IncrWrap,
            consts::GL_INVERT => Self::Invert,
            consts::GL_KEEP => Self::Keep,
            consts::GL_REPLACE => Self::Replace,
            consts::GL_ZERO => Self::Zero,
            v => panic!("Unknow value {v} for StencilOp"),
        }
    }
}

#[repr(u32)]
pub enum StringName {
    Extensions = consts::GL_EXTENSIONS,
    Renderer = consts::GL_RENDERER,
    ShadingLanguageVersion = consts::GL_SHADING_LANGUAGE_VERSION,
    Vendor = consts::GL_VENDOR,
    Version = consts::GL_VERSION,
}

impl From<u32> for StringName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_EXTENSIONS => Self::Extensions,
            consts::GL_RENDERER => Self::Renderer,
            consts::GL_SHADING_LANGUAGE_VERSION => Self::ShadingLanguageVersion,
            consts::GL_VENDOR => Self::Vendor,
            consts::GL_VERSION => Self::Version,
            v => panic!("Unknow value {v} for StringName"),
        }
    }
}

#[repr(u32)]
pub enum SyncBehaviorFlags {
    None = consts::GL_NONE,
}

impl From<u32> for SyncBehaviorFlags {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_NONE => Self::None,
            v => panic!("Unknow value {v} for SyncBehaviorFlags"),
        }
    }
}

#[repr(u32)]
pub enum SyncCondition {
    SyncGpuCommandsComplete = consts::GL_SYNC_GPU_COMMANDS_COMPLETE,
}

impl From<u32> for SyncCondition {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_SYNC_GPU_COMMANDS_COMPLETE => Self::SyncGpuCommandsComplete,
            v => panic!("Unknow value {v} for SyncCondition"),
        }
    }
}

#[repr(u32)]
pub enum SyncObjectMask {
    SyncFlushCommandsBit = consts::GL_SYNC_FLUSH_COMMANDS_BIT,
}

impl From<u32> for SyncObjectMask {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_SYNC_FLUSH_COMMANDS_BIT => Self::SyncFlushCommandsBit,
            v => panic!("Unknow value {v} for SyncObjectMask"),
        }
    }
}

#[repr(u32)]
pub enum SyncParameterName {
    ObjectType = consts::GL_OBJECT_TYPE,
    SyncCondition = consts::GL_SYNC_CONDITION,
    SyncFlags = consts::GL_SYNC_FLAGS,
    SyncStatus = consts::GL_SYNC_STATUS,
}

impl From<u32> for SyncParameterName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_OBJECT_TYPE => Self::ObjectType,
            consts::GL_SYNC_CONDITION => Self::SyncCondition,
            consts::GL_SYNC_FLAGS => Self::SyncFlags,
            consts::GL_SYNC_STATUS => Self::SyncStatus,
            v => panic!("Unknow value {v} for SyncParameterName"),
        }
    }
}

#[repr(u32)]
pub enum SyncStatus {
    AlreadySignaled = consts::GL_ALREADY_SIGNALED,
    ConditionSatisfied = consts::GL_CONDITION_SATISFIED,
    TimeoutExpired = consts::GL_TIMEOUT_EXPIRED,
    WaitFailed = consts::GL_WAIT_FAILED,
}

impl From<u32> for SyncStatus {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ALREADY_SIGNALED => Self::AlreadySignaled,
            consts::GL_CONDITION_SATISFIED => Self::ConditionSatisfied,
            consts::GL_TIMEOUT_EXPIRED => Self::TimeoutExpired,
            consts::GL_WAIT_FAILED => Self::WaitFailed,
            v => panic!("Unknow value {v} for SyncStatus"),
        }
    }
}

#[repr(u32)]
pub enum TexCoordPointerType {
    Double = consts::GL_DOUBLE,
    Float = consts::GL_FLOAT,
    Int = consts::GL_INT,
    Short = consts::GL_SHORT,
}

impl From<u32> for TexCoordPointerType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_DOUBLE => Self::Double,
            consts::GL_FLOAT => Self::Float,
            consts::GL_INT => Self::Int,
            consts::GL_SHORT => Self::Short,
            v => panic!("Unknow value {v} for TexCoordPointerType"),
        }
    }
}

#[repr(u32)]
pub enum TextureEnvParameter {
    AddSigned = consts::GL_ADD_SIGNED,
    AlphaScale = consts::GL_ALPHA_SCALE,
    Combine = consts::GL_COMBINE,
    CombineAlpha = consts::GL_COMBINE_ALPHA,
    CombineRgb = consts::GL_COMBINE_RGB,
    Constant = consts::GL_CONSTANT,
    Interpolate = consts::GL_INTERPOLATE,
    Operand0Alpha = consts::GL_OPERAND0_ALPHA,
    Operand0Rgb = consts::GL_OPERAND0_RGB,
    Operand1Alpha = consts::GL_OPERAND1_ALPHA,
    Operand1Rgb = consts::GL_OPERAND1_RGB,
    Operand2Alpha = consts::GL_OPERAND2_ALPHA,
    Operand2Rgb = consts::GL_OPERAND2_RGB,
    Previous = consts::GL_PREVIOUS,
    PrimaryColor = consts::GL_PRIMARY_COLOR,
    RgbScale = consts::GL_RGB_SCALE,
    Src0Alpha = consts::GL_SRC0_ALPHA,
    Src0Rgb = consts::GL_SRC0_RGB,
    Src1Alpha = consts::GL_SRC1_ALPHA,
    Src1Rgb = consts::GL_SRC1_RGB,
    Src2Alpha = consts::GL_SRC2_ALPHA,
    Src2Rgb = consts::GL_SRC2_RGB,
    TextureEnvColor = consts::GL_TEXTURE_ENV_COLOR,
    TextureEnvMode = consts::GL_TEXTURE_ENV_MODE,
    TextureLodBias = consts::GL_TEXTURE_LOD_BIAS,
}

impl From<u32> for TextureEnvParameter {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ADD_SIGNED => Self::AddSigned,
            consts::GL_ALPHA_SCALE => Self::AlphaScale,
            consts::GL_COMBINE => Self::Combine,
            consts::GL_COMBINE_ALPHA => Self::CombineAlpha,
            consts::GL_COMBINE_RGB => Self::CombineRgb,
            consts::GL_CONSTANT => Self::Constant,
            consts::GL_INTERPOLATE => Self::Interpolate,
            consts::GL_OPERAND0_ALPHA => Self::Operand0Alpha,
            consts::GL_OPERAND0_RGB => Self::Operand0Rgb,
            consts::GL_OPERAND1_ALPHA => Self::Operand1Alpha,
            consts::GL_OPERAND1_RGB => Self::Operand1Rgb,
            consts::GL_OPERAND2_ALPHA => Self::Operand2Alpha,
            consts::GL_OPERAND2_RGB => Self::Operand2Rgb,
            consts::GL_PREVIOUS => Self::Previous,
            consts::GL_PRIMARY_COLOR => Self::PrimaryColor,
            consts::GL_RGB_SCALE => Self::RgbScale,
            consts::GL_SRC0_ALPHA => Self::Src0Alpha,
            consts::GL_SRC0_RGB => Self::Src0Rgb,
            consts::GL_SRC1_ALPHA => Self::Src1Alpha,
            consts::GL_SRC1_RGB => Self::Src1Rgb,
            consts::GL_SRC2_ALPHA => Self::Src2Alpha,
            consts::GL_SRC2_RGB => Self::Src2Rgb,
            consts::GL_TEXTURE_ENV_COLOR => Self::TextureEnvColor,
            consts::GL_TEXTURE_ENV_MODE => Self::TextureEnvMode,
            consts::GL_TEXTURE_LOD_BIAS => Self::TextureLodBias,
            v => panic!("Unknow value {v} for TextureEnvParameter"),
        }
    }
}

#[repr(u32)]
pub enum TextureEnvTarget {
    TextureEnv = consts::GL_TEXTURE_ENV,
}

impl From<u32> for TextureEnvTarget {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_TEXTURE_ENV => Self::TextureEnv,
            v => panic!("Unknow value {v} for TextureEnvTarget"),
        }
    }
}

#[repr(u32)]
pub enum TextureParameterName {
    DepthStencilTextureMode = consts::GL_DEPTH_STENCIL_TEXTURE_MODE,
    GenerateMipmap = consts::GL_GENERATE_MIPMAP,
    TextureAlphaSize = consts::GL_TEXTURE_ALPHA_SIZE,
    TextureBaseLevel = consts::GL_TEXTURE_BASE_LEVEL,
    TextureBlueSize = consts::GL_TEXTURE_BLUE_SIZE,
    TextureBorderColor = consts::GL_TEXTURE_BORDER_COLOR,
    TextureCompareFunc = consts::GL_TEXTURE_COMPARE_FUNC,
    TextureCompareMode = consts::GL_TEXTURE_COMPARE_MODE,
    TextureGreenSize = consts::GL_TEXTURE_GREEN_SIZE,
    TextureHeight = consts::GL_TEXTURE_HEIGHT,
    TextureInternalFormat = consts::GL_TEXTURE_INTERNAL_FORMAT,
    TextureLodBias = consts::GL_TEXTURE_LOD_BIAS,
    TextureMagFilter = consts::GL_TEXTURE_MAG_FILTER,
    TextureMaxLevel = consts::GL_TEXTURE_MAX_LEVEL,
    TextureMaxLod = consts::GL_TEXTURE_MAX_LOD,
    TextureMinFilter = consts::GL_TEXTURE_MIN_FILTER,
    TextureMinLod = consts::GL_TEXTURE_MIN_LOD,
    TextureRedSize = consts::GL_TEXTURE_RED_SIZE,
    TextureSwizzleA = consts::GL_TEXTURE_SWIZZLE_A,
    TextureSwizzleB = consts::GL_TEXTURE_SWIZZLE_B,
    TextureSwizzleG = consts::GL_TEXTURE_SWIZZLE_G,
    TextureSwizzleR = consts::GL_TEXTURE_SWIZZLE_R,
    TextureSwizzleRgba = consts::GL_TEXTURE_SWIZZLE_RGBA,
    TextureWidth = consts::GL_TEXTURE_WIDTH,
    TextureWrapR = consts::GL_TEXTURE_WRAP_R,
    TextureWrapS = consts::GL_TEXTURE_WRAP_S,
    TextureWrapT = consts::GL_TEXTURE_WRAP_T,
}

impl From<u32> for TextureParameterName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_DEPTH_STENCIL_TEXTURE_MODE => Self::DepthStencilTextureMode,
            consts::GL_GENERATE_MIPMAP => Self::GenerateMipmap,
            consts::GL_TEXTURE_ALPHA_SIZE => Self::TextureAlphaSize,
            consts::GL_TEXTURE_BASE_LEVEL => Self::TextureBaseLevel,
            consts::GL_TEXTURE_BLUE_SIZE => Self::TextureBlueSize,
            consts::GL_TEXTURE_BORDER_COLOR => Self::TextureBorderColor,
            consts::GL_TEXTURE_COMPARE_FUNC => Self::TextureCompareFunc,
            consts::GL_TEXTURE_COMPARE_MODE => Self::TextureCompareMode,
            consts::GL_TEXTURE_GREEN_SIZE => Self::TextureGreenSize,
            consts::GL_TEXTURE_HEIGHT => Self::TextureHeight,
            consts::GL_TEXTURE_INTERNAL_FORMAT => Self::TextureInternalFormat,
            consts::GL_TEXTURE_LOD_BIAS => Self::TextureLodBias,
            consts::GL_TEXTURE_MAG_FILTER => Self::TextureMagFilter,
            consts::GL_TEXTURE_MAX_LEVEL => Self::TextureMaxLevel,
            consts::GL_TEXTURE_MAX_LOD => Self::TextureMaxLod,
            consts::GL_TEXTURE_MIN_FILTER => Self::TextureMinFilter,
            consts::GL_TEXTURE_MIN_LOD => Self::TextureMinLod,
            consts::GL_TEXTURE_RED_SIZE => Self::TextureRedSize,
            consts::GL_TEXTURE_SWIZZLE_A => Self::TextureSwizzleA,
            consts::GL_TEXTURE_SWIZZLE_B => Self::TextureSwizzleB,
            consts::GL_TEXTURE_SWIZZLE_G => Self::TextureSwizzleG,
            consts::GL_TEXTURE_SWIZZLE_R => Self::TextureSwizzleR,
            consts::GL_TEXTURE_SWIZZLE_RGBA => Self::TextureSwizzleRgba,
            consts::GL_TEXTURE_WIDTH => Self::TextureWidth,
            consts::GL_TEXTURE_WRAP_R => Self::TextureWrapR,
            consts::GL_TEXTURE_WRAP_S => Self::TextureWrapS,
            consts::GL_TEXTURE_WRAP_T => Self::TextureWrapT,
            v => panic!("Unknow value {v} for TextureParameterName"),
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TextureTarget {
    ProxyTexture1d = consts::GL_PROXY_TEXTURE_1D,
    ProxyTexture1dArray = consts::GL_PROXY_TEXTURE_1D_ARRAY,
    ProxyTexture2d = consts::GL_PROXY_TEXTURE_2D,
    ProxyTexture2dArray = consts::GL_PROXY_TEXTURE_2D_ARRAY,
    ProxyTexture2dMultisample = consts::GL_PROXY_TEXTURE_2D_MULTISAMPLE,
    ProxyTexture2dMultisampleArray = consts::GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY,
    ProxyTexture3d = consts::GL_PROXY_TEXTURE_3D,
    ProxyTextureCubeMap = consts::GL_PROXY_TEXTURE_CUBE_MAP,
    ProxyTextureRectangle = consts::GL_PROXY_TEXTURE_RECTANGLE,
    Renderbuffer = consts::GL_RENDERBUFFER,
    Texture1d = consts::GL_TEXTURE_1D,
    Texture1dArray = consts::GL_TEXTURE_1D_ARRAY,
    Texture2d = consts::GL_TEXTURE_2D,
    Texture2dArray = consts::GL_TEXTURE_2D_ARRAY,
    Texture2dMultisample = consts::GL_TEXTURE_2D_MULTISAMPLE,
    Texture2dMultisampleArray = consts::GL_TEXTURE_2D_MULTISAMPLE_ARRAY,
    Texture3d = consts::GL_TEXTURE_3D,
    TextureBuffer = consts::GL_TEXTURE_BUFFER,
    TextureCubeMap = consts::GL_TEXTURE_CUBE_MAP,
    TextureCubeMapArray = consts::GL_TEXTURE_CUBE_MAP_ARRAY,
    TextureCubeMapNegativeX = consts::GL_TEXTURE_CUBE_MAP_NEGATIVE_X,
    TextureCubeMapNegativeY = consts::GL_TEXTURE_CUBE_MAP_NEGATIVE_Y,
    TextureCubeMapNegativeZ = consts::GL_TEXTURE_CUBE_MAP_NEGATIVE_Z,
    TextureCubeMapPositiveX = consts::GL_TEXTURE_CUBE_MAP_POSITIVE_X,
    TextureCubeMapPositiveY = consts::GL_TEXTURE_CUBE_MAP_POSITIVE_Y,
    TextureCubeMapPositiveZ = consts::GL_TEXTURE_CUBE_MAP_POSITIVE_Z,
    TextureRectangle = consts::GL_TEXTURE_RECTANGLE,
}

impl From<u32> for TextureTarget {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_PROXY_TEXTURE_1D => Self::ProxyTexture1d,
            consts::GL_PROXY_TEXTURE_1D_ARRAY => Self::ProxyTexture1dArray,
            consts::GL_PROXY_TEXTURE_2D => Self::ProxyTexture2d,
            consts::GL_PROXY_TEXTURE_2D_ARRAY => Self::ProxyTexture2dArray,
            consts::GL_PROXY_TEXTURE_2D_MULTISAMPLE => Self::ProxyTexture2dMultisample,
            consts::GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY => Self::ProxyTexture2dMultisampleArray,
            consts::GL_PROXY_TEXTURE_3D => Self::ProxyTexture3d,
            consts::GL_PROXY_TEXTURE_CUBE_MAP => Self::ProxyTextureCubeMap,
            consts::GL_PROXY_TEXTURE_RECTANGLE => Self::ProxyTextureRectangle,
            consts::GL_RENDERBUFFER => Self::Renderbuffer,
            consts::GL_TEXTURE_1D => Self::Texture1d,
            consts::GL_TEXTURE_1D_ARRAY => Self::Texture1dArray,
            consts::GL_TEXTURE_2D => Self::Texture2d,
            consts::GL_TEXTURE_2D_ARRAY => Self::Texture2dArray,
            consts::GL_TEXTURE_2D_MULTISAMPLE => Self::Texture2dMultisample,
            consts::GL_TEXTURE_2D_MULTISAMPLE_ARRAY => Self::Texture2dMultisampleArray,
            consts::GL_TEXTURE_3D => Self::Texture3d,
            consts::GL_TEXTURE_BUFFER => Self::TextureBuffer,
            consts::GL_TEXTURE_CUBE_MAP => Self::TextureCubeMap,
            consts::GL_TEXTURE_CUBE_MAP_ARRAY => Self::TextureCubeMapArray,
            consts::GL_TEXTURE_CUBE_MAP_NEGATIVE_X => Self::TextureCubeMapNegativeX,
            consts::GL_TEXTURE_CUBE_MAP_NEGATIVE_Y => Self::TextureCubeMapNegativeY,
            consts::GL_TEXTURE_CUBE_MAP_NEGATIVE_Z => Self::TextureCubeMapNegativeZ,
            consts::GL_TEXTURE_CUBE_MAP_POSITIVE_X => Self::TextureCubeMapPositiveX,
            consts::GL_TEXTURE_CUBE_MAP_POSITIVE_Y => Self::TextureCubeMapPositiveY,
            consts::GL_TEXTURE_CUBE_MAP_POSITIVE_Z => Self::TextureCubeMapPositiveZ,
            consts::GL_TEXTURE_RECTANGLE => Self::TextureRectangle,
            v => panic!("Unknow value {v} for TextureTarget"),
        }
    }
}

#[repr(u32)]
pub enum TextureUnit {
    Texture0 = consts::GL_TEXTURE0,
    Texture1 = consts::GL_TEXTURE1,
    Texture10 = consts::GL_TEXTURE10,
    Texture11 = consts::GL_TEXTURE11,
    Texture12 = consts::GL_TEXTURE12,
    Texture13 = consts::GL_TEXTURE13,
    Texture14 = consts::GL_TEXTURE14,
    Texture15 = consts::GL_TEXTURE15,
    Texture16 = consts::GL_TEXTURE16,
    Texture17 = consts::GL_TEXTURE17,
    Texture18 = consts::GL_TEXTURE18,
    Texture19 = consts::GL_TEXTURE19,
    Texture2 = consts::GL_TEXTURE2,
    Texture20 = consts::GL_TEXTURE20,
    Texture21 = consts::GL_TEXTURE21,
    Texture22 = consts::GL_TEXTURE22,
    Texture23 = consts::GL_TEXTURE23,
    Texture24 = consts::GL_TEXTURE24,
    Texture25 = consts::GL_TEXTURE25,
    Texture26 = consts::GL_TEXTURE26,
    Texture27 = consts::GL_TEXTURE27,
    Texture28 = consts::GL_TEXTURE28,
    Texture29 = consts::GL_TEXTURE29,
    Texture3 = consts::GL_TEXTURE3,
    Texture30 = consts::GL_TEXTURE30,
    Texture31 = consts::GL_TEXTURE31,
    Texture4 = consts::GL_TEXTURE4,
    Texture5 = consts::GL_TEXTURE5,
    Texture6 = consts::GL_TEXTURE6,
    Texture7 = consts::GL_TEXTURE7,
    Texture8 = consts::GL_TEXTURE8,
    Texture9 = consts::GL_TEXTURE9,
}

impl From<u32> for TextureUnit {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_TEXTURE0 => Self::Texture0,
            consts::GL_TEXTURE1 => Self::Texture1,
            consts::GL_TEXTURE10 => Self::Texture10,
            consts::GL_TEXTURE11 => Self::Texture11,
            consts::GL_TEXTURE12 => Self::Texture12,
            consts::GL_TEXTURE13 => Self::Texture13,
            consts::GL_TEXTURE14 => Self::Texture14,
            consts::GL_TEXTURE15 => Self::Texture15,
            consts::GL_TEXTURE16 => Self::Texture16,
            consts::GL_TEXTURE17 => Self::Texture17,
            consts::GL_TEXTURE18 => Self::Texture18,
            consts::GL_TEXTURE19 => Self::Texture19,
            consts::GL_TEXTURE2 => Self::Texture2,
            consts::GL_TEXTURE20 => Self::Texture20,
            consts::GL_TEXTURE21 => Self::Texture21,
            consts::GL_TEXTURE22 => Self::Texture22,
            consts::GL_TEXTURE23 => Self::Texture23,
            consts::GL_TEXTURE24 => Self::Texture24,
            consts::GL_TEXTURE25 => Self::Texture25,
            consts::GL_TEXTURE26 => Self::Texture26,
            consts::GL_TEXTURE27 => Self::Texture27,
            consts::GL_TEXTURE28 => Self::Texture28,
            consts::GL_TEXTURE29 => Self::Texture29,
            consts::GL_TEXTURE3 => Self::Texture3,
            consts::GL_TEXTURE30 => Self::Texture30,
            consts::GL_TEXTURE31 => Self::Texture31,
            consts::GL_TEXTURE4 => Self::Texture4,
            consts::GL_TEXTURE5 => Self::Texture5,
            consts::GL_TEXTURE6 => Self::Texture6,
            consts::GL_TEXTURE7 => Self::Texture7,
            consts::GL_TEXTURE8 => Self::Texture8,
            consts::GL_TEXTURE9 => Self::Texture9,
            v => panic!("Unknow value {v} for TextureUnit"),
        }
    }
}

#[repr(u32)]
pub enum TransformFeedbackBufferMode {
    InterleavedAttribs = consts::GL_INTERLEAVED_ATTRIBS,
    SeparateAttribs = consts::GL_SEPARATE_ATTRIBS,
}

impl From<u32> for TransformFeedbackBufferMode {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_INTERLEAVED_ATTRIBS => Self::InterleavedAttribs,
            consts::GL_SEPARATE_ATTRIBS => Self::SeparateAttribs,
            v => panic!("Unknow value {v} for TransformFeedbackBufferMode"),
        }
    }
}

#[repr(u32)]
pub enum TriangleFace {
    Back = consts::GL_BACK,
    Front = consts::GL_FRONT,
    FrontAndBack = consts::GL_FRONT_AND_BACK,
}

impl From<u32> for TriangleFace {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BACK => Self::Back,
            consts::GL_FRONT => Self::Front,
            consts::GL_FRONT_AND_BACK => Self::FrontAndBack,
            v => panic!("Unknow value {v} for TriangleFace"),
        }
    }
}

#[repr(u32)]
pub enum UniformBlockPName {
    UniformBlockActiveUniforms = consts::GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS,
    UniformBlockActiveUniformIndices = consts::GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES,
    UniformBlockBinding = consts::GL_UNIFORM_BLOCK_BINDING,
    UniformBlockDataSize = consts::GL_UNIFORM_BLOCK_DATA_SIZE,
    UniformBlockNameLength = consts::GL_UNIFORM_BLOCK_NAME_LENGTH,
    UniformBlockReferencedByFragmentShader = consts::GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER,
    UniformBlockReferencedByGeometryShader = consts::GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER,
    UniformBlockReferencedByVertexShader = consts::GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER,
}

impl From<u32> for UniformBlockPName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS => Self::UniformBlockActiveUniforms,
            consts::GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES => {
                Self::UniformBlockActiveUniformIndices
            }
            consts::GL_UNIFORM_BLOCK_BINDING => Self::UniformBlockBinding,
            consts::GL_UNIFORM_BLOCK_DATA_SIZE => Self::UniformBlockDataSize,
            consts::GL_UNIFORM_BLOCK_NAME_LENGTH => Self::UniformBlockNameLength,
            consts::GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER => {
                Self::UniformBlockReferencedByFragmentShader
            }
            consts::GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER => {
                Self::UniformBlockReferencedByGeometryShader
            }
            consts::GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER => {
                Self::UniformBlockReferencedByVertexShader
            }
            v => panic!("Unknow value {v} for UniformBlockPName"),
        }
    }
}

#[repr(u32)]
pub enum UniformPName {
    UniformArrayStride = consts::GL_UNIFORM_ARRAY_STRIDE,
    UniformBlockIndex = consts::GL_UNIFORM_BLOCK_INDEX,
    UniformIsRowMajor = consts::GL_UNIFORM_IS_ROW_MAJOR,
    UniformMatrixStride = consts::GL_UNIFORM_MATRIX_STRIDE,
    UniformNameLength = consts::GL_UNIFORM_NAME_LENGTH,
    UniformOffset = consts::GL_UNIFORM_OFFSET,
    UniformSize = consts::GL_UNIFORM_SIZE,
    UniformType = consts::GL_UNIFORM_TYPE,
}

impl From<u32> for UniformPName {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_UNIFORM_ARRAY_STRIDE => Self::UniformArrayStride,
            consts::GL_UNIFORM_BLOCK_INDEX => Self::UniformBlockIndex,
            consts::GL_UNIFORM_IS_ROW_MAJOR => Self::UniformIsRowMajor,
            consts::GL_UNIFORM_MATRIX_STRIDE => Self::UniformMatrixStride,
            consts::GL_UNIFORM_NAME_LENGTH => Self::UniformNameLength,
            consts::GL_UNIFORM_OFFSET => Self::UniformOffset,
            consts::GL_UNIFORM_SIZE => Self::UniformSize,
            consts::GL_UNIFORM_TYPE => Self::UniformType,
            v => panic!("Unknow value {v} for UniformPName"),
        }
    }
}

#[repr(u32)]
pub enum UniformType {
    Bool = consts::GL_BOOL,
    BoolVec2 = consts::GL_BOOL_VEC2,
    BoolVec3 = consts::GL_BOOL_VEC3,
    BoolVec4 = consts::GL_BOOL_VEC4,
    Double = consts::GL_DOUBLE,
    Float = consts::GL_FLOAT,
    FloatMat2 = consts::GL_FLOAT_MAT2,
    FloatMat2x3 = consts::GL_FLOAT_MAT2x3,
    FloatMat2x4 = consts::GL_FLOAT_MAT2x4,
    FloatMat3 = consts::GL_FLOAT_MAT3,
    FloatMat3x2 = consts::GL_FLOAT_MAT3x2,
    FloatMat3x4 = consts::GL_FLOAT_MAT3x4,
    FloatMat4 = consts::GL_FLOAT_MAT4,
    FloatMat4x2 = consts::GL_FLOAT_MAT4x2,
    FloatMat4x3 = consts::GL_FLOAT_MAT4x3,
    FloatVec2 = consts::GL_FLOAT_VEC2,
    FloatVec3 = consts::GL_FLOAT_VEC3,
    FloatVec4 = consts::GL_FLOAT_VEC4,
    Int = consts::GL_INT,
    IntSampler1d = consts::GL_INT_SAMPLER_1D,
    IntSampler1dArray = consts::GL_INT_SAMPLER_1D_ARRAY,
    IntSampler2d = consts::GL_INT_SAMPLER_2D,
    IntSampler2dArray = consts::GL_INT_SAMPLER_2D_ARRAY,
    IntSampler2dMultisample = consts::GL_INT_SAMPLER_2D_MULTISAMPLE,
    IntSampler2dMultisampleArray = consts::GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY,
    IntSampler2dRect = consts::GL_INT_SAMPLER_2D_RECT,
    IntSampler3d = consts::GL_INT_SAMPLER_3D,
    IntSamplerBuffer = consts::GL_INT_SAMPLER_BUFFER,
    IntSamplerCube = consts::GL_INT_SAMPLER_CUBE,
    IntSamplerCubeMapArray = consts::GL_INT_SAMPLER_CUBE_MAP_ARRAY,
    IntVec2 = consts::GL_INT_VEC2,
    IntVec3 = consts::GL_INT_VEC3,
    IntVec4 = consts::GL_INT_VEC4,
    Sampler1d = consts::GL_SAMPLER_1D,
    Sampler1dArray = consts::GL_SAMPLER_1D_ARRAY,
    Sampler1dArrayShadow = consts::GL_SAMPLER_1D_ARRAY_SHADOW,
    Sampler1dShadow = consts::GL_SAMPLER_1D_SHADOW,
    Sampler2d = consts::GL_SAMPLER_2D,
    Sampler2dArray = consts::GL_SAMPLER_2D_ARRAY,
    Sampler2dArrayShadow = consts::GL_SAMPLER_2D_ARRAY_SHADOW,
    Sampler2dMultisample = consts::GL_SAMPLER_2D_MULTISAMPLE,
    Sampler2dMultisampleArray = consts::GL_SAMPLER_2D_MULTISAMPLE_ARRAY,
    Sampler2dRect = consts::GL_SAMPLER_2D_RECT,
    Sampler2dRectShadow = consts::GL_SAMPLER_2D_RECT_SHADOW,
    Sampler2dShadow = consts::GL_SAMPLER_2D_SHADOW,
    Sampler3d = consts::GL_SAMPLER_3D,
    SamplerBuffer = consts::GL_SAMPLER_BUFFER,
    SamplerCube = consts::GL_SAMPLER_CUBE,
    SamplerCubeMapArray = consts::GL_SAMPLER_CUBE_MAP_ARRAY,
    SamplerCubeMapArrayShadow = consts::GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW,
    SamplerCubeShadow = consts::GL_SAMPLER_CUBE_SHADOW,
    UnsignedInt = consts::GL_UNSIGNED_INT,
    UnsignedIntSampler1d = consts::GL_UNSIGNED_INT_SAMPLER_1D,
    UnsignedIntSampler1dArray = consts::GL_UNSIGNED_INT_SAMPLER_1D_ARRAY,
    UnsignedIntSampler2d = consts::GL_UNSIGNED_INT_SAMPLER_2D,
    UnsignedIntSampler2dArray = consts::GL_UNSIGNED_INT_SAMPLER_2D_ARRAY,
    UnsignedIntSampler2dMultisample = consts::GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE,
    UnsignedIntSampler2dMultisampleArray = consts::GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY,
    UnsignedIntSampler2dRect = consts::GL_UNSIGNED_INT_SAMPLER_2D_RECT,
    UnsignedIntSampler3d = consts::GL_UNSIGNED_INT_SAMPLER_3D,
    UnsignedIntSamplerBuffer = consts::GL_UNSIGNED_INT_SAMPLER_BUFFER,
    UnsignedIntSamplerCube = consts::GL_UNSIGNED_INT_SAMPLER_CUBE,
    UnsignedIntSamplerCubeMapArray = consts::GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY,
    UnsignedIntVec2 = consts::GL_UNSIGNED_INT_VEC2,
    UnsignedIntVec3 = consts::GL_UNSIGNED_INT_VEC3,
    UnsignedIntVec4 = consts::GL_UNSIGNED_INT_VEC4,
}

impl From<u32> for UniformType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BOOL => Self::Bool,
            consts::GL_BOOL_VEC2 => Self::BoolVec2,
            consts::GL_BOOL_VEC3 => Self::BoolVec3,
            consts::GL_BOOL_VEC4 => Self::BoolVec4,
            consts::GL_DOUBLE => Self::Double,
            consts::GL_FLOAT => Self::Float,
            consts::GL_FLOAT_MAT2 => Self::FloatMat2,
            consts::GL_FLOAT_MAT2x3 => Self::FloatMat2x3,
            consts::GL_FLOAT_MAT2x4 => Self::FloatMat2x4,
            consts::GL_FLOAT_MAT3 => Self::FloatMat3,
            consts::GL_FLOAT_MAT3x2 => Self::FloatMat3x2,
            consts::GL_FLOAT_MAT3x4 => Self::FloatMat3x4,
            consts::GL_FLOAT_MAT4 => Self::FloatMat4,
            consts::GL_FLOAT_MAT4x2 => Self::FloatMat4x2,
            consts::GL_FLOAT_MAT4x3 => Self::FloatMat4x3,
            consts::GL_FLOAT_VEC2 => Self::FloatVec2,
            consts::GL_FLOAT_VEC3 => Self::FloatVec3,
            consts::GL_FLOAT_VEC4 => Self::FloatVec4,
            consts::GL_INT => Self::Int,
            consts::GL_INT_SAMPLER_1D => Self::IntSampler1d,
            consts::GL_INT_SAMPLER_1D_ARRAY => Self::IntSampler1dArray,
            consts::GL_INT_SAMPLER_2D => Self::IntSampler2d,
            consts::GL_INT_SAMPLER_2D_ARRAY => Self::IntSampler2dArray,
            consts::GL_INT_SAMPLER_2D_MULTISAMPLE => Self::IntSampler2dMultisample,
            consts::GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY => Self::IntSampler2dMultisampleArray,
            consts::GL_INT_SAMPLER_2D_RECT => Self::IntSampler2dRect,
            consts::GL_INT_SAMPLER_3D => Self::IntSampler3d,
            consts::GL_INT_SAMPLER_BUFFER => Self::IntSamplerBuffer,
            consts::GL_INT_SAMPLER_CUBE => Self::IntSamplerCube,
            consts::GL_INT_SAMPLER_CUBE_MAP_ARRAY => Self::IntSamplerCubeMapArray,
            consts::GL_INT_VEC2 => Self::IntVec2,
            consts::GL_INT_VEC3 => Self::IntVec3,
            consts::GL_INT_VEC4 => Self::IntVec4,
            consts::GL_SAMPLER_1D => Self::Sampler1d,
            consts::GL_SAMPLER_1D_ARRAY => Self::Sampler1dArray,
            consts::GL_SAMPLER_1D_ARRAY_SHADOW => Self::Sampler1dArrayShadow,
            consts::GL_SAMPLER_1D_SHADOW => Self::Sampler1dShadow,
            consts::GL_SAMPLER_2D => Self::Sampler2d,
            consts::GL_SAMPLER_2D_ARRAY => Self::Sampler2dArray,
            consts::GL_SAMPLER_2D_ARRAY_SHADOW => Self::Sampler2dArrayShadow,
            consts::GL_SAMPLER_2D_MULTISAMPLE => Self::Sampler2dMultisample,
            consts::GL_SAMPLER_2D_MULTISAMPLE_ARRAY => Self::Sampler2dMultisampleArray,
            consts::GL_SAMPLER_2D_RECT => Self::Sampler2dRect,
            consts::GL_SAMPLER_2D_RECT_SHADOW => Self::Sampler2dRectShadow,
            consts::GL_SAMPLER_2D_SHADOW => Self::Sampler2dShadow,
            consts::GL_SAMPLER_3D => Self::Sampler3d,
            consts::GL_SAMPLER_BUFFER => Self::SamplerBuffer,
            consts::GL_SAMPLER_CUBE => Self::SamplerCube,
            consts::GL_SAMPLER_CUBE_MAP_ARRAY => Self::SamplerCubeMapArray,
            consts::GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW => Self::SamplerCubeMapArrayShadow,
            consts::GL_SAMPLER_CUBE_SHADOW => Self::SamplerCubeShadow,
            consts::GL_UNSIGNED_INT => Self::UnsignedInt,
            consts::GL_UNSIGNED_INT_SAMPLER_1D => Self::UnsignedIntSampler1d,
            consts::GL_UNSIGNED_INT_SAMPLER_1D_ARRAY => Self::UnsignedIntSampler1dArray,
            consts::GL_UNSIGNED_INT_SAMPLER_2D => Self::UnsignedIntSampler2d,
            consts::GL_UNSIGNED_INT_SAMPLER_2D_ARRAY => Self::UnsignedIntSampler2dArray,
            consts::GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE => Self::UnsignedIntSampler2dMultisample,
            consts::GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY => {
                Self::UnsignedIntSampler2dMultisampleArray
            }
            consts::GL_UNSIGNED_INT_SAMPLER_2D_RECT => Self::UnsignedIntSampler2dRect,
            consts::GL_UNSIGNED_INT_SAMPLER_3D => Self::UnsignedIntSampler3d,
            consts::GL_UNSIGNED_INT_SAMPLER_BUFFER => Self::UnsignedIntSamplerBuffer,
            consts::GL_UNSIGNED_INT_SAMPLER_CUBE => Self::UnsignedIntSamplerCube,
            consts::GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY => Self::UnsignedIntSamplerCubeMapArray,
            consts::GL_UNSIGNED_INT_VEC2 => Self::UnsignedIntVec2,
            consts::GL_UNSIGNED_INT_VEC3 => Self::UnsignedIntVec3,
            consts::GL_UNSIGNED_INT_VEC4 => Self::UnsignedIntVec4,
            v => panic!("Unknow value {v} for UniformType"),
        }
    }
}

#[repr(u32)]
pub enum UseProgramStageMask {
    AllShaderBits = consts::GL_ALL_SHADER_BITS,
    ComputeShaderBit = consts::GL_COMPUTE_SHADER_BIT,
    FragmentShaderBit = consts::GL_FRAGMENT_SHADER_BIT,
    GeometryShaderBit = consts::GL_GEOMETRY_SHADER_BIT,
    TessControlShaderBit = consts::GL_TESS_CONTROL_SHADER_BIT,
    TessEvaluationShaderBit = consts::GL_TESS_EVALUATION_SHADER_BIT,
    VertexShaderBit = consts::GL_VERTEX_SHADER_BIT,
}

impl From<u32> for UseProgramStageMask {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_ALL_SHADER_BITS => Self::AllShaderBits,
            consts::GL_COMPUTE_SHADER_BIT => Self::ComputeShaderBit,
            consts::GL_FRAGMENT_SHADER_BIT => Self::FragmentShaderBit,
            consts::GL_GEOMETRY_SHADER_BIT => Self::GeometryShaderBit,
            consts::GL_TESS_CONTROL_SHADER_BIT => Self::TessControlShaderBit,
            consts::GL_TESS_EVALUATION_SHADER_BIT => Self::TessEvaluationShaderBit,
            consts::GL_VERTEX_SHADER_BIT => Self::VertexShaderBit,
            v => panic!("Unknow value {v} for UseProgramStageMask"),
        }
    }
}

#[repr(u32)]
pub enum VertexAttribEnum {
    CurrentVertexAttrib = consts::GL_CURRENT_VERTEX_ATTRIB,
    VertexAttribArrayBufferBinding = consts::GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING,
    VertexAttribArrayDivisor = consts::GL_VERTEX_ATTRIB_ARRAY_DIVISOR,
    VertexAttribArrayEnabled = consts::GL_VERTEX_ATTRIB_ARRAY_ENABLED,
    VertexAttribArrayInteger = consts::GL_VERTEX_ATTRIB_ARRAY_INTEGER,
    VertexAttribArrayNormalized = consts::GL_VERTEX_ATTRIB_ARRAY_NORMALIZED,
    VertexAttribArraySize = consts::GL_VERTEX_ATTRIB_ARRAY_SIZE,
    VertexAttribArrayStride = consts::GL_VERTEX_ATTRIB_ARRAY_STRIDE,
    VertexAttribArrayType = consts::GL_VERTEX_ATTRIB_ARRAY_TYPE,
}

impl From<u32> for VertexAttribEnum {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_CURRENT_VERTEX_ATTRIB => Self::CurrentVertexAttrib,
            consts::GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING => Self::VertexAttribArrayBufferBinding,
            consts::GL_VERTEX_ATTRIB_ARRAY_DIVISOR => Self::VertexAttribArrayDivisor,
            consts::GL_VERTEX_ATTRIB_ARRAY_ENABLED => Self::VertexAttribArrayEnabled,
            consts::GL_VERTEX_ATTRIB_ARRAY_INTEGER => Self::VertexAttribArrayInteger,
            consts::GL_VERTEX_ATTRIB_ARRAY_NORMALIZED => Self::VertexAttribArrayNormalized,
            consts::GL_VERTEX_ATTRIB_ARRAY_SIZE => Self::VertexAttribArraySize,
            consts::GL_VERTEX_ATTRIB_ARRAY_STRIDE => Self::VertexAttribArrayStride,
            consts::GL_VERTEX_ATTRIB_ARRAY_TYPE => Self::VertexAttribArrayType,
            v => panic!("Unknow value {v} for VertexAttribEnum"),
        }
    }
}

#[repr(u32)]
pub enum VertexAttribIType {
    Byte = consts::GL_BYTE,
    Int = consts::GL_INT,
    Short = consts::GL_SHORT,
    UnsignedByte = consts::GL_UNSIGNED_BYTE,
    UnsignedInt = consts::GL_UNSIGNED_INT,
    UnsignedShort = consts::GL_UNSIGNED_SHORT,
}

impl From<u32> for VertexAttribIType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BYTE => Self::Byte,
            consts::GL_INT => Self::Int,
            consts::GL_SHORT => Self::Short,
            consts::GL_UNSIGNED_BYTE => Self::UnsignedByte,
            consts::GL_UNSIGNED_INT => Self::UnsignedInt,
            consts::GL_UNSIGNED_SHORT => Self::UnsignedShort,
            v => panic!("Unknow value {v} for VertexAttribIType"),
        }
    }
}

#[repr(u32)]
pub enum VertexAttribPointerPropertyARB {
    VertexAttribArrayPointer = consts::GL_VERTEX_ATTRIB_ARRAY_POINTER,
}

impl From<u32> for VertexAttribPointerPropertyARB {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_VERTEX_ATTRIB_ARRAY_POINTER => Self::VertexAttribArrayPointer,
            v => panic!("Unknow value {v} for VertexAttribPointerPropertyARB"),
        }
    }
}

#[repr(u32)]
pub enum VertexAttribPointerType {
    Byte = consts::GL_BYTE,
    Double = consts::GL_DOUBLE,
    Fixed = consts::GL_FIXED,
    Float = consts::GL_FLOAT,
    HalfFloat = consts::GL_HALF_FLOAT,
    Int = consts::GL_INT,
    Int2101010Rev = consts::GL_INT_2_10_10_10_REV,
    Short = consts::GL_SHORT,
    UnsignedByte = consts::GL_UNSIGNED_BYTE,
    UnsignedInt = consts::GL_UNSIGNED_INT,
    UnsignedInt10f11f11fRev = consts::GL_UNSIGNED_INT_10F_11F_11F_REV,
    UnsignedInt2101010Rev = consts::GL_UNSIGNED_INT_2_10_10_10_REV,
    UnsignedShort = consts::GL_UNSIGNED_SHORT,
}

impl From<u32> for VertexAttribPointerType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BYTE => Self::Byte,
            consts::GL_DOUBLE => Self::Double,
            consts::GL_FIXED => Self::Fixed,
            consts::GL_FLOAT => Self::Float,
            consts::GL_HALF_FLOAT => Self::HalfFloat,
            consts::GL_INT => Self::Int,
            consts::GL_INT_2_10_10_10_REV => Self::Int2101010Rev,
            consts::GL_SHORT => Self::Short,
            consts::GL_UNSIGNED_BYTE => Self::UnsignedByte,
            consts::GL_UNSIGNED_INT => Self::UnsignedInt,
            consts::GL_UNSIGNED_INT_10F_11F_11F_REV => Self::UnsignedInt10f11f11fRev,
            consts::GL_UNSIGNED_INT_2_10_10_10_REV => Self::UnsignedInt2101010Rev,
            consts::GL_UNSIGNED_SHORT => Self::UnsignedShort,
            v => panic!("Unknow value {v} for VertexAttribPointerType"),
        }
    }
}

#[repr(u32)]
pub enum VertexAttribPropertyARB {
    CurrentVertexAttrib = consts::GL_CURRENT_VERTEX_ATTRIB,
    VertexAttribArrayBufferBinding = consts::GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING,
    VertexAttribArrayDivisor = consts::GL_VERTEX_ATTRIB_ARRAY_DIVISOR,
    VertexAttribArrayEnabled = consts::GL_VERTEX_ATTRIB_ARRAY_ENABLED,
    VertexAttribArrayInteger = consts::GL_VERTEX_ATTRIB_ARRAY_INTEGER,
    VertexAttribArrayNormalized = consts::GL_VERTEX_ATTRIB_ARRAY_NORMALIZED,
    VertexAttribArraySize = consts::GL_VERTEX_ATTRIB_ARRAY_SIZE,
    VertexAttribArrayStride = consts::GL_VERTEX_ATTRIB_ARRAY_STRIDE,
    VertexAttribArrayType = consts::GL_VERTEX_ATTRIB_ARRAY_TYPE,
    VertexAttribBinding = consts::GL_VERTEX_ATTRIB_BINDING,
    VertexAttribRelativeOffset = consts::GL_VERTEX_ATTRIB_RELATIVE_OFFSET,
}

impl From<u32> for VertexAttribPropertyARB {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_CURRENT_VERTEX_ATTRIB => Self::CurrentVertexAttrib,
            consts::GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING => Self::VertexAttribArrayBufferBinding,
            consts::GL_VERTEX_ATTRIB_ARRAY_DIVISOR => Self::VertexAttribArrayDivisor,
            consts::GL_VERTEX_ATTRIB_ARRAY_ENABLED => Self::VertexAttribArrayEnabled,
            consts::GL_VERTEX_ATTRIB_ARRAY_INTEGER => Self::VertexAttribArrayInteger,
            consts::GL_VERTEX_ATTRIB_ARRAY_NORMALIZED => Self::VertexAttribArrayNormalized,
            consts::GL_VERTEX_ATTRIB_ARRAY_SIZE => Self::VertexAttribArraySize,
            consts::GL_VERTEX_ATTRIB_ARRAY_STRIDE => Self::VertexAttribArrayStride,
            consts::GL_VERTEX_ATTRIB_ARRAY_TYPE => Self::VertexAttribArrayType,
            consts::GL_VERTEX_ATTRIB_BINDING => Self::VertexAttribBinding,
            consts::GL_VERTEX_ATTRIB_RELATIVE_OFFSET => Self::VertexAttribRelativeOffset,
            v => panic!("Unknow value {v} for VertexAttribPropertyARB"),
        }
    }
}

#[repr(u32)]
pub enum VertexAttribType {
    Byte = consts::GL_BYTE,
    Double = consts::GL_DOUBLE,
    Fixed = consts::GL_FIXED,
    Float = consts::GL_FLOAT,
    HalfFloat = consts::GL_HALF_FLOAT,
    Int = consts::GL_INT,
    Int2101010Rev = consts::GL_INT_2_10_10_10_REV,
    Short = consts::GL_SHORT,
    UnsignedByte = consts::GL_UNSIGNED_BYTE,
    UnsignedInt = consts::GL_UNSIGNED_INT,
    UnsignedInt10f11f11fRev = consts::GL_UNSIGNED_INT_10F_11F_11F_REV,
    UnsignedInt2101010Rev = consts::GL_UNSIGNED_INT_2_10_10_10_REV,
    UnsignedShort = consts::GL_UNSIGNED_SHORT,
}

impl From<u32> for VertexAttribType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_BYTE => Self::Byte,
            consts::GL_DOUBLE => Self::Double,
            consts::GL_FIXED => Self::Fixed,
            consts::GL_FLOAT => Self::Float,
            consts::GL_HALF_FLOAT => Self::HalfFloat,
            consts::GL_INT => Self::Int,
            consts::GL_INT_2_10_10_10_REV => Self::Int2101010Rev,
            consts::GL_SHORT => Self::Short,
            consts::GL_UNSIGNED_BYTE => Self::UnsignedByte,
            consts::GL_UNSIGNED_INT => Self::UnsignedInt,
            consts::GL_UNSIGNED_INT_10F_11F_11F_REV => Self::UnsignedInt10f11f11fRev,
            consts::GL_UNSIGNED_INT_2_10_10_10_REV => Self::UnsignedInt2101010Rev,
            consts::GL_UNSIGNED_SHORT => Self::UnsignedShort,
            v => panic!("Unknow value {v} for VertexAttribType"),
        }
    }
}

#[repr(u32)]
pub enum VertexPointerType {
    Double = consts::GL_DOUBLE,
    Float = consts::GL_FLOAT,
    Int = consts::GL_INT,
    Short = consts::GL_SHORT,
}

impl From<u32> for VertexPointerType {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_DOUBLE => Self::Double,
            consts::GL_FLOAT => Self::Float,
            consts::GL_INT => Self::Int,
            consts::GL_SHORT => Self::Short,
            v => panic!("Unknow value {v} for VertexPointerType"),
        }
    }
}

#[repr(u32)]
pub enum VertexProvokingMode {
    FirstVertexConvention = consts::GL_FIRST_VERTEX_CONVENTION,
    LastVertexConvention = consts::GL_LAST_VERTEX_CONVENTION,
}

impl From<u32> for VertexProvokingMode {
    fn from(value: u32) -> Self {
        match value {
            consts::GL_FIRST_VERTEX_CONVENTION => Self::FirstVertexConvention,
            consts::GL_LAST_VERTEX_CONVENTION => Self::LastVertexConvention,
            v => panic!("Unknow value {v} for VertexProvokingMode"),
        }
    }
}
