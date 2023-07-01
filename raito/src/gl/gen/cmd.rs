use super::{enums, load};
use std::ffi::{c_char, c_double, c_float, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

type DebugProc = Option<
    extern "system" fn(
        source: c_uint,
        gltype: c_uint,
        id: c_uint,
        severity: c_uint,
        length: isize,
        message: *const c_char,
        userParam: *mut c_void,
    ),
>;

type ActiveShaderProgramT = extern "system" fn(c_uint, c_uint);
type ActiveTextureT = extern "system" fn(c_uint);
type AlphaFuncT = extern "system" fn(c_uint, c_float);
type AlphaFuncxT = extern "system" fn(c_uint, c_int);
type AttachShaderT = extern "system" fn(c_uint, c_uint);
type BeginConditionalRenderT = extern "system" fn(c_uint, c_uint);
type BeginQueryT = extern "system" fn(c_uint, c_uint);
type BeginTransformFeedbackT = extern "system" fn(c_uint);
type BindAttribLocationT = extern "system" fn(c_uint, c_uint, *const c_char);
type BindBufferT = extern "system" fn(c_uint, c_uint);
type BindBufferBaseT = extern "system" fn(c_uint, c_uint, c_uint);
type BindBufferRangeT = extern "system" fn(c_uint, c_uint, c_uint, isize, isize);
type BindFragDataLocationT = extern "system" fn(c_uint, c_uint, *const c_char);
type BindFragDataLocationIndexedT = extern "system" fn(c_uint, c_uint, c_uint, *const c_char);
type BindFramebufferT = extern "system" fn(c_uint, c_uint);
type BindImageTextureT = extern "system" fn(c_uint, c_uint, c_int, c_uchar, c_int, c_uint, c_uint);
type BindProgramPipelineT = extern "system" fn(c_uint);
type BindRenderbufferT = extern "system" fn(c_uint, c_uint);
type BindSamplerT = extern "system" fn(c_uint, c_uint);
type BindTextureT = extern "system" fn(c_uint, c_uint);
type BindTransformFeedbackT = extern "system" fn(c_uint, c_uint);
type BindVertexArrayT = extern "system" fn(c_uint);
type BindVertexBufferT = extern "system" fn(c_uint, c_uint, isize, c_int);
type BlendBarrierT = extern "system" fn();
type BlendColorT = extern "system" fn(c_float, c_float, c_float, c_float);
type BlendEquationT = extern "system" fn(c_uint);
type BlendEquationSeparateT = extern "system" fn(c_uint, c_uint);
type BlendEquationSeparateiT = extern "system" fn(c_uint, c_uint, c_uint);
type BlendEquationiT = extern "system" fn(c_uint, c_uint);
type BlendFuncT = extern "system" fn(c_uint, c_uint);
type BlendFuncSeparateT = extern "system" fn(c_uint, c_uint, c_uint, c_uint);
type BlendFuncSeparateiT = extern "system" fn(c_uint, c_uint, c_uint, c_uint, c_uint);
type BlendFunciT = extern "system" fn(c_uint, c_uint, c_uint);
type BlitFramebufferT =
    extern "system" fn(c_int, c_int, c_int, c_int, c_int, c_int, c_int, c_int, c_uint, c_uint);
type BufferDataT = extern "system" fn(c_uint, isize, *const c_void, c_uint);
type BufferSubDataT = extern "system" fn(c_uint, isize, isize, *const c_void);
type CheckFramebufferStatusT = extern "system" fn(c_uint) -> c_uint;
type ClampColorT = extern "system" fn(c_uint, c_uint);
type ClearT = extern "system" fn(c_uint);
type ClearBufferfiT = extern "system" fn(c_uint, c_int, c_float, c_int);
type ClearBufferfvT = extern "system" fn(c_uint, c_int, *const c_float);
type ClearBufferivT = extern "system" fn(c_uint, c_int, *const c_int);
type ClearBufferuivT = extern "system" fn(c_uint, c_int, *const c_uint);
type ClearColorT = extern "system" fn(c_float, c_float, c_float, c_float);
type ClearColorxT = extern "system" fn(c_int, c_int, c_int, c_int);
type ClearDepthT = extern "system" fn(c_double);
type ClearDepthfT = extern "system" fn(c_float);
type ClearDepthxT = extern "system" fn(c_int);
type ClearStencilT = extern "system" fn(c_int);
type ClientActiveTextureT = extern "system" fn(c_uint);
type ClientWaitSyncT = extern "system" fn(*mut c_void, c_uint, u64) -> c_uint;
type ClipPlanefT = extern "system" fn(c_uint, *const c_float);
type ClipPlanexT = extern "system" fn(c_uint, *const c_int);
type Color4fT = extern "system" fn(c_float, c_float, c_float, c_float);
type Color4ubT = extern "system" fn(c_uchar, c_uchar, c_uchar, c_uchar);
type Color4xT = extern "system" fn(c_int, c_int, c_int, c_int);
type ColorMaskT = extern "system" fn(c_uchar, c_uchar, c_uchar, c_uchar);
type ColorMaskiT = extern "system" fn(c_uint, c_uchar, c_uchar, c_uchar, c_uchar);
type ColorP3uiT = extern "system" fn(c_uint, c_uint);
type ColorP3uivT = extern "system" fn(c_uint, *const c_uint);
type ColorP4uiT = extern "system" fn(c_uint, c_uint);
type ColorP4uivT = extern "system" fn(c_uint, *const c_uint);
type ColorPointerT = extern "system" fn(c_int, c_uint, c_int, *const c_void);
type CompileShaderT = extern "system" fn(c_uint);
type CompressedTexImage1DT =
    extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int, *const c_void);
type CompressedTexImage2DT =
    extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int, c_int, *const c_void);
type CompressedTexImage3DT =
    extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int, c_int, c_int, *const c_void);
type CompressedTexSubImage1DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_uint, c_int, *const c_void);
type CompressedTexSubImage2DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_int, c_uint, c_int, *const c_void);
type CompressedTexSubImage3DT = extern "system" fn(
    c_uint,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_uint,
    c_int,
    *const c_void,
);
type CopyBufferSubDataT = extern "system" fn(c_uint, c_uint, isize, isize, isize);
type CopyImageSubDataT = extern "system" fn(
    c_uint,
    c_uint,
    c_int,
    c_int,
    c_int,
    c_int,
    c_uint,
    c_uint,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
);
type CopyTexImage1DT = extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int, c_int);
type CopyTexImage2DT = extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int, c_int, c_int);
type CopyTexSubImage1DT = extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_int);
type CopyTexSubImage2DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_int, c_int, c_int);
type CopyTexSubImage3DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_int, c_int, c_int, c_int);
type CreateProgramT = extern "system" fn() -> c_uint;
type CreateShaderT = extern "system" fn(c_uint) -> c_uint;
type CreateShaderProgramvT = extern "system" fn(c_uint, c_int, *const *const c_char) -> c_uint;
type CullFaceT = extern "system" fn(c_uint);
type DebugMessageCallbackT = extern "system" fn(DebugProc, *const c_void);
type DebugMessageControlT =
    extern "system" fn(c_uint, c_uint, c_uint, c_int, *const c_uint, c_uchar);
type DebugMessageInsertT = extern "system" fn(c_uint, c_uint, c_uint, c_uint, c_int, *const c_char);
type DeleteBuffersT = extern "system" fn(c_int, *const c_uint);
type DeleteFramebuffersT = extern "system" fn(c_int, *const c_uint);
type DeleteProgramT = extern "system" fn(c_uint);
type DeleteProgramPipelinesT = extern "system" fn(c_int, *const c_uint);
type DeleteQueriesT = extern "system" fn(c_int, *const c_uint);
type DeleteRenderbuffersT = extern "system" fn(c_int, *const c_uint);
type DeleteSamplersT = extern "system" fn(c_int, *const c_uint);
type DeleteShaderT = extern "system" fn(c_uint);
type DeleteSyncT = extern "system" fn(*mut c_void);
type DeleteTexturesT = extern "system" fn(c_int, *const c_uint);
type DeleteTransformFeedbacksT = extern "system" fn(c_int, *const c_uint);
type DeleteVertexArraysT = extern "system" fn(c_int, *const c_uint);
type DepthFuncT = extern "system" fn(c_uint);
type DepthMaskT = extern "system" fn(c_uchar);
type DepthRangeT = extern "system" fn(c_double, c_double);
type DepthRangefT = extern "system" fn(c_float, c_float);
type DepthRangexT = extern "system" fn(c_int, c_int);
type DetachShaderT = extern "system" fn(c_uint, c_uint);
type DisableT = extern "system" fn(c_uint);
type DisableClientStateT = extern "system" fn(c_uint);
type DisableVertexAttribArrayT = extern "system" fn(c_uint);
type DisableiT = extern "system" fn(c_uint, c_uint);
type DispatchComputeT = extern "system" fn(c_uint, c_uint, c_uint);
type DispatchComputeIndirectT = extern "system" fn(isize);
type DrawArraysT = extern "system" fn(c_uint, c_int, c_int);
type DrawArraysIndirectT = extern "system" fn(c_uint, *const c_void);
type DrawArraysInstancedT = extern "system" fn(c_uint, c_int, c_int, c_int);
type DrawBufferT = extern "system" fn(c_uint);
type DrawBuffersT = extern "system" fn(c_int, *const c_uint);
type DrawElementsT = extern "system" fn(c_uint, c_int, c_uint, *const c_void);
type DrawElementsBaseVertexT = extern "system" fn(c_uint, c_int, c_uint, *const c_void, c_int);
type DrawElementsIndirectT = extern "system" fn(c_uint, c_uint, *const c_void);
type DrawElementsInstancedT = extern "system" fn(c_uint, c_int, c_uint, *const c_void, c_int);
type DrawElementsInstancedBaseVertexT =
    extern "system" fn(c_uint, c_int, c_uint, *const c_void, c_int, c_int);
type DrawRangeElementsT = extern "system" fn(c_uint, c_uint, c_uint, c_int, c_uint, *const c_void);
type DrawRangeElementsBaseVertexT =
    extern "system" fn(c_uint, c_uint, c_uint, c_int, c_uint, *const c_void, c_int);
type EnableT = extern "system" fn(c_uint);
type EnableClientStateT = extern "system" fn(c_uint);
type EnableVertexAttribArrayT = extern "system" fn(c_uint);
type EnableiT = extern "system" fn(c_uint, c_uint);
type EndConditionalRenderT = extern "system" fn();
type EndQueryT = extern "system" fn(c_uint);
type EndTransformFeedbackT = extern "system" fn();
type FenceSyncT = extern "system" fn(c_uint, c_uint) -> *mut c_void;
type FinishT = extern "system" fn();
type FlushT = extern "system" fn();
type FlushMappedBufferRangeT = extern "system" fn(c_uint, isize, isize);
type FogfT = extern "system" fn(c_uint, c_float);
type FogfvT = extern "system" fn(c_uint, *const c_float);
type FogxT = extern "system" fn(c_uint, c_int);
type FogxvT = extern "system" fn(c_uint, *const c_int);
type FramebufferParameteriT = extern "system" fn(c_uint, c_uint, c_int);
type FramebufferRenderbufferT = extern "system" fn(c_uint, c_uint, c_uint, c_uint);
type FramebufferTextureT = extern "system" fn(c_uint, c_uint, c_uint, c_int);
type FramebufferTexture1DT = extern "system" fn(c_uint, c_uint, c_uint, c_uint, c_int);
type FramebufferTexture2DT = extern "system" fn(c_uint, c_uint, c_uint, c_uint, c_int);
type FramebufferTexture3DT = extern "system" fn(c_uint, c_uint, c_uint, c_uint, c_int, c_int);
type FramebufferTextureLayerT = extern "system" fn(c_uint, c_uint, c_uint, c_int, c_int);
type FrontFaceT = extern "system" fn(c_uint);
type FrustumfT = extern "system" fn(c_float, c_float, c_float, c_float, c_float, c_float);
type FrustumxT = extern "system" fn(c_int, c_int, c_int, c_int, c_int, c_int);
type GenBuffersT = extern "system" fn(c_int, *mut c_uint);
type GenFramebuffersT = extern "system" fn(c_int, *mut c_uint);
type GenProgramPipelinesT = extern "system" fn(c_int, *mut c_uint);
type GenQueriesT = extern "system" fn(c_int, *mut c_uint);
type GenRenderbuffersT = extern "system" fn(c_int, *mut c_uint);
type GenSamplersT = extern "system" fn(c_int, *mut c_uint);
type GenTexturesT = extern "system" fn(c_int, *mut c_uint);
type GenTransformFeedbacksT = extern "system" fn(c_int, *mut c_uint);
type GenVertexArraysT = extern "system" fn(c_int, *mut c_uint);
type GenerateMipmapT = extern "system" fn(c_uint);
type GetActiveAttribT =
    extern "system" fn(c_uint, c_uint, c_int, *mut c_int, *mut c_int, *mut c_uint, *mut c_char);
type GetActiveUniformT =
    extern "system" fn(c_uint, c_uint, c_int, *mut c_int, *mut c_int, *mut c_uint, *mut c_char);
type GetActiveUniformBlockNameT =
    extern "system" fn(c_uint, c_uint, c_int, *mut c_int, *mut c_char);
type GetActiveUniformBlockivT = extern "system" fn(c_uint, c_uint, c_uint, *mut c_int);
type GetActiveUniformNameT = extern "system" fn(c_uint, c_uint, c_int, *mut c_int, *mut c_char);
type GetActiveUniformsivT = extern "system" fn(c_uint, c_int, *const c_uint, c_uint, *mut c_int);
type GetAttachedShadersT = extern "system" fn(c_uint, c_int, *mut c_int, *mut c_uint);
type GetAttribLocationT = extern "system" fn(c_uint, *const c_char) -> c_int;
type GetBooleanivT = extern "system" fn(c_uint, c_uint, *mut c_uchar);
type GetBooleanvT = extern "system" fn(c_uint, *mut c_uchar);
type GetBufferParameteri64vT = extern "system" fn(c_uint, c_uint, *mut i64);
type GetBufferParameterivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetBufferPointervT = extern "system" fn(c_uint, c_uint, *mut *mut c_void);
type GetBufferSubDataT = extern "system" fn(c_uint, isize, isize, *mut c_void);
type GetClipPlanefT = extern "system" fn(c_uint, *mut c_float);
type GetClipPlanexT = extern "system" fn(c_uint, *mut c_int);
type GetCompressedTexImageT = extern "system" fn(c_uint, c_int, *mut c_void);
type GetDebugMessageLogT = extern "system" fn(
    c_uint,
    c_int,
    *mut c_uint,
    *mut c_uint,
    *mut c_uint,
    *mut c_uint,
    *mut c_int,
    *mut c_char,
) -> c_uint;
type GetDoublevT = extern "system" fn(c_uint, *mut c_double);
type GetErrorT = extern "system" fn() -> c_uint;
type GetFixedvT = extern "system" fn(c_uint, *mut c_int);
type GetFloatvT = extern "system" fn(c_uint, *mut c_float);
type GetFragDataIndexT = extern "system" fn(c_uint, *const c_char) -> c_int;
type GetFragDataLocationT = extern "system" fn(c_uint, *const c_char) -> c_int;
type GetFramebufferAttachmentParameterivT = extern "system" fn(c_uint, c_uint, c_uint, *mut c_int);
type GetFramebufferParameterivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetGraphicsResetStatusT = extern "system" fn() -> c_uint;
type GetInteger64ivT = extern "system" fn(c_uint, c_uint, *mut i64);
type GetInteger64vT = extern "system" fn(c_uint, *mut i64);
type GetIntegerivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetIntegervT = extern "system" fn(c_uint, *mut c_int);
type GetInternalformativT = extern "system" fn(c_uint, c_uint, c_uint, c_int, *mut c_int);
type GetLightfvT = extern "system" fn(c_uint, c_uint, *mut c_float);
type GetLightxvT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetMaterialfvT = extern "system" fn(c_uint, c_uint, *mut c_float);
type GetMaterialxvT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetMultisamplefvT = extern "system" fn(c_uint, c_uint, *mut c_float);
type GetObjectLabelT = extern "system" fn(c_uint, c_uint, c_int, *mut c_int, *mut c_char);
type GetObjectPtrLabelT = extern "system" fn(*const c_void, c_int, *mut c_int, *mut c_char);
type GetPointervT = extern "system" fn(c_uint, *mut *mut c_void);
type GetProgramBinaryT = extern "system" fn(c_uint, c_int, *mut c_int, *mut c_uint, *mut c_void);
type GetProgramInfoLogT = extern "system" fn(c_uint, c_int, *mut c_int, *mut c_char);
type GetProgramInterfaceivT = extern "system" fn(c_uint, c_uint, c_uint, *mut c_int);
type GetProgramPipelineInfoLogT = extern "system" fn(c_uint, c_int, *mut c_int, *mut c_char);
type GetProgramPipelineivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetProgramResourceIndexT = extern "system" fn(c_uint, c_uint, *const c_char) -> c_uint;
type GetProgramResourceLocationT = extern "system" fn(c_uint, c_uint, *const c_char) -> c_int;
type GetProgramResourceNameT =
    extern "system" fn(c_uint, c_uint, c_uint, c_int, *mut c_int, *mut c_char);
type GetProgramResourceivT =
    extern "system" fn(c_uint, c_uint, c_uint, c_int, *const c_uint, c_int, *mut c_int, *mut c_int);
type GetProgramivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetQueryObjecti64vT = extern "system" fn(c_uint, c_uint, *mut i64);
type GetQueryObjectivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetQueryObjectui64vT = extern "system" fn(c_uint, c_uint, *mut u64);
type GetQueryObjectuivT = extern "system" fn(c_uint, c_uint, *mut c_uint);
type GetQueryivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetRenderbufferParameterivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetSamplerParameterIivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetSamplerParameterIuivT = extern "system" fn(c_uint, c_uint, *mut c_uint);
type GetSamplerParameterfvT = extern "system" fn(c_uint, c_uint, *mut c_float);
type GetSamplerParameterivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetShaderInfoLogT = extern "system" fn(c_uint, c_int, *mut c_int, *mut c_char);
type GetShaderPrecisionFormatT = extern "system" fn(c_uint, c_uint, *mut c_int, *mut c_int);
type GetShaderSourceT = extern "system" fn(c_uint, c_int, *mut c_int, *mut c_char);
type GetShaderivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetStringT = extern "system" fn(c_uint) -> *const c_uchar;
type GetStringiT = extern "system" fn(c_uint, c_uint) -> *const c_uchar;
type GetSyncivT = extern "system" fn(*mut c_void, c_uint, c_int, *mut c_int, *mut c_int);
type GetTexEnvfvT = extern "system" fn(c_uint, c_uint, *mut c_float);
type GetTexEnvivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetTexEnvxvT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetTexImageT = extern "system" fn(c_uint, c_int, c_uint, c_uint, *mut c_void);
type GetTexLevelParameterfvT = extern "system" fn(c_uint, c_int, c_uint, *mut c_float);
type GetTexLevelParameterivT = extern "system" fn(c_uint, c_int, c_uint, *mut c_int);
type GetTexParameterIivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetTexParameterIuivT = extern "system" fn(c_uint, c_uint, *mut c_uint);
type GetTexParameterfvT = extern "system" fn(c_uint, c_uint, *mut c_float);
type GetTexParameterivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetTexParameterxvT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetTransformFeedbackVaryingT =
    extern "system" fn(c_uint, c_uint, c_int, *mut c_int, *mut c_int, *mut c_uint, *mut c_char);
type GetUniformBlockIndexT = extern "system" fn(c_uint, *const c_char) -> c_uint;
type GetUniformIndicesT = extern "system" fn(c_uint, c_int, *const *const c_char, *mut c_uint);
type GetUniformLocationT = extern "system" fn(c_uint, *const c_char) -> c_int;
type GetUniformfvT = extern "system" fn(c_uint, c_int, *mut c_float);
type GetUniformivT = extern "system" fn(c_uint, c_int, *mut c_int);
type GetUniformuivT = extern "system" fn(c_uint, c_int, *mut c_uint);
type GetVertexAttribIivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetVertexAttribIuivT = extern "system" fn(c_uint, c_uint, *mut c_uint);
type GetVertexAttribPointervT = extern "system" fn(c_uint, c_uint, *mut *mut c_void);
type GetVertexAttribdvT = extern "system" fn(c_uint, c_uint, *mut c_double);
type GetVertexAttribfvT = extern "system" fn(c_uint, c_uint, *mut c_float);
type GetVertexAttribivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetnUniformfvT = extern "system" fn(c_uint, c_int, c_int, *mut c_float);
type GetnUniformivT = extern "system" fn(c_uint, c_int, c_int, *mut c_int);
type GetnUniformuivT = extern "system" fn(c_uint, c_int, c_int, *mut c_uint);
type HintT = extern "system" fn(c_uint, c_uint);
type InvalidateFramebufferT = extern "system" fn(c_uint, c_int, *const c_uint);
type InvalidateSubFramebufferT =
    extern "system" fn(c_uint, c_int, *const c_uint, c_int, c_int, c_int, c_int);
type IsBufferT = extern "system" fn(c_uint) -> c_uchar;
type IsEnabledT = extern "system" fn(c_uint) -> c_uchar;
type IsEnablediT = extern "system" fn(c_uint, c_uint) -> c_uchar;
type IsFramebufferT = extern "system" fn(c_uint) -> c_uchar;
type IsProgramT = extern "system" fn(c_uint) -> c_uchar;
type IsProgramPipelineT = extern "system" fn(c_uint) -> c_uchar;
type IsQueryT = extern "system" fn(c_uint) -> c_uchar;
type IsRenderbufferT = extern "system" fn(c_uint) -> c_uchar;
type IsSamplerT = extern "system" fn(c_uint) -> c_uchar;
type IsShaderT = extern "system" fn(c_uint) -> c_uchar;
type IsSyncT = extern "system" fn(*mut c_void) -> c_uchar;
type IsTextureT = extern "system" fn(c_uint) -> c_uchar;
type IsTransformFeedbackT = extern "system" fn(c_uint) -> c_uchar;
type IsVertexArrayT = extern "system" fn(c_uint) -> c_uchar;
type LightModelfT = extern "system" fn(c_uint, c_float);
type LightModelfvT = extern "system" fn(c_uint, *const c_float);
type LightModelxT = extern "system" fn(c_uint, c_int);
type LightModelxvT = extern "system" fn(c_uint, *const c_int);
type LightfT = extern "system" fn(c_uint, c_uint, c_float);
type LightfvT = extern "system" fn(c_uint, c_uint, *const c_float);
type LightxT = extern "system" fn(c_uint, c_uint, c_int);
type LightxvT = extern "system" fn(c_uint, c_uint, *const c_int);
type LineWidthT = extern "system" fn(c_float);
type LineWidthxT = extern "system" fn(c_int);
type LinkProgramT = extern "system" fn(c_uint);
type LoadIdentityT = extern "system" fn();
type LoadMatrixfT = extern "system" fn(*const c_float);
type LoadMatrixxT = extern "system" fn(*const c_int);
type LogicOpT = extern "system" fn(c_uint);
type MapBufferT = extern "system" fn(c_uint, c_uint) -> *mut c_void;
type MapBufferRangeT = extern "system" fn(c_uint, isize, isize, c_uint) -> *mut c_void;
type MaterialfT = extern "system" fn(c_uint, c_uint, c_float);
type MaterialfvT = extern "system" fn(c_uint, c_uint, *const c_float);
type MaterialxT = extern "system" fn(c_uint, c_uint, c_int);
type MaterialxvT = extern "system" fn(c_uint, c_uint, *const c_int);
type MatrixModeT = extern "system" fn(c_uint);
type MemoryBarrierT = extern "system" fn(c_uint);
type MemoryBarrierByRegionT = extern "system" fn(c_uint);
type MinSampleShadingT = extern "system" fn(c_float);
type MultMatrixfT = extern "system" fn(*const c_float);
type MultMatrixxT = extern "system" fn(*const c_int);
type MultiDrawArraysT = extern "system" fn(c_uint, *const c_int, *const c_int, c_int);
type MultiDrawElementsT =
    extern "system" fn(c_uint, *const c_int, c_uint, *const *const c_void, c_int);
type MultiDrawElementsBaseVertexT =
    extern "system" fn(c_uint, *const c_int, c_uint, *const *const c_void, c_int, *const c_int);
type MultiTexCoord4fT = extern "system" fn(c_uint, c_float, c_float, c_float, c_float);
type MultiTexCoord4xT = extern "system" fn(c_uint, c_int, c_int, c_int, c_int);
type MultiTexCoordP1uiT = extern "system" fn(c_uint, c_uint, c_uint);
type MultiTexCoordP1uivT = extern "system" fn(c_uint, c_uint, *const c_uint);
type MultiTexCoordP2uiT = extern "system" fn(c_uint, c_uint, c_uint);
type MultiTexCoordP2uivT = extern "system" fn(c_uint, c_uint, *const c_uint);
type MultiTexCoordP3uiT = extern "system" fn(c_uint, c_uint, c_uint);
type MultiTexCoordP3uivT = extern "system" fn(c_uint, c_uint, *const c_uint);
type MultiTexCoordP4uiT = extern "system" fn(c_uint, c_uint, c_uint);
type MultiTexCoordP4uivT = extern "system" fn(c_uint, c_uint, *const c_uint);
type Normal3fT = extern "system" fn(c_float, c_float, c_float);
type Normal3xT = extern "system" fn(c_int, c_int, c_int);
type NormalP3uiT = extern "system" fn(c_uint, c_uint);
type NormalP3uivT = extern "system" fn(c_uint, *const c_uint);
type NormalPointerT = extern "system" fn(c_uint, c_int, *const c_void);
type ObjectLabelT = extern "system" fn(c_uint, c_uint, c_int, *const c_char);
type ObjectPtrLabelT = extern "system" fn(*const c_void, c_int, *const c_char);
type OrthofT = extern "system" fn(c_float, c_float, c_float, c_float, c_float, c_float);
type OrthoxT = extern "system" fn(c_int, c_int, c_int, c_int, c_int, c_int);
type PatchParameteriT = extern "system" fn(c_uint, c_int);
type PauseTransformFeedbackT = extern "system" fn();
type PixelStorefT = extern "system" fn(c_uint, c_float);
type PixelStoreiT = extern "system" fn(c_uint, c_int);
type PointParameterfT = extern "system" fn(c_uint, c_float);
type PointParameterfvT = extern "system" fn(c_uint, *const c_float);
type PointParameteriT = extern "system" fn(c_uint, c_int);
type PointParameterivT = extern "system" fn(c_uint, *const c_int);
type PointParameterxT = extern "system" fn(c_uint, c_int);
type PointParameterxvT = extern "system" fn(c_uint, *const c_int);
type PointSizeT = extern "system" fn(c_float);
type PointSizexT = extern "system" fn(c_int);
type PolygonModeT = extern "system" fn(c_uint, c_uint);
type PolygonOffsetT = extern "system" fn(c_float, c_float);
type PolygonOffsetxT = extern "system" fn(c_int, c_int);
type PopDebugGroupT = extern "system" fn();
type PopMatrixT = extern "system" fn();
type PrimitiveBoundingBoxT =
    extern "system" fn(c_float, c_float, c_float, c_float, c_float, c_float, c_float, c_float);
type PrimitiveRestartIndexT = extern "system" fn(c_uint);
type ProgramBinaryT = extern "system" fn(c_uint, c_uint, *const c_void, c_int);
type ProgramParameteriT = extern "system" fn(c_uint, c_uint, c_int);
type ProgramUniform1fT = extern "system" fn(c_uint, c_int, c_float);
type ProgramUniform1fvT = extern "system" fn(c_uint, c_int, c_int, *const c_float);
type ProgramUniform1iT = extern "system" fn(c_uint, c_int, c_int);
type ProgramUniform1ivT = extern "system" fn(c_uint, c_int, c_int, *const c_int);
type ProgramUniform1uiT = extern "system" fn(c_uint, c_int, c_uint);
type ProgramUniform1uivT = extern "system" fn(c_uint, c_int, c_int, *const c_uint);
type ProgramUniform2fT = extern "system" fn(c_uint, c_int, c_float, c_float);
type ProgramUniform2fvT = extern "system" fn(c_uint, c_int, c_int, *const c_float);
type ProgramUniform2iT = extern "system" fn(c_uint, c_int, c_int, c_int);
type ProgramUniform2ivT = extern "system" fn(c_uint, c_int, c_int, *const c_int);
type ProgramUniform2uiT = extern "system" fn(c_uint, c_int, c_uint, c_uint);
type ProgramUniform2uivT = extern "system" fn(c_uint, c_int, c_int, *const c_uint);
type ProgramUniform3fT = extern "system" fn(c_uint, c_int, c_float, c_float, c_float);
type ProgramUniform3fvT = extern "system" fn(c_uint, c_int, c_int, *const c_float);
type ProgramUniform3iT = extern "system" fn(c_uint, c_int, c_int, c_int, c_int);
type ProgramUniform3ivT = extern "system" fn(c_uint, c_int, c_int, *const c_int);
type ProgramUniform3uiT = extern "system" fn(c_uint, c_int, c_uint, c_uint, c_uint);
type ProgramUniform3uivT = extern "system" fn(c_uint, c_int, c_int, *const c_uint);
type ProgramUniform4fT = extern "system" fn(c_uint, c_int, c_float, c_float, c_float, c_float);
type ProgramUniform4fvT = extern "system" fn(c_uint, c_int, c_int, *const c_float);
type ProgramUniform4iT = extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_int);
type ProgramUniform4ivT = extern "system" fn(c_uint, c_int, c_int, *const c_int);
type ProgramUniform4uiT = extern "system" fn(c_uint, c_int, c_uint, c_uint, c_uint, c_uint);
type ProgramUniform4uivT = extern "system" fn(c_uint, c_int, c_int, *const c_uint);
type ProgramUniformMatrix2fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix2x3fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix2x4fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix3fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix3x2fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix3x4fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix4fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix4x2fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix4x3fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProvokingVertexT = extern "system" fn(c_uint);
type PushDebugGroupT = extern "system" fn(c_uint, c_uint, c_int, *const c_char);
type PushMatrixT = extern "system" fn();
type QueryCounterT = extern "system" fn(c_uint, c_uint);
type ReadBufferT = extern "system" fn(c_uint);
type ReadPixelsT = extern "system" fn(c_int, c_int, c_int, c_int, c_uint, c_uint, *mut c_void);
type ReadnPixelsT =
    extern "system" fn(c_int, c_int, c_int, c_int, c_uint, c_uint, c_int, *mut c_void);
type ReleaseShaderCompilerT = extern "system" fn();
type RenderbufferStorageT = extern "system" fn(c_uint, c_uint, c_int, c_int);
type RenderbufferStorageMultisampleT = extern "system" fn(c_uint, c_int, c_uint, c_int, c_int);
type ResumeTransformFeedbackT = extern "system" fn();
type RotatefT = extern "system" fn(c_float, c_float, c_float, c_float);
type RotatexT = extern "system" fn(c_int, c_int, c_int, c_int);
type SampleCoverageT = extern "system" fn(c_float, c_uchar);
type SampleCoveragexT = extern "system" fn(c_int, c_uchar);
type SampleMaskiT = extern "system" fn(c_uint, c_uint);
type SamplerParameterIivT = extern "system" fn(c_uint, c_uint, *const c_int);
type SamplerParameterIuivT = extern "system" fn(c_uint, c_uint, *const c_uint);
type SamplerParameterfT = extern "system" fn(c_uint, c_uint, c_float);
type SamplerParameterfvT = extern "system" fn(c_uint, c_uint, *const c_float);
type SamplerParameteriT = extern "system" fn(c_uint, c_uint, c_int);
type SamplerParameterivT = extern "system" fn(c_uint, c_uint, *const c_int);
type ScalefT = extern "system" fn(c_float, c_float, c_float);
type ScalexT = extern "system" fn(c_int, c_int, c_int);
type ScissorT = extern "system" fn(c_int, c_int, c_int, c_int);
type SecondaryColorP3uiT = extern "system" fn(c_uint, c_uint);
type SecondaryColorP3uivT = extern "system" fn(c_uint, *const c_uint);
type ShadeModelT = extern "system" fn(c_uint);
type ShaderBinaryT = extern "system" fn(c_int, *const c_uint, c_uint, *const c_void, c_int);
type ShaderSourceT = extern "system" fn(c_uint, c_int, *const *const c_char, *const c_int);
type StencilFuncT = extern "system" fn(c_uint, c_int, c_uint);
type StencilFuncSeparateT = extern "system" fn(c_uint, c_uint, c_int, c_uint);
type StencilMaskT = extern "system" fn(c_uint);
type StencilMaskSeparateT = extern "system" fn(c_uint, c_uint);
type StencilOpT = extern "system" fn(c_uint, c_uint, c_uint);
type StencilOpSeparateT = extern "system" fn(c_uint, c_uint, c_uint, c_uint);
type TexBufferT = extern "system" fn(c_uint, c_uint, c_uint);
type TexBufferRangeT = extern "system" fn(c_uint, c_uint, c_uint, isize, isize);
type TexCoordP1uiT = extern "system" fn(c_uint, c_uint);
type TexCoordP1uivT = extern "system" fn(c_uint, *const c_uint);
type TexCoordP2uiT = extern "system" fn(c_uint, c_uint);
type TexCoordP2uivT = extern "system" fn(c_uint, *const c_uint);
type TexCoordP3uiT = extern "system" fn(c_uint, c_uint);
type TexCoordP3uivT = extern "system" fn(c_uint, *const c_uint);
type TexCoordP4uiT = extern "system" fn(c_uint, c_uint);
type TexCoordP4uivT = extern "system" fn(c_uint, *const c_uint);
type TexCoordPointerT = extern "system" fn(c_int, c_uint, c_int, *const c_void);
type TexEnvfT = extern "system" fn(c_uint, c_uint, c_float);
type TexEnvfvT = extern "system" fn(c_uint, c_uint, *const c_float);
type TexEnviT = extern "system" fn(c_uint, c_uint, c_int);
type TexEnvivT = extern "system" fn(c_uint, c_uint, *const c_int);
type TexEnvxT = extern "system" fn(c_uint, c_uint, c_int);
type TexEnvxvT = extern "system" fn(c_uint, c_uint, *const c_int);
type TexImage1DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_uint, c_uint, *const c_void);
type TexImage2DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_int, c_uint, c_uint, *const c_void);
type TexImage2DMultisampleT = extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_uchar);
type TexImage3DT = extern "system" fn(
    c_uint,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_uint,
    c_uint,
    *const c_void,
);
type TexImage3DMultisampleT =
    extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int, c_uchar);
type TexParameterIivT = extern "system" fn(c_uint, c_uint, *const c_int);
type TexParameterIuivT = extern "system" fn(c_uint, c_uint, *const c_uint);
type TexParameterfT = extern "system" fn(c_uint, c_uint, c_float);
type TexParameterfvT = extern "system" fn(c_uint, c_uint, *const c_float);
type TexParameteriT = extern "system" fn(c_uint, c_uint, c_int);
type TexParameterivT = extern "system" fn(c_uint, c_uint, *const c_int);
type TexParameterxT = extern "system" fn(c_uint, c_uint, c_int);
type TexParameterxvT = extern "system" fn(c_uint, c_uint, *const c_int);
type TexStorage2DT = extern "system" fn(c_uint, c_int, c_uint, c_int, c_int);
type TexStorage2DMultisampleT = extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_uchar);
type TexStorage3DT = extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int);
type TexStorage3DMultisampleT =
    extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int, c_uchar);
type TexSubImage1DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_uint, c_uint, *const c_void);
type TexSubImage2DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_int, c_uint, c_uint, *const c_void);
type TexSubImage3DT = extern "system" fn(
    c_uint,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_uint,
    c_uint,
    *const c_void,
);
type TransformFeedbackVaryingsT = extern "system" fn(c_uint, c_int, *const *const c_char, c_uint);
type TranslatefT = extern "system" fn(c_float, c_float, c_float);
type TranslatexT = extern "system" fn(c_int, c_int, c_int);
type Uniform1fT = extern "system" fn(c_int, c_float);
type Uniform1fvT = extern "system" fn(c_int, c_int, *const c_float);
type Uniform1iT = extern "system" fn(c_int, c_int);
type Uniform1ivT = extern "system" fn(c_int, c_int, *const c_int);
type Uniform1uiT = extern "system" fn(c_int, c_uint);
type Uniform1uivT = extern "system" fn(c_int, c_int, *const c_uint);
type Uniform2fT = extern "system" fn(c_int, c_float, c_float);
type Uniform2fvT = extern "system" fn(c_int, c_int, *const c_float);
type Uniform2iT = extern "system" fn(c_int, c_int, c_int);
type Uniform2ivT = extern "system" fn(c_int, c_int, *const c_int);
type Uniform2uiT = extern "system" fn(c_int, c_uint, c_uint);
type Uniform2uivT = extern "system" fn(c_int, c_int, *const c_uint);
type Uniform3fT = extern "system" fn(c_int, c_float, c_float, c_float);
type Uniform3fvT = extern "system" fn(c_int, c_int, *const c_float);
type Uniform3iT = extern "system" fn(c_int, c_int, c_int, c_int);
type Uniform3ivT = extern "system" fn(c_int, c_int, *const c_int);
type Uniform3uiT = extern "system" fn(c_int, c_uint, c_uint, c_uint);
type Uniform3uivT = extern "system" fn(c_int, c_int, *const c_uint);
type Uniform4fT = extern "system" fn(c_int, c_float, c_float, c_float, c_float);
type Uniform4fvT = extern "system" fn(c_int, c_int, *const c_float);
type Uniform4iT = extern "system" fn(c_int, c_int, c_int, c_int, c_int);
type Uniform4ivT = extern "system" fn(c_int, c_int, *const c_int);
type Uniform4uiT = extern "system" fn(c_int, c_uint, c_uint, c_uint, c_uint);
type Uniform4uivT = extern "system" fn(c_int, c_int, *const c_uint);
type UniformBlockBindingT = extern "system" fn(c_uint, c_uint, c_uint);
type UniformMatrix2fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix2x3fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix2x4fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix3fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix3x2fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix3x4fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix4fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix4x2fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix4x3fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UnmapBufferT = extern "system" fn(c_uint) -> c_uchar;
type UseProgramT = extern "system" fn(c_uint);
type UseProgramStagesT = extern "system" fn(c_uint, c_uint, c_uint);
type ValidateProgramT = extern "system" fn(c_uint);
type ValidateProgramPipelineT = extern "system" fn(c_uint);
type VertexAttrib1dT = extern "system" fn(c_uint, c_double);
type VertexAttrib1dvT = extern "system" fn(c_uint, *const c_double);
type VertexAttrib1fT = extern "system" fn(c_uint, c_float);
type VertexAttrib1fvT = extern "system" fn(c_uint, *const c_float);
type VertexAttrib1sT = extern "system" fn(c_uint, c_short);
type VertexAttrib1svT = extern "system" fn(c_uint, *const c_short);
type VertexAttrib2dT = extern "system" fn(c_uint, c_double, c_double);
type VertexAttrib2dvT = extern "system" fn(c_uint, *const c_double);
type VertexAttrib2fT = extern "system" fn(c_uint, c_float, c_float);
type VertexAttrib2fvT = extern "system" fn(c_uint, *const c_float);
type VertexAttrib2sT = extern "system" fn(c_uint, c_short, c_short);
type VertexAttrib2svT = extern "system" fn(c_uint, *const c_short);
type VertexAttrib3dT = extern "system" fn(c_uint, c_double, c_double, c_double);
type VertexAttrib3dvT = extern "system" fn(c_uint, *const c_double);
type VertexAttrib3fT = extern "system" fn(c_uint, c_float, c_float, c_float);
type VertexAttrib3fvT = extern "system" fn(c_uint, *const c_float);
type VertexAttrib3sT = extern "system" fn(c_uint, c_short, c_short, c_short);
type VertexAttrib3svT = extern "system" fn(c_uint, *const c_short);
type VertexAttrib4NbvT = extern "system" fn(c_uint, *const c_char);
type VertexAttrib4NivT = extern "system" fn(c_uint, *const c_int);
type VertexAttrib4NsvT = extern "system" fn(c_uint, *const c_short);
type VertexAttrib4NubT = extern "system" fn(c_uint, c_uchar, c_uchar, c_uchar, c_uchar);
type VertexAttrib4NubvT = extern "system" fn(c_uint, *const c_uchar);
type VertexAttrib4NuivT = extern "system" fn(c_uint, *const c_uint);
type VertexAttrib4NusvT = extern "system" fn(c_uint, *const c_ushort);
type VertexAttrib4bvT = extern "system" fn(c_uint, *const c_char);
type VertexAttrib4dT = extern "system" fn(c_uint, c_double, c_double, c_double, c_double);
type VertexAttrib4dvT = extern "system" fn(c_uint, *const c_double);
type VertexAttrib4fT = extern "system" fn(c_uint, c_float, c_float, c_float, c_float);
type VertexAttrib4fvT = extern "system" fn(c_uint, *const c_float);
type VertexAttrib4ivT = extern "system" fn(c_uint, *const c_int);
type VertexAttrib4sT = extern "system" fn(c_uint, c_short, c_short, c_short, c_short);
type VertexAttrib4svT = extern "system" fn(c_uint, *const c_short);
type VertexAttrib4ubvT = extern "system" fn(c_uint, *const c_uchar);
type VertexAttrib4uivT = extern "system" fn(c_uint, *const c_uint);
type VertexAttrib4usvT = extern "system" fn(c_uint, *const c_ushort);
type VertexAttribBindingT = extern "system" fn(c_uint, c_uint);
type VertexAttribDivisorT = extern "system" fn(c_uint, c_uint);
type VertexAttribFormatT = extern "system" fn(c_uint, c_int, c_uint, c_uchar, c_uint);
type VertexAttribI1iT = extern "system" fn(c_uint, c_int);
type VertexAttribI1ivT = extern "system" fn(c_uint, *const c_int);
type VertexAttribI1uiT = extern "system" fn(c_uint, c_uint);
type VertexAttribI1uivT = extern "system" fn(c_uint, *const c_uint);
type VertexAttribI2iT = extern "system" fn(c_uint, c_int, c_int);
type VertexAttribI2ivT = extern "system" fn(c_uint, *const c_int);
type VertexAttribI2uiT = extern "system" fn(c_uint, c_uint, c_uint);
type VertexAttribI2uivT = extern "system" fn(c_uint, *const c_uint);
type VertexAttribI3iT = extern "system" fn(c_uint, c_int, c_int, c_int);
type VertexAttribI3ivT = extern "system" fn(c_uint, *const c_int);
type VertexAttribI3uiT = extern "system" fn(c_uint, c_uint, c_uint, c_uint);
type VertexAttribI3uivT = extern "system" fn(c_uint, *const c_uint);
type VertexAttribI4bvT = extern "system" fn(c_uint, *const c_char);
type VertexAttribI4iT = extern "system" fn(c_uint, c_int, c_int, c_int, c_int);
type VertexAttribI4ivT = extern "system" fn(c_uint, *const c_int);
type VertexAttribI4svT = extern "system" fn(c_uint, *const c_short);
type VertexAttribI4ubvT = extern "system" fn(c_uint, *const c_uchar);
type VertexAttribI4uiT = extern "system" fn(c_uint, c_uint, c_uint, c_uint, c_uint);
type VertexAttribI4uivT = extern "system" fn(c_uint, *const c_uint);
type VertexAttribI4usvT = extern "system" fn(c_uint, *const c_ushort);
type VertexAttribIFormatT = extern "system" fn(c_uint, c_int, c_uint, c_uint);
type VertexAttribIPointerT = extern "system" fn(c_uint, c_int, c_uint, c_int, *const c_void);
type VertexAttribP1uiT = extern "system" fn(c_uint, c_uint, c_uchar, c_uint);
type VertexAttribP1uivT = extern "system" fn(c_uint, c_uint, c_uchar, *const c_uint);
type VertexAttribP2uiT = extern "system" fn(c_uint, c_uint, c_uchar, c_uint);
type VertexAttribP2uivT = extern "system" fn(c_uint, c_uint, c_uchar, *const c_uint);
type VertexAttribP3uiT = extern "system" fn(c_uint, c_uint, c_uchar, c_uint);
type VertexAttribP3uivT = extern "system" fn(c_uint, c_uint, c_uchar, *const c_uint);
type VertexAttribP4uiT = extern "system" fn(c_uint, c_uint, c_uchar, c_uint);
type VertexAttribP4uivT = extern "system" fn(c_uint, c_uint, c_uchar, *const c_uint);
type VertexAttribPointerT =
    extern "system" fn(c_uint, c_int, c_uint, c_uchar, c_int, *const c_void);
type VertexBindingDivisorT = extern "system" fn(c_uint, c_uint);
type VertexP2uiT = extern "system" fn(c_uint, c_uint);
type VertexP2uivT = extern "system" fn(c_uint, *const c_uint);
type VertexP3uiT = extern "system" fn(c_uint, c_uint);
type VertexP3uivT = extern "system" fn(c_uint, *const c_uint);
type VertexP4uiT = extern "system" fn(c_uint, c_uint);
type VertexP4uivT = extern "system" fn(c_uint, *const c_uint);
type VertexPointerT = extern "system" fn(c_int, c_uint, c_int, *const c_void);
type ViewportT = extern "system" fn(c_int, c_int, c_int, c_int);
type WaitSyncT = extern "system" fn(*mut c_void, c_uint, u64);

pub struct GlCmd {
    active_shader_program_p: ActiveShaderProgramT,
    active_texture_p: ActiveTextureT,
    alpha_func_p: AlphaFuncT,
    alpha_funcx_p: AlphaFuncxT,
    attach_shader_p: AttachShaderT,
    begin_conditional_render_p: BeginConditionalRenderT,
    begin_query_p: BeginQueryT,
    begin_transform_feedback_p: BeginTransformFeedbackT,
    bind_attrib_location_p: BindAttribLocationT,
    bind_buffer_p: BindBufferT,
    bind_buffer_base_p: BindBufferBaseT,
    bind_buffer_range_p: BindBufferRangeT,
    bind_frag_data_location_p: BindFragDataLocationT,
    bind_frag_data_location_indexed_p: BindFragDataLocationIndexedT,
    bind_framebuffer_p: BindFramebufferT,
    bind_image_texture_p: BindImageTextureT,
    bind_program_pipeline_p: BindProgramPipelineT,
    bind_renderbuffer_p: BindRenderbufferT,
    bind_sampler_p: BindSamplerT,
    bind_texture_p: BindTextureT,
    bind_transform_feedback_p: BindTransformFeedbackT,
    bind_vertex_array_p: BindVertexArrayT,
    bind_vertex_buffer_p: BindVertexBufferT,
    blend_barrier_p: BlendBarrierT,
    blend_color_p: BlendColorT,
    blend_equation_p: BlendEquationT,
    blend_equation_separate_p: BlendEquationSeparateT,
    blend_equation_separatei_p: BlendEquationSeparateiT,
    blend_equationi_p: BlendEquationiT,
    blend_func_p: BlendFuncT,
    blend_func_separate_p: BlendFuncSeparateT,
    blend_func_separatei_p: BlendFuncSeparateiT,
    blend_funci_p: BlendFunciT,
    blit_framebuffer_p: BlitFramebufferT,
    buffer_data_p: BufferDataT,
    buffer_sub_data_p: BufferSubDataT,
    check_framebuffer_status_p: CheckFramebufferStatusT,
    clamp_color_p: ClampColorT,
    clear_p: ClearT,
    clear_bufferfi_p: ClearBufferfiT,
    clear_bufferfv_p: ClearBufferfvT,
    clear_bufferiv_p: ClearBufferivT,
    clear_bufferuiv_p: ClearBufferuivT,
    clear_color_p: ClearColorT,
    clear_colorx_p: ClearColorxT,
    clear_depth_p: ClearDepthT,
    clear_depthf_p: ClearDepthfT,
    clear_depthx_p: ClearDepthxT,
    clear_stencil_p: ClearStencilT,
    client_active_texture_p: ClientActiveTextureT,
    client_wait_sync_p: ClientWaitSyncT,
    clip_planef_p: ClipPlanefT,
    clip_planex_p: ClipPlanexT,
    color4f_p: Color4fT,
    color4ub_p: Color4ubT,
    color4x_p: Color4xT,
    color_mask_p: ColorMaskT,
    color_maski_p: ColorMaskiT,
    color_p3ui_p: ColorP3uiT,
    color_p3uiv_p: ColorP3uivT,
    color_p4ui_p: ColorP4uiT,
    color_p4uiv_p: ColorP4uivT,
    color_pointer_p: ColorPointerT,
    compile_shader_p: CompileShaderT,
    compressed_tex_image1_d_p: CompressedTexImage1DT,
    compressed_tex_image2_d_p: CompressedTexImage2DT,
    compressed_tex_image3_d_p: CompressedTexImage3DT,
    compressed_tex_sub_image1_d_p: CompressedTexSubImage1DT,
    compressed_tex_sub_image2_d_p: CompressedTexSubImage2DT,
    compressed_tex_sub_image3_d_p: CompressedTexSubImage3DT,
    copy_buffer_sub_data_p: CopyBufferSubDataT,
    copy_image_sub_data_p: CopyImageSubDataT,
    copy_tex_image1_d_p: CopyTexImage1DT,
    copy_tex_image2_d_p: CopyTexImage2DT,
    copy_tex_sub_image1_d_p: CopyTexSubImage1DT,
    copy_tex_sub_image2_d_p: CopyTexSubImage2DT,
    copy_tex_sub_image3_d_p: CopyTexSubImage3DT,
    create_program_p: CreateProgramT,
    create_shader_p: CreateShaderT,
    create_shader_programv_p: CreateShaderProgramvT,
    cull_face_p: CullFaceT,
    debug_message_callback_p: DebugMessageCallbackT,
    debug_message_control_p: DebugMessageControlT,
    debug_message_insert_p: DebugMessageInsertT,
    delete_buffers_p: DeleteBuffersT,
    delete_framebuffers_p: DeleteFramebuffersT,
    delete_program_p: DeleteProgramT,
    delete_program_pipelines_p: DeleteProgramPipelinesT,
    delete_queries_p: DeleteQueriesT,
    delete_renderbuffers_p: DeleteRenderbuffersT,
    delete_samplers_p: DeleteSamplersT,
    delete_shader_p: DeleteShaderT,
    delete_sync_p: DeleteSyncT,
    delete_textures_p: DeleteTexturesT,
    delete_transform_feedbacks_p: DeleteTransformFeedbacksT,
    delete_vertex_arrays_p: DeleteVertexArraysT,
    depth_func_p: DepthFuncT,
    depth_mask_p: DepthMaskT,
    depth_range_p: DepthRangeT,
    depth_rangef_p: DepthRangefT,
    depth_rangex_p: DepthRangexT,
    detach_shader_p: DetachShaderT,
    disable_p: DisableT,
    disable_client_state_p: DisableClientStateT,
    disable_vertex_attrib_array_p: DisableVertexAttribArrayT,
    disablei_p: DisableiT,
    dispatch_compute_p: DispatchComputeT,
    dispatch_compute_indirect_p: DispatchComputeIndirectT,
    draw_arrays_p: DrawArraysT,
    draw_arrays_indirect_p: DrawArraysIndirectT,
    draw_arrays_instanced_p: DrawArraysInstancedT,
    draw_buffer_p: DrawBufferT,
    draw_buffers_p: DrawBuffersT,
    draw_elements_p: DrawElementsT,
    draw_elements_base_vertex_p: DrawElementsBaseVertexT,
    draw_elements_indirect_p: DrawElementsIndirectT,
    draw_elements_instanced_p: DrawElementsInstancedT,
    draw_elements_instanced_base_vertex_p: DrawElementsInstancedBaseVertexT,
    draw_range_elements_p: DrawRangeElementsT,
    draw_range_elements_base_vertex_p: DrawRangeElementsBaseVertexT,
    enable_p: EnableT,
    enable_client_state_p: EnableClientStateT,
    enable_vertex_attrib_array_p: EnableVertexAttribArrayT,
    enablei_p: EnableiT,
    end_conditional_render_p: EndConditionalRenderT,
    end_query_p: EndQueryT,
    end_transform_feedback_p: EndTransformFeedbackT,
    fence_sync_p: FenceSyncT,
    finish_p: FinishT,
    flush_p: FlushT,
    flush_mapped_buffer_range_p: FlushMappedBufferRangeT,
    fogf_p: FogfT,
    fogfv_p: FogfvT,
    fogx_p: FogxT,
    fogxv_p: FogxvT,
    framebuffer_parameteri_p: FramebufferParameteriT,
    framebuffer_renderbuffer_p: FramebufferRenderbufferT,
    framebuffer_texture_p: FramebufferTextureT,
    framebuffer_texture1_d_p: FramebufferTexture1DT,
    framebuffer_texture2_d_p: FramebufferTexture2DT,
    framebuffer_texture3_d_p: FramebufferTexture3DT,
    framebuffer_texture_layer_p: FramebufferTextureLayerT,
    front_face_p: FrontFaceT,
    frustumf_p: FrustumfT,
    frustumx_p: FrustumxT,
    gen_buffers_p: GenBuffersT,
    gen_framebuffers_p: GenFramebuffersT,
    gen_program_pipelines_p: GenProgramPipelinesT,
    gen_queries_p: GenQueriesT,
    gen_renderbuffers_p: GenRenderbuffersT,
    gen_samplers_p: GenSamplersT,
    gen_textures_p: GenTexturesT,
    gen_transform_feedbacks_p: GenTransformFeedbacksT,
    gen_vertex_arrays_p: GenVertexArraysT,
    generate_mipmap_p: GenerateMipmapT,
    get_active_attrib_p: GetActiveAttribT,
    get_active_uniform_p: GetActiveUniformT,
    get_active_uniform_block_name_p: GetActiveUniformBlockNameT,
    get_active_uniform_blockiv_p: GetActiveUniformBlockivT,
    get_active_uniform_name_p: GetActiveUniformNameT,
    get_active_uniformsiv_p: GetActiveUniformsivT,
    get_attached_shaders_p: GetAttachedShadersT,
    get_attrib_location_p: GetAttribLocationT,
    get_booleani_v_p: GetBooleanivT,
    get_booleanv_p: GetBooleanvT,
    get_buffer_parameteri64v_p: GetBufferParameteri64vT,
    get_buffer_parameteriv_p: GetBufferParameterivT,
    get_buffer_pointerv_p: GetBufferPointervT,
    get_buffer_sub_data_p: GetBufferSubDataT,
    get_clip_planef_p: GetClipPlanefT,
    get_clip_planex_p: GetClipPlanexT,
    get_compressed_tex_image_p: GetCompressedTexImageT,
    get_debug_message_log_p: GetDebugMessageLogT,
    get_doublev_p: GetDoublevT,
    get_error_p: GetErrorT,
    get_fixedv_p: GetFixedvT,
    get_floatv_p: GetFloatvT,
    get_frag_data_index_p: GetFragDataIndexT,
    get_frag_data_location_p: GetFragDataLocationT,
    get_framebuffer_attachment_parameteriv_p: GetFramebufferAttachmentParameterivT,
    get_framebuffer_parameteriv_p: GetFramebufferParameterivT,
    get_graphics_reset_status_p: GetGraphicsResetStatusT,
    get_integer64i_v_p: GetInteger64ivT,
    get_integer64v_p: GetInteger64vT,
    get_integeri_v_p: GetIntegerivT,
    get_integerv_p: GetIntegervT,
    get_internalformativ_p: GetInternalformativT,
    get_lightfv_p: GetLightfvT,
    get_lightxv_p: GetLightxvT,
    get_materialfv_p: GetMaterialfvT,
    get_materialxv_p: GetMaterialxvT,
    get_multisamplefv_p: GetMultisamplefvT,
    get_object_label_p: GetObjectLabelT,
    get_object_ptr_label_p: GetObjectPtrLabelT,
    get_pointerv_p: GetPointervT,
    get_program_binary_p: GetProgramBinaryT,
    get_program_info_log_p: GetProgramInfoLogT,
    get_program_interfaceiv_p: GetProgramInterfaceivT,
    get_program_pipeline_info_log_p: GetProgramPipelineInfoLogT,
    get_program_pipelineiv_p: GetProgramPipelineivT,
    get_program_resource_index_p: GetProgramResourceIndexT,
    get_program_resource_location_p: GetProgramResourceLocationT,
    get_program_resource_name_p: GetProgramResourceNameT,
    get_program_resourceiv_p: GetProgramResourceivT,
    get_programiv_p: GetProgramivT,
    get_query_objecti64v_p: GetQueryObjecti64vT,
    get_query_objectiv_p: GetQueryObjectivT,
    get_query_objectui64v_p: GetQueryObjectui64vT,
    get_query_objectuiv_p: GetQueryObjectuivT,
    get_queryiv_p: GetQueryivT,
    get_renderbuffer_parameteriv_p: GetRenderbufferParameterivT,
    get_sampler_parameter_iiv_p: GetSamplerParameterIivT,
    get_sampler_parameter_iuiv_p: GetSamplerParameterIuivT,
    get_sampler_parameterfv_p: GetSamplerParameterfvT,
    get_sampler_parameteriv_p: GetSamplerParameterivT,
    get_shader_info_log_p: GetShaderInfoLogT,
    get_shader_precision_format_p: GetShaderPrecisionFormatT,
    get_shader_source_p: GetShaderSourceT,
    get_shaderiv_p: GetShaderivT,
    get_string_p: GetStringT,
    get_stringi_p: GetStringiT,
    get_synciv_p: GetSyncivT,
    get_tex_envfv_p: GetTexEnvfvT,
    get_tex_enviv_p: GetTexEnvivT,
    get_tex_envxv_p: GetTexEnvxvT,
    get_tex_image_p: GetTexImageT,
    get_tex_level_parameterfv_p: GetTexLevelParameterfvT,
    get_tex_level_parameteriv_p: GetTexLevelParameterivT,
    get_tex_parameter_iiv_p: GetTexParameterIivT,
    get_tex_parameter_iuiv_p: GetTexParameterIuivT,
    get_tex_parameterfv_p: GetTexParameterfvT,
    get_tex_parameteriv_p: GetTexParameterivT,
    get_tex_parameterxv_p: GetTexParameterxvT,
    get_transform_feedback_varying_p: GetTransformFeedbackVaryingT,
    get_uniform_block_index_p: GetUniformBlockIndexT,
    get_uniform_indices_p: GetUniformIndicesT,
    get_uniform_location_p: GetUniformLocationT,
    get_uniformfv_p: GetUniformfvT,
    get_uniformiv_p: GetUniformivT,
    get_uniformuiv_p: GetUniformuivT,
    get_vertex_attrib_iiv_p: GetVertexAttribIivT,
    get_vertex_attrib_iuiv_p: GetVertexAttribIuivT,
    get_vertex_attrib_pointerv_p: GetVertexAttribPointervT,
    get_vertex_attribdv_p: GetVertexAttribdvT,
    get_vertex_attribfv_p: GetVertexAttribfvT,
    get_vertex_attribiv_p: GetVertexAttribivT,
    getn_uniformfv_p: GetnUniformfvT,
    getn_uniformiv_p: GetnUniformivT,
    getn_uniformuiv_p: GetnUniformuivT,
    hint_p: HintT,
    invalidate_framebuffer_p: InvalidateFramebufferT,
    invalidate_sub_framebuffer_p: InvalidateSubFramebufferT,
    is_buffer_p: IsBufferT,
    is_enabled_p: IsEnabledT,
    is_enabledi_p: IsEnablediT,
    is_framebuffer_p: IsFramebufferT,
    is_program_p: IsProgramT,
    is_program_pipeline_p: IsProgramPipelineT,
    is_query_p: IsQueryT,
    is_renderbuffer_p: IsRenderbufferT,
    is_sampler_p: IsSamplerT,
    is_shader_p: IsShaderT,
    is_sync_p: IsSyncT,
    is_texture_p: IsTextureT,
    is_transform_feedback_p: IsTransformFeedbackT,
    is_vertex_array_p: IsVertexArrayT,
    light_modelf_p: LightModelfT,
    light_modelfv_p: LightModelfvT,
    light_modelx_p: LightModelxT,
    light_modelxv_p: LightModelxvT,
    lightf_p: LightfT,
    lightfv_p: LightfvT,
    lightx_p: LightxT,
    lightxv_p: LightxvT,
    line_width_p: LineWidthT,
    line_widthx_p: LineWidthxT,
    link_program_p: LinkProgramT,
    load_identity_p: LoadIdentityT,
    load_matrixf_p: LoadMatrixfT,
    load_matrixx_p: LoadMatrixxT,
    logic_op_p: LogicOpT,
    map_buffer_p: MapBufferT,
    map_buffer_range_p: MapBufferRangeT,
    materialf_p: MaterialfT,
    materialfv_p: MaterialfvT,
    materialx_p: MaterialxT,
    materialxv_p: MaterialxvT,
    matrix_mode_p: MatrixModeT,
    memory_barrier_p: MemoryBarrierT,
    memory_barrier_by_region_p: MemoryBarrierByRegionT,
    min_sample_shading_p: MinSampleShadingT,
    mult_matrixf_p: MultMatrixfT,
    mult_matrixx_p: MultMatrixxT,
    multi_draw_arrays_p: MultiDrawArraysT,
    multi_draw_elements_p: MultiDrawElementsT,
    multi_draw_elements_base_vertex_p: MultiDrawElementsBaseVertexT,
    multi_tex_coord4f_p: MultiTexCoord4fT,
    multi_tex_coord4x_p: MultiTexCoord4xT,
    multi_tex_coord_p1ui_p: MultiTexCoordP1uiT,
    multi_tex_coord_p1uiv_p: MultiTexCoordP1uivT,
    multi_tex_coord_p2ui_p: MultiTexCoordP2uiT,
    multi_tex_coord_p2uiv_p: MultiTexCoordP2uivT,
    multi_tex_coord_p3ui_p: MultiTexCoordP3uiT,
    multi_tex_coord_p3uiv_p: MultiTexCoordP3uivT,
    multi_tex_coord_p4ui_p: MultiTexCoordP4uiT,
    multi_tex_coord_p4uiv_p: MultiTexCoordP4uivT,
    normal3f_p: Normal3fT,
    normal3x_p: Normal3xT,
    normal_p3ui_p: NormalP3uiT,
    normal_p3uiv_p: NormalP3uivT,
    normal_pointer_p: NormalPointerT,
    object_label_p: ObjectLabelT,
    object_ptr_label_p: ObjectPtrLabelT,
    orthof_p: OrthofT,
    orthox_p: OrthoxT,
    patch_parameteri_p: PatchParameteriT,
    pause_transform_feedback_p: PauseTransformFeedbackT,
    pixel_storef_p: PixelStorefT,
    pixel_storei_p: PixelStoreiT,
    point_parameterf_p: PointParameterfT,
    point_parameterfv_p: PointParameterfvT,
    point_parameteri_p: PointParameteriT,
    point_parameteriv_p: PointParameterivT,
    point_parameterx_p: PointParameterxT,
    point_parameterxv_p: PointParameterxvT,
    point_size_p: PointSizeT,
    point_sizex_p: PointSizexT,
    polygon_mode_p: PolygonModeT,
    polygon_offset_p: PolygonOffsetT,
    polygon_offsetx_p: PolygonOffsetxT,
    pop_debug_group_p: PopDebugGroupT,
    pop_matrix_p: PopMatrixT,
    primitive_bounding_box_p: PrimitiveBoundingBoxT,
    primitive_restart_index_p: PrimitiveRestartIndexT,
    program_binary_p: ProgramBinaryT,
    program_parameteri_p: ProgramParameteriT,
    program_uniform1f_p: ProgramUniform1fT,
    program_uniform1fv_p: ProgramUniform1fvT,
    program_uniform1i_p: ProgramUniform1iT,
    program_uniform1iv_p: ProgramUniform1ivT,
    program_uniform1ui_p: ProgramUniform1uiT,
    program_uniform1uiv_p: ProgramUniform1uivT,
    program_uniform2f_p: ProgramUniform2fT,
    program_uniform2fv_p: ProgramUniform2fvT,
    program_uniform2i_p: ProgramUniform2iT,
    program_uniform2iv_p: ProgramUniform2ivT,
    program_uniform2ui_p: ProgramUniform2uiT,
    program_uniform2uiv_p: ProgramUniform2uivT,
    program_uniform3f_p: ProgramUniform3fT,
    program_uniform3fv_p: ProgramUniform3fvT,
    program_uniform3i_p: ProgramUniform3iT,
    program_uniform3iv_p: ProgramUniform3ivT,
    program_uniform3ui_p: ProgramUniform3uiT,
    program_uniform3uiv_p: ProgramUniform3uivT,
    program_uniform4f_p: ProgramUniform4fT,
    program_uniform4fv_p: ProgramUniform4fvT,
    program_uniform4i_p: ProgramUniform4iT,
    program_uniform4iv_p: ProgramUniform4ivT,
    program_uniform4ui_p: ProgramUniform4uiT,
    program_uniform4uiv_p: ProgramUniform4uivT,
    program_uniform_matrix2fv_p: ProgramUniformMatrix2fvT,
    program_uniform_matrix2x3fv_p: ProgramUniformMatrix2x3fvT,
    program_uniform_matrix2x4fv_p: ProgramUniformMatrix2x4fvT,
    program_uniform_matrix3fv_p: ProgramUniformMatrix3fvT,
    program_uniform_matrix3x2fv_p: ProgramUniformMatrix3x2fvT,
    program_uniform_matrix3x4fv_p: ProgramUniformMatrix3x4fvT,
    program_uniform_matrix4fv_p: ProgramUniformMatrix4fvT,
    program_uniform_matrix4x2fv_p: ProgramUniformMatrix4x2fvT,
    program_uniform_matrix4x3fv_p: ProgramUniformMatrix4x3fvT,
    provoking_vertex_p: ProvokingVertexT,
    push_debug_group_p: PushDebugGroupT,
    push_matrix_p: PushMatrixT,
    query_counter_p: QueryCounterT,
    read_buffer_p: ReadBufferT,
    read_pixels_p: ReadPixelsT,
    readn_pixels_p: ReadnPixelsT,
    release_shader_compiler_p: ReleaseShaderCompilerT,
    renderbuffer_storage_p: RenderbufferStorageT,
    renderbuffer_storage_multisample_p: RenderbufferStorageMultisampleT,
    resume_transform_feedback_p: ResumeTransformFeedbackT,
    rotatef_p: RotatefT,
    rotatex_p: RotatexT,
    sample_coverage_p: SampleCoverageT,
    sample_coveragex_p: SampleCoveragexT,
    sample_maski_p: SampleMaskiT,
    sampler_parameter_iiv_p: SamplerParameterIivT,
    sampler_parameter_iuiv_p: SamplerParameterIuivT,
    sampler_parameterf_p: SamplerParameterfT,
    sampler_parameterfv_p: SamplerParameterfvT,
    sampler_parameteri_p: SamplerParameteriT,
    sampler_parameteriv_p: SamplerParameterivT,
    scalef_p: ScalefT,
    scalex_p: ScalexT,
    scissor_p: ScissorT,
    secondary_color_p3ui_p: SecondaryColorP3uiT,
    secondary_color_p3uiv_p: SecondaryColorP3uivT,
    shade_model_p: ShadeModelT,
    shader_binary_p: ShaderBinaryT,
    shader_source_p: ShaderSourceT,
    stencil_func_p: StencilFuncT,
    stencil_func_separate_p: StencilFuncSeparateT,
    stencil_mask_p: StencilMaskT,
    stencil_mask_separate_p: StencilMaskSeparateT,
    stencil_op_p: StencilOpT,
    stencil_op_separate_p: StencilOpSeparateT,
    tex_buffer_p: TexBufferT,
    tex_buffer_range_p: TexBufferRangeT,
    tex_coord_p1ui_p: TexCoordP1uiT,
    tex_coord_p1uiv_p: TexCoordP1uivT,
    tex_coord_p2ui_p: TexCoordP2uiT,
    tex_coord_p2uiv_p: TexCoordP2uivT,
    tex_coord_p3ui_p: TexCoordP3uiT,
    tex_coord_p3uiv_p: TexCoordP3uivT,
    tex_coord_p4ui_p: TexCoordP4uiT,
    tex_coord_p4uiv_p: TexCoordP4uivT,
    tex_coord_pointer_p: TexCoordPointerT,
    tex_envf_p: TexEnvfT,
    tex_envfv_p: TexEnvfvT,
    tex_envi_p: TexEnviT,
    tex_enviv_p: TexEnvivT,
    tex_envx_p: TexEnvxT,
    tex_envxv_p: TexEnvxvT,
    tex_image1_d_p: TexImage1DT,
    tex_image2_d_p: TexImage2DT,
    tex_image2_d_multisample_p: TexImage2DMultisampleT,
    tex_image3_d_p: TexImage3DT,
    tex_image3_d_multisample_p: TexImage3DMultisampleT,
    tex_parameter_iiv_p: TexParameterIivT,
    tex_parameter_iuiv_p: TexParameterIuivT,
    tex_parameterf_p: TexParameterfT,
    tex_parameterfv_p: TexParameterfvT,
    tex_parameteri_p: TexParameteriT,
    tex_parameteriv_p: TexParameterivT,
    tex_parameterx_p: TexParameterxT,
    tex_parameterxv_p: TexParameterxvT,
    tex_storage2_d_p: TexStorage2DT,
    tex_storage2_d_multisample_p: TexStorage2DMultisampleT,
    tex_storage3_d_p: TexStorage3DT,
    tex_storage3_d_multisample_p: TexStorage3DMultisampleT,
    tex_sub_image1_d_p: TexSubImage1DT,
    tex_sub_image2_d_p: TexSubImage2DT,
    tex_sub_image3_d_p: TexSubImage3DT,
    transform_feedback_varyings_p: TransformFeedbackVaryingsT,
    translatef_p: TranslatefT,
    translatex_p: TranslatexT,
    uniform1f_p: Uniform1fT,
    uniform1fv_p: Uniform1fvT,
    uniform1i_p: Uniform1iT,
    uniform1iv_p: Uniform1ivT,
    uniform1ui_p: Uniform1uiT,
    uniform1uiv_p: Uniform1uivT,
    uniform2f_p: Uniform2fT,
    uniform2fv_p: Uniform2fvT,
    uniform2i_p: Uniform2iT,
    uniform2iv_p: Uniform2ivT,
    uniform2ui_p: Uniform2uiT,
    uniform2uiv_p: Uniform2uivT,
    uniform3f_p: Uniform3fT,
    uniform3fv_p: Uniform3fvT,
    uniform3i_p: Uniform3iT,
    uniform3iv_p: Uniform3ivT,
    uniform3ui_p: Uniform3uiT,
    uniform3uiv_p: Uniform3uivT,
    uniform4f_p: Uniform4fT,
    uniform4fv_p: Uniform4fvT,
    uniform4i_p: Uniform4iT,
    uniform4iv_p: Uniform4ivT,
    uniform4ui_p: Uniform4uiT,
    uniform4uiv_p: Uniform4uivT,
    uniform_block_binding_p: UniformBlockBindingT,
    uniform_matrix2fv_p: UniformMatrix2fvT,
    uniform_matrix2x3fv_p: UniformMatrix2x3fvT,
    uniform_matrix2x4fv_p: UniformMatrix2x4fvT,
    uniform_matrix3fv_p: UniformMatrix3fvT,
    uniform_matrix3x2fv_p: UniformMatrix3x2fvT,
    uniform_matrix3x4fv_p: UniformMatrix3x4fvT,
    uniform_matrix4fv_p: UniformMatrix4fvT,
    uniform_matrix4x2fv_p: UniformMatrix4x2fvT,
    uniform_matrix4x3fv_p: UniformMatrix4x3fvT,
    unmap_buffer_p: UnmapBufferT,
    use_program_p: UseProgramT,
    use_program_stages_p: UseProgramStagesT,
    validate_program_p: ValidateProgramT,
    validate_program_pipeline_p: ValidateProgramPipelineT,
    vertex_attrib1d_p: VertexAttrib1dT,
    vertex_attrib1dv_p: VertexAttrib1dvT,
    vertex_attrib1f_p: VertexAttrib1fT,
    vertex_attrib1fv_p: VertexAttrib1fvT,
    vertex_attrib1s_p: VertexAttrib1sT,
    vertex_attrib1sv_p: VertexAttrib1svT,
    vertex_attrib2d_p: VertexAttrib2dT,
    vertex_attrib2dv_p: VertexAttrib2dvT,
    vertex_attrib2f_p: VertexAttrib2fT,
    vertex_attrib2fv_p: VertexAttrib2fvT,
    vertex_attrib2s_p: VertexAttrib2sT,
    vertex_attrib2sv_p: VertexAttrib2svT,
    vertex_attrib3d_p: VertexAttrib3dT,
    vertex_attrib3dv_p: VertexAttrib3dvT,
    vertex_attrib3f_p: VertexAttrib3fT,
    vertex_attrib3fv_p: VertexAttrib3fvT,
    vertex_attrib3s_p: VertexAttrib3sT,
    vertex_attrib3sv_p: VertexAttrib3svT,
    vertex_attrib4_nbv_p: VertexAttrib4NbvT,
    vertex_attrib4_niv_p: VertexAttrib4NivT,
    vertex_attrib4_nsv_p: VertexAttrib4NsvT,
    vertex_attrib4_nub_p: VertexAttrib4NubT,
    vertex_attrib4_nubv_p: VertexAttrib4NubvT,
    vertex_attrib4_nuiv_p: VertexAttrib4NuivT,
    vertex_attrib4_nusv_p: VertexAttrib4NusvT,
    vertex_attrib4bv_p: VertexAttrib4bvT,
    vertex_attrib4d_p: VertexAttrib4dT,
    vertex_attrib4dv_p: VertexAttrib4dvT,
    vertex_attrib4f_p: VertexAttrib4fT,
    vertex_attrib4fv_p: VertexAttrib4fvT,
    vertex_attrib4iv_p: VertexAttrib4ivT,
    vertex_attrib4s_p: VertexAttrib4sT,
    vertex_attrib4sv_p: VertexAttrib4svT,
    vertex_attrib4ubv_p: VertexAttrib4ubvT,
    vertex_attrib4uiv_p: VertexAttrib4uivT,
    vertex_attrib4usv_p: VertexAttrib4usvT,
    vertex_attrib_binding_p: VertexAttribBindingT,
    vertex_attrib_divisor_p: VertexAttribDivisorT,
    vertex_attrib_format_p: VertexAttribFormatT,
    vertex_attrib_i1i_p: VertexAttribI1iT,
    vertex_attrib_i1iv_p: VertexAttribI1ivT,
    vertex_attrib_i1ui_p: VertexAttribI1uiT,
    vertex_attrib_i1uiv_p: VertexAttribI1uivT,
    vertex_attrib_i2i_p: VertexAttribI2iT,
    vertex_attrib_i2iv_p: VertexAttribI2ivT,
    vertex_attrib_i2ui_p: VertexAttribI2uiT,
    vertex_attrib_i2uiv_p: VertexAttribI2uivT,
    vertex_attrib_i3i_p: VertexAttribI3iT,
    vertex_attrib_i3iv_p: VertexAttribI3ivT,
    vertex_attrib_i3ui_p: VertexAttribI3uiT,
    vertex_attrib_i3uiv_p: VertexAttribI3uivT,
    vertex_attrib_i4bv_p: VertexAttribI4bvT,
    vertex_attrib_i4i_p: VertexAttribI4iT,
    vertex_attrib_i4iv_p: VertexAttribI4ivT,
    vertex_attrib_i4sv_p: VertexAttribI4svT,
    vertex_attrib_i4ubv_p: VertexAttribI4ubvT,
    vertex_attrib_i4ui_p: VertexAttribI4uiT,
    vertex_attrib_i4uiv_p: VertexAttribI4uivT,
    vertex_attrib_i4usv_p: VertexAttribI4usvT,
    vertex_attrib_i_format_p: VertexAttribIFormatT,
    vertex_attrib_i_pointer_p: VertexAttribIPointerT,
    vertex_attrib_p1ui_p: VertexAttribP1uiT,
    vertex_attrib_p1uiv_p: VertexAttribP1uivT,
    vertex_attrib_p2ui_p: VertexAttribP2uiT,
    vertex_attrib_p2uiv_p: VertexAttribP2uivT,
    vertex_attrib_p3ui_p: VertexAttribP3uiT,
    vertex_attrib_p3uiv_p: VertexAttribP3uivT,
    vertex_attrib_p4ui_p: VertexAttribP4uiT,
    vertex_attrib_p4uiv_p: VertexAttribP4uivT,
    vertex_attrib_pointer_p: VertexAttribPointerT,
    vertex_binding_divisor_p: VertexBindingDivisorT,
    vertex_p2ui_p: VertexP2uiT,
    vertex_p2uiv_p: VertexP2uivT,
    vertex_p3ui_p: VertexP3uiT,
    vertex_p3uiv_p: VertexP3uivT,
    vertex_p4ui_p: VertexP4uiT,
    vertex_p4uiv_p: VertexP4uivT,
    vertex_pointer_p: VertexPointerT,
    viewport_p: ViewportT,
    wait_sync_p: WaitSyncT,
}

impl GlCmd {
    pub fn load<FnLoad>(loader: FnLoad) -> Self
    where
        FnLoad: Fn(*const c_char) -> *mut c_void,
    {
        Self {
            active_shader_program_p: load(&loader, "glActiveShaderProgram"),
            active_texture_p: load(&loader, "glActiveTexture"),
            alpha_func_p: load(&loader, "glAlphaFunc"),
            alpha_funcx_p: load(&loader, "glAlphaFuncx"),
            attach_shader_p: load(&loader, "glAttachShader"),
            begin_conditional_render_p: load(&loader, "glBeginConditionalRender"),
            begin_query_p: load(&loader, "glBeginQuery"),
            begin_transform_feedback_p: load(&loader, "glBeginTransformFeedback"),
            bind_attrib_location_p: load(&loader, "glBindAttribLocation"),
            bind_buffer_p: load(&loader, "glBindBuffer"),
            bind_buffer_base_p: load(&loader, "glBindBufferBase"),
            bind_buffer_range_p: load(&loader, "glBindBufferRange"),
            bind_frag_data_location_p: load(&loader, "glBindFragDataLocation"),
            bind_frag_data_location_indexed_p: load(&loader, "glBindFragDataLocationIndexed"),
            bind_framebuffer_p: load(&loader, "glBindFramebuffer"),
            bind_image_texture_p: load(&loader, "glBindImageTexture"),
            bind_program_pipeline_p: load(&loader, "glBindProgramPipeline"),
            bind_renderbuffer_p: load(&loader, "glBindRenderbuffer"),
            bind_sampler_p: load(&loader, "glBindSampler"),
            bind_texture_p: load(&loader, "glBindTexture"),
            bind_transform_feedback_p: load(&loader, "glBindTransformFeedback"),
            bind_vertex_array_p: load(&loader, "glBindVertexArray"),
            bind_vertex_buffer_p: load(&loader, "glBindVertexBuffer"),
            blend_barrier_p: load(&loader, "glBlendBarrier"),
            blend_color_p: load(&loader, "glBlendColor"),
            blend_equation_p: load(&loader, "glBlendEquation"),
            blend_equation_separate_p: load(&loader, "glBlendEquationSeparate"),
            blend_equation_separatei_p: load(&loader, "glBlendEquationSeparatei"),
            blend_equationi_p: load(&loader, "glBlendEquationi"),
            blend_func_p: load(&loader, "glBlendFunc"),
            blend_func_separate_p: load(&loader, "glBlendFuncSeparate"),
            blend_func_separatei_p: load(&loader, "glBlendFuncSeparatei"),
            blend_funci_p: load(&loader, "glBlendFunci"),
            blit_framebuffer_p: load(&loader, "glBlitFramebuffer"),
            buffer_data_p: load(&loader, "glBufferData"),
            buffer_sub_data_p: load(&loader, "glBufferSubData"),
            check_framebuffer_status_p: load(&loader, "glCheckFramebufferStatus"),
            clamp_color_p: load(&loader, "glClampColor"),
            clear_p: load(&loader, "glClear"),
            clear_bufferfi_p: load(&loader, "glClearBufferfi"),
            clear_bufferfv_p: load(&loader, "glClearBufferfv"),
            clear_bufferiv_p: load(&loader, "glClearBufferiv"),
            clear_bufferuiv_p: load(&loader, "glClearBufferuiv"),
            clear_color_p: load(&loader, "glClearColor"),
            clear_colorx_p: load(&loader, "glClearColorx"),
            clear_depth_p: load(&loader, "glClearDepth"),
            clear_depthf_p: load(&loader, "glClearDepthf"),
            clear_depthx_p: load(&loader, "glClearDepthx"),
            clear_stencil_p: load(&loader, "glClearStencil"),
            client_active_texture_p: load(&loader, "glClientActiveTexture"),
            client_wait_sync_p: load(&loader, "glClientWaitSync"),
            clip_planef_p: load(&loader, "glClipPlanef"),
            clip_planex_p: load(&loader, "glClipPlanex"),
            color4f_p: load(&loader, "glColor4f"),
            color4ub_p: load(&loader, "glColor4ub"),
            color4x_p: load(&loader, "glColor4x"),
            color_mask_p: load(&loader, "glColorMask"),
            color_maski_p: load(&loader, "glColorMaski"),
            color_p3ui_p: load(&loader, "glColorP3ui"),
            color_p3uiv_p: load(&loader, "glColorP3uiv"),
            color_p4ui_p: load(&loader, "glColorP4ui"),
            color_p4uiv_p: load(&loader, "glColorP4uiv"),
            color_pointer_p: load(&loader, "glColorPointer"),
            compile_shader_p: load(&loader, "glCompileShader"),
            compressed_tex_image1_d_p: load(&loader, "glCompressedTexImage1D"),
            compressed_tex_image2_d_p: load(&loader, "glCompressedTexImage2D"),
            compressed_tex_image3_d_p: load(&loader, "glCompressedTexImage3D"),
            compressed_tex_sub_image1_d_p: load(&loader, "glCompressedTexSubImage1D"),
            compressed_tex_sub_image2_d_p: load(&loader, "glCompressedTexSubImage2D"),
            compressed_tex_sub_image3_d_p: load(&loader, "glCompressedTexSubImage3D"),
            copy_buffer_sub_data_p: load(&loader, "glCopyBufferSubData"),
            copy_image_sub_data_p: load(&loader, "glCopyImageSubData"),
            copy_tex_image1_d_p: load(&loader, "glCopyTexImage1D"),
            copy_tex_image2_d_p: load(&loader, "glCopyTexImage2D"),
            copy_tex_sub_image1_d_p: load(&loader, "glCopyTexSubImage1D"),
            copy_tex_sub_image2_d_p: load(&loader, "glCopyTexSubImage2D"),
            copy_tex_sub_image3_d_p: load(&loader, "glCopyTexSubImage3D"),
            create_program_p: load(&loader, "glCreateProgram"),
            create_shader_p: load(&loader, "glCreateShader"),
            create_shader_programv_p: load(&loader, "glCreateShaderProgramv"),
            cull_face_p: load(&loader, "glCullFace"),
            debug_message_callback_p: load(&loader, "glDebugMessageCallback"),
            debug_message_control_p: load(&loader, "glDebugMessageControl"),
            debug_message_insert_p: load(&loader, "glDebugMessageInsert"),
            delete_buffers_p: load(&loader, "glDeleteBuffers"),
            delete_framebuffers_p: load(&loader, "glDeleteFramebuffers"),
            delete_program_p: load(&loader, "glDeleteProgram"),
            delete_program_pipelines_p: load(&loader, "glDeleteProgramPipelines"),
            delete_queries_p: load(&loader, "glDeleteQueries"),
            delete_renderbuffers_p: load(&loader, "glDeleteRenderbuffers"),
            delete_samplers_p: load(&loader, "glDeleteSamplers"),
            delete_shader_p: load(&loader, "glDeleteShader"),
            delete_sync_p: load(&loader, "glDeleteSync"),
            delete_textures_p: load(&loader, "glDeleteTextures"),
            delete_transform_feedbacks_p: load(&loader, "glDeleteTransformFeedbacks"),
            delete_vertex_arrays_p: load(&loader, "glDeleteVertexArrays"),
            depth_func_p: load(&loader, "glDepthFunc"),
            depth_mask_p: load(&loader, "glDepthMask"),
            depth_range_p: load(&loader, "glDepthRange"),
            depth_rangef_p: load(&loader, "glDepthRangef"),
            depth_rangex_p: load(&loader, "glDepthRangex"),
            detach_shader_p: load(&loader, "glDetachShader"),
            disable_p: load(&loader, "glDisable"),
            disable_client_state_p: load(&loader, "glDisableClientState"),
            disable_vertex_attrib_array_p: load(&loader, "glDisableVertexAttribArray"),
            disablei_p: load(&loader, "glDisablei"),
            dispatch_compute_p: load(&loader, "glDispatchCompute"),
            dispatch_compute_indirect_p: load(&loader, "glDispatchComputeIndirect"),
            draw_arrays_p: load(&loader, "glDrawArrays"),
            draw_arrays_indirect_p: load(&loader, "glDrawArraysIndirect"),
            draw_arrays_instanced_p: load(&loader, "glDrawArraysInstanced"),
            draw_buffer_p: load(&loader, "glDrawBuffer"),
            draw_buffers_p: load(&loader, "glDrawBuffers"),
            draw_elements_p: load(&loader, "glDrawElements"),
            draw_elements_base_vertex_p: load(&loader, "glDrawElementsBaseVertex"),
            draw_elements_indirect_p: load(&loader, "glDrawElementsIndirect"),
            draw_elements_instanced_p: load(&loader, "glDrawElementsInstanced"),
            draw_elements_instanced_base_vertex_p: load(
                &loader,
                "glDrawElementsInstancedBaseVertex",
            ),
            draw_range_elements_p: load(&loader, "glDrawRangeElements"),
            draw_range_elements_base_vertex_p: load(&loader, "glDrawRangeElementsBaseVertex"),
            enable_p: load(&loader, "glEnable"),
            enable_client_state_p: load(&loader, "glEnableClientState"),
            enable_vertex_attrib_array_p: load(&loader, "glEnableVertexAttribArray"),
            enablei_p: load(&loader, "glEnablei"),
            end_conditional_render_p: load(&loader, "glEndConditionalRender"),
            end_query_p: load(&loader, "glEndQuery"),
            end_transform_feedback_p: load(&loader, "glEndTransformFeedback"),
            fence_sync_p: load(&loader, "glFenceSync"),
            finish_p: load(&loader, "glFinish"),
            flush_p: load(&loader, "glFlush"),
            flush_mapped_buffer_range_p: load(&loader, "glFlushMappedBufferRange"),
            fogf_p: load(&loader, "glFogf"),
            fogfv_p: load(&loader, "glFogfv"),
            fogx_p: load(&loader, "glFogx"),
            fogxv_p: load(&loader, "glFogxv"),
            framebuffer_parameteri_p: load(&loader, "glFramebufferParameteri"),
            framebuffer_renderbuffer_p: load(&loader, "glFramebufferRenderbuffer"),
            framebuffer_texture_p: load(&loader, "glFramebufferTexture"),
            framebuffer_texture1_d_p: load(&loader, "glFramebufferTexture1D"),
            framebuffer_texture2_d_p: load(&loader, "glFramebufferTexture2D"),
            framebuffer_texture3_d_p: load(&loader, "glFramebufferTexture3D"),
            framebuffer_texture_layer_p: load(&loader, "glFramebufferTextureLayer"),
            front_face_p: load(&loader, "glFrontFace"),
            frustumf_p: load(&loader, "glFrustumf"),
            frustumx_p: load(&loader, "glFrustumx"),
            gen_buffers_p: load(&loader, "glGenBuffers"),
            gen_framebuffers_p: load(&loader, "glGenFramebuffers"),
            gen_program_pipelines_p: load(&loader, "glGenProgramPipelines"),
            gen_queries_p: load(&loader, "glGenQueries"),
            gen_renderbuffers_p: load(&loader, "glGenRenderbuffers"),
            gen_samplers_p: load(&loader, "glGenSamplers"),
            gen_textures_p: load(&loader, "glGenTextures"),
            gen_transform_feedbacks_p: load(&loader, "glGenTransformFeedbacks"),
            gen_vertex_arrays_p: load(&loader, "glGenVertexArrays"),
            generate_mipmap_p: load(&loader, "glGenerateMipmap"),
            get_active_attrib_p: load(&loader, "glGetActiveAttrib"),
            get_active_uniform_p: load(&loader, "glGetActiveUniform"),
            get_active_uniform_block_name_p: load(&loader, "glGetActiveUniformBlockName"),
            get_active_uniform_blockiv_p: load(&loader, "glGetActiveUniformBlockiv"),
            get_active_uniform_name_p: load(&loader, "glGetActiveUniformName"),
            get_active_uniformsiv_p: load(&loader, "glGetActiveUniformsiv"),
            get_attached_shaders_p: load(&loader, "glGetAttachedShaders"),
            get_attrib_location_p: load(&loader, "glGetAttribLocation"),
            get_booleani_v_p: load(&loader, "glGetBooleani_v"),
            get_booleanv_p: load(&loader, "glGetBooleanv"),
            get_buffer_parameteri64v_p: load(&loader, "glGetBufferParameteri64v"),
            get_buffer_parameteriv_p: load(&loader, "glGetBufferParameteriv"),
            get_buffer_pointerv_p: load(&loader, "glGetBufferPointerv"),
            get_buffer_sub_data_p: load(&loader, "glGetBufferSubData"),
            get_clip_planef_p: load(&loader, "glGetClipPlanef"),
            get_clip_planex_p: load(&loader, "glGetClipPlanex"),
            get_compressed_tex_image_p: load(&loader, "glGetCompressedTexImage"),
            get_debug_message_log_p: load(&loader, "glGetDebugMessageLog"),
            get_doublev_p: load(&loader, "glGetDoublev"),
            get_error_p: load(&loader, "glGetError"),
            get_fixedv_p: load(&loader, "glGetFixedv"),
            get_floatv_p: load(&loader, "glGetFloatv"),
            get_frag_data_index_p: load(&loader, "glGetFragDataIndex"),
            get_frag_data_location_p: load(&loader, "glGetFragDataLocation"),
            get_framebuffer_attachment_parameteriv_p: load(
                &loader,
                "glGetFramebufferAttachmentParameteriv",
            ),
            get_framebuffer_parameteriv_p: load(&loader, "glGetFramebufferParameteriv"),
            get_graphics_reset_status_p: load(&loader, "glGetGraphicsResetStatus"),
            get_integer64i_v_p: load(&loader, "glGetInteger64i_v"),
            get_integer64v_p: load(&loader, "glGetInteger64v"),
            get_integeri_v_p: load(&loader, "glGetIntegeri_v"),
            get_integerv_p: load(&loader, "glGetIntegerv"),
            get_internalformativ_p: load(&loader, "glGetInternalformativ"),
            get_lightfv_p: load(&loader, "glGetLightfv"),
            get_lightxv_p: load(&loader, "glGetLightxv"),
            get_materialfv_p: load(&loader, "glGetMaterialfv"),
            get_materialxv_p: load(&loader, "glGetMaterialxv"),
            get_multisamplefv_p: load(&loader, "glGetMultisamplefv"),
            get_object_label_p: load(&loader, "glGetObjectLabel"),
            get_object_ptr_label_p: load(&loader, "glGetObjectPtrLabel"),
            get_pointerv_p: load(&loader, "glGetPointerv"),
            get_program_binary_p: load(&loader, "glGetProgramBinary"),
            get_program_info_log_p: load(&loader, "glGetProgramInfoLog"),
            get_program_interfaceiv_p: load(&loader, "glGetProgramInterfaceiv"),
            get_program_pipeline_info_log_p: load(&loader, "glGetProgramPipelineInfoLog"),
            get_program_pipelineiv_p: load(&loader, "glGetProgramPipelineiv"),
            get_program_resource_index_p: load(&loader, "glGetProgramResourceIndex"),
            get_program_resource_location_p: load(&loader, "glGetProgramResourceLocation"),
            get_program_resource_name_p: load(&loader, "glGetProgramResourceName"),
            get_program_resourceiv_p: load(&loader, "glGetProgramResourceiv"),
            get_programiv_p: load(&loader, "glGetProgramiv"),
            get_query_objecti64v_p: load(&loader, "glGetQueryObjecti64v"),
            get_query_objectiv_p: load(&loader, "glGetQueryObjectiv"),
            get_query_objectui64v_p: load(&loader, "glGetQueryObjectui64v"),
            get_query_objectuiv_p: load(&loader, "glGetQueryObjectuiv"),
            get_queryiv_p: load(&loader, "glGetQueryiv"),
            get_renderbuffer_parameteriv_p: load(&loader, "glGetRenderbufferParameteriv"),
            get_sampler_parameter_iiv_p: load(&loader, "glGetSamplerParameterIiv"),
            get_sampler_parameter_iuiv_p: load(&loader, "glGetSamplerParameterIuiv"),
            get_sampler_parameterfv_p: load(&loader, "glGetSamplerParameterfv"),
            get_sampler_parameteriv_p: load(&loader, "glGetSamplerParameteriv"),
            get_shader_info_log_p: load(&loader, "glGetShaderInfoLog"),
            get_shader_precision_format_p: load(&loader, "glGetShaderPrecisionFormat"),
            get_shader_source_p: load(&loader, "glGetShaderSource"),
            get_shaderiv_p: load(&loader, "glGetShaderiv"),
            get_string_p: load(&loader, "glGetString"),
            get_stringi_p: load(&loader, "glGetStringi"),
            get_synciv_p: load(&loader, "glGetSynciv"),
            get_tex_envfv_p: load(&loader, "glGetTexEnvfv"),
            get_tex_enviv_p: load(&loader, "glGetTexEnviv"),
            get_tex_envxv_p: load(&loader, "glGetTexEnvxv"),
            get_tex_image_p: load(&loader, "glGetTexImage"),
            get_tex_level_parameterfv_p: load(&loader, "glGetTexLevelParameterfv"),
            get_tex_level_parameteriv_p: load(&loader, "glGetTexLevelParameteriv"),
            get_tex_parameter_iiv_p: load(&loader, "glGetTexParameterIiv"),
            get_tex_parameter_iuiv_p: load(&loader, "glGetTexParameterIuiv"),
            get_tex_parameterfv_p: load(&loader, "glGetTexParameterfv"),
            get_tex_parameteriv_p: load(&loader, "glGetTexParameteriv"),
            get_tex_parameterxv_p: load(&loader, "glGetTexParameterxv"),
            get_transform_feedback_varying_p: load(&loader, "glGetTransformFeedbackVarying"),
            get_uniform_block_index_p: load(&loader, "glGetUniformBlockIndex"),
            get_uniform_indices_p: load(&loader, "glGetUniformIndices"),
            get_uniform_location_p: load(&loader, "glGetUniformLocation"),
            get_uniformfv_p: load(&loader, "glGetUniformfv"),
            get_uniformiv_p: load(&loader, "glGetUniformiv"),
            get_uniformuiv_p: load(&loader, "glGetUniformuiv"),
            get_vertex_attrib_iiv_p: load(&loader, "glGetVertexAttribIiv"),
            get_vertex_attrib_iuiv_p: load(&loader, "glGetVertexAttribIuiv"),
            get_vertex_attrib_pointerv_p: load(&loader, "glGetVertexAttribPointerv"),
            get_vertex_attribdv_p: load(&loader, "glGetVertexAttribdv"),
            get_vertex_attribfv_p: load(&loader, "glGetVertexAttribfv"),
            get_vertex_attribiv_p: load(&loader, "glGetVertexAttribiv"),
            getn_uniformfv_p: load(&loader, "glGetnUniformfv"),
            getn_uniformiv_p: load(&loader, "glGetnUniformiv"),
            getn_uniformuiv_p: load(&loader, "glGetnUniformuiv"),
            hint_p: load(&loader, "glHint"),
            invalidate_framebuffer_p: load(&loader, "glInvalidateFramebuffer"),
            invalidate_sub_framebuffer_p: load(&loader, "glInvalidateSubFramebuffer"),
            is_buffer_p: load(&loader, "glIsBuffer"),
            is_enabled_p: load(&loader, "glIsEnabled"),
            is_enabledi_p: load(&loader, "glIsEnabledi"),
            is_framebuffer_p: load(&loader, "glIsFramebuffer"),
            is_program_p: load(&loader, "glIsProgram"),
            is_program_pipeline_p: load(&loader, "glIsProgramPipeline"),
            is_query_p: load(&loader, "glIsQuery"),
            is_renderbuffer_p: load(&loader, "glIsRenderbuffer"),
            is_sampler_p: load(&loader, "glIsSampler"),
            is_shader_p: load(&loader, "glIsShader"),
            is_sync_p: load(&loader, "glIsSync"),
            is_texture_p: load(&loader, "glIsTexture"),
            is_transform_feedback_p: load(&loader, "glIsTransformFeedback"),
            is_vertex_array_p: load(&loader, "glIsVertexArray"),
            light_modelf_p: load(&loader, "glLightModelf"),
            light_modelfv_p: load(&loader, "glLightModelfv"),
            light_modelx_p: load(&loader, "glLightModelx"),
            light_modelxv_p: load(&loader, "glLightModelxv"),
            lightf_p: load(&loader, "glLightf"),
            lightfv_p: load(&loader, "glLightfv"),
            lightx_p: load(&loader, "glLightx"),
            lightxv_p: load(&loader, "glLightxv"),
            line_width_p: load(&loader, "glLineWidth"),
            line_widthx_p: load(&loader, "glLineWidthx"),
            link_program_p: load(&loader, "glLinkProgram"),
            load_identity_p: load(&loader, "glLoadIdentity"),
            load_matrixf_p: load(&loader, "glLoadMatrixf"),
            load_matrixx_p: load(&loader, "glLoadMatrixx"),
            logic_op_p: load(&loader, "glLogicOp"),
            map_buffer_p: load(&loader, "glMapBuffer"),
            map_buffer_range_p: load(&loader, "glMapBufferRange"),
            materialf_p: load(&loader, "glMaterialf"),
            materialfv_p: load(&loader, "glMaterialfv"),
            materialx_p: load(&loader, "glMaterialx"),
            materialxv_p: load(&loader, "glMaterialxv"),
            matrix_mode_p: load(&loader, "glMatrixMode"),
            memory_barrier_p: load(&loader, "glMemoryBarrier"),
            memory_barrier_by_region_p: load(&loader, "glMemoryBarrierByRegion"),
            min_sample_shading_p: load(&loader, "glMinSampleShading"),
            mult_matrixf_p: load(&loader, "glMultMatrixf"),
            mult_matrixx_p: load(&loader, "glMultMatrixx"),
            multi_draw_arrays_p: load(&loader, "glMultiDrawArrays"),
            multi_draw_elements_p: load(&loader, "glMultiDrawElements"),
            multi_draw_elements_base_vertex_p: load(&loader, "glMultiDrawElementsBaseVertex"),
            multi_tex_coord4f_p: load(&loader, "glMultiTexCoord4f"),
            multi_tex_coord4x_p: load(&loader, "glMultiTexCoord4x"),
            multi_tex_coord_p1ui_p: load(&loader, "glMultiTexCoordP1ui"),
            multi_tex_coord_p1uiv_p: load(&loader, "glMultiTexCoordP1uiv"),
            multi_tex_coord_p2ui_p: load(&loader, "glMultiTexCoordP2ui"),
            multi_tex_coord_p2uiv_p: load(&loader, "glMultiTexCoordP2uiv"),
            multi_tex_coord_p3ui_p: load(&loader, "glMultiTexCoordP3ui"),
            multi_tex_coord_p3uiv_p: load(&loader, "glMultiTexCoordP3uiv"),
            multi_tex_coord_p4ui_p: load(&loader, "glMultiTexCoordP4ui"),
            multi_tex_coord_p4uiv_p: load(&loader, "glMultiTexCoordP4uiv"),
            normal3f_p: load(&loader, "glNormal3f"),
            normal3x_p: load(&loader, "glNormal3x"),
            normal_p3ui_p: load(&loader, "glNormalP3ui"),
            normal_p3uiv_p: load(&loader, "glNormalP3uiv"),
            normal_pointer_p: load(&loader, "glNormalPointer"),
            object_label_p: load(&loader, "glObjectLabel"),
            object_ptr_label_p: load(&loader, "glObjectPtrLabel"),
            orthof_p: load(&loader, "glOrthof"),
            orthox_p: load(&loader, "glOrthox"),
            patch_parameteri_p: load(&loader, "glPatchParameteri"),
            pause_transform_feedback_p: load(&loader, "glPauseTransformFeedback"),
            pixel_storef_p: load(&loader, "glPixelStoref"),
            pixel_storei_p: load(&loader, "glPixelStorei"),
            point_parameterf_p: load(&loader, "glPointParameterf"),
            point_parameterfv_p: load(&loader, "glPointParameterfv"),
            point_parameteri_p: load(&loader, "glPointParameteri"),
            point_parameteriv_p: load(&loader, "glPointParameteriv"),
            point_parameterx_p: load(&loader, "glPointParameterx"),
            point_parameterxv_p: load(&loader, "glPointParameterxv"),
            point_size_p: load(&loader, "glPointSize"),
            point_sizex_p: load(&loader, "glPointSizex"),
            polygon_mode_p: load(&loader, "glPolygonMode"),
            polygon_offset_p: load(&loader, "glPolygonOffset"),
            polygon_offsetx_p: load(&loader, "glPolygonOffsetx"),
            pop_debug_group_p: load(&loader, "glPopDebugGroup"),
            pop_matrix_p: load(&loader, "glPopMatrix"),
            primitive_bounding_box_p: load(&loader, "glPrimitiveBoundingBox"),
            primitive_restart_index_p: load(&loader, "glPrimitiveRestartIndex"),
            program_binary_p: load(&loader, "glProgramBinary"),
            program_parameteri_p: load(&loader, "glProgramParameteri"),
            program_uniform1f_p: load(&loader, "glProgramUniform1f"),
            program_uniform1fv_p: load(&loader, "glProgramUniform1fv"),
            program_uniform1i_p: load(&loader, "glProgramUniform1i"),
            program_uniform1iv_p: load(&loader, "glProgramUniform1iv"),
            program_uniform1ui_p: load(&loader, "glProgramUniform1ui"),
            program_uniform1uiv_p: load(&loader, "glProgramUniform1uiv"),
            program_uniform2f_p: load(&loader, "glProgramUniform2f"),
            program_uniform2fv_p: load(&loader, "glProgramUniform2fv"),
            program_uniform2i_p: load(&loader, "glProgramUniform2i"),
            program_uniform2iv_p: load(&loader, "glProgramUniform2iv"),
            program_uniform2ui_p: load(&loader, "glProgramUniform2ui"),
            program_uniform2uiv_p: load(&loader, "glProgramUniform2uiv"),
            program_uniform3f_p: load(&loader, "glProgramUniform3f"),
            program_uniform3fv_p: load(&loader, "glProgramUniform3fv"),
            program_uniform3i_p: load(&loader, "glProgramUniform3i"),
            program_uniform3iv_p: load(&loader, "glProgramUniform3iv"),
            program_uniform3ui_p: load(&loader, "glProgramUniform3ui"),
            program_uniform3uiv_p: load(&loader, "glProgramUniform3uiv"),
            program_uniform4f_p: load(&loader, "glProgramUniform4f"),
            program_uniform4fv_p: load(&loader, "glProgramUniform4fv"),
            program_uniform4i_p: load(&loader, "glProgramUniform4i"),
            program_uniform4iv_p: load(&loader, "glProgramUniform4iv"),
            program_uniform4ui_p: load(&loader, "glProgramUniform4ui"),
            program_uniform4uiv_p: load(&loader, "glProgramUniform4uiv"),
            program_uniform_matrix2fv_p: load(&loader, "glProgramUniformMatrix2fv"),
            program_uniform_matrix2x3fv_p: load(&loader, "glProgramUniformMatrix2x3fv"),
            program_uniform_matrix2x4fv_p: load(&loader, "glProgramUniformMatrix2x4fv"),
            program_uniform_matrix3fv_p: load(&loader, "glProgramUniformMatrix3fv"),
            program_uniform_matrix3x2fv_p: load(&loader, "glProgramUniformMatrix3x2fv"),
            program_uniform_matrix3x4fv_p: load(&loader, "glProgramUniformMatrix3x4fv"),
            program_uniform_matrix4fv_p: load(&loader, "glProgramUniformMatrix4fv"),
            program_uniform_matrix4x2fv_p: load(&loader, "glProgramUniformMatrix4x2fv"),
            program_uniform_matrix4x3fv_p: load(&loader, "glProgramUniformMatrix4x3fv"),
            provoking_vertex_p: load(&loader, "glProvokingVertex"),
            push_debug_group_p: load(&loader, "glPushDebugGroup"),
            push_matrix_p: load(&loader, "glPushMatrix"),
            query_counter_p: load(&loader, "glQueryCounter"),
            read_buffer_p: load(&loader, "glReadBuffer"),
            read_pixels_p: load(&loader, "glReadPixels"),
            readn_pixels_p: load(&loader, "glReadnPixels"),
            release_shader_compiler_p: load(&loader, "glReleaseShaderCompiler"),
            renderbuffer_storage_p: load(&loader, "glRenderbufferStorage"),
            renderbuffer_storage_multisample_p: load(&loader, "glRenderbufferStorageMultisample"),
            resume_transform_feedback_p: load(&loader, "glResumeTransformFeedback"),
            rotatef_p: load(&loader, "glRotatef"),
            rotatex_p: load(&loader, "glRotatex"),
            sample_coverage_p: load(&loader, "glSampleCoverage"),
            sample_coveragex_p: load(&loader, "glSampleCoveragex"),
            sample_maski_p: load(&loader, "glSampleMaski"),
            sampler_parameter_iiv_p: load(&loader, "glSamplerParameterIiv"),
            sampler_parameter_iuiv_p: load(&loader, "glSamplerParameterIuiv"),
            sampler_parameterf_p: load(&loader, "glSamplerParameterf"),
            sampler_parameterfv_p: load(&loader, "glSamplerParameterfv"),
            sampler_parameteri_p: load(&loader, "glSamplerParameteri"),
            sampler_parameteriv_p: load(&loader, "glSamplerParameteriv"),
            scalef_p: load(&loader, "glScalef"),
            scalex_p: load(&loader, "glScalex"),
            scissor_p: load(&loader, "glScissor"),
            secondary_color_p3ui_p: load(&loader, "glSecondaryColorP3ui"),
            secondary_color_p3uiv_p: load(&loader, "glSecondaryColorP3uiv"),
            shade_model_p: load(&loader, "glShadeModel"),
            shader_binary_p: load(&loader, "glShaderBinary"),
            shader_source_p: load(&loader, "glShaderSource"),
            stencil_func_p: load(&loader, "glStencilFunc"),
            stencil_func_separate_p: load(&loader, "glStencilFuncSeparate"),
            stencil_mask_p: load(&loader, "glStencilMask"),
            stencil_mask_separate_p: load(&loader, "glStencilMaskSeparate"),
            stencil_op_p: load(&loader, "glStencilOp"),
            stencil_op_separate_p: load(&loader, "glStencilOpSeparate"),
            tex_buffer_p: load(&loader, "glTexBuffer"),
            tex_buffer_range_p: load(&loader, "glTexBufferRange"),
            tex_coord_p1ui_p: load(&loader, "glTexCoordP1ui"),
            tex_coord_p1uiv_p: load(&loader, "glTexCoordP1uiv"),
            tex_coord_p2ui_p: load(&loader, "glTexCoordP2ui"),
            tex_coord_p2uiv_p: load(&loader, "glTexCoordP2uiv"),
            tex_coord_p3ui_p: load(&loader, "glTexCoordP3ui"),
            tex_coord_p3uiv_p: load(&loader, "glTexCoordP3uiv"),
            tex_coord_p4ui_p: load(&loader, "glTexCoordP4ui"),
            tex_coord_p4uiv_p: load(&loader, "glTexCoordP4uiv"),
            tex_coord_pointer_p: load(&loader, "glTexCoordPointer"),
            tex_envf_p: load(&loader, "glTexEnvf"),
            tex_envfv_p: load(&loader, "glTexEnvfv"),
            tex_envi_p: load(&loader, "glTexEnvi"),
            tex_enviv_p: load(&loader, "glTexEnviv"),
            tex_envx_p: load(&loader, "glTexEnvx"),
            tex_envxv_p: load(&loader, "glTexEnvxv"),
            tex_image1_d_p: load(&loader, "glTexImage1D"),
            tex_image2_d_p: load(&loader, "glTexImage2D"),
            tex_image2_d_multisample_p: load(&loader, "glTexImage2DMultisample"),
            tex_image3_d_p: load(&loader, "glTexImage3D"),
            tex_image3_d_multisample_p: load(&loader, "glTexImage3DMultisample"),
            tex_parameter_iiv_p: load(&loader, "glTexParameterIiv"),
            tex_parameter_iuiv_p: load(&loader, "glTexParameterIuiv"),
            tex_parameterf_p: load(&loader, "glTexParameterf"),
            tex_parameterfv_p: load(&loader, "glTexParameterfv"),
            tex_parameteri_p: load(&loader, "glTexParameteri"),
            tex_parameteriv_p: load(&loader, "glTexParameteriv"),
            tex_parameterx_p: load(&loader, "glTexParameterx"),
            tex_parameterxv_p: load(&loader, "glTexParameterxv"),
            tex_storage2_d_p: load(&loader, "glTexStorage2D"),
            tex_storage2_d_multisample_p: load(&loader, "glTexStorage2DMultisample"),
            tex_storage3_d_p: load(&loader, "glTexStorage3D"),
            tex_storage3_d_multisample_p: load(&loader, "glTexStorage3DMultisample"),
            tex_sub_image1_d_p: load(&loader, "glTexSubImage1D"),
            tex_sub_image2_d_p: load(&loader, "glTexSubImage2D"),
            tex_sub_image3_d_p: load(&loader, "glTexSubImage3D"),
            transform_feedback_varyings_p: load(&loader, "glTransformFeedbackVaryings"),
            translatef_p: load(&loader, "glTranslatef"),
            translatex_p: load(&loader, "glTranslatex"),
            uniform1f_p: load(&loader, "glUniform1f"),
            uniform1fv_p: load(&loader, "glUniform1fv"),
            uniform1i_p: load(&loader, "glUniform1i"),
            uniform1iv_p: load(&loader, "glUniform1iv"),
            uniform1ui_p: load(&loader, "glUniform1ui"),
            uniform1uiv_p: load(&loader, "glUniform1uiv"),
            uniform2f_p: load(&loader, "glUniform2f"),
            uniform2fv_p: load(&loader, "glUniform2fv"),
            uniform2i_p: load(&loader, "glUniform2i"),
            uniform2iv_p: load(&loader, "glUniform2iv"),
            uniform2ui_p: load(&loader, "glUniform2ui"),
            uniform2uiv_p: load(&loader, "glUniform2uiv"),
            uniform3f_p: load(&loader, "glUniform3f"),
            uniform3fv_p: load(&loader, "glUniform3fv"),
            uniform3i_p: load(&loader, "glUniform3i"),
            uniform3iv_p: load(&loader, "glUniform3iv"),
            uniform3ui_p: load(&loader, "glUniform3ui"),
            uniform3uiv_p: load(&loader, "glUniform3uiv"),
            uniform4f_p: load(&loader, "glUniform4f"),
            uniform4fv_p: load(&loader, "glUniform4fv"),
            uniform4i_p: load(&loader, "glUniform4i"),
            uniform4iv_p: load(&loader, "glUniform4iv"),
            uniform4ui_p: load(&loader, "glUniform4ui"),
            uniform4uiv_p: load(&loader, "glUniform4uiv"),
            uniform_block_binding_p: load(&loader, "glUniformBlockBinding"),
            uniform_matrix2fv_p: load(&loader, "glUniformMatrix2fv"),
            uniform_matrix2x3fv_p: load(&loader, "glUniformMatrix2x3fv"),
            uniform_matrix2x4fv_p: load(&loader, "glUniformMatrix2x4fv"),
            uniform_matrix3fv_p: load(&loader, "glUniformMatrix3fv"),
            uniform_matrix3x2fv_p: load(&loader, "glUniformMatrix3x2fv"),
            uniform_matrix3x4fv_p: load(&loader, "glUniformMatrix3x4fv"),
            uniform_matrix4fv_p: load(&loader, "glUniformMatrix4fv"),
            uniform_matrix4x2fv_p: load(&loader, "glUniformMatrix4x2fv"),
            uniform_matrix4x3fv_p: load(&loader, "glUniformMatrix4x3fv"),
            unmap_buffer_p: load(&loader, "glUnmapBuffer"),
            use_program_p: load(&loader, "glUseProgram"),
            use_program_stages_p: load(&loader, "glUseProgramStages"),
            validate_program_p: load(&loader, "glValidateProgram"),
            validate_program_pipeline_p: load(&loader, "glValidateProgramPipeline"),
            vertex_attrib1d_p: load(&loader, "glVertexAttrib1d"),
            vertex_attrib1dv_p: load(&loader, "glVertexAttrib1dv"),
            vertex_attrib1f_p: load(&loader, "glVertexAttrib1f"),
            vertex_attrib1fv_p: load(&loader, "glVertexAttrib1fv"),
            vertex_attrib1s_p: load(&loader, "glVertexAttrib1s"),
            vertex_attrib1sv_p: load(&loader, "glVertexAttrib1sv"),
            vertex_attrib2d_p: load(&loader, "glVertexAttrib2d"),
            vertex_attrib2dv_p: load(&loader, "glVertexAttrib2dv"),
            vertex_attrib2f_p: load(&loader, "glVertexAttrib2f"),
            vertex_attrib2fv_p: load(&loader, "glVertexAttrib2fv"),
            vertex_attrib2s_p: load(&loader, "glVertexAttrib2s"),
            vertex_attrib2sv_p: load(&loader, "glVertexAttrib2sv"),
            vertex_attrib3d_p: load(&loader, "glVertexAttrib3d"),
            vertex_attrib3dv_p: load(&loader, "glVertexAttrib3dv"),
            vertex_attrib3f_p: load(&loader, "glVertexAttrib3f"),
            vertex_attrib3fv_p: load(&loader, "glVertexAttrib3fv"),
            vertex_attrib3s_p: load(&loader, "glVertexAttrib3s"),
            vertex_attrib3sv_p: load(&loader, "glVertexAttrib3sv"),
            vertex_attrib4_nbv_p: load(&loader, "glVertexAttrib4Nbv"),
            vertex_attrib4_niv_p: load(&loader, "glVertexAttrib4Niv"),
            vertex_attrib4_nsv_p: load(&loader, "glVertexAttrib4Nsv"),
            vertex_attrib4_nub_p: load(&loader, "glVertexAttrib4Nub"),
            vertex_attrib4_nubv_p: load(&loader, "glVertexAttrib4Nubv"),
            vertex_attrib4_nuiv_p: load(&loader, "glVertexAttrib4Nuiv"),
            vertex_attrib4_nusv_p: load(&loader, "glVertexAttrib4Nusv"),
            vertex_attrib4bv_p: load(&loader, "glVertexAttrib4bv"),
            vertex_attrib4d_p: load(&loader, "glVertexAttrib4d"),
            vertex_attrib4dv_p: load(&loader, "glVertexAttrib4dv"),
            vertex_attrib4f_p: load(&loader, "glVertexAttrib4f"),
            vertex_attrib4fv_p: load(&loader, "glVertexAttrib4fv"),
            vertex_attrib4iv_p: load(&loader, "glVertexAttrib4iv"),
            vertex_attrib4s_p: load(&loader, "glVertexAttrib4s"),
            vertex_attrib4sv_p: load(&loader, "glVertexAttrib4sv"),
            vertex_attrib4ubv_p: load(&loader, "glVertexAttrib4ubv"),
            vertex_attrib4uiv_p: load(&loader, "glVertexAttrib4uiv"),
            vertex_attrib4usv_p: load(&loader, "glVertexAttrib4usv"),
            vertex_attrib_binding_p: load(&loader, "glVertexAttribBinding"),
            vertex_attrib_divisor_p: load(&loader, "glVertexAttribDivisor"),
            vertex_attrib_format_p: load(&loader, "glVertexAttribFormat"),
            vertex_attrib_i1i_p: load(&loader, "glVertexAttribI1i"),
            vertex_attrib_i1iv_p: load(&loader, "glVertexAttribI1iv"),
            vertex_attrib_i1ui_p: load(&loader, "glVertexAttribI1ui"),
            vertex_attrib_i1uiv_p: load(&loader, "glVertexAttribI1uiv"),
            vertex_attrib_i2i_p: load(&loader, "glVertexAttribI2i"),
            vertex_attrib_i2iv_p: load(&loader, "glVertexAttribI2iv"),
            vertex_attrib_i2ui_p: load(&loader, "glVertexAttribI2ui"),
            vertex_attrib_i2uiv_p: load(&loader, "glVertexAttribI2uiv"),
            vertex_attrib_i3i_p: load(&loader, "glVertexAttribI3i"),
            vertex_attrib_i3iv_p: load(&loader, "glVertexAttribI3iv"),
            vertex_attrib_i3ui_p: load(&loader, "glVertexAttribI3ui"),
            vertex_attrib_i3uiv_p: load(&loader, "glVertexAttribI3uiv"),
            vertex_attrib_i4bv_p: load(&loader, "glVertexAttribI4bv"),
            vertex_attrib_i4i_p: load(&loader, "glVertexAttribI4i"),
            vertex_attrib_i4iv_p: load(&loader, "glVertexAttribI4iv"),
            vertex_attrib_i4sv_p: load(&loader, "glVertexAttribI4sv"),
            vertex_attrib_i4ubv_p: load(&loader, "glVertexAttribI4ubv"),
            vertex_attrib_i4ui_p: load(&loader, "glVertexAttribI4ui"),
            vertex_attrib_i4uiv_p: load(&loader, "glVertexAttribI4uiv"),
            vertex_attrib_i4usv_p: load(&loader, "glVertexAttribI4usv"),
            vertex_attrib_i_format_p: load(&loader, "glVertexAttribIFormat"),
            vertex_attrib_i_pointer_p: load(&loader, "glVertexAttribIPointer"),
            vertex_attrib_p1ui_p: load(&loader, "glVertexAttribP1ui"),
            vertex_attrib_p1uiv_p: load(&loader, "glVertexAttribP1uiv"),
            vertex_attrib_p2ui_p: load(&loader, "glVertexAttribP2ui"),
            vertex_attrib_p2uiv_p: load(&loader, "glVertexAttribP2uiv"),
            vertex_attrib_p3ui_p: load(&loader, "glVertexAttribP3ui"),
            vertex_attrib_p3uiv_p: load(&loader, "glVertexAttribP3uiv"),
            vertex_attrib_p4ui_p: load(&loader, "glVertexAttribP4ui"),
            vertex_attrib_p4uiv_p: load(&loader, "glVertexAttribP4uiv"),
            vertex_attrib_pointer_p: load(&loader, "glVertexAttribPointer"),
            vertex_binding_divisor_p: load(&loader, "glVertexBindingDivisor"),
            vertex_p2ui_p: load(&loader, "glVertexP2ui"),
            vertex_p2uiv_p: load(&loader, "glVertexP2uiv"),
            vertex_p3ui_p: load(&loader, "glVertexP3ui"),
            vertex_p3uiv_p: load(&loader, "glVertexP3uiv"),
            vertex_p4ui_p: load(&loader, "glVertexP4ui"),
            vertex_p4uiv_p: load(&loader, "glVertexP4uiv"),
            vertex_pointer_p: load(&loader, "glVertexPointer"),
            viewport_p: load(&loader, "glViewport"),
            wait_sync_p: load(&loader, "glWaitSync"),
        }
    }
    pub fn active_shader_program(&self, pipeline: u32, program: u32) {
        (self.active_shader_program_p)(pipeline, program)
    }

    pub fn active_texture(&self, texture: enums::TextureUnit) {
        (self.active_texture_p)(texture as u32)
    }

    pub fn alpha_func(&self, func: enums::AlphaFunction, ref_: f32) {
        (self.alpha_func_p)(func as u32, ref_)
    }

    pub fn alpha_funcx(&self, func: enums::AlphaFunction, ref_: i32) {
        (self.alpha_funcx_p)(func as u32, ref_)
    }

    pub fn attach_shader(&self, program: u32, shader: u32) {
        (self.attach_shader_p)(program, shader)
    }

    pub fn begin_conditional_render(&self, id: u32, mode: enums::ConditionalRenderMode) {
        (self.begin_conditional_render_p)(id, mode as u32)
    }

    pub fn begin_query(&self, target: enums::QueryTarget, id: u32) {
        (self.begin_query_p)(target as u32, id)
    }

    pub fn begin_transform_feedback(&self, primitive_mode: enums::PrimitiveType) {
        (self.begin_transform_feedback_p)(primitive_mode as u32)
    }

    pub fn bind_attrib_location(&self, program: u32, index: u32, name: *const i8) {
        (self.bind_attrib_location_p)(program, index, name)
    }

    pub fn bind_buffer(&self, target: enums::BufferTargetARB, buffer: u32) {
        (self.bind_buffer_p)(target as u32, buffer)
    }

    pub fn bind_buffer_base(&self, target: enums::BufferTargetARB, index: u32, buffer: u32) {
        (self.bind_buffer_base_p)(target as u32, index, buffer)
    }

    pub fn bind_buffer_range(
        &self,
        target: enums::BufferTargetARB,
        index: u32,
        buffer: u32,
        offset: isize,
        size: isize,
    ) {
        (self.bind_buffer_range_p)(target as u32, index, buffer, offset, size)
    }

    pub fn bind_frag_data_location(&self, program: u32, color: u32, name: *const i8) {
        (self.bind_frag_data_location_p)(program, color, name)
    }

    pub fn bind_frag_data_location_indexed(
        &self,
        program: u32,
        color_number: u32,
        index: u32,
        name: *const i8,
    ) {
        (self.bind_frag_data_location_indexed_p)(program, color_number, index, name)
    }

    pub fn bind_framebuffer(&self, target: enums::FramebufferTarget, framebuffer: u32) {
        (self.bind_framebuffer_p)(target as u32, framebuffer)
    }

    pub fn bind_image_texture(
        &self,
        unit: u32,
        texture: u32,
        level: i32,
        layered: bool,
        layer: i32,
        access: enums::BufferAccessARB,
        format: enums::InternalFormat,
    ) {
        (self.bind_image_texture_p)(
            unit,
            texture,
            level,
            layered as u8,
            layer,
            access as u32,
            format as u32,
        )
    }

    pub fn bind_program_pipeline(&self, pipeline: u32) {
        (self.bind_program_pipeline_p)(pipeline)
    }

    pub fn bind_renderbuffer(&self, target: enums::RenderbufferTarget, renderbuffer: u32) {
        (self.bind_renderbuffer_p)(target as u32, renderbuffer)
    }

    pub fn bind_sampler(&self, unit: u32, sampler: u32) {
        (self.bind_sampler_p)(unit, sampler)
    }

    pub fn bind_texture(&self, target: enums::TextureTarget, texture: u32) {
        (self.bind_texture_p)(target as u32, texture)
    }

    pub fn bind_transform_feedback(&self, target: enums::BindTransformFeedbackTarget, id: u32) {
        (self.bind_transform_feedback_p)(target as u32, id)
    }

    pub fn bind_vertex_array(&self, array: u32) {
        (self.bind_vertex_array_p)(array)
    }

    pub fn bind_vertex_buffer(&self, bindingindex: u32, buffer: u32, offset: isize, stride: i32) {
        (self.bind_vertex_buffer_p)(bindingindex, buffer, offset, stride)
    }

    pub fn blend_barrier(&self) {
        (self.blend_barrier_p)()
    }

    pub fn blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        (self.blend_color_p)(red, green, blue, alpha)
    }

    pub fn blend_equation(&self, mode: enums::BlendEquationModeEXT) {
        (self.blend_equation_p)(mode as u32)
    }

    pub fn blend_equation_separate(
        &self,
        mode_r_g_b: enums::BlendEquationModeEXT,
        mode_alpha: enums::BlendEquationModeEXT,
    ) {
        (self.blend_equation_separate_p)(mode_r_g_b as u32, mode_alpha as u32)
    }

    pub fn blend_equation_separatei(
        &self,
        buf: u32,
        mode_r_g_b: enums::BlendEquationModeEXT,
        mode_alpha: enums::BlendEquationModeEXT,
    ) {
        (self.blend_equation_separatei_p)(buf, mode_r_g_b as u32, mode_alpha as u32)
    }

    pub fn blend_equationi(&self, buf: u32, mode: enums::BlendEquationModeEXT) {
        (self.blend_equationi_p)(buf, mode as u32)
    }

    pub fn blend_func(&self, sfactor: enums::BlendingFactor, dfactor: enums::BlendingFactor) {
        (self.blend_func_p)(sfactor as u32, dfactor as u32)
    }

    pub fn blend_func_separate(
        &self,
        sfactor_r_g_b: enums::BlendingFactor,
        dfactor_r_g_b: enums::BlendingFactor,
        sfactor_alpha: enums::BlendingFactor,
        dfactor_alpha: enums::BlendingFactor,
    ) {
        (self.blend_func_separate_p)(
            sfactor_r_g_b as u32,
            dfactor_r_g_b as u32,
            sfactor_alpha as u32,
            dfactor_alpha as u32,
        )
    }

    pub fn blend_func_separatei(
        &self,
        buf: u32,
        src_r_g_b: enums::BlendingFactor,
        dst_r_g_b: enums::BlendingFactor,
        src_alpha: enums::BlendingFactor,
        dst_alpha: enums::BlendingFactor,
    ) {
        (self.blend_func_separatei_p)(
            buf,
            src_r_g_b as u32,
            dst_r_g_b as u32,
            src_alpha as u32,
            dst_alpha as u32,
        )
    }

    pub fn blend_funci(&self, buf: u32, src: enums::BlendingFactor, dst: enums::BlendingFactor) {
        (self.blend_funci_p)(buf, src as u32, dst as u32)
    }

    pub fn blit_framebuffer(
        &self,
        src_x0: i32,
        src_y0: i32,
        src_x1: i32,
        src_y1: i32,
        dst_x0: i32,
        dst_y0: i32,
        dst_x1: i32,
        dst_y1: i32,
        mask: enums::ClearBufferMask,
        filter: enums::BlitFramebufferFilter,
    ) {
        (self.blit_framebuffer_p)(
            src_x0,
            src_y0,
            src_x1,
            src_y1,
            dst_x0,
            dst_y0,
            dst_x1,
            dst_y1,
            mask as u32,
            filter as u32,
        )
    }

    pub fn buffer_data(
        &self,
        target: enums::BufferTargetARB,
        size: isize,
        data: *const c_void,
        usage: enums::BufferUsageARB,
    ) {
        (self.buffer_data_p)(target as u32, size, data, usage as u32)
    }

    pub fn buffer_sub_data(
        &self,
        target: enums::BufferTargetARB,
        offset: isize,
        size: isize,
        data: *const c_void,
    ) {
        (self.buffer_sub_data_p)(target as u32, offset, size, data)
    }

    pub fn check_framebuffer_status(
        &self,
        target: enums::FramebufferTarget,
    ) -> enums::FramebufferStatus {
        (self.check_framebuffer_status_p)(target as u32).into()
    }

    pub fn clamp_color(&self, target: enums::ClampColorTargetARB, clamp: enums::ClampColorModeARB) {
        (self.clamp_color_p)(target as u32, clamp as u32)
    }

    pub fn clear(&self, mask: enums::ClearBufferMask) {
        (self.clear_p)(mask as u32)
    }

    pub fn clear_bufferfi(&self, buffer: enums::Buffer, drawbuffer: i32, depth: f32, stencil: i32) {
        (self.clear_bufferfi_p)(buffer as u32, drawbuffer, depth, stencil)
    }

    pub fn clear_bufferfv(&self, buffer: enums::Buffer, drawbuffer: i32, value: *const f32) {
        (self.clear_bufferfv_p)(buffer as u32, drawbuffer, value)
    }

    pub fn clear_bufferiv(&self, buffer: enums::Buffer, drawbuffer: i32, value: *const i32) {
        (self.clear_bufferiv_p)(buffer as u32, drawbuffer, value)
    }

    pub fn clear_bufferuiv(&self, buffer: enums::Buffer, drawbuffer: i32, value: *const u32) {
        (self.clear_bufferuiv_p)(buffer as u32, drawbuffer, value)
    }

    pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        (self.clear_color_p)(red, green, blue, alpha)
    }

    pub fn clear_colorx(&self, red: i32, green: i32, blue: i32, alpha: i32) {
        (self.clear_colorx_p)(red, green, blue, alpha)
    }

    pub fn clear_depth(&self, depth: f64) {
        (self.clear_depth_p)(depth)
    }

    pub fn clear_depthf(&self, d: f32) {
        (self.clear_depthf_p)(d)
    }

    pub fn clear_depthx(&self, depth: i32) {
        (self.clear_depthx_p)(depth)
    }

    pub fn clear_stencil(&self, s: i32) {
        (self.clear_stencil_p)(s)
    }

    pub fn client_active_texture(&self, texture: enums::TextureUnit) {
        (self.client_active_texture_p)(texture as u32)
    }

    pub fn client_wait_sync(
        &self,
        sync: *mut c_void,
        flags: enums::SyncObjectMask,
        timeout: u64,
    ) -> enums::SyncStatus {
        (self.client_wait_sync_p)(sync, flags as u32, timeout).into()
    }

    pub fn clip_planef(&self, p: enums::ClipPlaneName, eqn: *const f32) {
        (self.clip_planef_p)(p as u32, eqn)
    }

    pub fn clip_planex(&self, plane: enums::ClipPlaneName, equation: *const i32) {
        (self.clip_planex_p)(plane as u32, equation)
    }

    pub fn color4f(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        (self.color4f_p)(red, green, blue, alpha)
    }

    pub fn color4ub(&self, red: u8, green: u8, blue: u8, alpha: u8) {
        (self.color4ub_p)(red, green, blue, alpha)
    }

    pub fn color4x(&self, red: i32, green: i32, blue: i32, alpha: i32) {
        (self.color4x_p)(red, green, blue, alpha)
    }

    pub fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool) {
        (self.color_mask_p)(red as u8, green as u8, blue as u8, alpha as u8)
    }

    pub fn color_maski(&self, index: u32, r: bool, g: bool, b: bool, a: bool) {
        (self.color_maski_p)(index, r as u8, g as u8, b as u8, a as u8)
    }

    pub fn color_p3ui(&self, type_: enums::ColorPointerType, color: u32) {
        (self.color_p3ui_p)(type_ as u32, color)
    }

    pub fn color_p3uiv(&self, type_: enums::ColorPointerType, color: *const u32) {
        (self.color_p3uiv_p)(type_ as u32, color)
    }

    pub fn color_p4ui(&self, type_: enums::ColorPointerType, color: u32) {
        (self.color_p4ui_p)(type_ as u32, color)
    }

    pub fn color_p4uiv(&self, type_: enums::ColorPointerType, color: *const u32) {
        (self.color_p4uiv_p)(type_ as u32, color)
    }

    pub fn color_pointer(
        &self,
        size: i32,
        type_: enums::ColorPointerType,
        stride: i32,
        pointer: *const c_void,
    ) {
        (self.color_pointer_p)(size, type_ as u32, stride, pointer)
    }

    pub fn compile_shader(&self, shader: u32) {
        (self.compile_shader_p)(shader)
    }

    pub fn compressed_tex_image1_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        internalformat: enums::InternalFormat,
        width: i32,
        border: i32,
        image_size: i32,
        data: *const c_void,
    ) {
        (self.compressed_tex_image1_d_p)(
            target as u32,
            level,
            internalformat as u32,
            width,
            border,
            image_size,
            data,
        )
    }

    pub fn compressed_tex_image2_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        internalformat: enums::InternalFormat,
        width: i32,
        height: i32,
        border: i32,
        image_size: i32,
        data: *const c_void,
    ) {
        (self.compressed_tex_image2_d_p)(
            target as u32,
            level,
            internalformat as u32,
            width,
            height,
            border,
            image_size,
            data,
        )
    }

    pub fn compressed_tex_image3_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        internalformat: enums::InternalFormat,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        image_size: i32,
        data: *const c_void,
    ) {
        (self.compressed_tex_image3_d_p)(
            target as u32,
            level,
            internalformat as u32,
            width,
            height,
            depth,
            border,
            image_size,
            data,
        )
    }

    pub fn compressed_tex_sub_image1_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        xoffset: i32,
        width: i32,
        format: enums::InternalFormat,
        image_size: i32,
        data: *const c_void,
    ) {
        (self.compressed_tex_sub_image1_d_p)(
            target as u32,
            level,
            xoffset,
            width,
            format as u32,
            image_size,
            data,
        )
    }

    pub fn compressed_tex_sub_image2_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: enums::InternalFormat,
        image_size: i32,
        data: *const c_void,
    ) {
        (self.compressed_tex_sub_image2_d_p)(
            target as u32,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format as u32,
            image_size,
            data,
        )
    }

    pub fn compressed_tex_sub_image3_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: enums::InternalFormat,
        image_size: i32,
        data: *const c_void,
    ) {
        (self.compressed_tex_sub_image3_d_p)(
            target as u32,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format as u32,
            image_size,
            data,
        )
    }

    pub fn copy_buffer_sub_data(
        &self,
        read_target: enums::CopyBufferSubDataTarget,
        write_target: enums::CopyBufferSubDataTarget,
        read_offset: isize,
        write_offset: isize,
        size: isize,
    ) {
        (self.copy_buffer_sub_data_p)(
            read_target as u32,
            write_target as u32,
            read_offset,
            write_offset,
            size,
        )
    }

    pub fn copy_image_sub_data(
        &self,
        src_name: u32,
        src_target: enums::CopyImageSubDataTarget,
        src_level: i32,
        src_x: i32,
        src_y: i32,
        src_z: i32,
        dst_name: u32,
        dst_target: enums::CopyImageSubDataTarget,
        dst_level: i32,
        dst_x: i32,
        dst_y: i32,
        dst_z: i32,
        src_width: i32,
        src_height: i32,
        src_depth: i32,
    ) {
        (self.copy_image_sub_data_p)(
            src_name,
            src_target as u32,
            src_level,
            src_x,
            src_y,
            src_z,
            dst_name,
            dst_target as u32,
            dst_level,
            dst_x,
            dst_y,
            dst_z,
            src_width,
            src_height,
            src_depth,
        )
    }

    pub fn copy_tex_image1_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        internalformat: enums::InternalFormat,
        x: i32,
        y: i32,
        width: i32,
        border: i32,
    ) {
        (self.copy_tex_image1_d_p)(
            target as u32,
            level,
            internalformat as u32,
            x,
            y,
            width,
            border,
        )
    }

    pub fn copy_tex_image2_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        internalformat: enums::InternalFormat,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        border: i32,
    ) {
        (self.copy_tex_image2_d_p)(
            target as u32,
            level,
            internalformat as u32,
            x,
            y,
            width,
            height,
            border,
        )
    }

    pub fn copy_tex_sub_image1_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        xoffset: i32,
        x: i32,
        y: i32,
        width: i32,
    ) {
        (self.copy_tex_sub_image1_d_p)(target as u32, level, xoffset, x, y, width)
    }

    pub fn copy_tex_sub_image2_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        (self.copy_tex_sub_image2_d_p)(target as u32, level, xoffset, yoffset, x, y, width, height)
    }

    pub fn copy_tex_sub_image3_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        (self.copy_tex_sub_image3_d_p)(
            target as u32,
            level,
            xoffset,
            yoffset,
            zoffset,
            x,
            y,
            width,
            height,
        )
    }

    pub fn create_program(&self) -> u32 {
        (self.create_program_p)()
    }

    pub fn create_shader(&self, type_: enums::ShaderType) -> u32 {
        (self.create_shader_p)(type_ as u32)
    }

    pub fn create_shader_programv(
        &self,
        type_: enums::ShaderType,
        count: i32,
        strings: *const *const i8,
    ) -> u32 {
        (self.create_shader_programv_p)(type_ as u32, count, strings)
    }

    pub fn cull_face(&self, mode: enums::TriangleFace) {
        (self.cull_face_p)(mode as u32)
    }

    pub fn debug_message_callback(&self, callback: DebugProc, user_param: *const c_void) {
        (self.debug_message_callback_p)(callback, user_param)
    }

    pub fn debug_message_control(
        &self,
        source: enums::DebugSource,
        type_: enums::DebugType,
        severity: enums::DebugSeverity,
        count: i32,
        ids: *const u32,
        enabled: bool,
    ) {
        (self.debug_message_control_p)(
            source as u32,
            type_ as u32,
            severity as u32,
            count,
            ids,
            enabled as u8,
        )
    }

    pub fn debug_message_insert(
        &self,
        source: enums::DebugSource,
        type_: enums::DebugType,
        id: u32,
        severity: enums::DebugSeverity,
        length: i32,
        buf: *const i8,
    ) {
        (self.debug_message_insert_p)(
            source as u32,
            type_ as u32,
            id,
            severity as u32,
            length,
            buf,
        )
    }

    pub fn delete_buffers(&self, n: i32, buffers: *const u32) {
        (self.delete_buffers_p)(n, buffers)
    }

    pub fn delete_framebuffers(&self, n: i32, framebuffers: *const u32) {
        (self.delete_framebuffers_p)(n, framebuffers)
    }

    pub fn delete_program(&self, program: u32) {
        (self.delete_program_p)(program)
    }

    pub fn delete_program_pipelines(&self, n: i32, pipelines: *const u32) {
        (self.delete_program_pipelines_p)(n, pipelines)
    }

    pub fn delete_queries(&self, n: i32, ids: *const u32) {
        (self.delete_queries_p)(n, ids)
    }

    pub fn delete_renderbuffers(&self, n: i32, renderbuffers: *const u32) {
        (self.delete_renderbuffers_p)(n, renderbuffers)
    }

    pub fn delete_samplers(&self, count: i32, samplers: *const u32) {
        (self.delete_samplers_p)(count, samplers)
    }

    pub fn delete_shader(&self, shader: u32) {
        (self.delete_shader_p)(shader)
    }

    pub fn delete_sync(&self, sync: *mut c_void) {
        (self.delete_sync_p)(sync)
    }

    pub fn delete_textures(&self, n: i32, textures: *const u32) {
        (self.delete_textures_p)(n, textures)
    }

    pub fn delete_transform_feedbacks(&self, n: i32, ids: *const u32) {
        (self.delete_transform_feedbacks_p)(n, ids)
    }

    pub fn delete_vertex_arrays(&self, n: i32, arrays: *const u32) {
        (self.delete_vertex_arrays_p)(n, arrays)
    }

    pub fn depth_func(&self, func: enums::DepthFunction) {
        (self.depth_func_p)(func as u32)
    }

    pub fn depth_mask(&self, flag: bool) {
        (self.depth_mask_p)(flag as u8)
    }

    pub fn depth_range(&self, n: f64, f: f64) {
        (self.depth_range_p)(n, f)
    }

    pub fn depth_rangef(&self, n: f32, f: f32) {
        (self.depth_rangef_p)(n, f)
    }

    pub fn depth_rangex(&self, n: i32, f: i32) {
        (self.depth_rangex_p)(n, f)
    }

    pub fn detach_shader(&self, program: u32, shader: u32) {
        (self.detach_shader_p)(program, shader)
    }

    pub fn disable(&self, cap: enums::EnableCap) {
        (self.disable_p)(cap as u32)
    }

    pub fn disable_client_state(&self, array: enums::EnableCap) {
        (self.disable_client_state_p)(array as u32)
    }

    pub fn disable_vertex_attrib_array(&self, index: u32) {
        (self.disable_vertex_attrib_array_p)(index)
    }

    pub fn disablei(&self, target: enums::EnableCap, index: u32) {
        (self.disablei_p)(target as u32, index)
    }

    pub fn dispatch_compute(&self, num_groups_x: u32, num_groups_y: u32, num_groups_z: u32) {
        (self.dispatch_compute_p)(num_groups_x, num_groups_y, num_groups_z)
    }

    pub fn dispatch_compute_indirect(&self, indirect: isize) {
        (self.dispatch_compute_indirect_p)(indirect)
    }

    pub fn draw_arrays(&self, mode: enums::PrimitiveType, first: i32, count: i32) {
        (self.draw_arrays_p)(mode as u32, first, count)
    }

    pub fn draw_arrays_indirect(&self, mode: enums::PrimitiveType, indirect: *const c_void) {
        (self.draw_arrays_indirect_p)(mode as u32, indirect)
    }

    pub fn draw_arrays_instanced(
        &self,
        mode: enums::PrimitiveType,
        first: i32,
        count: i32,
        instancecount: i32,
    ) {
        (self.draw_arrays_instanced_p)(mode as u32, first, count, instancecount)
    }

    pub fn draw_buffer(&self, buf: enums::DrawBufferMode) {
        (self.draw_buffer_p)(buf as u32)
    }

    pub fn draw_buffers(&self, n: i32, bufs: *const enums::DrawBufferMode) {
        (self.draw_buffers_p)(n, bufs as *const u32)
    }

    pub fn draw_elements(
        &self,
        mode: enums::PrimitiveType,
        count: i32,
        type_: enums::DrawElementsType,
        indices: *const c_void,
    ) {
        (self.draw_elements_p)(mode as u32, count, type_ as u32, indices)
    }

    pub fn draw_elements_base_vertex(
        &self,
        mode: enums::PrimitiveType,
        count: i32,
        type_: enums::DrawElementsType,
        indices: *const c_void,
        basevertex: i32,
    ) {
        (self.draw_elements_base_vertex_p)(mode as u32, count, type_ as u32, indices, basevertex)
    }

    pub fn draw_elements_indirect(
        &self,
        mode: enums::PrimitiveType,
        type_: enums::DrawElementsType,
        indirect: *const c_void,
    ) {
        (self.draw_elements_indirect_p)(mode as u32, type_ as u32, indirect)
    }

    pub fn draw_elements_instanced(
        &self,
        mode: enums::PrimitiveType,
        count: i32,
        type_: enums::DrawElementsType,
        indices: *const c_void,
        instancecount: i32,
    ) {
        (self.draw_elements_instanced_p)(mode as u32, count, type_ as u32, indices, instancecount)
    }

    pub fn draw_elements_instanced_base_vertex(
        &self,
        mode: enums::PrimitiveType,
        count: i32,
        type_: enums::DrawElementsType,
        indices: *const c_void,
        instancecount: i32,
        basevertex: i32,
    ) {
        (self.draw_elements_instanced_base_vertex_p)(
            mode as u32,
            count,
            type_ as u32,
            indices,
            instancecount,
            basevertex,
        )
    }

    pub fn draw_range_elements(
        &self,
        mode: enums::PrimitiveType,
        start: u32,
        end: u32,
        count: i32,
        type_: enums::DrawElementsType,
        indices: *const c_void,
    ) {
        (self.draw_range_elements_p)(mode as u32, start, end, count, type_ as u32, indices)
    }

    pub fn draw_range_elements_base_vertex(
        &self,
        mode: enums::PrimitiveType,
        start: u32,
        end: u32,
        count: i32,
        type_: enums::DrawElementsType,
        indices: *const c_void,
        basevertex: i32,
    ) {
        (self.draw_range_elements_base_vertex_p)(
            mode as u32,
            start,
            end,
            count,
            type_ as u32,
            indices,
            basevertex,
        )
    }

    pub fn enable(&self, cap: enums::EnableCap) {
        (self.enable_p)(cap as u32)
    }

    pub fn enable_client_state(&self, array: enums::EnableCap) {
        (self.enable_client_state_p)(array as u32)
    }

    pub fn enable_vertex_attrib_array(&self, index: u32) {
        (self.enable_vertex_attrib_array_p)(index)
    }

    pub fn enablei(&self, target: enums::EnableCap, index: u32) {
        (self.enablei_p)(target as u32, index)
    }

    pub fn end_conditional_render(&self) {
        (self.end_conditional_render_p)()
    }

    pub fn end_query(&self, target: enums::QueryTarget) {
        (self.end_query_p)(target as u32)
    }

    pub fn end_transform_feedback(&self) {
        (self.end_transform_feedback_p)()
    }

    pub fn fence_sync(
        &self,
        condition: enums::SyncCondition,
        flags: enums::SyncBehaviorFlags,
    ) -> *mut c_void {
        (self.fence_sync_p)(condition as u32, flags as u32)
    }

    pub fn finish(&self) {
        (self.finish_p)()
    }

    pub fn flush(&self) {
        (self.flush_p)()
    }

    pub fn flush_mapped_buffer_range(
        &self,
        target: enums::BufferTargetARB,
        offset: isize,
        length: isize,
    ) {
        (self.flush_mapped_buffer_range_p)(target as u32, offset, length)
    }

    pub fn fogf(&self, pname: enums::FogParameter, param: f32) {
        (self.fogf_p)(pname as u32, param)
    }

    pub fn fogfv(&self, pname: enums::FogParameter, params: *const f32) {
        (self.fogfv_p)(pname as u32, params)
    }

    pub fn fogx(&self, pname: enums::FogPName, param: i32) {
        (self.fogx_p)(pname as u32, param)
    }

    pub fn fogxv(&self, pname: enums::FogPName, param: *const i32) {
        (self.fogxv_p)(pname as u32, param)
    }

    pub fn framebuffer_parameteri(
        &self,
        target: enums::FramebufferTarget,
        pname: enums::FramebufferParameterName,
        param: i32,
    ) {
        (self.framebuffer_parameteri_p)(target as u32, pname as u32, param)
    }

    pub fn framebuffer_renderbuffer(
        &self,
        target: enums::FramebufferTarget,
        attachment: enums::FramebufferAttachment,
        renderbuffertarget: enums::RenderbufferTarget,
        renderbuffer: u32,
    ) {
        (self.framebuffer_renderbuffer_p)(
            target as u32,
            attachment as u32,
            renderbuffertarget as u32,
            renderbuffer,
        )
    }

    pub fn framebuffer_texture(
        &self,
        target: enums::FramebufferTarget,
        attachment: enums::FramebufferAttachment,
        texture: u32,
        level: i32,
    ) {
        (self.framebuffer_texture_p)(target as u32, attachment as u32, texture, level)
    }

    pub fn framebuffer_texture1_d(
        &self,
        target: enums::FramebufferTarget,
        attachment: enums::FramebufferAttachment,
        textarget: enums::TextureTarget,
        texture: u32,
        level: i32,
    ) {
        (self.framebuffer_texture1_d_p)(
            target as u32,
            attachment as u32,
            textarget as u32,
            texture,
            level,
        )
    }

    pub fn framebuffer_texture2_d(
        &self,
        target: enums::FramebufferTarget,
        attachment: enums::FramebufferAttachment,
        textarget: enums::TextureTarget,
        texture: u32,
        level: i32,
    ) {
        (self.framebuffer_texture2_d_p)(
            target as u32,
            attachment as u32,
            textarget as u32,
            texture,
            level,
        )
    }

    pub fn framebuffer_texture3_d(
        &self,
        target: enums::FramebufferTarget,
        attachment: enums::FramebufferAttachment,
        textarget: enums::TextureTarget,
        texture: u32,
        level: i32,
        zoffset: i32,
    ) {
        (self.framebuffer_texture3_d_p)(
            target as u32,
            attachment as u32,
            textarget as u32,
            texture,
            level,
            zoffset,
        )
    }

    pub fn framebuffer_texture_layer(
        &self,
        target: enums::FramebufferTarget,
        attachment: enums::FramebufferAttachment,
        texture: u32,
        level: i32,
        layer: i32,
    ) {
        (self.framebuffer_texture_layer_p)(target as u32, attachment as u32, texture, level, layer)
    }

    pub fn front_face(&self, mode: enums::FrontFaceDirection) {
        (self.front_face_p)(mode as u32)
    }

    pub fn frustumf(&self, l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) {
        (self.frustumf_p)(l, r, b, t, n, f)
    }

    pub fn frustumx(&self, l: i32, r: i32, b: i32, t: i32, n: i32, f: i32) {
        (self.frustumx_p)(l, r, b, t, n, f)
    }

    pub fn gen_buffers(&self, n: i32, buffers: *mut u32) {
        (self.gen_buffers_p)(n, buffers)
    }

    pub fn gen_framebuffers(&self, n: i32, framebuffers: *mut u32) {
        (self.gen_framebuffers_p)(n, framebuffers)
    }

    pub fn gen_program_pipelines(&self, n: i32, pipelines: *mut u32) {
        (self.gen_program_pipelines_p)(n, pipelines)
    }

    pub fn gen_queries(&self, n: i32, ids: *mut u32) {
        (self.gen_queries_p)(n, ids)
    }

    pub fn gen_renderbuffers(&self, n: i32, renderbuffers: *mut u32) {
        (self.gen_renderbuffers_p)(n, renderbuffers)
    }

    pub fn gen_samplers(&self, count: i32, samplers: *mut u32) {
        (self.gen_samplers_p)(count, samplers)
    }

    pub fn gen_textures(&self, n: i32, textures: *mut u32) {
        (self.gen_textures_p)(n, textures)
    }

    pub fn gen_transform_feedbacks(&self, n: i32, ids: *mut u32) {
        (self.gen_transform_feedbacks_p)(n, ids)
    }

    pub fn gen_vertex_arrays(&self, n: i32, arrays: *mut u32) {
        (self.gen_vertex_arrays_p)(n, arrays)
    }

    pub fn generate_mipmap(&self, target: enums::TextureTarget) {
        (self.generate_mipmap_p)(target as u32)
    }

    pub fn get_active_attrib(
        &self,
        program: u32,
        index: u32,
        buf_size: i32,
        length: *mut i32,
        size: *mut i32,
        type_: *mut enums::AttributeType,
        name: *mut i8,
    ) {
        (self.get_active_attrib_p)(
            program,
            index,
            buf_size,
            length,
            size,
            type_ as *mut u32,
            name,
        )
    }

    pub fn get_active_uniform(
        &self,
        program: u32,
        index: u32,
        buf_size: i32,
        length: *mut i32,
        size: *mut i32,
        type_: *mut enums::UniformType,
        name: *mut i8,
    ) {
        (self.get_active_uniform_p)(
            program,
            index,
            buf_size,
            length,
            size,
            type_ as *mut u32,
            name,
        )
    }

    pub fn get_active_uniform_block_name(
        &self,
        program: u32,
        uniform_block_index: u32,
        buf_size: i32,
        length: *mut i32,
        uniform_block_name: *mut i8,
    ) {
        (self.get_active_uniform_block_name_p)(
            program,
            uniform_block_index,
            buf_size,
            length,
            uniform_block_name,
        )
    }

    pub fn get_active_uniform_blockiv(
        &self,
        program: u32,
        uniform_block_index: u32,
        pname: enums::UniformBlockPName,
        params: *mut i32,
    ) {
        (self.get_active_uniform_blockiv_p)(program, uniform_block_index, pname as u32, params)
    }

    pub fn get_active_uniform_name(
        &self,
        program: u32,
        uniform_index: u32,
        buf_size: i32,
        length: *mut i32,
        uniform_name: *mut i8,
    ) {
        (self.get_active_uniform_name_p)(program, uniform_index, buf_size, length, uniform_name)
    }

    pub fn get_active_uniformsiv(
        &self,
        program: u32,
        uniform_count: i32,
        uniform_indices: *const u32,
        pname: enums::UniformPName,
        params: *mut i32,
    ) {
        (self.get_active_uniformsiv_p)(
            program,
            uniform_count,
            uniform_indices,
            pname as u32,
            params,
        )
    }

    pub fn get_attached_shaders(
        &self,
        program: u32,
        max_count: i32,
        count: *mut i32,
        shaders: *mut u32,
    ) {
        (self.get_attached_shaders_p)(program, max_count, count, shaders)
    }

    pub fn get_attrib_location(&self, program: u32, name: *const i8) -> i32 {
        (self.get_attrib_location_p)(program, name)
    }

    pub fn get_booleani_v(&self, target: enums::BufferTargetARB, index: u32, data: *mut bool) {
        (self.get_booleani_v_p)(target as u32, index, data as *mut u8)
    }

    pub fn get_booleanv(&self, pname: enums::GetPName, data: *mut bool) {
        (self.get_booleanv_p)(pname as u32, data as *mut u8)
    }

    pub fn get_buffer_parameteri64v(
        &self,
        target: enums::BufferTargetARB,
        pname: enums::BufferPNameARB,
        params: *mut i64,
    ) {
        (self.get_buffer_parameteri64v_p)(target as u32, pname as u32, params)
    }

    pub fn get_buffer_parameteriv(
        &self,
        target: enums::BufferTargetARB,
        pname: enums::BufferPNameARB,
        params: *mut i32,
    ) {
        (self.get_buffer_parameteriv_p)(target as u32, pname as u32, params)
    }

    pub fn get_buffer_pointerv(
        &self,
        target: enums::BufferTargetARB,
        pname: enums::BufferPointerNameARB,
        params: *mut *mut c_void,
    ) {
        (self.get_buffer_pointerv_p)(target as u32, pname as u32, params)
    }

    pub fn get_buffer_sub_data(
        &self,
        target: enums::BufferTargetARB,
        offset: isize,
        size: isize,
        data: *mut c_void,
    ) {
        (self.get_buffer_sub_data_p)(target as u32, offset, size, data)
    }

    pub fn get_clip_planef(&self, plane: enums::ClipPlaneName, equation: *mut f32) {
        (self.get_clip_planef_p)(plane as u32, equation)
    }

    pub fn get_clip_planex(&self, plane: enums::ClipPlaneName, equation: *mut i32) {
        (self.get_clip_planex_p)(plane as u32, equation)
    }

    pub fn get_compressed_tex_image(
        &self,
        target: enums::TextureTarget,
        level: i32,
        img: *mut c_void,
    ) {
        (self.get_compressed_tex_image_p)(target as u32, level, img)
    }

    pub fn get_debug_message_log(
        &self,
        count: u32,
        buf_size: i32,
        sources: *mut enums::DebugSource,
        types: *mut enums::DebugType,
        ids: *mut u32,
        severities: *mut enums::DebugSeverity,
        lengths: *mut i32,
        message_log: *mut i8,
    ) -> u32 {
        (self.get_debug_message_log_p)(
            count,
            buf_size,
            sources as *mut u32,
            types as *mut u32,
            ids,
            severities as *mut u32,
            lengths,
            message_log,
        )
    }

    pub fn get_doublev(&self, pname: enums::GetPName, data: *mut f64) {
        (self.get_doublev_p)(pname as u32, data)
    }

    pub fn get_error(&self) -> enums::ErrorCode {
        (self.get_error_p)().into()
    }

    pub fn get_fixedv(&self, pname: enums::GetPName, params: *mut i32) {
        (self.get_fixedv_p)(pname as u32, params)
    }

    pub fn get_floatv(&self, pname: enums::GetPName, data: *mut f32) {
        (self.get_floatv_p)(pname as u32, data)
    }

    pub fn get_frag_data_index(&self, program: u32, name: *const i8) -> i32 {
        (self.get_frag_data_index_p)(program, name)
    }

    pub fn get_frag_data_location(&self, program: u32, name: *const i8) -> i32 {
        (self.get_frag_data_location_p)(program, name)
    }

    pub fn get_framebuffer_attachment_parameteriv(
        &self,
        target: enums::FramebufferTarget,
        attachment: enums::FramebufferAttachment,
        pname: enums::FramebufferAttachmentParameterName,
        params: *mut i32,
    ) {
        (self.get_framebuffer_attachment_parameteriv_p)(
            target as u32,
            attachment as u32,
            pname as u32,
            params,
        )
    }

    pub fn get_framebuffer_parameteriv(
        &self,
        target: enums::FramebufferTarget,
        pname: enums::FramebufferAttachmentParameterName,
        params: *mut i32,
    ) {
        (self.get_framebuffer_parameteriv_p)(target as u32, pname as u32, params)
    }

    pub fn get_graphics_reset_status(&self) -> enums::GraphicsResetStatus {
        (self.get_graphics_reset_status_p)().into()
    }

    pub fn get_integer64i_v(&self, target: enums::GetPName, index: u32, data: *mut i64) {
        (self.get_integer64i_v_p)(target as u32, index, data)
    }

    pub fn get_integer64v(&self, pname: enums::GetPName, data: *mut i64) {
        (self.get_integer64v_p)(pname as u32, data)
    }

    pub fn get_integeri_v(&self, target: enums::GetPName, index: u32, data: *mut i32) {
        (self.get_integeri_v_p)(target as u32, index, data)
    }

    pub fn get_integerv(&self, pname: enums::GetPName, data: *mut i32) {
        (self.get_integerv_p)(pname as u32, data)
    }

    pub fn get_internalformativ(
        &self,
        target: enums::TextureTarget,
        internalformat: enums::InternalFormat,
        pname: enums::InternalFormatPName,
        count: i32,
        params: *mut i32,
    ) {
        (self.get_internalformativ_p)(
            target as u32,
            internalformat as u32,
            pname as u32,
            count,
            params,
        )
    }

    pub fn get_lightfv(
        &self,
        light: enums::LightName,
        pname: enums::LightParameter,
        params: *mut f32,
    ) {
        (self.get_lightfv_p)(light as u32, pname as u32, params)
    }

    pub fn get_lightxv(
        &self,
        light: enums::LightName,
        pname: enums::LightParameter,
        params: *mut i32,
    ) {
        (self.get_lightxv_p)(light as u32, pname as u32, params)
    }

    pub fn get_materialfv(
        &self,
        face: enums::TriangleFace,
        pname: enums::MaterialParameter,
        params: *mut f32,
    ) {
        (self.get_materialfv_p)(face as u32, pname as u32, params)
    }

    pub fn get_materialxv(
        &self,
        face: enums::TriangleFace,
        pname: enums::MaterialParameter,
        params: *mut i32,
    ) {
        (self.get_materialxv_p)(face as u32, pname as u32, params)
    }

    pub fn get_multisamplefv(
        &self,
        pname: enums::GetMultisamplePNameNV,
        index: u32,
        val: *mut f32,
    ) {
        (self.get_multisamplefv_p)(pname as u32, index, val)
    }

    pub fn get_object_label(
        &self,
        identifier: enums::ObjectIdentifier,
        name: u32,
        buf_size: i32,
        length: *mut i32,
        label: *mut i8,
    ) {
        (self.get_object_label_p)(identifier as u32, name, buf_size, length, label)
    }

    pub fn get_object_ptr_label(
        &self,
        ptr: *const c_void,
        buf_size: i32,
        length: *mut i32,
        label: *mut i8,
    ) {
        (self.get_object_ptr_label_p)(ptr, buf_size, length, label)
    }

    pub fn get_pointerv(&self, pname: enums::GetPointervPName, params: *mut *mut c_void) {
        (self.get_pointerv_p)(pname as u32, params)
    }

    pub fn get_program_binary(
        &self,
        program: u32,
        buf_size: i32,
        length: *mut i32,
        binary_format: *mut u32,
        binary: *mut c_void,
    ) {
        (self.get_program_binary_p)(program, buf_size, length, binary_format, binary)
    }

    pub fn get_program_info_log(
        &self,
        program: u32,
        buf_size: i32,
        length: *mut i32,
        info_log: *mut i8,
    ) {
        (self.get_program_info_log_p)(program, buf_size, length, info_log)
    }

    pub fn get_program_interfaceiv(
        &self,
        program: u32,
        program_interface: enums::ProgramInterface,
        pname: enums::ProgramInterfacePName,
        params: *mut i32,
    ) {
        (self.get_program_interfaceiv_p)(program, program_interface as u32, pname as u32, params)
    }

    pub fn get_program_pipeline_info_log(
        &self,
        pipeline: u32,
        buf_size: i32,
        length: *mut i32,
        info_log: *mut i8,
    ) {
        (self.get_program_pipeline_info_log_p)(pipeline, buf_size, length, info_log)
    }

    pub fn get_program_pipelineiv(
        &self,
        pipeline: u32,
        pname: enums::PipelineParameterName,
        params: *mut i32,
    ) {
        (self.get_program_pipelineiv_p)(pipeline, pname as u32, params)
    }

    pub fn get_program_resource_index(
        &self,
        program: u32,
        program_interface: enums::ProgramInterface,
        name: *const i8,
    ) -> u32 {
        (self.get_program_resource_index_p)(program, program_interface as u32, name)
    }

    pub fn get_program_resource_location(
        &self,
        program: u32,
        program_interface: enums::ProgramInterface,
        name: *const i8,
    ) -> i32 {
        (self.get_program_resource_location_p)(program, program_interface as u32, name)
    }

    pub fn get_program_resource_name(
        &self,
        program: u32,
        program_interface: enums::ProgramInterface,
        index: u32,
        buf_size: i32,
        length: *mut i32,
        name: *mut i8,
    ) {
        (self.get_program_resource_name_p)(
            program,
            program_interface as u32,
            index,
            buf_size,
            length,
            name,
        )
    }

    pub fn get_program_resourceiv(
        &self,
        program: u32,
        program_interface: enums::ProgramInterface,
        index: u32,
        prop_count: i32,
        props: *const enums::ProgramResourceProperty,
        count: i32,
        length: *mut i32,
        params: *mut i32,
    ) {
        (self.get_program_resourceiv_p)(
            program,
            program_interface as u32,
            index,
            prop_count,
            props as *const u32,
            count,
            length,
            params,
        )
    }

    pub fn get_programiv(&self, program: u32, pname: enums::ProgramPropertyARB, params: *mut i32) {
        (self.get_programiv_p)(program, pname as u32, params)
    }

    pub fn get_query_objecti64v(
        &self,
        id: u32,
        pname: enums::QueryObjectParameterName,
        params: *mut i64,
    ) {
        (self.get_query_objecti64v_p)(id, pname as u32, params)
    }

    pub fn get_query_objectiv(
        &self,
        id: u32,
        pname: enums::QueryObjectParameterName,
        params: *mut i32,
    ) {
        (self.get_query_objectiv_p)(id, pname as u32, params)
    }

    pub fn get_query_objectui64v(
        &self,
        id: u32,
        pname: enums::QueryObjectParameterName,
        params: *mut u64,
    ) {
        (self.get_query_objectui64v_p)(id, pname as u32, params)
    }

    pub fn get_query_objectuiv(
        &self,
        id: u32,
        pname: enums::QueryObjectParameterName,
        params: *mut u32,
    ) {
        (self.get_query_objectuiv_p)(id, pname as u32, params)
    }

    pub fn get_queryiv(
        &self,
        target: enums::QueryTarget,
        pname: enums::QueryParameterName,
        params: *mut i32,
    ) {
        (self.get_queryiv_p)(target as u32, pname as u32, params)
    }

    pub fn get_renderbuffer_parameteriv(
        &self,
        target: enums::RenderbufferTarget,
        pname: enums::RenderbufferParameterName,
        params: *mut i32,
    ) {
        (self.get_renderbuffer_parameteriv_p)(target as u32, pname as u32, params)
    }

    pub fn get_sampler_parameter_iiv(
        &self,
        sampler: u32,
        pname: enums::SamplerParameterI,
        params: *mut i32,
    ) {
        (self.get_sampler_parameter_iiv_p)(sampler, pname as u32, params)
    }

    pub fn get_sampler_parameter_iuiv(
        &self,
        sampler: u32,
        pname: enums::SamplerParameterI,
        params: *mut u32,
    ) {
        (self.get_sampler_parameter_iuiv_p)(sampler, pname as u32, params)
    }

    pub fn get_sampler_parameterfv(
        &self,
        sampler: u32,
        pname: enums::SamplerParameterF,
        params: *mut f32,
    ) {
        (self.get_sampler_parameterfv_p)(sampler, pname as u32, params)
    }

    pub fn get_sampler_parameteriv(
        &self,
        sampler: u32,
        pname: enums::SamplerParameterI,
        params: *mut i32,
    ) {
        (self.get_sampler_parameteriv_p)(sampler, pname as u32, params)
    }

    pub fn get_shader_info_log(
        &self,
        shader: u32,
        buf_size: i32,
        length: *mut i32,
        info_log: *mut i8,
    ) {
        (self.get_shader_info_log_p)(shader, buf_size, length, info_log)
    }

    pub fn get_shader_precision_format(
        &self,
        shadertype: enums::ShaderType,
        precisiontype: enums::PrecisionType,
        range: *mut i32,
        precision: *mut i32,
    ) {
        (self.get_shader_precision_format_p)(
            shadertype as u32,
            precisiontype as u32,
            range,
            precision,
        )
    }

    pub fn get_shader_source(&self, shader: u32, buf_size: i32, length: *mut i32, source: *mut i8) {
        (self.get_shader_source_p)(shader, buf_size, length, source)
    }

    pub fn get_shaderiv(&self, shader: u32, pname: enums::ShaderParameterName, params: *mut i32) {
        (self.get_shaderiv_p)(shader, pname as u32, params)
    }

    pub fn get_string(&self, name: enums::StringName) -> *const u8 {
        (self.get_string_p)(name as u32)
    }

    pub fn get_stringi(&self, name: enums::StringName, index: u32) -> *const u8 {
        (self.get_stringi_p)(name as u32, index)
    }

    pub fn get_synciv(
        &self,
        sync: *mut c_void,
        pname: enums::SyncParameterName,
        count: i32,
        length: *mut i32,
        values: *mut i32,
    ) {
        (self.get_synciv_p)(sync, pname as u32, count, length, values)
    }

    pub fn get_tex_envfv(
        &self,
        target: enums::TextureEnvTarget,
        pname: enums::TextureEnvParameter,
        params: *mut f32,
    ) {
        (self.get_tex_envfv_p)(target as u32, pname as u32, params)
    }

    pub fn get_tex_enviv(
        &self,
        target: enums::TextureEnvTarget,
        pname: enums::TextureEnvParameter,
        params: *mut i32,
    ) {
        (self.get_tex_enviv_p)(target as u32, pname as u32, params)
    }

    pub fn get_tex_envxv(
        &self,
        target: enums::TextureEnvTarget,
        pname: enums::TextureEnvParameter,
        params: *mut i32,
    ) {
        (self.get_tex_envxv_p)(target as u32, pname as u32, params)
    }

    pub fn get_tex_image(
        &self,
        target: enums::TextureTarget,
        level: i32,
        format: enums::PixelFormat,
        type_: enums::PixelType,
        pixels: *mut c_void,
    ) {
        (self.get_tex_image_p)(target as u32, level, format as u32, type_ as u32, pixels)
    }

    pub fn get_tex_level_parameterfv(
        &self,
        target: enums::TextureTarget,
        level: i32,
        pname: enums::GetTextureParameter,
        params: *mut f32,
    ) {
        (self.get_tex_level_parameterfv_p)(target as u32, level, pname as u32, params)
    }

    pub fn get_tex_level_parameteriv(
        &self,
        target: enums::TextureTarget,
        level: i32,
        pname: enums::GetTextureParameter,
        params: *mut i32,
    ) {
        (self.get_tex_level_parameteriv_p)(target as u32, level, pname as u32, params)
    }

    pub fn get_tex_parameter_iiv(
        &self,
        target: enums::TextureTarget,
        pname: enums::GetTextureParameter,
        params: *mut i32,
    ) {
        (self.get_tex_parameter_iiv_p)(target as u32, pname as u32, params)
    }

    pub fn get_tex_parameter_iuiv(
        &self,
        target: enums::TextureTarget,
        pname: enums::GetTextureParameter,
        params: *mut u32,
    ) {
        (self.get_tex_parameter_iuiv_p)(target as u32, pname as u32, params)
    }

    pub fn get_tex_parameterfv(
        &self,
        target: enums::TextureTarget,
        pname: enums::GetTextureParameter,
        params: *mut f32,
    ) {
        (self.get_tex_parameterfv_p)(target as u32, pname as u32, params)
    }

    pub fn get_tex_parameteriv(
        &self,
        target: enums::TextureTarget,
        pname: enums::GetTextureParameter,
        params: *mut i32,
    ) {
        (self.get_tex_parameteriv_p)(target as u32, pname as u32, params)
    }

    pub fn get_tex_parameterxv(
        &self,
        target: enums::TextureTarget,
        pname: enums::GetTextureParameter,
        params: *mut i32,
    ) {
        (self.get_tex_parameterxv_p)(target as u32, pname as u32, params)
    }

    pub fn get_transform_feedback_varying(
        &self,
        program: u32,
        index: u32,
        buf_size: i32,
        length: *mut i32,
        size: *mut i32,
        type_: *mut enums::AttributeType,
        name: *mut i8,
    ) {
        (self.get_transform_feedback_varying_p)(
            program,
            index,
            buf_size,
            length,
            size,
            type_ as *mut u32,
            name,
        )
    }

    pub fn get_uniform_block_index(&self, program: u32, uniform_block_name: *const i8) -> u32 {
        (self.get_uniform_block_index_p)(program, uniform_block_name)
    }

    pub fn get_uniform_indices(
        &self,
        program: u32,
        uniform_count: i32,
        uniform_names: *const *const i8,
        uniform_indices: *mut u32,
    ) {
        (self.get_uniform_indices_p)(program, uniform_count, uniform_names, uniform_indices)
    }

    pub fn get_uniform_location(&self, program: u32, name: *const i8) -> i32 {
        (self.get_uniform_location_p)(program, name)
    }

    pub fn get_uniformfv(&self, program: u32, location: i32, params: *mut f32) {
        (self.get_uniformfv_p)(program, location, params)
    }

    pub fn get_uniformiv(&self, program: u32, location: i32, params: *mut i32) {
        (self.get_uniformiv_p)(program, location, params)
    }

    pub fn get_uniformuiv(&self, program: u32, location: i32, params: *mut u32) {
        (self.get_uniformuiv_p)(program, location, params)
    }

    pub fn get_vertex_attrib_iiv(
        &self,
        index: u32,
        pname: enums::VertexAttribEnum,
        params: *mut i32,
    ) {
        (self.get_vertex_attrib_iiv_p)(index, pname as u32, params)
    }

    pub fn get_vertex_attrib_iuiv(
        &self,
        index: u32,
        pname: enums::VertexAttribEnum,
        params: *mut u32,
    ) {
        (self.get_vertex_attrib_iuiv_p)(index, pname as u32, params)
    }

    pub fn get_vertex_attrib_pointerv(
        &self,
        index: u32,
        pname: enums::VertexAttribPointerPropertyARB,
        pointer: *mut *mut c_void,
    ) {
        (self.get_vertex_attrib_pointerv_p)(index, pname as u32, pointer)
    }

    pub fn get_vertex_attribdv(
        &self,
        index: u32,
        pname: enums::VertexAttribPropertyARB,
        params: *mut f64,
    ) {
        (self.get_vertex_attribdv_p)(index, pname as u32, params)
    }

    pub fn get_vertex_attribfv(
        &self,
        index: u32,
        pname: enums::VertexAttribPropertyARB,
        params: *mut f32,
    ) {
        (self.get_vertex_attribfv_p)(index, pname as u32, params)
    }

    pub fn get_vertex_attribiv(
        &self,
        index: u32,
        pname: enums::VertexAttribPropertyARB,
        params: *mut i32,
    ) {
        (self.get_vertex_attribiv_p)(index, pname as u32, params)
    }

    pub fn getn_uniformfv(&self, program: u32, location: i32, buf_size: i32, params: *mut f32) {
        (self.getn_uniformfv_p)(program, location, buf_size, params)
    }

    pub fn getn_uniformiv(&self, program: u32, location: i32, buf_size: i32, params: *mut i32) {
        (self.getn_uniformiv_p)(program, location, buf_size, params)
    }

    pub fn getn_uniformuiv(&self, program: u32, location: i32, buf_size: i32, params: *mut u32) {
        (self.getn_uniformuiv_p)(program, location, buf_size, params)
    }

    pub fn hint(&self, target: enums::HintTarget, mode: enums::HintMode) {
        (self.hint_p)(target as u32, mode as u32)
    }

    pub fn invalidate_framebuffer(
        &self,
        target: enums::FramebufferTarget,
        num_attachments: i32,
        attachments: *const enums::InvalidateFramebufferAttachment,
    ) {
        (self.invalidate_framebuffer_p)(target as u32, num_attachments, attachments as *const u32)
    }

    pub fn invalidate_sub_framebuffer(
        &self,
        target: enums::FramebufferTarget,
        num_attachments: i32,
        attachments: *const enums::InvalidateFramebufferAttachment,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        (self.invalidate_sub_framebuffer_p)(
            target as u32,
            num_attachments,
            attachments as *const u32,
            x,
            y,
            width,
            height,
        )
    }

    pub fn is_buffer(&self, buffer: u32) -> bool {
        (self.is_buffer_p)(buffer) != 0
    }

    pub fn is_enabled(&self, cap: enums::EnableCap) -> bool {
        (self.is_enabled_p)(cap as u32) != 0
    }

    pub fn is_enabledi(&self, target: enums::EnableCap, index: u32) -> bool {
        (self.is_enabledi_p)(target as u32, index) != 0
    }

    pub fn is_framebuffer(&self, framebuffer: u32) -> bool {
        (self.is_framebuffer_p)(framebuffer) != 0
    }

    pub fn is_program(&self, program: u32) -> bool {
        (self.is_program_p)(program) != 0
    }

    pub fn is_program_pipeline(&self, pipeline: u32) -> bool {
        (self.is_program_pipeline_p)(pipeline) != 0
    }

    pub fn is_query(&self, id: u32) -> bool {
        (self.is_query_p)(id) != 0
    }

    pub fn is_renderbuffer(&self, renderbuffer: u32) -> bool {
        (self.is_renderbuffer_p)(renderbuffer) != 0
    }

    pub fn is_sampler(&self, sampler: u32) -> bool {
        (self.is_sampler_p)(sampler) != 0
    }

    pub fn is_shader(&self, shader: u32) -> bool {
        (self.is_shader_p)(shader) != 0
    }

    pub fn is_sync(&self, sync: *mut c_void) -> bool {
        (self.is_sync_p)(sync) != 0
    }

    pub fn is_texture(&self, texture: u32) -> bool {
        (self.is_texture_p)(texture) != 0
    }

    pub fn is_transform_feedback(&self, id: u32) -> bool {
        (self.is_transform_feedback_p)(id) != 0
    }

    pub fn is_vertex_array(&self, array: u32) -> bool {
        (self.is_vertex_array_p)(array) != 0
    }

    pub fn light_modelf(&self, pname: enums::LightModelParameter, param: f32) {
        (self.light_modelf_p)(pname as u32, param)
    }

    pub fn light_modelfv(&self, pname: enums::LightModelParameter, params: *const f32) {
        (self.light_modelfv_p)(pname as u32, params)
    }

    pub fn light_modelx(&self, pname: enums::LightModelParameter, param: i32) {
        (self.light_modelx_p)(pname as u32, param)
    }

    pub fn light_modelxv(&self, pname: enums::LightModelParameter, param: *const i32) {
        (self.light_modelxv_p)(pname as u32, param)
    }

    pub fn lightf(&self, light: enums::LightName, pname: enums::LightParameter, param: f32) {
        (self.lightf_p)(light as u32, pname as u32, param)
    }

    pub fn lightfv(
        &self,
        light: enums::LightName,
        pname: enums::LightParameter,
        params: *const f32,
    ) {
        (self.lightfv_p)(light as u32, pname as u32, params)
    }

    pub fn lightx(&self, light: enums::LightName, pname: enums::LightParameter, param: i32) {
        (self.lightx_p)(light as u32, pname as u32, param)
    }

    pub fn lightxv(
        &self,
        light: enums::LightName,
        pname: enums::LightParameter,
        params: *const i32,
    ) {
        (self.lightxv_p)(light as u32, pname as u32, params)
    }

    pub fn line_width(&self, width: f32) {
        (self.line_width_p)(width)
    }

    pub fn line_widthx(&self, width: i32) {
        (self.line_widthx_p)(width)
    }

    pub fn link_program(&self, program: u32) {
        (self.link_program_p)(program)
    }

    pub fn load_identity(&self) {
        (self.load_identity_p)()
    }

    pub fn load_matrixf(&self, m: *const f32) {
        (self.load_matrixf_p)(m)
    }

    pub fn load_matrixx(&self, m: *const i32) {
        (self.load_matrixx_p)(m)
    }

    pub fn logic_op(&self, opcode: enums::LogicOp) {
        (self.logic_op_p)(opcode as u32)
    }

    pub fn map_buffer(
        &self,
        target: enums::BufferTargetARB,
        access: enums::BufferAccessARB,
    ) -> *mut c_void {
        (self.map_buffer_p)(target as u32, access as u32)
    }

    pub fn map_buffer_range(
        &self,
        target: enums::BufferTargetARB,
        offset: isize,
        length: isize,
        access: enums::MapBufferAccessMask,
    ) -> *mut c_void {
        (self.map_buffer_range_p)(target as u32, offset, length, access as u32)
    }

    pub fn materialf(
        &self,
        face: enums::TriangleFace,
        pname: enums::MaterialParameter,
        param: f32,
    ) {
        (self.materialf_p)(face as u32, pname as u32, param)
    }

    pub fn materialfv(
        &self,
        face: enums::TriangleFace,
        pname: enums::MaterialParameter,
        params: *const f32,
    ) {
        (self.materialfv_p)(face as u32, pname as u32, params)
    }

    pub fn materialx(
        &self,
        face: enums::TriangleFace,
        pname: enums::MaterialParameter,
        param: i32,
    ) {
        (self.materialx_p)(face as u32, pname as u32, param)
    }

    pub fn materialxv(
        &self,
        face: enums::TriangleFace,
        pname: enums::MaterialParameter,
        param: *const i32,
    ) {
        (self.materialxv_p)(face as u32, pname as u32, param)
    }

    pub fn matrix_mode(&self, mode: enums::MatrixMode) {
        (self.matrix_mode_p)(mode as u32)
    }

    pub fn memory_barrier(&self, barriers: enums::MemoryBarrierMask) {
        (self.memory_barrier_p)(barriers as u32)
    }

    pub fn memory_barrier_by_region(&self, barriers: enums::MemoryBarrierMask) {
        (self.memory_barrier_by_region_p)(barriers as u32)
    }

    pub fn min_sample_shading(&self, value: f32) {
        (self.min_sample_shading_p)(value)
    }

    pub fn mult_matrixf(&self, m: *const f32) {
        (self.mult_matrixf_p)(m)
    }

    pub fn mult_matrixx(&self, m: *const i32) {
        (self.mult_matrixx_p)(m)
    }

    pub fn multi_draw_arrays(
        &self,
        mode: enums::PrimitiveType,
        first: *const i32,
        count: *const i32,
        drawcount: i32,
    ) {
        (self.multi_draw_arrays_p)(mode as u32, first, count, drawcount)
    }

    pub fn multi_draw_elements(
        &self,
        mode: enums::PrimitiveType,
        count: *const i32,
        type_: enums::DrawElementsType,
        indices: *const *const c_void,
        drawcount: i32,
    ) {
        (self.multi_draw_elements_p)(mode as u32, count, type_ as u32, indices, drawcount)
    }

    pub fn multi_draw_elements_base_vertex(
        &self,
        mode: enums::PrimitiveType,
        count: *const i32,
        type_: enums::DrawElementsType,
        indices: *const *const c_void,
        drawcount: i32,
        basevertex: *const i32,
    ) {
        (self.multi_draw_elements_base_vertex_p)(
            mode as u32,
            count,
            type_ as u32,
            indices,
            drawcount,
            basevertex,
        )
    }

    pub fn multi_tex_coord4f(&self, target: enums::TextureUnit, s: f32, t: f32, r: f32, q: f32) {
        (self.multi_tex_coord4f_p)(target as u32, s, t, r, q)
    }

    pub fn multi_tex_coord4x(&self, texture: enums::TextureUnit, s: i32, t: i32, r: i32, q: i32) {
        (self.multi_tex_coord4x_p)(texture as u32, s, t, r, q)
    }

    pub fn multi_tex_coord_p1ui(
        &self,
        texture: enums::TextureUnit,
        type_: enums::TexCoordPointerType,
        coords: u32,
    ) {
        (self.multi_tex_coord_p1ui_p)(texture as u32, type_ as u32, coords)
    }

    pub fn multi_tex_coord_p1uiv(
        &self,
        texture: enums::TextureUnit,
        type_: enums::TexCoordPointerType,
        coords: *const u32,
    ) {
        (self.multi_tex_coord_p1uiv_p)(texture as u32, type_ as u32, coords)
    }

    pub fn multi_tex_coord_p2ui(
        &self,
        texture: enums::TextureUnit,
        type_: enums::TexCoordPointerType,
        coords: u32,
    ) {
        (self.multi_tex_coord_p2ui_p)(texture as u32, type_ as u32, coords)
    }

    pub fn multi_tex_coord_p2uiv(
        &self,
        texture: enums::TextureUnit,
        type_: enums::TexCoordPointerType,
        coords: *const u32,
    ) {
        (self.multi_tex_coord_p2uiv_p)(texture as u32, type_ as u32, coords)
    }

    pub fn multi_tex_coord_p3ui(
        &self,
        texture: enums::TextureUnit,
        type_: enums::TexCoordPointerType,
        coords: u32,
    ) {
        (self.multi_tex_coord_p3ui_p)(texture as u32, type_ as u32, coords)
    }

    pub fn multi_tex_coord_p3uiv(
        &self,
        texture: enums::TextureUnit,
        type_: enums::TexCoordPointerType,
        coords: *const u32,
    ) {
        (self.multi_tex_coord_p3uiv_p)(texture as u32, type_ as u32, coords)
    }

    pub fn multi_tex_coord_p4ui(
        &self,
        texture: enums::TextureUnit,
        type_: enums::TexCoordPointerType,
        coords: u32,
    ) {
        (self.multi_tex_coord_p4ui_p)(texture as u32, type_ as u32, coords)
    }

    pub fn multi_tex_coord_p4uiv(
        &self,
        texture: enums::TextureUnit,
        type_: enums::TexCoordPointerType,
        coords: *const u32,
    ) {
        (self.multi_tex_coord_p4uiv_p)(texture as u32, type_ as u32, coords)
    }

    pub fn normal3f(&self, nx: f32, ny: f32, nz: f32) {
        (self.normal3f_p)(nx, ny, nz)
    }

    pub fn normal3x(&self, nx: i32, ny: i32, nz: i32) {
        (self.normal3x_p)(nx, ny, nz)
    }

    pub fn normal_p3ui(&self, type_: enums::NormalPointerType, coords: u32) {
        (self.normal_p3ui_p)(type_ as u32, coords)
    }

    pub fn normal_p3uiv(&self, type_: enums::NormalPointerType, coords: *const u32) {
        (self.normal_p3uiv_p)(type_ as u32, coords)
    }

    pub fn normal_pointer(
        &self,
        type_: enums::NormalPointerType,
        stride: i32,
        pointer: *const c_void,
    ) {
        (self.normal_pointer_p)(type_ as u32, stride, pointer)
    }

    pub fn object_label(
        &self,
        identifier: enums::ObjectIdentifier,
        name: u32,
        length: i32,
        label: *const i8,
    ) {
        (self.object_label_p)(identifier as u32, name, length, label)
    }

    pub fn object_ptr_label(&self, ptr: *const c_void, length: i32, label: *const i8) {
        (self.object_ptr_label_p)(ptr, length, label)
    }

    pub fn orthof(&self, l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) {
        (self.orthof_p)(l, r, b, t, n, f)
    }

    pub fn orthox(&self, l: i32, r: i32, b: i32, t: i32, n: i32, f: i32) {
        (self.orthox_p)(l, r, b, t, n, f)
    }

    pub fn patch_parameteri(&self, pname: enums::PatchParameterName, value: i32) {
        (self.patch_parameteri_p)(pname as u32, value)
    }

    pub fn pause_transform_feedback(&self) {
        (self.pause_transform_feedback_p)()
    }

    pub fn pixel_storef(&self, pname: enums::PixelStoreParameter, param: f32) {
        (self.pixel_storef_p)(pname as u32, param)
    }

    pub fn pixel_storei(&self, pname: enums::PixelStoreParameter, param: i32) {
        (self.pixel_storei_p)(pname as u32, param)
    }

    pub fn point_parameterf(&self, pname: enums::PointParameterNameARB, param: f32) {
        (self.point_parameterf_p)(pname as u32, param)
    }

    pub fn point_parameterfv(&self, pname: enums::PointParameterNameARB, params: *const f32) {
        (self.point_parameterfv_p)(pname as u32, params)
    }

    pub fn point_parameteri(&self, pname: enums::PointParameterNameARB, param: i32) {
        (self.point_parameteri_p)(pname as u32, param)
    }

    pub fn point_parameteriv(&self, pname: enums::PointParameterNameARB, params: *const i32) {
        (self.point_parameteriv_p)(pname as u32, params)
    }

    pub fn point_parameterx(&self, pname: enums::PointParameterNameARB, param: i32) {
        (self.point_parameterx_p)(pname as u32, param)
    }

    pub fn point_parameterxv(&self, pname: enums::PointParameterNameARB, params: *const i32) {
        (self.point_parameterxv_p)(pname as u32, params)
    }

    pub fn point_size(&self, size: f32) {
        (self.point_size_p)(size)
    }

    pub fn point_sizex(&self, size: i32) {
        (self.point_sizex_p)(size)
    }

    pub fn polygon_mode(&self, face: enums::TriangleFace, mode: enums::PolygonMode) {
        (self.polygon_mode_p)(face as u32, mode as u32)
    }

    pub fn polygon_offset(&self, factor: f32, units: f32) {
        (self.polygon_offset_p)(factor, units)
    }

    pub fn polygon_offsetx(&self, factor: i32, units: i32) {
        (self.polygon_offsetx_p)(factor, units)
    }

    pub fn pop_debug_group(&self) {
        (self.pop_debug_group_p)()
    }

    pub fn pop_matrix(&self) {
        (self.pop_matrix_p)()
    }

    pub fn primitive_bounding_box(
        &self,
        min_x: f32,
        min_y: f32,
        min_z: f32,
        min_w: f32,
        max_x: f32,
        max_y: f32,
        max_z: f32,
        max_w: f32,
    ) {
        (self.primitive_bounding_box_p)(min_x, min_y, min_z, min_w, max_x, max_y, max_z, max_w)
    }

    pub fn primitive_restart_index(&self, index: u32) {
        (self.primitive_restart_index_p)(index)
    }

    pub fn program_binary(
        &self,
        program: u32,
        binary_format: u32,
        binary: *const c_void,
        length: i32,
    ) {
        (self.program_binary_p)(program, binary_format, binary, length)
    }

    pub fn program_parameteri(
        &self,
        program: u32,
        pname: enums::ProgramParameterPName,
        value: i32,
    ) {
        (self.program_parameteri_p)(program, pname as u32, value)
    }

    pub fn program_uniform1f(&self, program: u32, location: i32, v0: f32) {
        (self.program_uniform1f_p)(program, location, v0)
    }

    pub fn program_uniform1fv(&self, program: u32, location: i32, count: i32, value: *const f32) {
        (self.program_uniform1fv_p)(program, location, count, value)
    }

    pub fn program_uniform1i(&self, program: u32, location: i32, v0: i32) {
        (self.program_uniform1i_p)(program, location, v0)
    }

    pub fn program_uniform1iv(&self, program: u32, location: i32, count: i32, value: *const i32) {
        (self.program_uniform1iv_p)(program, location, count, value)
    }

    pub fn program_uniform1ui(&self, program: u32, location: i32, v0: u32) {
        (self.program_uniform1ui_p)(program, location, v0)
    }

    pub fn program_uniform1uiv(&self, program: u32, location: i32, count: i32, value: *const u32) {
        (self.program_uniform1uiv_p)(program, location, count, value)
    }

    pub fn program_uniform2f(&self, program: u32, location: i32, v0: f32, v1: f32) {
        (self.program_uniform2f_p)(program, location, v0, v1)
    }

    pub fn program_uniform2fv(&self, program: u32, location: i32, count: i32, value: *const f32) {
        (self.program_uniform2fv_p)(program, location, count, value)
    }

    pub fn program_uniform2i(&self, program: u32, location: i32, v0: i32, v1: i32) {
        (self.program_uniform2i_p)(program, location, v0, v1)
    }

    pub fn program_uniform2iv(&self, program: u32, location: i32, count: i32, value: *const i32) {
        (self.program_uniform2iv_p)(program, location, count, value)
    }

    pub fn program_uniform2ui(&self, program: u32, location: i32, v0: u32, v1: u32) {
        (self.program_uniform2ui_p)(program, location, v0, v1)
    }

    pub fn program_uniform2uiv(&self, program: u32, location: i32, count: i32, value: *const u32) {
        (self.program_uniform2uiv_p)(program, location, count, value)
    }

    pub fn program_uniform3f(&self, program: u32, location: i32, v0: f32, v1: f32, v2: f32) {
        (self.program_uniform3f_p)(program, location, v0, v1, v2)
    }

    pub fn program_uniform3fv(&self, program: u32, location: i32, count: i32, value: *const f32) {
        (self.program_uniform3fv_p)(program, location, count, value)
    }

    pub fn program_uniform3i(&self, program: u32, location: i32, v0: i32, v1: i32, v2: i32) {
        (self.program_uniform3i_p)(program, location, v0, v1, v2)
    }

    pub fn program_uniform3iv(&self, program: u32, location: i32, count: i32, value: *const i32) {
        (self.program_uniform3iv_p)(program, location, count, value)
    }

    pub fn program_uniform3ui(&self, program: u32, location: i32, v0: u32, v1: u32, v2: u32) {
        (self.program_uniform3ui_p)(program, location, v0, v1, v2)
    }

    pub fn program_uniform3uiv(&self, program: u32, location: i32, count: i32, value: *const u32) {
        (self.program_uniform3uiv_p)(program, location, count, value)
    }

    pub fn program_uniform4f(
        &self,
        program: u32,
        location: i32,
        v0: f32,
        v1: f32,
        v2: f32,
        v3: f32,
    ) {
        (self.program_uniform4f_p)(program, location, v0, v1, v2, v3)
    }

    pub fn program_uniform4fv(&self, program: u32, location: i32, count: i32, value: *const f32) {
        (self.program_uniform4fv_p)(program, location, count, value)
    }

    pub fn program_uniform4i(
        &self,
        program: u32,
        location: i32,
        v0: i32,
        v1: i32,
        v2: i32,
        v3: i32,
    ) {
        (self.program_uniform4i_p)(program, location, v0, v1, v2, v3)
    }

    pub fn program_uniform4iv(&self, program: u32, location: i32, count: i32, value: *const i32) {
        (self.program_uniform4iv_p)(program, location, count, value)
    }

    pub fn program_uniform4ui(
        &self,
        program: u32,
        location: i32,
        v0: u32,
        v1: u32,
        v2: u32,
        v3: u32,
    ) {
        (self.program_uniform4ui_p)(program, location, v0, v1, v2, v3)
    }

    pub fn program_uniform4uiv(&self, program: u32, location: i32, count: i32, value: *const u32) {
        (self.program_uniform4uiv_p)(program, location, count, value)
    }

    pub fn program_uniform_matrix2fv(
        &self,
        program: u32,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.program_uniform_matrix2fv_p)(program, location, count, transpose as u8, value)
    }

    pub fn program_uniform_matrix2x3fv(
        &self,
        program: u32,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.program_uniform_matrix2x3fv_p)(program, location, count, transpose as u8, value)
    }

    pub fn program_uniform_matrix2x4fv(
        &self,
        program: u32,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.program_uniform_matrix2x4fv_p)(program, location, count, transpose as u8, value)
    }

    pub fn program_uniform_matrix3fv(
        &self,
        program: u32,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.program_uniform_matrix3fv_p)(program, location, count, transpose as u8, value)
    }

    pub fn program_uniform_matrix3x2fv(
        &self,
        program: u32,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.program_uniform_matrix3x2fv_p)(program, location, count, transpose as u8, value)
    }

    pub fn program_uniform_matrix3x4fv(
        &self,
        program: u32,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.program_uniform_matrix3x4fv_p)(program, location, count, transpose as u8, value)
    }

    pub fn program_uniform_matrix4fv(
        &self,
        program: u32,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.program_uniform_matrix4fv_p)(program, location, count, transpose as u8, value)
    }

    pub fn program_uniform_matrix4x2fv(
        &self,
        program: u32,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.program_uniform_matrix4x2fv_p)(program, location, count, transpose as u8, value)
    }

    pub fn program_uniform_matrix4x3fv(
        &self,
        program: u32,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.program_uniform_matrix4x3fv_p)(program, location, count, transpose as u8, value)
    }

    pub fn provoking_vertex(&self, mode: enums::VertexProvokingMode) {
        (self.provoking_vertex_p)(mode as u32)
    }

    pub fn push_debug_group(
        &self,
        source: enums::DebugSource,
        id: u32,
        length: i32,
        message: *const i8,
    ) {
        (self.push_debug_group_p)(source as u32, id, length, message)
    }

    pub fn push_matrix(&self) {
        (self.push_matrix_p)()
    }

    pub fn query_counter(&self, id: u32, target: enums::QueryCounterTarget) {
        (self.query_counter_p)(id, target as u32)
    }

    pub fn read_buffer(&self, src: enums::ReadBufferMode) {
        (self.read_buffer_p)(src as u32)
    }

    pub fn read_pixels(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: enums::PixelFormat,
        type_: enums::PixelType,
        pixels: *mut c_void,
    ) {
        (self.read_pixels_p)(x, y, width, height, format as u32, type_ as u32, pixels)
    }

    pub fn readn_pixels(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: enums::PixelFormat,
        type_: enums::PixelType,
        buf_size: i32,
        data: *mut c_void,
    ) {
        (self.readn_pixels_p)(
            x,
            y,
            width,
            height,
            format as u32,
            type_ as u32,
            buf_size,
            data,
        )
    }

    pub fn release_shader_compiler(&self) {
        (self.release_shader_compiler_p)()
    }

    pub fn renderbuffer_storage(
        &self,
        target: enums::RenderbufferTarget,
        internalformat: enums::InternalFormat,
        width: i32,
        height: i32,
    ) {
        (self.renderbuffer_storage_p)(target as u32, internalformat as u32, width, height)
    }

    pub fn renderbuffer_storage_multisample(
        &self,
        target: enums::RenderbufferTarget,
        samples: i32,
        internalformat: enums::InternalFormat,
        width: i32,
        height: i32,
    ) {
        (self.renderbuffer_storage_multisample_p)(
            target as u32,
            samples,
            internalformat as u32,
            width,
            height,
        )
    }

    pub fn resume_transform_feedback(&self) {
        (self.resume_transform_feedback_p)()
    }

    pub fn rotatef(&self, angle: f32, x: f32, y: f32, z: f32) {
        (self.rotatef_p)(angle, x, y, z)
    }

    pub fn rotatex(&self, angle: i32, x: i32, y: i32, z: i32) {
        (self.rotatex_p)(angle, x, y, z)
    }

    pub fn sample_coverage(&self, value: f32, invert: bool) {
        (self.sample_coverage_p)(value, invert as u8)
    }

    pub fn sample_coveragex(&self, value: i32, invert: bool) {
        (self.sample_coveragex_p)(value, invert as u8)
    }

    pub fn sample_maski(&self, mask_number: u32, mask: u32) {
        (self.sample_maski_p)(mask_number, mask)
    }

    pub fn sampler_parameter_iiv(
        &self,
        sampler: u32,
        pname: enums::SamplerParameterI,
        param: *const i32,
    ) {
        (self.sampler_parameter_iiv_p)(sampler, pname as u32, param)
    }

    pub fn sampler_parameter_iuiv(
        &self,
        sampler: u32,
        pname: enums::SamplerParameterI,
        param: *const u32,
    ) {
        (self.sampler_parameter_iuiv_p)(sampler, pname as u32, param)
    }

    pub fn sampler_parameterf(&self, sampler: u32, pname: enums::SamplerParameterF, param: f32) {
        (self.sampler_parameterf_p)(sampler, pname as u32, param)
    }

    pub fn sampler_parameterfv(
        &self,
        sampler: u32,
        pname: enums::SamplerParameterF,
        param: *const f32,
    ) {
        (self.sampler_parameterfv_p)(sampler, pname as u32, param)
    }

    pub fn sampler_parameteri(&self, sampler: u32, pname: enums::SamplerParameterI, param: i32) {
        (self.sampler_parameteri_p)(sampler, pname as u32, param)
    }

    pub fn sampler_parameteriv(
        &self,
        sampler: u32,
        pname: enums::SamplerParameterI,
        param: *const i32,
    ) {
        (self.sampler_parameteriv_p)(sampler, pname as u32, param)
    }

    pub fn scalef(&self, x: f32, y: f32, z: f32) {
        (self.scalef_p)(x, y, z)
    }

    pub fn scalex(&self, x: i32, y: i32, z: i32) {
        (self.scalex_p)(x, y, z)
    }

    pub fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {
        (self.scissor_p)(x, y, width, height)
    }

    pub fn secondary_color_p3ui(&self, type_: enums::ColorPointerType, color: u32) {
        (self.secondary_color_p3ui_p)(type_ as u32, color)
    }

    pub fn secondary_color_p3uiv(&self, type_: enums::ColorPointerType, color: *const u32) {
        (self.secondary_color_p3uiv_p)(type_ as u32, color)
    }

    pub fn shade_model(&self, mode: enums::ShadingModel) {
        (self.shade_model_p)(mode as u32)
    }

    pub fn shader_binary(
        &self,
        count: i32,
        shaders: *const u32,
        binary_format: u32,
        binary: *const c_void,
        length: i32,
    ) {
        (self.shader_binary_p)(count, shaders, binary_format, binary, length)
    }

    pub fn shader_source(
        &self,
        shader: u32,
        count: i32,
        string: *const *const i8,
        length: *const i32,
    ) {
        (self.shader_source_p)(shader, count, string, length)
    }

    pub fn stencil_func(&self, func: enums::StencilFunction, ref_: i32, mask: u32) {
        (self.stencil_func_p)(func as u32, ref_, mask)
    }

    pub fn stencil_func_separate(
        &self,
        face: enums::TriangleFace,
        func: enums::StencilFunction,
        ref_: i32,
        mask: u32,
    ) {
        (self.stencil_func_separate_p)(face as u32, func as u32, ref_, mask)
    }

    pub fn stencil_mask(&self, mask: u32) {
        (self.stencil_mask_p)(mask)
    }

    pub fn stencil_mask_separate(&self, face: enums::TriangleFace, mask: u32) {
        (self.stencil_mask_separate_p)(face as u32, mask)
    }

    pub fn stencil_op(
        &self,
        fail: enums::StencilOp,
        zfail: enums::StencilOp,
        zpass: enums::StencilOp,
    ) {
        (self.stencil_op_p)(fail as u32, zfail as u32, zpass as u32)
    }

    pub fn stencil_op_separate(
        &self,
        face: enums::TriangleFace,
        sfail: enums::StencilOp,
        dpfail: enums::StencilOp,
        dppass: enums::StencilOp,
    ) {
        (self.stencil_op_separate_p)(face as u32, sfail as u32, dpfail as u32, dppass as u32)
    }

    pub fn tex_buffer(
        &self,
        target: enums::TextureTarget,
        internalformat: enums::SizedInternalFormat,
        buffer: u32,
    ) {
        (self.tex_buffer_p)(target as u32, internalformat as u32, buffer)
    }

    pub fn tex_buffer_range(
        &self,
        target: enums::TextureTarget,
        internalformat: enums::SizedInternalFormat,
        buffer: u32,
        offset: isize,
        size: isize,
    ) {
        (self.tex_buffer_range_p)(target as u32, internalformat as u32, buffer, offset, size)
    }

    pub fn tex_coord_p1ui(&self, type_: enums::TexCoordPointerType, coords: u32) {
        (self.tex_coord_p1ui_p)(type_ as u32, coords)
    }

    pub fn tex_coord_p1uiv(&self, type_: enums::TexCoordPointerType, coords: *const u32) {
        (self.tex_coord_p1uiv_p)(type_ as u32, coords)
    }

    pub fn tex_coord_p2ui(&self, type_: enums::TexCoordPointerType, coords: u32) {
        (self.tex_coord_p2ui_p)(type_ as u32, coords)
    }

    pub fn tex_coord_p2uiv(&self, type_: enums::TexCoordPointerType, coords: *const u32) {
        (self.tex_coord_p2uiv_p)(type_ as u32, coords)
    }

    pub fn tex_coord_p3ui(&self, type_: enums::TexCoordPointerType, coords: u32) {
        (self.tex_coord_p3ui_p)(type_ as u32, coords)
    }

    pub fn tex_coord_p3uiv(&self, type_: enums::TexCoordPointerType, coords: *const u32) {
        (self.tex_coord_p3uiv_p)(type_ as u32, coords)
    }

    pub fn tex_coord_p4ui(&self, type_: enums::TexCoordPointerType, coords: u32) {
        (self.tex_coord_p4ui_p)(type_ as u32, coords)
    }

    pub fn tex_coord_p4uiv(&self, type_: enums::TexCoordPointerType, coords: *const u32) {
        (self.tex_coord_p4uiv_p)(type_ as u32, coords)
    }

    pub fn tex_coord_pointer(
        &self,
        size: i32,
        type_: enums::TexCoordPointerType,
        stride: i32,
        pointer: *const c_void,
    ) {
        (self.tex_coord_pointer_p)(size, type_ as u32, stride, pointer)
    }

    pub fn tex_envf(
        &self,
        target: enums::TextureEnvTarget,
        pname: enums::TextureEnvParameter,
        param: f32,
    ) {
        (self.tex_envf_p)(target as u32, pname as u32, param)
    }

    pub fn tex_envfv(
        &self,
        target: enums::TextureEnvTarget,
        pname: enums::TextureEnvParameter,
        params: *const f32,
    ) {
        (self.tex_envfv_p)(target as u32, pname as u32, params)
    }

    pub fn tex_envi(
        &self,
        target: enums::TextureEnvTarget,
        pname: enums::TextureEnvParameter,
        param: i32,
    ) {
        (self.tex_envi_p)(target as u32, pname as u32, param)
    }

    pub fn tex_enviv(
        &self,
        target: enums::TextureEnvTarget,
        pname: enums::TextureEnvParameter,
        params: *const i32,
    ) {
        (self.tex_enviv_p)(target as u32, pname as u32, params)
    }

    pub fn tex_envx(
        &self,
        target: enums::TextureEnvTarget,
        pname: enums::TextureEnvParameter,
        param: i32,
    ) {
        (self.tex_envx_p)(target as u32, pname as u32, param)
    }

    pub fn tex_envxv(
        &self,
        target: enums::TextureEnvTarget,
        pname: enums::TextureEnvParameter,
        params: *const i32,
    ) {
        (self.tex_envxv_p)(target as u32, pname as u32, params)
    }

    pub fn tex_image1_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        internalformat: enums::InternalFormat,
        width: i32,
        border: i32,
        format: enums::PixelFormat,
        type_: enums::PixelType,
        pixels: *const c_void,
    ) {
        (self.tex_image1_d_p)(
            target as u32,
            level,
            internalformat as i32,
            width,
            border,
            format as u32,
            type_ as u32,
            pixels,
        )
    }

    pub fn tex_image2_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        internalformat: enums::InternalFormat,
        width: i32,
        height: i32,
        border: i32,
        format: enums::PixelFormat,
        type_: enums::PixelType,
        pixels: *const c_void,
    ) {
        (self.tex_image2_d_p)(
            target as u32,
            level,
            internalformat as i32,
            width,
            height,
            border,
            format as u32,
            type_ as u32,
            pixels,
        )
    }

    pub fn tex_image2_d_multisample(
        &self,
        target: enums::TextureTarget,
        samples: i32,
        internalformat: enums::InternalFormat,
        width: i32,
        height: i32,
        fixedsamplelocations: bool,
    ) {
        (self.tex_image2_d_multisample_p)(
            target as u32,
            samples,
            internalformat as u32,
            width,
            height,
            fixedsamplelocations as u8,
        )
    }

    pub fn tex_image3_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        internalformat: enums::InternalFormat,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: enums::PixelFormat,
        type_: enums::PixelType,
        pixels: *const c_void,
    ) {
        (self.tex_image3_d_p)(
            target as u32,
            level,
            internalformat as i32,
            width,
            height,
            depth,
            border,
            format as u32,
            type_ as u32,
            pixels,
        )
    }

    pub fn tex_image3_d_multisample(
        &self,
        target: enums::TextureTarget,
        samples: i32,
        internalformat: enums::InternalFormat,
        width: i32,
        height: i32,
        depth: i32,
        fixedsamplelocations: bool,
    ) {
        (self.tex_image3_d_multisample_p)(
            target as u32,
            samples,
            internalformat as u32,
            width,
            height,
            depth,
            fixedsamplelocations as u8,
        )
    }

    pub fn tex_parameter_iiv(
        &self,
        target: enums::TextureTarget,
        pname: enums::TextureParameterName,
        params: *const i32,
    ) {
        (self.tex_parameter_iiv_p)(target as u32, pname as u32, params)
    }

    pub fn tex_parameter_iuiv(
        &self,
        target: enums::TextureTarget,
        pname: enums::TextureParameterName,
        params: *const u32,
    ) {
        (self.tex_parameter_iuiv_p)(target as u32, pname as u32, params)
    }

    pub fn tex_parameterf(
        &self,
        target: enums::TextureTarget,
        pname: enums::TextureParameterName,
        param: f32,
    ) {
        (self.tex_parameterf_p)(target as u32, pname as u32, param)
    }

    pub fn tex_parameterfv(
        &self,
        target: enums::TextureTarget,
        pname: enums::TextureParameterName,
        params: *const f32,
    ) {
        (self.tex_parameterfv_p)(target as u32, pname as u32, params)
    }

    pub fn tex_parameteri(
        &self,
        target: enums::TextureTarget,
        pname: enums::TextureParameterName,
        param: i32,
    ) {
        (self.tex_parameteri_p)(target as u32, pname as u32, param)
    }

    pub fn tex_parameteriv(
        &self,
        target: enums::TextureTarget,
        pname: enums::TextureParameterName,
        params: *const i32,
    ) {
        (self.tex_parameteriv_p)(target as u32, pname as u32, params)
    }

    pub fn tex_parameterx(
        &self,
        target: enums::TextureTarget,
        pname: enums::GetTextureParameter,
        param: i32,
    ) {
        (self.tex_parameterx_p)(target as u32, pname as u32, param)
    }

    pub fn tex_parameterxv(
        &self,
        target: enums::TextureTarget,
        pname: enums::GetTextureParameter,
        params: *const i32,
    ) {
        (self.tex_parameterxv_p)(target as u32, pname as u32, params)
    }

    pub fn tex_storage2_d(
        &self,
        target: enums::TextureTarget,
        levels: i32,
        internalformat: enums::SizedInternalFormat,
        width: i32,
        height: i32,
    ) {
        (self.tex_storage2_d_p)(target as u32, levels, internalformat as u32, width, height)
    }

    pub fn tex_storage2_d_multisample(
        &self,
        target: enums::TextureTarget,
        samples: i32,
        internalformat: enums::SizedInternalFormat,
        width: i32,
        height: i32,
        fixedsamplelocations: bool,
    ) {
        (self.tex_storage2_d_multisample_p)(
            target as u32,
            samples,
            internalformat as u32,
            width,
            height,
            fixedsamplelocations as u8,
        )
    }

    pub fn tex_storage3_d(
        &self,
        target: enums::TextureTarget,
        levels: i32,
        internalformat: enums::SizedInternalFormat,
        width: i32,
        height: i32,
        depth: i32,
    ) {
        (self.tex_storage3_d_p)(
            target as u32,
            levels,
            internalformat as u32,
            width,
            height,
            depth,
        )
    }

    pub fn tex_storage3_d_multisample(
        &self,
        target: enums::TextureTarget,
        samples: i32,
        internalformat: enums::SizedInternalFormat,
        width: i32,
        height: i32,
        depth: i32,
        fixedsamplelocations: bool,
    ) {
        (self.tex_storage3_d_multisample_p)(
            target as u32,
            samples,
            internalformat as u32,
            width,
            height,
            depth,
            fixedsamplelocations as u8,
        )
    }

    pub fn tex_sub_image1_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        xoffset: i32,
        width: i32,
        format: enums::PixelFormat,
        type_: enums::PixelType,
        pixels: *const c_void,
    ) {
        (self.tex_sub_image1_d_p)(
            target as u32,
            level,
            xoffset,
            width,
            format as u32,
            type_ as u32,
            pixels,
        )
    }

    pub fn tex_sub_image2_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: enums::PixelFormat,
        type_: enums::PixelType,
        pixels: *const c_void,
    ) {
        (self.tex_sub_image2_d_p)(
            target as u32,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format as u32,
            type_ as u32,
            pixels,
        )
    }

    pub fn tex_sub_image3_d(
        &self,
        target: enums::TextureTarget,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: enums::PixelFormat,
        type_: enums::PixelType,
        pixels: *const c_void,
    ) {
        (self.tex_sub_image3_d_p)(
            target as u32,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format as u32,
            type_ as u32,
            pixels,
        )
    }

    pub fn transform_feedback_varyings(
        &self,
        program: u32,
        count: i32,
        varyings: *const *const i8,
        buffer_mode: enums::TransformFeedbackBufferMode,
    ) {
        (self.transform_feedback_varyings_p)(program, count, varyings, buffer_mode as u32)
    }

    pub fn translatef(&self, x: f32, y: f32, z: f32) {
        (self.translatef_p)(x, y, z)
    }

    pub fn translatex(&self, x: i32, y: i32, z: i32) {
        (self.translatex_p)(x, y, z)
    }

    pub fn uniform1f(&self, location: i32, v0: f32) {
        (self.uniform1f_p)(location, v0)
    }

    pub fn uniform1fv(&self, location: i32, count: i32, value: *const f32) {
        (self.uniform1fv_p)(location, count, value)
    }

    pub fn uniform1i(&self, location: i32, v0: i32) {
        (self.uniform1i_p)(location, v0)
    }

    pub fn uniform1iv(&self, location: i32, count: i32, value: *const i32) {
        (self.uniform1iv_p)(location, count, value)
    }

    pub fn uniform1ui(&self, location: i32, v0: u32) {
        (self.uniform1ui_p)(location, v0)
    }

    pub fn uniform1uiv(&self, location: i32, count: i32, value: *const u32) {
        (self.uniform1uiv_p)(location, count, value)
    }

    pub fn uniform2f(&self, location: i32, v0: f32, v1: f32) {
        (self.uniform2f_p)(location, v0, v1)
    }

    pub fn uniform2fv(&self, location: i32, count: i32, value: *const f32) {
        (self.uniform2fv_p)(location, count, value)
    }

    pub fn uniform2i(&self, location: i32, v0: i32, v1: i32) {
        (self.uniform2i_p)(location, v0, v1)
    }

    pub fn uniform2iv(&self, location: i32, count: i32, value: *const i32) {
        (self.uniform2iv_p)(location, count, value)
    }

    pub fn uniform2ui(&self, location: i32, v0: u32, v1: u32) {
        (self.uniform2ui_p)(location, v0, v1)
    }

    pub fn uniform2uiv(&self, location: i32, count: i32, value: *const u32) {
        (self.uniform2uiv_p)(location, count, value)
    }

    pub fn uniform3f(&self, location: i32, v0: f32, v1: f32, v2: f32) {
        (self.uniform3f_p)(location, v0, v1, v2)
    }

    pub fn uniform3fv(&self, location: i32, count: i32, value: *const f32) {
        (self.uniform3fv_p)(location, count, value)
    }

    pub fn uniform3i(&self, location: i32, v0: i32, v1: i32, v2: i32) {
        (self.uniform3i_p)(location, v0, v1, v2)
    }

    pub fn uniform3iv(&self, location: i32, count: i32, value: *const i32) {
        (self.uniform3iv_p)(location, count, value)
    }

    pub fn uniform3ui(&self, location: i32, v0: u32, v1: u32, v2: u32) {
        (self.uniform3ui_p)(location, v0, v1, v2)
    }

    pub fn uniform3uiv(&self, location: i32, count: i32, value: *const u32) {
        (self.uniform3uiv_p)(location, count, value)
    }

    pub fn uniform4f(&self, location: i32, v0: f32, v1: f32, v2: f32, v3: f32) {
        (self.uniform4f_p)(location, v0, v1, v2, v3)
    }

    pub fn uniform4fv(&self, location: i32, count: i32, value: *const f32) {
        (self.uniform4fv_p)(location, count, value)
    }

    pub fn uniform4i(&self, location: i32, v0: i32, v1: i32, v2: i32, v3: i32) {
        (self.uniform4i_p)(location, v0, v1, v2, v3)
    }

    pub fn uniform4iv(&self, location: i32, count: i32, value: *const i32) {
        (self.uniform4iv_p)(location, count, value)
    }

    pub fn uniform4ui(&self, location: i32, v0: u32, v1: u32, v2: u32, v3: u32) {
        (self.uniform4ui_p)(location, v0, v1, v2, v3)
    }

    pub fn uniform4uiv(&self, location: i32, count: i32, value: *const u32) {
        (self.uniform4uiv_p)(location, count, value)
    }

    pub fn uniform_block_binding(
        &self,
        program: u32,
        uniform_block_index: u32,
        uniform_block_binding: u32,
    ) {
        (self.uniform_block_binding_p)(program, uniform_block_index, uniform_block_binding)
    }

    pub fn uniform_matrix2fv(&self, location: i32, count: i32, transpose: bool, value: *const f32) {
        (self.uniform_matrix2fv_p)(location, count, transpose as u8, value)
    }

    pub fn uniform_matrix2x3fv(
        &self,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.uniform_matrix2x3fv_p)(location, count, transpose as u8, value)
    }

    pub fn uniform_matrix2x4fv(
        &self,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.uniform_matrix2x4fv_p)(location, count, transpose as u8, value)
    }

    pub fn uniform_matrix3fv(&self, location: i32, count: i32, transpose: bool, value: *const f32) {
        (self.uniform_matrix3fv_p)(location, count, transpose as u8, value)
    }

    pub fn uniform_matrix3x2fv(
        &self,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.uniform_matrix3x2fv_p)(location, count, transpose as u8, value)
    }

    pub fn uniform_matrix3x4fv(
        &self,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.uniform_matrix3x4fv_p)(location, count, transpose as u8, value)
    }

    pub fn uniform_matrix4fv(&self, location: i32, count: i32, transpose: bool, value: *const f32) {
        (self.uniform_matrix4fv_p)(location, count, transpose as u8, value)
    }

    pub fn uniform_matrix4x2fv(
        &self,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.uniform_matrix4x2fv_p)(location, count, transpose as u8, value)
    }

    pub fn uniform_matrix4x3fv(
        &self,
        location: i32,
        count: i32,
        transpose: bool,
        value: *const f32,
    ) {
        (self.uniform_matrix4x3fv_p)(location, count, transpose as u8, value)
    }

    pub fn unmap_buffer(&self, target: enums::BufferTargetARB) -> bool {
        (self.unmap_buffer_p)(target as u32) != 0
    }

    pub fn use_program(&self, program: u32) {
        (self.use_program_p)(program)
    }

    pub fn use_program_stages(
        &self,
        pipeline: u32,
        stages: enums::UseProgramStageMask,
        program: u32,
    ) {
        (self.use_program_stages_p)(pipeline, stages as u32, program)
    }

    pub fn validate_program(&self, program: u32) {
        (self.validate_program_p)(program)
    }

    pub fn validate_program_pipeline(&self, pipeline: u32) {
        (self.validate_program_pipeline_p)(pipeline)
    }

    pub fn vertex_attrib1d(&self, index: u32, x: f64) {
        (self.vertex_attrib1d_p)(index, x)
    }

    pub fn vertex_attrib1dv(&self, index: u32, v: *const f64) {
        (self.vertex_attrib1dv_p)(index, v)
    }

    pub fn vertex_attrib1f(&self, index: u32, x: f32) {
        (self.vertex_attrib1f_p)(index, x)
    }

    pub fn vertex_attrib1fv(&self, index: u32, v: *const f32) {
        (self.vertex_attrib1fv_p)(index, v)
    }

    pub fn vertex_attrib1s(&self, index: u32, x: i16) {
        (self.vertex_attrib1s_p)(index, x)
    }

    pub fn vertex_attrib1sv(&self, index: u32, v: *const i16) {
        (self.vertex_attrib1sv_p)(index, v)
    }

    pub fn vertex_attrib2d(&self, index: u32, x: f64, y: f64) {
        (self.vertex_attrib2d_p)(index, x, y)
    }

    pub fn vertex_attrib2dv(&self, index: u32, v: *const f64) {
        (self.vertex_attrib2dv_p)(index, v)
    }

    pub fn vertex_attrib2f(&self, index: u32, x: f32, y: f32) {
        (self.vertex_attrib2f_p)(index, x, y)
    }

    pub fn vertex_attrib2fv(&self, index: u32, v: *const f32) {
        (self.vertex_attrib2fv_p)(index, v)
    }

    pub fn vertex_attrib2s(&self, index: u32, x: i16, y: i16) {
        (self.vertex_attrib2s_p)(index, x, y)
    }

    pub fn vertex_attrib2sv(&self, index: u32, v: *const i16) {
        (self.vertex_attrib2sv_p)(index, v)
    }

    pub fn vertex_attrib3d(&self, index: u32, x: f64, y: f64, z: f64) {
        (self.vertex_attrib3d_p)(index, x, y, z)
    }

    pub fn vertex_attrib3dv(&self, index: u32, v: *const f64) {
        (self.vertex_attrib3dv_p)(index, v)
    }

    pub fn vertex_attrib3f(&self, index: u32, x: f32, y: f32, z: f32) {
        (self.vertex_attrib3f_p)(index, x, y, z)
    }

    pub fn vertex_attrib3fv(&self, index: u32, v: *const f32) {
        (self.vertex_attrib3fv_p)(index, v)
    }

    pub fn vertex_attrib3s(&self, index: u32, x: i16, y: i16, z: i16) {
        (self.vertex_attrib3s_p)(index, x, y, z)
    }

    pub fn vertex_attrib3sv(&self, index: u32, v: *const i16) {
        (self.vertex_attrib3sv_p)(index, v)
    }

    pub fn vertex_attrib4_nbv(&self, index: u32, v: *const i8) {
        (self.vertex_attrib4_nbv_p)(index, v)
    }

    pub fn vertex_attrib4_niv(&self, index: u32, v: *const i32) {
        (self.vertex_attrib4_niv_p)(index, v)
    }

    pub fn vertex_attrib4_nsv(&self, index: u32, v: *const i16) {
        (self.vertex_attrib4_nsv_p)(index, v)
    }

    pub fn vertex_attrib4_nub(&self, index: u32, x: u8, y: u8, z: u8, w: u8) {
        (self.vertex_attrib4_nub_p)(index, x, y, z, w)
    }

    pub fn vertex_attrib4_nubv(&self, index: u32, v: *const u8) {
        (self.vertex_attrib4_nubv_p)(index, v)
    }

    pub fn vertex_attrib4_nuiv(&self, index: u32, v: *const u32) {
        (self.vertex_attrib4_nuiv_p)(index, v)
    }

    pub fn vertex_attrib4_nusv(&self, index: u32, v: *const u16) {
        (self.vertex_attrib4_nusv_p)(index, v)
    }

    pub fn vertex_attrib4bv(&self, index: u32, v: *const i8) {
        (self.vertex_attrib4bv_p)(index, v)
    }

    pub fn vertex_attrib4d(&self, index: u32, x: f64, y: f64, z: f64, w: f64) {
        (self.vertex_attrib4d_p)(index, x, y, z, w)
    }

    pub fn vertex_attrib4dv(&self, index: u32, v: *const f64) {
        (self.vertex_attrib4dv_p)(index, v)
    }

    pub fn vertex_attrib4f(&self, index: u32, x: f32, y: f32, z: f32, w: f32) {
        (self.vertex_attrib4f_p)(index, x, y, z, w)
    }

    pub fn vertex_attrib4fv(&self, index: u32, v: *const f32) {
        (self.vertex_attrib4fv_p)(index, v)
    }

    pub fn vertex_attrib4iv(&self, index: u32, v: *const i32) {
        (self.vertex_attrib4iv_p)(index, v)
    }

    pub fn vertex_attrib4s(&self, index: u32, x: i16, y: i16, z: i16, w: i16) {
        (self.vertex_attrib4s_p)(index, x, y, z, w)
    }

    pub fn vertex_attrib4sv(&self, index: u32, v: *const i16) {
        (self.vertex_attrib4sv_p)(index, v)
    }

    pub fn vertex_attrib4ubv(&self, index: u32, v: *const u8) {
        (self.vertex_attrib4ubv_p)(index, v)
    }

    pub fn vertex_attrib4uiv(&self, index: u32, v: *const u32) {
        (self.vertex_attrib4uiv_p)(index, v)
    }

    pub fn vertex_attrib4usv(&self, index: u32, v: *const u16) {
        (self.vertex_attrib4usv_p)(index, v)
    }

    pub fn vertex_attrib_binding(&self, attribindex: u32, bindingindex: u32) {
        (self.vertex_attrib_binding_p)(attribindex, bindingindex)
    }

    pub fn vertex_attrib_divisor(&self, index: u32, divisor: u32) {
        (self.vertex_attrib_divisor_p)(index, divisor)
    }

    pub fn vertex_attrib_format(
        &self,
        attribindex: u32,
        size: i32,
        type_: enums::VertexAttribType,
        normalized: bool,
        relativeoffset: u32,
    ) {
        (self.vertex_attrib_format_p)(
            attribindex,
            size,
            type_ as u32,
            normalized as u8,
            relativeoffset,
        )
    }

    pub fn vertex_attrib_i1i(&self, index: u32, x: i32) {
        (self.vertex_attrib_i1i_p)(index, x)
    }

    pub fn vertex_attrib_i1iv(&self, index: u32, v: *const i32) {
        (self.vertex_attrib_i1iv_p)(index, v)
    }

    pub fn vertex_attrib_i1ui(&self, index: u32, x: u32) {
        (self.vertex_attrib_i1ui_p)(index, x)
    }

    pub fn vertex_attrib_i1uiv(&self, index: u32, v: *const u32) {
        (self.vertex_attrib_i1uiv_p)(index, v)
    }

    pub fn vertex_attrib_i2i(&self, index: u32, x: i32, y: i32) {
        (self.vertex_attrib_i2i_p)(index, x, y)
    }

    pub fn vertex_attrib_i2iv(&self, index: u32, v: *const i32) {
        (self.vertex_attrib_i2iv_p)(index, v)
    }

    pub fn vertex_attrib_i2ui(&self, index: u32, x: u32, y: u32) {
        (self.vertex_attrib_i2ui_p)(index, x, y)
    }

    pub fn vertex_attrib_i2uiv(&self, index: u32, v: *const u32) {
        (self.vertex_attrib_i2uiv_p)(index, v)
    }

    pub fn vertex_attrib_i3i(&self, index: u32, x: i32, y: i32, z: i32) {
        (self.vertex_attrib_i3i_p)(index, x, y, z)
    }

    pub fn vertex_attrib_i3iv(&self, index: u32, v: *const i32) {
        (self.vertex_attrib_i3iv_p)(index, v)
    }

    pub fn vertex_attrib_i3ui(&self, index: u32, x: u32, y: u32, z: u32) {
        (self.vertex_attrib_i3ui_p)(index, x, y, z)
    }

    pub fn vertex_attrib_i3uiv(&self, index: u32, v: *const u32) {
        (self.vertex_attrib_i3uiv_p)(index, v)
    }

    pub fn vertex_attrib_i4bv(&self, index: u32, v: *const i8) {
        (self.vertex_attrib_i4bv_p)(index, v)
    }

    pub fn vertex_attrib_i4i(&self, index: u32, x: i32, y: i32, z: i32, w: i32) {
        (self.vertex_attrib_i4i_p)(index, x, y, z, w)
    }

    pub fn vertex_attrib_i4iv(&self, index: u32, v: *const i32) {
        (self.vertex_attrib_i4iv_p)(index, v)
    }

    pub fn vertex_attrib_i4sv(&self, index: u32, v: *const i16) {
        (self.vertex_attrib_i4sv_p)(index, v)
    }

    pub fn vertex_attrib_i4ubv(&self, index: u32, v: *const u8) {
        (self.vertex_attrib_i4ubv_p)(index, v)
    }

    pub fn vertex_attrib_i4ui(&self, index: u32, x: u32, y: u32, z: u32, w: u32) {
        (self.vertex_attrib_i4ui_p)(index, x, y, z, w)
    }

    pub fn vertex_attrib_i4uiv(&self, index: u32, v: *const u32) {
        (self.vertex_attrib_i4uiv_p)(index, v)
    }

    pub fn vertex_attrib_i4usv(&self, index: u32, v: *const u16) {
        (self.vertex_attrib_i4usv_p)(index, v)
    }

    pub fn vertex_attrib_i_format(
        &self,
        attribindex: u32,
        size: i32,
        type_: enums::VertexAttribIType,
        relativeoffset: u32,
    ) {
        (self.vertex_attrib_i_format_p)(attribindex, size, type_ as u32, relativeoffset)
    }

    pub fn vertex_attrib_i_pointer(
        &self,
        index: u32,
        size: i32,
        type_: enums::VertexAttribIType,
        stride: i32,
        pointer: *const c_void,
    ) {
        (self.vertex_attrib_i_pointer_p)(index, size, type_ as u32, stride, pointer)
    }

    pub fn vertex_attrib_p1ui(
        &self,
        index: u32,
        type_: enums::VertexAttribPointerType,
        normalized: bool,
        value: u32,
    ) {
        (self.vertex_attrib_p1ui_p)(index, type_ as u32, normalized as u8, value)
    }

    pub fn vertex_attrib_p1uiv(
        &self,
        index: u32,
        type_: enums::VertexAttribPointerType,
        normalized: bool,
        value: *const u32,
    ) {
        (self.vertex_attrib_p1uiv_p)(index, type_ as u32, normalized as u8, value)
    }

    pub fn vertex_attrib_p2ui(
        &self,
        index: u32,
        type_: enums::VertexAttribPointerType,
        normalized: bool,
        value: u32,
    ) {
        (self.vertex_attrib_p2ui_p)(index, type_ as u32, normalized as u8, value)
    }

    pub fn vertex_attrib_p2uiv(
        &self,
        index: u32,
        type_: enums::VertexAttribPointerType,
        normalized: bool,
        value: *const u32,
    ) {
        (self.vertex_attrib_p2uiv_p)(index, type_ as u32, normalized as u8, value)
    }

    pub fn vertex_attrib_p3ui(
        &self,
        index: u32,
        type_: enums::VertexAttribPointerType,
        normalized: bool,
        value: u32,
    ) {
        (self.vertex_attrib_p3ui_p)(index, type_ as u32, normalized as u8, value)
    }

    pub fn vertex_attrib_p3uiv(
        &self,
        index: u32,
        type_: enums::VertexAttribPointerType,
        normalized: bool,
        value: *const u32,
    ) {
        (self.vertex_attrib_p3uiv_p)(index, type_ as u32, normalized as u8, value)
    }

    pub fn vertex_attrib_p4ui(
        &self,
        index: u32,
        type_: enums::VertexAttribPointerType,
        normalized: bool,
        value: u32,
    ) {
        (self.vertex_attrib_p4ui_p)(index, type_ as u32, normalized as u8, value)
    }

    pub fn vertex_attrib_p4uiv(
        &self,
        index: u32,
        type_: enums::VertexAttribPointerType,
        normalized: bool,
        value: *const u32,
    ) {
        (self.vertex_attrib_p4uiv_p)(index, type_ as u32, normalized as u8, value)
    }

    pub fn vertex_attrib_pointer(
        &self,
        index: u32,
        size: i32,
        type_: enums::VertexAttribPointerType,
        normalized: bool,
        stride: i32,
        pointer: *const c_void,
    ) {
        (self.vertex_attrib_pointer_p)(index, size, type_ as u32, normalized as u8, stride, pointer)
    }

    pub fn vertex_binding_divisor(&self, bindingindex: u32, divisor: u32) {
        (self.vertex_binding_divisor_p)(bindingindex, divisor)
    }

    pub fn vertex_p2ui(&self, type_: enums::VertexPointerType, value: u32) {
        (self.vertex_p2ui_p)(type_ as u32, value)
    }

    pub fn vertex_p2uiv(&self, type_: enums::VertexPointerType, value: *const u32) {
        (self.vertex_p2uiv_p)(type_ as u32, value)
    }

    pub fn vertex_p3ui(&self, type_: enums::VertexPointerType, value: u32) {
        (self.vertex_p3ui_p)(type_ as u32, value)
    }

    pub fn vertex_p3uiv(&self, type_: enums::VertexPointerType, value: *const u32) {
        (self.vertex_p3uiv_p)(type_ as u32, value)
    }

    pub fn vertex_p4ui(&self, type_: enums::VertexPointerType, value: u32) {
        (self.vertex_p4ui_p)(type_ as u32, value)
    }

    pub fn vertex_p4uiv(&self, type_: enums::VertexPointerType, value: *const u32) {
        (self.vertex_p4uiv_p)(type_ as u32, value)
    }

    pub fn vertex_pointer(
        &self,
        size: i32,
        type_: enums::VertexPointerType,
        stride: i32,
        pointer: *const c_void,
    ) {
        (self.vertex_pointer_p)(size, type_ as u32, stride, pointer)
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        (self.viewport_p)(x, y, width, height)
    }

    pub fn wait_sync(&self, sync: *mut c_void, flags: enums::SyncBehaviorFlags, timeout: u64) {
        (self.wait_sync_p)(sync, flags as u32, timeout)
    }
}
