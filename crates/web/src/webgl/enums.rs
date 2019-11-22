pub enum WebGlVersion {
    One,
    Two,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum DataType {
    Byte = 0x1400,
    UnsignedByte = 0x1401,
    Short = 0x1402,
    UnsignedShort = 0x1403,
    Int = 0x1404,
    UnsignedInt = 0x1405,
    Float = 0x1406,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum BufferTarget {
    ArrayBuffer = 0x8892,
    ElementArrayBuffer = 0x8893,
    //webgl 2 only
    UniformBuffer = 0x8A11,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum BufferUsage {
    StreamDraw = 0x88E0,
    StaticDraw = 0x88E4,
    DynamicDraw = 0x88E8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum BeginMode {
    Points = 0x0000,
    Lines = 0x0001,
    LineLoop = 0x0002,
    LineStrip = 0x0003,
    Triangles = 0x0004,
    TriangleStrip = 0x0005,
    TriangleFan = 0x0006,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
//All of them - even though we have API's to use some separately too
//For example awsm_texture_set_wrap(), awsm_texture_set_mag_filter() &
//awsm_texture_set_min_filter()
pub enum TextureParameterName {
    MagFilter = 0x2800,
    MinFilter = 0x2801,
    WrapS = 0x2802,
    WrapT = 0x2803,
    WrapR = 0x8072,
    MinLod = 0x813A,
    MaxLod = 0x813B,
    BaseLevel = 0x813C,
    MaxLevel = 0x813D,
    CompareMode = 0x884C,
    CompareFunc = 0x884D,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureWrapTarget {
    S = 0x2802,
    T = 0x2803,
    R = 0x8072,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureWrapMode {
    Repeat = 0x2901,
    ClampToEdge = 0x812F,
    MirroredRepeat = 0x8370,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureMagFilter {
    Nearest = 0x2600,
    Linear = 0x2601,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureMinFilter {
    Nearest = 0x2600,
    Linear = 0x2601,
    NearestMipMapNearest = 0x2700,
    LinearMipMapNearest = 0x2701,
    NearestMipMapLinear = 0x2702,
    LinearMipMapLinear = 0x2703,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureTarget {
    Texture2d = 0x0DE1,
    Texture3d = 0x806F,
    Array2d = 0x8C1A,
    CubeMap = 0x8513,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureCubeFace {
    PositiveX = 0x8515,
    NegativeX = 0x8516,
    PositiveY = 0x8517,
    NegativeY = 0x8518,
    PositiveZ = 0x8519,
    NegativeZ = 0x851A,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureQuery {
    //Not actually totally sure that these are queries - but looks like it?
    Array2d = 0x8C1D,
    Texture = 0x1702,
    Binding3d = 0x806A,
    BindingCubeMap = 0x8514,
    MaxCubeTextureSize = 0x851C,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureUnit {
    Texture0 = 0x84C0,
    Texture1 = 0x84C1,
    Texture2 = 0x84C2,
    Texture3 = 0x84C3,
    Texture4 = 0x84C4,
    Texture5 = 0x84C5,
    Texture6 = 0x84C6,
    Texture7 = 0x84C7,
    Texture8 = 0x84C8,
    Texture9 = 0x84C9,
    Texture10 = 0x84CA,
    Texture11 = 0x84CB,
    Texture12 = 0x84CC,
    Texture13 = 0x84CD,
    Texture14 = 0x84CE,
    Texture15 = 0x84CF,
    Texture16 = 0x84D0,
    Texture17 = 0x84D1,
    Texture18 = 0x84D2,
    Texture19 = 0x84D3,
    Texture20 = 0x84D4,
    Texture21 = 0x84D5,
    Texture22 = 0x84D6,
    Texture23 = 0x84D7,
    Texture24 = 0x84D8,
    Texture25 = 0x84D9,
    Texture26 = 0x84DA,
    Texture27 = 0x84DB,
    Texture28 = 0x84DC,
    Texture29 = 0x84DD,
    Texture30 = 0x84DE,
    Texture31 = 0x84DF,
    ActiveTexture = 0x84E0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum PixelFormat {
    //WebGL1 and 2
    Alpha = 0x1906,
    Rgb = 0x1907,
    Rgba = 0x1908,
    Luminance = 0x1909,
    LuminanceAlpha = 0x190A,

    //When using the WEBGL_depth_texture extension
    DepthComponent = 0x1902,
    DepthStencil = 0x84F9,

    //When using the SRGB extension
    //SrgbExt = 0x8C40, //- same as Srgb for webgl2
    SrgbAlphaExt = 0x8C42,

    //WebGL2 only
    R8 = 0x8229,
    Rg8 = 0x822B,
    R16f = 0x822D,
    R32f = 0x822E,
    RG16f = 0x822F,
    RG32f = 0x8230,
    R8i = 0x8231,
    R8ui = 0x8232,
    R16i = 0x8233,
    R16ui = 0x8234,
    R32i = 0x8235,
    R32ui = 0x8236,
    RG8i = 0x8237,
    RG8ui = 0x8238,
    RG16i = 0x8239,
    RG16ui = 0x823A,
    RG32i = 0x823B,
    RG32ui = 0x823C,
    Srgb = 0x8C40,
    Srgb8 = 0x8C41,
    Srgb8Alpha8 = 0x8C43,
    Rgba32f = 0x8814,
    Rgb32f = 0x8815,
    Rgba16f = 0x881A,
    Rgb16f = 0x881B,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ClearBufferMask {
    DepthBufferBit = 0x00000100,
    StencilBufferBit = 0x00000400,
    ColorBufferBit = 0x00004000,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum GlToggle {
    Blend = 0x0BE2,
    CullFace = 0x0B44,
    DepthTest = 0x0B71,
    Dither = 0x0BD0,
    PolygonOffsetFill = 0x8037,
    SampleAlphaToCoverage = 0x809E,
    SampleCoverage = 0x80A0,
    ScissorTest = 0x0C11,
    StencilTest = 0x0B90,
    RasterizerDiscard = 0x8C89,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum GlQuery {
    FragmentShader = 0x8B30,
    VertexShader = 0x8B31,
    MaxVertexAttribs = 0x8869,
    MaxVertexUniformVectors = 0x8DFB,
    MaxVaryingVectors = 0x8DFC,
    MaxCombinedTextureImageUnits = 0x8B4D,
    MaxVertexTextureImageUnits = 0x8B4C,
    MaxTextureImageUnits = 0x8872,
    MaxFragmentUniformVectors = 0x8DFD,
    ShadingLanguageVersion = 0x8B8C,
    CurrentProgram = 0x8B8D,
    BlendColor = 0x8005,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum CmpFunction {
    Never = 0x0200,
    Less = 0x0201,
    Equal = 0x0202,
    Lequal = 0x0203,
    Greater = 0x0204,
    NotEqual = 0x0205,
    Gequal = 0x0206,
    Always = 0x0207,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum BlendEquation {
    Add = 0x8006,
    Subtract = 0x800A,
    ReverseSubtract = 0x800B,
    Min = 0x8007,
    Max = 0x8008,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum BlendFactor {
    Zero = 0,
    One = 1,
    SrcColor = 0x0300,
    OneMinusSrcColor = 0x0301,
    DstColor = 0x0306,
    OneMinusDstColor = 0x0307,
    SrcAlpha = 0x0302,
    OneMinusSrcAlpha = 0x0303,
    DstAlpha = 0x0304,
    OneMinusDstAlpha = 0x0305,
    ConstantColor = 0x8001,
    OneMinusConstantColor = 0x8002,
    ConstantAlpha = 0x8003,
    OneMinusConstantAlpha = 0x8004,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum UniformBlockQuery {
    BindingPoint = 0x8A3F,
    DataSize = 0x8A40,
    ActiveUniforms = 0x8A42,
    ActiveUniformIndices = 0x8A43,
    ReferencedByVertexShader = 0x8A44,
    ReferencedByFragmentShader = 0x8A46,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum UniformBlockActiveQuery {
    Type = 0x8A37,
    Size = 0x8A38,
    BlockIndex = 0x8A3A,
    Offset = 0x8A3B,
    ArrayStride = 0x8A3C,
    MatrixStride = 0x8A3D,
    IsRowMajor = 0x8A3E,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum WebGlSpecific {
    UnpackFlipY = 0x9240,
    UnpackPremultiplyAlpha = 0x9241,
    ContextLost = 0x9242,
    UnpackColorspaceConversion = 0x9243,
    BrowserDefault = 0x9244,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum UniformDataType {
    FloatVec2 = 0x8B50,
    FloatVec3 = 0x8B51,
    FloatVec4 = 0x8B52,
    IntVec2 = 0x8B53,
    IntVec3 = 0x8B54,
    IntVec4 = 0x8B55,
    Bool = 0x8B56,
    BoolVec2 = 0x8B57,
    BoolVec3 = 0x8B58,
    BoolVec4 = 0x8B59,
    FloatMat2 = 0x8B5A,
    FloatMat3 = 0x8B5B,
    FloatMat4 = 0x8B5C,
    Sampler2d = 0x8B5E,
    SamplerCube = 0x8B60,

    //WebGL2 only
    Sampler3d = 0x8B5F,
    Sampler2dShadow = 0x8B62,
    SamplerCubeShadow = 0x8DC5,
    Sampler2dArray = 0x8DC1,
    Sampler2dArrayShadow = 0x8DC4,
    IntSampler2d = 0x8DCA,
    IntSampler3d = 0x8DCB,
    IntSamplerCube = 0x8DCC,
    IntSampler2dArray = 0x8DCF,
    UnsignedIntSampler2d = 0x8DD2,
    UnsignedIntSampler3d = 0x8DD3,
    UnsignedIntSamplerCube = 0x8DD4,
    UnsignedIntSampler2dArray = 0x8DD7,

    //WEBGL_depth_texture extension
    UnsignedInt24_8 = 0x84FA,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ShaderQuery {
    DeleteStatus = 0x8B80,
    CompileStatus = 0x8B81,
    ShaderType = 0x8B4F,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ShaderType {
    Fragment = 0x8B30,
    Vertex = 0x8B31,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ProgramQuery {
    DeleteStatus = 0x8B80,
    LinkStatus = 0x8B82,
    ValidateStatus = 0x8B83,
    AttachedShaders = 0x8B85,
    ActiveUniforms = 0x8B86,
    ActiveAttributes = 0x8B89,
    TransformFeedbackBufferMode = 0x8C7F,
    TransformFeedbackVaryings = 0x8C83,
    ActiveUniformBlocks = 0x8A36,
}
/*
 * NOTE - all the below are copy/pasted from the WebIDL
 * If they're implemented above, they are DELETED below!!
 * Ideally we'd convert all these over...
 */
/* more buffer objects?
ARRAY_BUFFER_BINDING           = 0x8894,
ELEMENT_ARRAY_BUFFER_BINDING   = 0x8895,

const GLenum BUFFER_SIZE                    = 0x8764;
const GLenum BUFFER_USAGE                   = 0x8765;

const GLenum CURRENT_VERTEX_ATTRIB          = 0x8626;
*/
/*
 *
 * TODO - convert all these??


    * AlphaFunction (not supported in ES20) *
    *      NEVER *
    *      LESS *
    *      EQUAL *
    *      LEQUAL *
    *      GREATER *
    *      NOTEQUAL *
    *      GEQUAL *
    *      ALWAYS *

    * BlendingFactorDest *
ZERO                           = 0;
ONE                            = 1;
SRC_COLOR                      = 0x0300;
ONE_MINUS_SRC_COLOR            = 0x0301;
SRC_ALPHA                      = 0x0302;
ONE_MINUS_SRC_ALPHA            = 0x0303;
DST_ALPHA                      = 0x0304;
ONE_MINUS_DST_ALPHA            = 0x0305;

    * BlendingFactorSrc *
    *      ZERO *
    *      ONE *
    const GLenum DST_COLOR                      = 0x0306;
    const GLenum ONE_MINUS_DST_COLOR            = 0x0307;
    const GLenum SRC_ALPHA_SATURATE             = 0x0308;
    *      SRC_ALPHA *
    *      ONE_MINUS_SRC_ALPHA *
    *      DST_ALPHA *
    *&      ONE_MINUS_DST_ALPHA *

    * BlendEquationSeparate *
    const GLenum FUNC_ADD                       =& 0x8006;
    const GLenum BLEND_EQUATION                 = 0x8009;
    const GLenum BLEND_EQUATION_RGB             = 0x8009;   * same as BLEND_EQUATION *
    const GLenum BLEND_EQUATION_ALPHA           = 0x883D;

    * BlendSubtract *
    const GLenum FUNC_SUBTRACT                  = 0x800A;
    const GLenum FUNC_REVERSE_SUBTRACT          = 0x800B;

    * Separate Blend Functions *
    const GLenum BLEND_DST_RGB                  = 0x80C8;
    const GLenum BLEND_SRC_RGB                  = 0x80C9;
    const GLenum BLEND_DST_ALPHA                = 0x80CA;
    const GLenum BLEND_SRC_ALPHA                = 0x80CB;
    const GLenum CONSTANT_COLOR                 = 0x8001;
    const GLenum ONE_MINUS_CONSTANT_COLOR       = 0x8002;
    const GLenum CONSTANT_ALPHA                 = 0x8003;
    const GLenum ONE_MINUS_CONSTANT_ALPHA       = 0x8004;
    const GLenum BLEND_COLOR                    = 0x8005;


    * CullFaceMode *
    const GLenum FRONT                          = 0x0404;
    const GLenum BACK                           = 0x0405;
    const GLenum FRONT_AND_BACK                 = 0x0408;

    * EnableCap *
    * TEXTURE_2D *

    * ErrorCode *
    const GLenum NO_ERROR                       = 0;
    const GLenum INVALID_ENUM                   = 0x0500;
    const GLenum INVALID_VALUE                  = 0x0501;
    const GLenum INVALID_OPERATION              = 0x0502;
    const GLenum OUT_OF_MEMORY                  = 0x0505;

    * FrontFaceDirection *
    const GLenum CW                             = 0x0900;
    const GLenum CCW                            = 0x0901;

    * GetPName *
    const GLenum LINE_WIDTH                     = 0x0B21;
    const GLenum ALIASED_POINT_SIZE_RANGE       = 0x846D;
    const GLenum ALIASED_LINE_WIDTH_RANGE       = 0x846E;
    const GLenum CULL_FACE_MODE                 = 0x0B45;
    const GLenum FRONT_FACE                     = 0x0B46;
    const GLenum DEPTH_RANGE                    = 0x0B70;
    const GLenum DEPTH_WRITEMASK                = 0x0B72;
    const GLenum DEPTH_CLEAR_VALUE              = 0x0B73;
    const GLenum DEPTH_FUNC                     = 0x0B74;
    const GLenum STENCIL_CLEAR_VALUE            = 0x0B91;
    const GLenum STENCIL_FUNC                   = 0x0B92;
    const GLenum STENCIL_FAIL                   = 0x0B94;
    const GLenum STENCIL_PASS_DEPTH_FAIL        = 0x0B95;
    const GLenum STENCIL_PASS_DEPTH_PASS        = 0x0B96;
    const GLenum STENCIL_REF                    = 0x0B97;
    const GLenum STENCIL_VALUE_MASK             = 0x0B93;
    const GLenum STENCIL_WRITEMASK              = 0x0B98;
    const GLenum STENCIL_BACK_FUNC              = 0x8800;
    const GLenum STENCIL_BACK_FAIL              = 0x8801;
    const GLenum STENCIL_BACK_PASS_DEPTH_FAIL   = 0x8802;
    const GLenum STENCIL_BACK_PASS_DEPTH_PASS   = 0x8803;
    const GLenum STENCIL_BACK_REF               = 0x8CA3;
    const GLenum STENCIL_BACK_VALUE_MASK        = 0x8CA4;
    const GLenum STENCIL_BACK_WRITEMASK         = 0x8CA5;
    const GLenum VIEWPORT                       = 0x0BA2;
    const GLenum SCISSOR_BOX                    = 0x0C10;
    *      SCISSOR_TEST *
    const GLenum COLOR_CLEAR_VALUE              = 0x0C22;
    const GLenum COLOR_WRITEMASK                = 0x0C23;
    const GLenum UNPACK_ALIGNMENT               = 0x0CF5;
    const GLenum PACK_ALIGNMENT                 = 0x0D05;
    const GLenum MAX_TEXTURE_SIZE               = 0x0D33;
    const GLenum MAX_VIEWPORT_DIMS              = 0x0D3A;
    const GLenum SUBPIXEL_BITS                  = 0x0D50;
    const GLenum RED_BITS                       = 0x0D52;
    const GLenum GREEN_BITS                     = 0x0D53;
    const GLenum BLUE_BITS                      = 0x0D54;
    const GLenum ALPHA_BITS                     = 0x0D55;
    const GLenum DEPTH_BITS                     = 0x0D56;
    const GLenum STENCIL_BITS                   = 0x0D57;
    const GLenum POLYGON_OFFSET_UNITS           = 0x2A00;
    *      POLYGON_OFFSET_FILL *
    const GLenum POLYGON_OFFSET_FACTOR          = 0x8038;
    const GLenum TEXTURE_BINDING_2D             = 0x8069;
    const GLenum SAMPLE_BUFFERS                 = 0x80A8;
    const GLenum SAMPLES                        = 0x80A9;
    const GLenum SAMPLE_COVERAGE_VALUE          = 0x80AA;
    const GLenum SAMPLE_COVERAGE_INVERT         = 0x80AB;


    const GLenum COMPRESSED_TEXTURE_FORMATS     = 0x86A3;

    * HintMode *
    const GLenum DONT_CARE                      = 0x1100;
    const GLenum FASTEST                        = 0x1101;
    const GLenum NICEST                         = 0x1102;

    * HintTarget *
    const GLenum GENERATE_MIPMAP_HINT            = 0x8192;



    * PixelType *
    *      UNSIGNED_BYTE *
    const GLenum UNSIGNED_SHORT_4_4_4_4         = 0x8033;
    const GLenum UNSIGNED_SHORT_5_5_5_1         = 0x8034;
    const GLenum UNSIGNED_SHORT_5_6_5           = 0x8363;


    * StencilFunction *
    const GLenum NEVER                          = 0x0200;
    const GLenum LESS                           = 0x0201;
    const GLenum EQUAL                          = 0x0202;
    const GLenum LEQUAL                         = 0x0203;
    const GLenum GREATER                        = 0x0204;
    const GLenum NOTEQUAL                       = 0x0205;
    const GLenum GEQUAL                         = 0x0206;
    const GLenum ALWAYS                         = 0x0207;

    * StencilOp *
    *      ZERO *
    const GLenum KEEP                           = 0x1E00;
    const GLenum REPLACE                        = 0x1E01;
    const GLenum INCR                           = 0x1E02;
    const GLenum DECR                           = 0x1E03;
    const GLenum INVERT                         = 0x150A;
    const GLenum INCR_WRAP                      = 0x8507;
    const GLenum DECR_WRAP                      = 0x8508;

    * StringName *
    const GLenum VENDOR                         = 0x1F00;
    const GLenum RENDERER                       = 0x1F01;
    const GLenum VERSION                        = 0x1F02;


    * Vertex Arrays *
    const GLenum VERTEX_ATTRIB_ARRAY_ENABLED        = 0x8622;
    const GLenum VERTEX_ATTRIB_ARRAY_SIZE           = 0x8623;
    const GLenum VERTEX_ATTRIB_ARRAY_STRIDE         = 0x8624;
    const GLenum VERTEX_ATTRIB_ARRAY_TYPE           = 0x8625;
    const GLenum VERTEX_ATTRIB_ARRAY_NORMALIZED     = 0x886A;
    const GLenum VERTEX_ATTRIB_ARRAY_POINTER        = 0x8645;
    const GLenum VERTEX_ATTRIB_ARRAY_BUFFER_BINDING = 0x889F;

    * Read Format *
    const GLenum IMPLEMENTATION_COLOR_READ_TYPE   = 0x8B9A;
    const GLenum IMPLEMENTATION_COLOR_READ_FORMAT = 0x8B9B;


    * Shader Precision-Specified Types *
    const GLenum LOW_FLOAT                      = 0x8DF0;
    const GLenum MEDIUM_FLOAT                   = 0x8DF1;
    const GLenum HIGH_FLOAT                     = 0x8DF2;
    const GLenum LOW_INT                        = 0x8DF3;
    const GLenum MEDIUM_INT                     = 0x8DF4;
    const GLenum HIGH_INT                       = 0x8DF5;

    * Framebuffer Object. *
    const GLenum FRAMEBUFFER                    = 0x8D40;
    const GLenum RENDERBUFFER                   = 0x8D41;

    const GLenum RGBA4                          = 0x8056;
    const GLenum RGB5_A1                        = 0x8057;
    const GLenum RGB565                         = 0x8D62;
    const GLenum DEPTH_COMPONENT16              = 0x81A5;
    const GLenum STENCIL_INDEX8                 = 0x8D48;
    const GLenum DEPTH_STENCIL                  = 0x84F9;

    const GLenum RENDERBUFFER_WIDTH             = 0x8D42;
    const GLenum RENDERBUFFER_HEIGHT            = 0x8D43;
    const GLenum RENDERBUFFER_INTERNAL_FORMAT   = 0x8D44;
    const GLenum RENDERBUFFER_RED_SIZE          = 0x8D50;
    const GLenum RENDERBUFFER_GREEN_SIZE        = 0x8D51;
    const GLenum RENDERBUFFER_BLUE_SIZE         = 0x8D52;
    const GLenum RENDERBUFFER_ALPHA_SIZE        = 0x8D53;
    const GLenum RENDERBUFFER_DEPTH_SIZE        = 0x8D54;
    const GLenum RENDERBUFFER_STENCIL_SIZE      = 0x8D55;

    const GLenum FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE           = 0x8CD0;
    const GLenum FRAMEBUFFER_ATTACHMENT_OBJECT_NAME           = 0x8CD1;
    const GLenum FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL         = 0x8CD2;
    const GLenum FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE = 0x8CD3;

    const GLenum COLOR_ATTACHMENT0              = 0x8CE0;
    const GLenum DEPTH_ATTACHMENT               = 0x8D00;
    const GLenum STENCIL_ATTACHMENT             = 0x8D20;
    const GLenum DEPTH_STENCIL_ATTACHMENT       = 0x821A;

    const GLenum NONE                           = 0;

    const GLenum FRAMEBUFFER_COMPLETE                      = 0x8CD5;
    const GLenum FRAMEBUFFER_INCOMPLETE_ATTACHMENT         = 0x8CD6;
    const GLenum FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT = 0x8CD7;
    const GLenum FRAMEBUFFER_INCOMPLETE_DIMENSIONS         = 0x8CD9;
    const GLenum FRAMEBUFFER_UNSUPPORTED                   = 0x8CDD;

    const GLenum FRAMEBUFFER_BINDING            = 0x8CA6;
    const GLenum RENDERBUFFER_BINDING           = 0x8CA7;
    const GLenum MAX_RENDERBUFFER_SIZE          = 0x84E8;

    const GLenum INVALID_FRAMEBUFFER_OPERATION  = 0x0506;

    */
