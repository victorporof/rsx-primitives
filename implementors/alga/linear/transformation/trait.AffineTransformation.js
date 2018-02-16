(function() {var implementors = {};
implementors["nalgebra"] = ["impl&lt;N, D:&nbsp;<a class=\"trait\" href=\"nalgebra/core/dimension/trait.DimName.html\" title=\"trait nalgebra::core::dimension::DimName\">DimName</a>, SA, SB&gt; <a class=\"trait\" href=\"alga/linear/transformation/trait.AffineTransformation.html\" title=\"trait alga::linear::transformation::AffineTransformation\">AffineTransformation</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.PointBase.html\" title=\"struct nalgebra::geometry::PointBase\">PointBase</a>&lt;N, D, SB&gt;&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.RotationBase.html\" title=\"struct nalgebra::geometry::RotationBase\">RotationBase</a>&lt;N, D, SA&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"alga/general/real/trait.Real.html\" title=\"trait alga::general::real::Real\">Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;SA: <a class=\"trait\" href=\"nalgebra/core/storage/trait.OwnedStorage.html\" title=\"trait nalgebra::core::storage::OwnedStorage\">OwnedStorage</a>&lt;N, D, D&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;SB: <a class=\"trait\" href=\"nalgebra/core/storage/trait.OwnedStorage.html\" title=\"trait nalgebra::core::storage::OwnedStorage\">OwnedStorage</a>&lt;N, D, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U1.html\" title=\"struct nalgebra::core::dimension::U1\">U1</a>, Alloc = SA::<a class=\"type\" href=\"nalgebra/core/storage/trait.Storage.html#associatedtype.Alloc\" title=\"type nalgebra::core::storage::Storage::Alloc\">Alloc</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;SA::<a class=\"type\" href=\"nalgebra/core/storage/trait.Storage.html#associatedtype.Alloc\" title=\"type nalgebra::core::storage::Storage::Alloc\">Alloc</a>: <a class=\"trait\" href=\"nalgebra/core/allocator/trait.OwnedAllocator.html\" title=\"trait nalgebra::core::allocator::OwnedAllocator\">OwnedAllocator</a>&lt;N, D, D, SA&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;SB::<a class=\"type\" href=\"nalgebra/core/storage/trait.Storage.html#associatedtype.Alloc\" title=\"type nalgebra::core::storage::Storage::Alloc\">Alloc</a>: <a class=\"trait\" href=\"nalgebra/core/allocator/trait.OwnedAllocator.html\" title=\"trait nalgebra::core::allocator::OwnedAllocator\">OwnedAllocator</a>&lt;N, D, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U1.html\" title=\"struct nalgebra::core::dimension::U1\">U1</a>, SB&gt;,&nbsp;</span>","impl&lt;N, SA, SB&gt; <a class=\"trait\" href=\"alga/linear/transformation/trait.AffineTransformation.html\" title=\"trait alga::linear::transformation::AffineTransformation\">AffineTransformation</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.PointBase.html\" title=\"struct nalgebra::geometry::PointBase\">PointBase</a>&lt;N, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U3.html\" title=\"struct nalgebra::core::dimension::U3\">U3</a>, SB&gt;&gt; for <a class=\"type\" href=\"nalgebra/geometry/type.UnitQuaternionBase.html\" title=\"type nalgebra::geometry::UnitQuaternionBase\">UnitQuaternionBase</a>&lt;N, SA&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"alga/general/real/trait.Real.html\" title=\"trait alga::general::real::Real\">Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;SA: <a class=\"trait\" href=\"nalgebra/core/storage/trait.OwnedStorage.html\" title=\"trait nalgebra::core::storage::OwnedStorage\">OwnedStorage</a>&lt;N, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U4.html\" title=\"struct nalgebra::core::dimension::U4\">U4</a>, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U1.html\" title=\"struct nalgebra::core::dimension::U1\">U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;SB: <a class=\"trait\" href=\"nalgebra/core/storage/trait.OwnedStorage.html\" title=\"trait nalgebra::core::storage::OwnedStorage\">OwnedStorage</a>&lt;N, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U3.html\" title=\"struct nalgebra::core::dimension::U3\">U3</a>, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U1.html\" title=\"struct nalgebra::core::dimension::U1\">U1</a>, Alloc = SA::<a class=\"type\" href=\"nalgebra/core/storage/trait.Storage.html#associatedtype.Alloc\" title=\"type nalgebra::core::storage::Storage::Alloc\">Alloc</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;SA::<a class=\"type\" href=\"nalgebra/core/storage/trait.Storage.html#associatedtype.Alloc\" title=\"type nalgebra::core::storage::Storage::Alloc\">Alloc</a>: <a class=\"trait\" href=\"nalgebra/core/allocator/trait.OwnedAllocator.html\" title=\"trait nalgebra::core::allocator::OwnedAllocator\">OwnedAllocator</a>&lt;N, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U4.html\" title=\"struct nalgebra::core::dimension::U4\">U4</a>, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U1.html\" title=\"struct nalgebra::core::dimension::U1\">U1</a>, SA&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;SB::<a class=\"type\" href=\"nalgebra/core/storage/trait.Storage.html#associatedtype.Alloc\" title=\"type nalgebra::core::storage::Storage::Alloc\">Alloc</a>: <a class=\"trait\" href=\"nalgebra/core/allocator/trait.OwnedAllocator.html\" title=\"trait nalgebra::core::allocator::OwnedAllocator\">OwnedAllocator</a>&lt;N, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U3.html\" title=\"struct nalgebra::core::dimension::U3\">U3</a>, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U1.html\" title=\"struct nalgebra::core::dimension::U1\">U1</a>, SB&gt;,&nbsp;</span>","impl&lt;N, S&gt; <a class=\"trait\" href=\"alga/linear/transformation/trait.AffineTransformation.html\" title=\"trait alga::linear::transformation::AffineTransformation\">AffineTransformation</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.PointBase.html\" title=\"struct nalgebra::geometry::PointBase\">PointBase</a>&lt;N, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U2.html\" title=\"struct nalgebra::core::dimension::U2\">U2</a>, S&gt;&gt; for <a class=\"type\" href=\"nalgebra/geometry/type.UnitComplex.html\" title=\"type nalgebra::geometry::UnitComplex\">UnitComplex</a>&lt;N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"alga/general/real/trait.Real.html\" title=\"trait alga::general::real::Real\">Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"nalgebra/core/storage/trait.OwnedStorage.html\" title=\"trait nalgebra::core::storage::OwnedStorage\">OwnedStorage</a>&lt;N, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U2.html\" title=\"struct nalgebra::core::dimension::U2\">U2</a>, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U1.html\" title=\"struct nalgebra::core::dimension::U1\">U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"nalgebra/core/storage/trait.Storage.html#associatedtype.Alloc\" title=\"type nalgebra::core::storage::Storage::Alloc\">Alloc</a>: <a class=\"trait\" href=\"nalgebra/core/allocator/trait.OwnedAllocator.html\" title=\"trait nalgebra::core::allocator::OwnedAllocator\">OwnedAllocator</a>&lt;N, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U2.html\" title=\"struct nalgebra::core::dimension::U2\">U2</a>, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U1.html\" title=\"struct nalgebra::core::dimension::U1\">U1</a>, S&gt;,&nbsp;</span>","impl&lt;N, D:&nbsp;<a class=\"trait\" href=\"nalgebra/core/dimension/trait.DimName.html\" title=\"trait nalgebra::core::dimension::DimName\">DimName</a>, S&gt; <a class=\"trait\" href=\"alga/linear/transformation/trait.AffineTransformation.html\" title=\"trait alga::linear::transformation::AffineTransformation\">AffineTransformation</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.PointBase.html\" title=\"struct nalgebra::geometry::PointBase\">PointBase</a>&lt;N, D, S&gt;&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.TranslationBase.html\" title=\"struct nalgebra::geometry::TranslationBase\">TranslationBase</a>&lt;N, D, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"alga/general/real/trait.Real.html\" title=\"trait alga::general::real::Real\">Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"nalgebra/core/storage/trait.OwnedStorage.html\" title=\"trait nalgebra::core::storage::OwnedStorage\">OwnedStorage</a>&lt;N, D, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U1.html\" title=\"struct nalgebra::core::dimension::U1\">U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"nalgebra/core/storage/trait.Storage.html#associatedtype.Alloc\" title=\"type nalgebra::core::storage::Storage::Alloc\">Alloc</a>: <a class=\"trait\" href=\"nalgebra/core/allocator/trait.OwnedAllocator.html\" title=\"trait nalgebra::core::allocator::OwnedAllocator\">OwnedAllocator</a>&lt;N, D, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U1.html\" title=\"struct nalgebra::core::dimension::U1\">U1</a>, S&gt;,&nbsp;</span>","impl&lt;N, D:&nbsp;<a class=\"trait\" href=\"nalgebra/core/dimension/trait.DimName.html\" title=\"trait nalgebra::core::dimension::DimName\">DimName</a>, S, R&gt; <a class=\"trait\" href=\"alga/linear/transformation/trait.AffineTransformation.html\" title=\"trait alga::linear::transformation::AffineTransformation\">AffineTransformation</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.PointBase.html\" title=\"struct nalgebra::geometry::PointBase\">PointBase</a>&lt;N, D, S&gt;&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.IsometryBase.html\" title=\"struct nalgebra::geometry::IsometryBase\">IsometryBase</a>&lt;N, D, S, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"alga/general/real/trait.Real.html\" title=\"trait alga::general::real::Real\">Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"nalgebra/core/storage/trait.OwnedStorage.html\" title=\"trait nalgebra::core::storage::OwnedStorage\">OwnedStorage</a>&lt;N, D, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U1.html\" title=\"struct nalgebra::core::dimension::U1\">U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"alga/linear/transformation/trait.Rotation.html\" title=\"trait alga::linear::transformation::Rotation\">Rotation</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.PointBase.html\" title=\"struct nalgebra::geometry::PointBase\">PointBase</a>&lt;N, D, S&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"nalgebra/core/storage/trait.Storage.html#associatedtype.Alloc\" title=\"type nalgebra::core::storage::Storage::Alloc\">Alloc</a>: <a class=\"trait\" href=\"nalgebra/core/allocator/trait.OwnedAllocator.html\" title=\"trait nalgebra::core::allocator::OwnedAllocator\">OwnedAllocator</a>&lt;N, D, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U1.html\" title=\"struct nalgebra::core::dimension::U1\">U1</a>, S&gt;,&nbsp;</span>","impl&lt;N, D:&nbsp;<a class=\"trait\" href=\"nalgebra/core/dimension/trait.DimName.html\" title=\"trait nalgebra::core::dimension::DimName\">DimName</a>, S, R&gt; <a class=\"trait\" href=\"alga/linear/transformation/trait.AffineTransformation.html\" title=\"trait alga::linear::transformation::AffineTransformation\">AffineTransformation</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.PointBase.html\" title=\"struct nalgebra::geometry::PointBase\">PointBase</a>&lt;N, D, S&gt;&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.SimilarityBase.html\" title=\"struct nalgebra::geometry::SimilarityBase\">SimilarityBase</a>&lt;N, D, S, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"alga/general/real/trait.Real.html\" title=\"trait alga::general::real::Real\">Real</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"nalgebra/core/storage/trait.OwnedStorage.html\" title=\"trait nalgebra::core::storage::OwnedStorage\">OwnedStorage</a>&lt;N, D, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U1.html\" title=\"struct nalgebra::core::dimension::U1\">U1</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"alga/linear/transformation/trait.Rotation.html\" title=\"trait alga::linear::transformation::Rotation\">Rotation</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.PointBase.html\" title=\"struct nalgebra::geometry::PointBase\">PointBase</a>&lt;N, D, S&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"nalgebra/core/storage/trait.Storage.html#associatedtype.Alloc\" title=\"type nalgebra::core::storage::Storage::Alloc\">Alloc</a>: <a class=\"trait\" href=\"nalgebra/core/allocator/trait.OwnedAllocator.html\" title=\"trait nalgebra::core::allocator::OwnedAllocator\">OwnedAllocator</a>&lt;N, D, <a class=\"struct\" href=\"nalgebra/core/dimension/struct.U1.html\" title=\"struct nalgebra::core::dimension::U1\">U1</a>, S&gt;,&nbsp;</span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
