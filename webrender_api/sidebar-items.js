initSidebarItems({"constant":[["MAX_TEXT_RUN_LENGTH",""],["YUV_COLOR_SPACES",""],["YUV_FORMATS",""]],"enum":[["AddFont",""],["AlphaType",""],["ApiMsg",""],["BlobImageError",""],["BorderDetails",""],["BorderRadiusKind",""],["BorderStyle",""],["BoxShadowClipMode",""],["ClipId",""],["ClipMode",""],["DebugCommand",""],["DocumentMsg",""],["ExtendMode",""],["ExternalImageType",""],["FilterOp",""],["FontHinting",""],["FontLCDFilter",""],["FontRenderMode",""],["FontTemplate",""],["ImageData",""],["ImageFormat",""],["ImageRendering",""],["LineOrientation",""],["LineStyle",""],["LocalClip",""],["MixBlendMode",""],["PropertyBinding","A binding property can either be a specific value (the normal, non-animated case) or point to a binding location to fetch the current value from."],["RepeatMode",""],["ResourceUpdate",""],["ScrollClamping",""],["ScrollEventPhase",""],["ScrollLocation",""],["ScrollNodeIdType",""],["ScrollPolicy",""],["ScrollSensitivity",""],["SpecificDisplayItem",""],["SubpixelDirection",""],["SubpixelOffset",""],["TextureTarget",""],["TransformStyle",""],["YuvColorSpace",""],["YuvData",""],["YuvFormat",""]],"fn":[["as_scroll_parent_rect",""],["as_scroll_parent_vector",""]],"mod":[["channel",""]],"struct":[["AddFontInstance",""],["AddImage",""],["AuxIter",""],["BlobImageDescriptor",""],["BlobImageRequest",""],["BorderDisplayItem",""],["BorderRadius",""],["BorderSide",""],["BorderWidths",""],["BoxShadowDisplayItem",""],["BuiltDisplayList","A display list."],["BuiltDisplayListDescriptor","Describes the memory layout of a display list."],["BuiltDisplayListIter",""],["CaptureBits","Bit flags for WR stages to store in a capture."],["CapturedDocument","Information about a loaded capture of each document that is returned by `RenderBackend`."],["ClearCache","Mask for clearing caches in debug commands."],["ClipAndScrollInfo",""],["ClipChainId",""],["ClipChainItem",""],["ClipDisplayItem",""],["ColorF","Represents RGBA screen colors with floating point numbers."],["ColorU","Represents RGBA screen colors with one byte per channel."],["ComplexClipRegion",""],["DevicePixel","Geometry in the coordinate system of the render target (screen or intermediate surface) in physical pixels."],["DisplayItemRef",""],["DisplayListBuilder",""],["DocumentId",""],["DynamicProperties","When using `generate_frame()`, a list of `PropertyValue` structures can optionally be supplied to provide the current value of any animated properties."],["Epoch",""],["ExternalEvent","An opaque pointer-sized value."],["ExternalImageData",""],["ExternalImageId","An arbitrary identifier for an external image provided by the application. It must be a unique identifier for each external image."],["ExternalScrollId","An external identifier that uniquely identifies a scroll frame independent of its ClipId, which may change from frame to frame. This should be unique within a pipeline. WebRender makes no attempt to ensure uniqueness. The zero value is reserved for use by the root scroll node of every pipeline, which always has an external id."],["FontInstanceFlags",""],["FontInstanceKey",""],["FontInstanceOptions",""],["FontInstancePlatformOptions",""],["FontKey",""],["FontVariation",""],["GenericDisplayItem","The DI is generic over the specifics, while allows to use the \"complete\" version of it for convenient serialization."],["GlyphDimensions",""],["GlyphInstance",""],["GlyphKey",""],["GlyphOptions",""],["Gradient",""],["GradientBorder",""],["GradientDisplayItem",""],["GradientStop",""],["HitTestFlags",""],["HitTestItem",""],["HitTestResult",""],["IdNamespace",""],["IframeDisplayItem",""],["ImageBorder",""],["ImageDescriptor",""],["ImageDisplayItem",""],["ImageKey",""],["ImageMask",""],["ItemRange",""],["LayerPixel","Geometry in a layer's local coordinate space (logical pixels)."],["LineDisplayItem",""],["NativeFontHandle",""],["NinePatchDescriptor",""],["NormalBorder",""],["PipelineId","From the point of view of WR, `PipelineId` is completely opaque and generic as long as it's clonable, serializable, comparable, and hashable."],["PremultipliedColorF","Represents pre-multiplied RGBA colors with floating point numbers."],["PrimitiveInfo",""],["PropertyBindingId",""],["PropertyBindingKey","A unique key that is used for connecting animated property values to bindings in the display list."],["PropertyValue","The current value of an animated property. This is supplied by the calling code."],["PushStackingContextDisplayItem",""],["RadialGradient",""],["RadialGradientBorder",""],["RadialGradientDisplayItem",""],["RasterizedBlobImage",""],["RectangleDisplayItem",""],["RenderApi",""],["RenderApiSender",""],["ResourceId",""],["ResourceUpdates","The resource updates for a given transaction (they must be applied in the same frame)."],["SaveState",""],["ScrollFrameDisplayItem",""],["ScrollLayerPixel","Geometry in a layer's scrollable parent coordinate space (logical pixels)."],["ScrollNodeState",""],["Shadow",""],["StackingContext",""],["StickyFrameDisplayItem",""],["StickyOffsetBounds","The minimum and maximum allowable offset for a sticky frame in a single dimension."],["TexelRect","Stores two coordinates in texel space. The coordinates are stored in texel coordinates because the texture atlas may grow. Storing them as texel coords and normalizing the UVs in the vertex shader means nothing needs to be updated on the CPU when the texture size changes."],["TextDisplayItem",""],["Tiles","Offset in number of tiles."],["Transaction","A Transaction is a group of commands to apply atomically to a document."],["UpdateImage",""],["WorldPixel","Geometry in the document's coordinate space (logical pixels)."],["YuvImageDisplayItem",""],["ZoomFactor","Represents a zoom factor."]],"trait":[["BlobImageRenderer",""],["BlobImageResources",""],["RenderNotifier",""]],"type":[["BlobImageData",""],["BlobImageResult",""],["DeviceIntLength",""],["DeviceIntPoint",""],["DeviceIntRect",""],["DeviceIntSize",""],["DevicePixelScale","Scaling ratio from world pixels to device pixels."],["DevicePoint",""],["DeviceRect",""],["DeviceSize",""],["DeviceUintPoint",""],["DeviceUintRect",""],["DeviceUintSize",""],["DeviceVector2D",""],["DisplayItem",""],["DocumentLayer","Documents are rendered in the ascending order of their associated layer values."],["GlyphIndex",""],["ItemTag","A tag that can be used to identify items during hit testing. If the tag is missing then the item doesn't take part in hit testing at all. This is composed of two numbers. In Servo, the first is an identifier while the second is used to select the cursor that should be used during mouse movement. In Gecko, the first is a scrollframe identifier, while the second is used to store various flags that APZ needs to properly process input events."],["LayerPoint",""],["LayerPoint3D",""],["LayerPointAu",""],["LayerPrimitiveInfo",""],["LayerRect",""],["LayerRectAu",""],["LayerSize",""],["LayerSizeAu",""],["LayerToScrollTransform",""],["LayerToWorldScale","Scaling ratio from layer to world. Used for cases where we know the layer is in world space, or specifically want to treat it this way."],["LayerToWorldTransform",""],["LayerTransform",""],["LayerVector2D",""],["LayerVector3D",""],["LayoutPixel","Geometry in a stacking context's local coordinate space (logical pixels)."],["LayoutPoint",""],["LayoutPrimitiveInfo",""],["LayoutRect",""],["LayoutSize",""],["LayoutTransform",""],["LayoutVector2D",""],["LayoutVector3D",""],["PipelineSourceId","This type carries no valuable semantics for WR. However, it reflects the fact that clients (Servo) may generate pipelines by different semi-independent sources. These pipelines still belong to the same `IdNamespace` and the same `DocumentId`. Having this extra Id field enables them to generate `PipelineId` without collision."],["ScrollLayerPoint",""],["ScrollLayerRect",""],["ScrollLayerSize",""],["ScrollLayerVector2D",""],["ScrollToLayerTransform",""],["ScrollToWorldTransform",""],["TileOffset",""],["TileSize",""],["WorldPoint",""],["WorldPoint3D",""],["WorldRect",""],["WorldSize",""],["WorldToLayerTransform",""],["WorldVector2D",""],["WorldVector3D",""]]});